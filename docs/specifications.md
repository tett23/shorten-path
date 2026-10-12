# shorten-path 仕様

このドキュメントは現時点の仕様を表す。仕様の変更はADR（[`docs/adr/`](adr/)）を経て行い、その都度このドキュメントを更新する。

## コマンド

```
shorten-path <PATH>
```

`PATH` を短縮した文字列を、末尾に改行を付けずに標準出力へ書く。
引数が複数ある場合は最後の引数を `PATH` として扱う。

### 変換規則

以下の規則を順に適用する。

1. ホームディレクトリの置換
   - `PATH` がホームディレクトリの文字列で始まる場合、その部分を `~` に置き換える。
   - ホームディレクトリは `std::env::home_dir()` で取得する。環境変数 `HOME` が空でなければその値を、空または未設定ならパスワードデータベース（`getpwuid_r`）の値を使う。
   - 判定は文字列の前方一致で行い、パスの区切りは考慮しない。
2. 親ディレクトリの短縮
   - 規則1の結果が `~` で始まる場合のみ適用する。
   - 最後の要素はそのまま残し、それより前の各要素を先頭の1文字に置き換える。
   - 要素の区切りはプラットフォームのパス区切り文字（`MAIN_SEPARATOR`）とする。

### 例

ホームディレクトリが `/Users/alice` の場合。

| 入力 | 出力 |
| --- | --- |
| `/Users/alice/repositories/src/github.com` | `~/r/s/github.com` |
| `/Users/alice/foo` | `~/foo` |
| `/Users/alice` | `~` |
| `/usr/local/bin` | `/usr/local/bin` |
| `/Users/alicefoo/bar` | `~/bar`（前方一致のため `~foo/bar` を経て短縮される） |

### エラー

- ホームディレクトリを取得できない場合や、パスがUTF-8として不正な場合はpanicする。

## 動作環境

- stable Rustの最新版でビルドする（edition 2024、rustc 1.85以降）。
- 対象はLinuxとmacOS。Windowsは対象外（[ADR 0004](adr/0004-binary-distribution.md)、[ADR 0007](adr/0007-zero-dependencies.md)）。

## 配布

`v` から始まるタグのpushで、GitHub Releasesにビルド済みバイナリを公開する（[ADR 0004](adr/0004-binary-distribution.md)）。

- タグ名から `v` を除いた値は `Cargo.toml` の `version` と一致しなければならない。
- タグ名に `-` を含む場合（`v1.0.0-rc` など）はpre-releaseとして公開する。
- 成果物は以下のターゲットごとの `shorten-path-<version>-<target>.tar.gz` と、それらの `SHA256SUMS`。
  - `x86_64-unknown-linux-musl`（静的リンク）
  - `aarch64-unknown-linux-musl`（静的リンク）
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
- アーカイブには `shorten-path-<version>-<target>/` ディレクトリの下にバイナリ、`LICENSE`、`README.md` を含む。
- バイナリは署名・公証しない。macOSでの解除手順はREADMEに記載する。
- `workflow_dispatch` で実行した場合はビルドのみを行い、リリースは作成しない。

## 開発プロセス

- 仕様の変更はADRを作成し、レビューを受けてから実装する。
- コミット済みのADRは、`ステータス:` の行を除いて変更できない。削除・リネームもできない（[ADR 0005](adr/0005-migrate-to-adr-guard.md)）。
  - 検査には [adr-guard](https://github.com/tett23/adr-guard) を使う。
  - gitのpre-commit hook（`.githooks/pre-commit`）が `adr-guard pre-commit` でコミットを拒否する。
  - CIの `adr` ジョブが `tett23/adr-guard` のactionで、範囲内の各コミットを検査する。
  - Claude Codeのhookはリポジトリには置かず、開発者のユーザー設定で `adr-guard claude-hook` を有効にする。
- CI（[ADR 0003](adr/0003-ci.md)）はGitHub Actionsで、`main` へのpushとpull requestごとに以下を実行する。
  - `cargo fmt --check`、`cargo clippy --all-targets --locked -- -D warnings`、`cargo test --locked`（Ubuntu、macOS）
  - コミット済みADRの変更検査

## ADR一覧

| 番号 | タイトル | ステータス |
| --- | --- | --- |
| [0001](adr/0001-build-on-stable-rust.md) | stable Rustの最新版でビルドできるようにする | 採択 |
| [0002](adr/0002-adr-immutability-hooks.md) | コミット済みADRの変更をhookで禁止する | 採択→ADR 0005 |
| [0003](adr/0003-ci.md) | GitHub ActionsによるCI | 採択→adrジョブはADR 0005 |
| [0004](adr/0004-binary-distribution.md) | GitHub Releasesによるバイナリ配布 | 採択 |
| [0005](adr/0005-migrate-to-adr-guard.md) | ADRの変更検査をadr-guardに移行する | 採択 |
| [0006](adr/0006-update-actions-major-versions.md) | GitHub Actionsのアクションを最新のメジャー版に更新する | 採択 |
| [0007](adr/0007-zero-dependencies.md) | 外部クレートへの依存をなくす | 採択 |

## 破棄したADR

なし。
