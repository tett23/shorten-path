# ADR 0007: 外部クレートへの依存をなくす

ステータス: 採択

## 文脈

このコマンドが依存している外部クレートは `dirs` 3.0（`dirs-sys`、`libc`、`redox_users` などの推移的な依存を含め、ロックファイル上は12クレート）だけで、用途は `dirs::home_dir()` によるホームディレクトリの取得のみである。
運用の方針として、ライセンスに問題がなく再実装が容易なものは、依存に入れずに再実装する。

Rustの標準ライブラリには `std::env::home_dir()` がある。
かつてはWindowsでの挙動の問題から非推奨だったが、その後挙動が修正されて非推奨は解除された。
stable 1.99.0 で `-D warnings` を付けてコンパイルしても警告は出ない。

Unix系での両者の挙動は以下のとおりで、[ADR 0004](0004-binary-distribution.md) で配布しているLinuxとmacOSでは同じ結果になる。

| 状況 | `dirs::home_dir()`（`dirs-sys` 0.3.6） | `std::env::home_dir()`（1.99.0） |
| --- | --- | --- |
| `HOME` が空でない | `HOME` の値 | `HOME` の値 |
| `HOME` が空 | `getpwuid_r` によるパスワードデータベースの値 | 同左 |
| `HOME` が未設定 | 同上 | 同左 |

`std::env::home_dir()` の挙動は手元の1.99.0で `HOME` を設定した場合、空にした場合、未設定にした場合の三通りを実行して確認した。

## 実装すること

- `Cargo.toml` の `[dependencies]` から `dirs` を削除する。`Cargo.lock` には `shorten-path` だけが残る。
- `dirs::home_dir()` を `std::env::home_dir()` に置き換える。
- テストのため、`replace_home` がホームディレクトリを引数で受け取るようにする。環境から値を取るのは `main` だけにする。

  ```rust
  fn replace_home(buf: String, home: &str) -> String
  ```

  `rules` の要素は `Box<dyn Fn(String) -> String>` のままとし、`main` で取得したホームディレクトリをクロージャに捕捉させる。
- 変換の挙動、panicの条件とメッセージは変えない。
- `docs/specifications.md` の「ホームディレクトリの置換」の `dirs::home_dir()` を、上の取得規則（`HOME`、空または未設定ならパスワードデータベース）に書きかえる。

## 実装しないこと

- `getpwuid_r` などを呼ぶ独自のホームディレクトリ取得は実装しない。標準ライブラリで足りるため。
- Windowsでの挙動の互換性は確認しない。Windowsは配布対象外（[ADR 0004](0004-binary-distribution.md)）であり、`std::env::home_dir()` はWindowsで `dirs` と異なる取得方法を使う。
- ホームディレクトリの判定を前方一致からパス要素単位に直すなど、変換規則は変えない。
- 依存が増えないことをCIで検査する仕組みは入れない。

## テスト設計

### 単体テスト（`src/main.rs`）

既存の `shorten_dirs` のテストはそのまま残す。`replace_home` のテストは、ホームディレクトリを引数で渡す形に書きかえる。

- `replace_home("/home/alice/foo", "/home/alice")` → `~/foo`
- `replace_home("/home/alice", "/home/alice")` → `~`
- `replace_home("/usr/local/bin", "/home/alice")` → `/usr/local/bin`
- `replace_home("/home/alicefoo/bar", "/home/alice")` → `~foo/bar`（前方一致の現状の挙動を固定する）

### 結合テスト（`tests/cli.rs`）

`env!("CARGO_BIN_EXE_shorten-path")` でビルドしたバイナリを `std::process::Command` で実行し、`HOME` を明示して標準出力を確認する。テスト用のクレートは追加しない。

- `HOME=/home/alice` で `/home/alice/repositories/src/github.com` → `~/r/s/github.com`
- `HOME=/home/alice` で `/home/alice` → `~`
- `HOME=/home/alice` で `/usr/local/bin` → `/usr/local/bin`
- 出力の末尾に改行が無いこと

`HOME` が空または未設定の場合は、パスワードデータベースの内容が実行環境に依存するため、結合テストでは扱わない。

### 確認

- `cargo tree` で依存が `shorten-path` だけであることを確認する。
- CIの `check`（Ubuntu、macOS）が成功すること。
- 依存を外す前のバイナリと外した後のバイナリで、同じ入力に対する出力が一致することを手元で確認する。

## トレードオフ

- Windowsで `HOME` などの環境変数の扱いが `dirs` と異なりうるが、配布対象外なので受け入れる。Windowsを対象にする場合は別のADRで扱う。
- `std::env::home_dir()` が再び非推奨になった場合はCIのclippy（`-D warnings`）で検出できる。その時点で対応を別のADRで決める。
- `replace_home` の引数が変わるため、`rules` の組み立てにクロージャが一つ増える。代わりに、環境に依存しない純粋な関数としてテストできるようになる。
