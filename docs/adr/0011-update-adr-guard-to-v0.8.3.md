# ADR 0011: adr-guard を v0.8.3 に上げる

ステータス: 採択

## 文脈

[ADR 0010](0010-update-adr-guard-to-v0.8.2.md) で、adr-guard を v0.8.2 に上げた。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.8.2` に固定し、pre-commit hookは `adr-guard pre-commit --allow-status --allow-links` を実行している。

その後、adr-guard は v0.8.3 に更新された（[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.3)）。変更点は次のとおりである（tett23/adr-guard の [採択（未実装）のステータス](https://github.com/tett23/adr-guard/blob/main/docs/adr/0016-accepted-not-implemented-status.md)）。

- 新しいステータス `採択（未実装）`（英語は `Accepted (not implemented)`。括弧は全角でも半角でもよい）を解釈する。採択されたがまだ実装されていないADRを表す。
- 新しいサブコマンド `adr-guard unimplemented [--status]` で、ステータスが `採択（未実装）` のADRを一覧できる。

pre-commit hookとCIのコマンドとオプションは変わらない。

手元の adr-guard はすでに v0.8.3 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.3` にする。
  - `base`、`head`、`allow-status: true`、`allow-links: true` の入力と、`actions/checkout` の設定はそのままとする。
- `README.md` の開発の節を v0.8.3 に合わせる。必要なバージョンを v0.8.3 以降（`cargo install ... --tag v0.8.3`）にし、採択されたがまだ実装されていないADRのステータスを `採択（未実装）` とし、`adr-guard unimplemented` で一覧できることを書く。
- `docs/specifications.md` の開発プロセスの記述を v0.8.3 に合わせる（使うバージョン、actionのバージョン、`採択（未実装）` のステータス）。ADR一覧に本ADRを追加し、[ADR 0010](0010-update-adr-guard-to-v0.8.2.md) のステータスを更新する。
- [ADR 0010](0010-update-adr-guard-to-v0.8.2.md) のステータスを `採択→指定するバージョンは[ADR 0011](0011-update-adr-guard-to-v0.8.3.md)` にする。

## 実装しないこと

- `.githooks/pre-commit` の変更。コマンドとオプションは v0.8.3 でも変わらない。
- 既存のADRのステータスを `採択（未実装）` に書きかえること。既存のADRはすべて実装済みである。
- Claude Codeのhookの変更。全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## テスト設計

- 手元の adr-guard（v0.8.3）で、次を確かめる。
  - `adr-guard links --fix --include-committed` を実行したあと、`adr-guard links` と `adr-guard check-status` が終了コード0で終わる。
  - 変更をステージして `.githooks/pre-commit` が成功する。ステージには [ADR 0010](0010-update-adr-guard-to-v0.8.2.md) のステータスの行の変更が含まれる。
  - コミット後、`adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
  - `adr-guard unimplemented` が終了コード0で終わり、何も出力しない（未実装のADRが無い）。
  - `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.3のactionで実行され、成功することを確かめる。

## トレードオフ

- 手元の adr-guard が v0.8.3 より古くても、hookとCIの検査の結果は変わらない。ただし `adr-guard unimplemented` は使えない。READMEに必要なバージョンを書いて補う。
- ステータスの候補が1つ増えるが、`採択（未実装）` は採択の一種であり、実装後に `採択` へ変えるだけで済む。ステータスの行の変更は `--allow-status` で許可されている。

## 参考文献

- tett23/adr-guard [採択（未実装）のステータス](https://github.com/tett23/adr-guard/blob/main/docs/adr/0016-accepted-not-implemented-status.md)
- tett23/adr-guard [リリース v0.8.3](https://github.com/tett23/adr-guard/releases/tag/v0.8.3)
