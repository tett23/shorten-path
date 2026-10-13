# ADR 0009: adr-guard を v0.8.1 に上げる

ステータス: 採択

## 文脈

[ADR 0008](0008-update-adr-guard-to-v0.8.0.md) で、adr-guard を v0.8.0 に上げた。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.8.0` に固定し、pre-commit hookは `adr-guard pre-commit --allow-status --allow-links` を実行している。

その後、adr-guard は v0.8.1 に更新された（[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.1)）。変更点は次のとおりである（tett23/adr-guard の [ADR 0014](https://github.com/tett23/adr-guard/blob/main/docs/adr/0014-directory-name-variants.md)）。

- ADRのディレクトリとして、`adr` のほか `adrs`、`decisions`、`decision` も保護する。
- 記録のディレクトリとして、`docs/audit/` と `docs/benchmarks/` のほか `docs/audits/` と `docs/benchmark/` も保護する。

このリポジトリのADRは `docs/adr/` だけであり、`adrs`・`decisions`・`decision`・`audits`・`benchmark` の名前のディレクトリは、追跡しているファイルにも未追跡のファイルにも無い。
そのため、新しく保護されるファイルは無く、上げる前に名前を変えるべきディレクトリも無い。

手元の adr-guard はすでに v0.8.1 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.1` にする。
  - `base`、`head`、`allow-status: true`、`allow-links: true` の入力と、`actions/checkout@v7` の設定はそのままとする。
- `README.md` の開発の節を v0.8.1 に合わせる。必要なバージョンを v0.8.1 以降（`cargo install ... --tag v0.8.1`）にし、保護される記録のディレクトリに `docs/audits/` と `docs/benchmark/` を加える。
- `docs/specifications.md` の開発プロセスの記述を v0.8.1 に合わせる（使うバージョン、actionのバージョン、保護される記録のディレクトリ）。ADR一覧に本ADRを追加し、[ADR 0008](0008-update-adr-guard-to-v0.8.0.md) のステータスを更新する。
- [ADR 0008](0008-update-adr-guard-to-v0.8.0.md) のステータスを `採択→指定するバージョンは[ADR 0009](0009-update-adr-guard-to-v0.8.1.md)` にする。

## 実装しないこと

- `.githooks/pre-commit` の変更。コマンドとオプションは v0.8.1 でも変わらない。
- ADRのディレクトリの別名（`adrs`・`decisions`・`decision`）の利用。このリポジトリのADRは引き続き `docs/adr/` に置く。
- `docs/audit/`・`docs/audits/`・`docs/benchmarks/`・`docs/benchmark/` の作成。記録を置くときに、v0.8.1 の規則に従う。
- Claude Codeのhookの変更。全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## テスト設計

- 手元の adr-guard（v0.8.1）で、次を確かめる（コミットはしない）。
  - `adr-guard links` と `adr-guard check-status` が終了コード0で終わる。
  - 変更をステージして `.githooks/pre-commit` が成功する。ステージには [ADR 0008](0008-update-adr-guard-to-v0.8.0.md) のステータスの行の変更が含まれる。
  - `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.1のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.1-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。

## トレードオフ

- 手元の adr-guard が v0.8.1 より古いと、`docs/audits/` と `docs/benchmark/` の記録の保護が手元で検査されず、CIで初めて違反がわかる。READMEに必要なバージョンを書いて補う。
- 今後 `docs/` の下に `adrs`・`decisions`・`decision`・`audits`・`benchmark` の名前のディレクトリを作ると、ADRや記録として保護される。ADRや記録以外の用途には、これらの名前を使えない。
