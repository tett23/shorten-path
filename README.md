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

clone後に、コミット済みADRの変更を禁止するgit hookを有効にする。

```sh
git config core.hooksPath .githooks
```

Claude Code用のhookは `.claude/settings.json` で設定済みで、`jq` が必要。

設計判断は [docs/adr/](docs/adr/) にADRとして記録する。

## License

MIT
