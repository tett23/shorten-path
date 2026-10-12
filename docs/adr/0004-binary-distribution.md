# ADR 0004: GitHub Releasesによるバイナリ配布

ステータス: 採択

## 文脈

現在の利用方法は、Rustのツールチェインを用意してソースからビルドすることだけである。
`v` から始まるタグをリリース対象とする運用はあるが、リリースの成果物は決まっていない（[ADR 0003](0003-ci.md)で別のADRに先送りした）。
プロンプト表示用の小さなコマンドなので、ツールチェインなしで導入できるようにしたい。

## 実装すること

`.github/workflows/release.yml` を追加する。

### トリガー

- `v*` タグのpush。リリースを作成する。
- `workflow_dispatch`。ビルドとパッケージングだけを行い、リリースは作成しない（動作確認用）。

### 対象プラットフォーム

| ターゲット | ランナー | 備考 |
| --- | --- | --- |
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | `musl-tools` を使う。静的リンク |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | `musl-tools` を使う。静的リンク |
| `aarch64-apple-darwin` | `macos-latest` | |
| `x86_64-apple-darwin` | `macos-latest` | クロスコンパイル。テストは実行しない |

### `build` ジョブ（ターゲットごとのマトリクス）

1. タグのpushの場合、タグ名から `v` を除いた値と `Cargo.toml` の `version` が一致することを確認する。一致しなければ失敗とする。
2. `rustup` でstableツールチェインとターゲットを入れる。
3. ランナーと同じアーキテクチャのターゲットでは `cargo test --locked --target <target>` を実行する。
4. `cargo build --release --locked --target <target>` でビルドする。
5. `shorten-path-<version>-<target>.tar.gz` を作る。中身はバイナリ、`LICENSE`、`README.md` とし、`shorten-path-<version>-<target>/` ディレクトリの下に置く。`workflow_dispatch` の場合、`<version>` はコミットハッシュの短縮形とする。
6. `actions/upload-artifact` でアーカイブをアップロードする。

### `release` ジョブ（タグのpushのときだけ実行）

1. 全ターゲットの `build` が成功したあとに実行する。
2. `actions/download-artifact` でアーカイブを集め、`sha256sum` で `SHA256SUMS` を作る。
3. ランナーにプリインストールされている `gh release create` でリリースを作り、アーカイブと `SHA256SUMS` を添付する。
   - リリースノートは `--generate-notes` で生成する。
   - タグ名に `-` が含まれる場合（`v1.0.0-rc` など）は `--prerelease` を付ける。
4. ジョブの権限は `contents: write` のみとする。

### ドキュメント

- READMEに、リリースからバイナリを取得して導入する手順と、`SHA256SUMS` による検証の手順を書く。
- `docs/specifications.md` に、リリースの成果物と対象プラットフォームを書く。

## 実装しないこと

- Windows向けのバイナリは作らない。[ADR 0003](0003-ci.md)と同じく、テストが `/` 区切りを前提にしているため。
- Homebrew、crates.io、各種パッケージマネージャーへの公開は行わない。
- バイナリへの署名、macOSの公証（notarization）は行わない。
- サードパーティのアクション（`softprops/action-gh-release`、`cross` など）は使わない。GitHub公式の `actions/checkout`、`actions/upload-artifact`、`actions/download-artifact` と `gh` コマンドで足りるため。
- 既存のタグに対するリリースの作り直しは自動化しない。

## テスト設計

- `workflow_dispatch` で実行し、全ターゲットのビルドが成功してアーカイブがアップロードされることを確認する。
- アーカイブを取得し、以下を確認する。
  - 展開するとディレクトリの下にバイナリ、`LICENSE`、`README.md` がある。
  - `aarch64-apple-darwin` と `x86_64-apple-darwin` のバイナリが `file` コマンドで期待どおりのアーキテクチャになっている。
  - Linux向けのバイナリが `file` コマンドで静的リンクと表示される。
  - macOS上で `aarch64-apple-darwin` のバイナリを実行し、`shorten-path "$HOME/foo/bar"` が `~/f/bar` を出力する。
- タグと `Cargo.toml` の `version` が一致しない場合に失敗することは、ワークフロー内の比較処理をローカルで同じ値で実行して確認する。
- 公開リリースでの確認は、このADRの採択後に作る最初のリリース（v1.0.0）で行う。

## トレードオフ

- 未署名・未公証のため、macOSではダウンロードしたバイナリがGatekeeperに止められる。READMEに `xattr -d com.apple.quarantine` による解除手順を書いて補う。
- `x86_64-apple-darwin` はクロスコンパイルのため、CI上でテストを実行できない。コードがプラットフォームに依存する部分は `dirs` と `MAIN_SEPARATOR` だけで、`aarch64-apple-darwin` のテストで代替できると判断する。
- musl向けの静的リンクにより、glibcのバージョンに依存しない代わりに、バイナリのサイズがやや大きくなる。
- `ubuntu-24.04-arm` ランナーが使えなくなった場合は、ターゲットの扱いを別のADRで見直す。
