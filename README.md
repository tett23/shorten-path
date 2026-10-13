# shorten-path

ホームディレクトリ以下のパスを、プロンプト表示向けに短縮するコマンド。

```console
$ shorten-path /Users/alice/repositories/src/github.com
~/r/s/github.com
```

詳しい変換規則は [docs/specifications.md](docs/specifications.md) を参照。

## インストール

### ビルド済みバイナリ

[Releases](https://github.com/tett23/shorten-path/releases) から環境に合ったアーカイブを取得して展開し、`shorten-path` を `PATH` の通ったディレクトリに置く。

```sh
shasum -a 256 -c SHA256SUMS --ignore-missing
tar xzf shorten-path-<version>-<target>.tar.gz
```

#### macOSでGatekeeperに止められる場合

配布しているバイナリは署名・公証していないため、ブラウザでダウンロードすると「開発元を検証できません」として実行を拒否される。
ブラウザはダウンロードしたファイルに `com.apple.quarantine` 属性を付け、Gatekeeperはこの属性があるファイルだけを検査する。
`SHA256SUMS` で内容を検証したうえで、以下のいずれかで解除する。

- 属性を削除する。

  ```sh
  xattr -d com.apple.quarantine /path/to/shorten-path
  ```

- 一度実行して拒否されたあと、「システム設定」→「プライバシーとセキュリティ」に表示される「このまま開く」を選ぶ。

`curl` などブラウザ以外でダウンロードした場合は属性が付かないため、この操作は不要。

### ソースからビルド

stable Rustの最新版が必要。

```sh
cargo build --release
```

## 開発

コミット済みADRの変更は [adr-guard](https://github.com/tett23/adr-guard) で禁止している。
adr-guard v0.8.1以降を[インストール](https://github.com/tett23/adr-guard#インストール)したうえで、clone後にgit hookを有効にする。
adr-guardが無いと、pre-commit hookがすべてのコミットを拒否する。v0.5.0より古いと、引数の誤りでコミットが止まる。v0.8.1より古いと、ステータスの行の検査と記録の保護の一部または全部が手元では行われない。

```sh
cargo install --git https://github.com/tett23/adr-guard --tag v0.8.1
git config core.hooksPath .githooks
```

Claude Codeを使う場合は、ユーザー設定（`~/.claude/settings.json`）で `adr-guard claude-hook --allow-status` を有効にする。手順はadr-guardのREADMEを参照。

コミット済みADRのステータスの行の変更と、ADRの参照のリンクだけの変更は、pre-commit hookとCIの `--allow-status` と `--allow-links` で許可される。
文書（`.md`）の中のADRの参照（`ADR 0001` の形）には、ADRのファイルへのリンクが要る。リンクが無いと、pre-commit hookとCIでエラーになる。`adr-guard links --fix` で付けられる。
ADRには、行頭が `ステータス:` のステータスの行をちょうど1つ書く。無いADRと2つ以上あるADRは、pre-commit hookとCI（`adr-guard check-status`）でエラーになる。
`docs/audit/`・`docs/audits/`・`docs/benchmarks/`・`docs/benchmark/` に置く記録（監査や性能計測のスナップショット）も、コミット後は変更・削除・移動できない。記録には `--allow-status` と `--allow-links` も効かない。

設計判断は [docs/adr/](docs/adr/) にADRとして記録する。

## License

MIT
