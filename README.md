# shorten-path

ホームディレクトリ以下のパスを、プロンプト表示向けに短縮するコマンド。

```console
$ shorten-path /Users/alice/repositories/src/github.com
~/r/s/github.com
```

詳しい変換規則は [docs/specifications.md](docs/specifications.md) を参照。

## ビルド

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
