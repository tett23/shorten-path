# ADR 0005: ADRの変更検査をadr-guardに移行する

ステータス: 採択

## 文脈

[ADR 0002](0002-adr-immutability-hooks.md) で、コミット済みADRの変更を禁止する検査を、このリポジトリ独自のシェルスクリプトとして実装した。
検査はClaude Codeのhook（`.claude/hooks/protect-adr.sh`）、gitのpre-commit hook（`.githooks/pre-commit`）、CIの `adr` ジョブ（[ADR 0003](0003-ci.md)）の三か所で行っている。
すべて `scripts/check-adr-immutable.sh` などの独自実装に依存している。

同じ規則を強制するツールとして [adr-guard](https://github.com/tett23/adr-guard) が公開されており、ユーザーの全リポジトリ共通の運用はadr-guardを使う形に変わった。
Claude Codeのhookはユーザー全体の設定（`~/.claude/settings.json`）で `adr-guard claude-hook` が有効になっており、このリポジトリの独自hookと二重に動いている。

独自実装には以下の不足もある。

- Claude Codeのhookが `MultiEdit` を検査しない。
- `Edit` の検査が `old_string` と `new_string` の比較に頼っており、ステータス行の一部だけを置換するEditを誤って拒否する。
- CIの検査が範囲の両端だけを比較するため、途中のコミットでADRを書きかえて後で戻した場合を検出できない。

## 実装すること

### gitのpre-commit hook

`.githooks/pre-commit` を以下に置き換える。

```sh
#!/bin/sh
exec adr-guard pre-commit
```

`git config core.hooksPath .githooks` で有効化する手順は変えない。

### Claude Codeのhook

- `.claude/hooks/protect-adr.sh` を削除する。
- `.claude/settings.json` から `PreToolUse` hookの設定を削除する。設定がほかに無くなるため、ファイルごと削除する。
- Claude Codeのhookはユーザー全体の設定で有効なものに任せる。

### CI

`.github/workflows/ci.yml` の `adr` ジョブを、adr-guardのcomposite actionを使う形に置き換える。

```yaml
adr:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v7
      with:
        fetch-depth: 0
        persist-credentials: false
    - uses: tett23/adr-guard@v0.2.4
      with:
        base: ${{ github.event_name == 'pull_request' && github.event.pull_request.base.sha || github.event.before }}
        head: ${{ github.event_name == 'pull_request' && github.event.pull_request.head.sha || github.sha }}
```

- adr-guardのバージョンは `v0.2.4` に固定する。

### 独自スクリプトの削除

- `scripts/check-adr-immutable.sh` を削除する。`scripts/` にほかのファイルは無いため、ディレクトリごと無くなる。

### ドキュメント

- READMEの「開発」に、adr-guardのインストールが必要であることと、インストール手順へのリンクを書く。
- `docs/specifications.md` の「開発プロセス」を、adr-guardによる検査に書きかえる。ADR一覧に本ADRを追加する。

### ADRのステータス

本ADRの採択時に、以下のステータス行を更新する。

- [ADR 0002](0002-adr-immutability-hooks.md): `採択→ADR 0005`
- [ADR 0003](0003-ci.md): `採択→adrジョブはADR 0005`

## 実装しないこと

- `check` ジョブや `.github/workflows/release.yml` の `actions/checkout`、`actions/upload-artifact`、`actions/download-artifact` のメジャー版の更新は行わない。Node 24以降で動く最新版への更新は別のADRで扱う。本ADRの `adr` ジョブは、adr-guardのREADMEの例に合わせて `actions/checkout@v7` を使う。
- adr-guardの自動インストール（`mise` の設定ファイルなど）はリポジトリに含めない。インストール方法は開発者に任せる。
- 監査や性能計測のスナップショットなど、ADR以外の変更不可ファイルの検査は行わない（[ADR 0002](0002-adr-immutability-hooks.md) と同じ）。
- 過去のコミット（ADR 0002〜0004をリンク化したコミット）を書きかえることはしない。CIの検査範囲は移行後のpushからになるため、影響しない。

## テスト設計

- pre-commit hook（手元のadr-guard 0.2.4で確認する）
  - 新しいADRの追加はコミットできる。
  - コミット済みADRのステータス行だけの変更はコミットできる。
  - コミット済みADRの本文の変更、削除、リネームはコミットできない。
- Claude Codeのhook
  - 独自hookを削除したあとも、コミット済みADRの本文のEditがユーザー全体の設定の `adr-guard claude-hook` で拒否されることを確認する。
- CI
  - 移行のpull requestで `adr` ジョブが成功することを確認する。このpull requestには、ADR 0002と0003のステータス行の変更が含まれる。
  - `adr-guard check-range <base> <head>` を、コミット済みADRの本文を書きかえたコミットを含む範囲に対して手元で実行し、失敗することを確認する。

## トレードオフ

- adr-guardが手元に無いと、pre-commit hookはコマンドが見つからずにすべてのコミットを拒否する。READMEにインストール手順を書いて補う。
- ADRの保護が外部のツールに依存する。adr-guardは同じユーザーが管理しており、バージョンを固定するので、意図しない挙動の変化は起きにくい。
- Claude Codeのhookをリポジトリに置かないため、ユーザー全体の設定を持たない環境（ほかの開発者やクラウドのセッション）ではClaude Codeでの編集を止められない。pre-commit hookとCIで検出する。
- CIはLinux（x86_64）のランナーでしか動かないが、`adr` ジョブは `ubuntu-latest` で動かしているので問題にならない。
- 範囲内のコミットを一つずつ検査するため、途中でADRを書きかえたコミットがあれば、後で戻していてもCIは失敗する。その場合はブランチの履歴を書きかえて直す。
