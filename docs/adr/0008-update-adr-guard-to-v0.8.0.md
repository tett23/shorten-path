# ADR 0008: adr-guard を v0.8.0 に上げ、ステータスの行とリンクの変更を許可する

ステータス: 採択→指定するバージョンは[ADR 0009](0009-update-adr-guard-to-v0.8.1.md)

## 文脈

[ADR 0005](0005-migrate-to-adr-guard.md) で、ADRの変更検査を [adr-guard](https://github.com/tett23/adr-guard) に移行した。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.2.4` に固定し、pre-commit hookはオプションを付けずに `adr-guard pre-commit` を実行している。

その後、adr-guard は v0.8.0 に更新された。このリポジトリに関わる変更は次のとおりである。

- v0.4.0 から、文書（`.md`）の中のADRの参照（`ADR 0001` の形）にADRのファイルへのリンクが無いと、pre-commit hookがエラーにする。actionも既定で `adr-guard links` を実行し、リンクの無い参照があれば失敗する。コミット済みのADRにリンクを付けるだけの変更は、`--allow-links` を付けたときだけ許可される（tett23/adr-guard の [ADR 0009](https://github.com/tett23/adr-guard/blob/main/docs/adr/0009-require-links-for-adr-references.md)）。
- v0.5.0 から、コミット済みのADRのステータスの行の変更も、`--allow-status` を付けたときだけ許可される（tett23/adr-guard の [ADR 0010](https://github.com/tett23/adr-guard/blob/main/docs/adr/0010-allow-status-option.md)）。
- v0.6.0 から、どの深さの `docs/adr/`・`docs/<モジュール>/adr/` も保護対象になる（tett23/adr-guard の [ADR 0011](https://github.com/tett23/adr-guard/blob/main/docs/adr/0011-nested-adr-directories.md)）。このリポジトリのADRは `docs/adr/` だけなので、影響しない。
- v0.7.0 から、ADRは行頭が `ステータス:` か `Status:` のステータスの行をちょうど1つ持たねばならない。無いADRと2つ以上あるADRは、pre-commit hookと `adr-guard check-status` で違反になる。actionも既定で `check-status` を実行する（tett23/adr-guard の [ADR 0012](https://github.com/tett23/adr-guard/blob/main/docs/adr/0012-status-line-and-status-adt.md)）。
- v0.8.0 から、`docs/audit/` と `docs/benchmarks/`（どの深さの `docs/` でも）のコミット済みのファイルも、変更・削除・移動ができない。`--allow-status` と `--allow-links` はこれらの記録には効かない（tett23/adr-guard の [ADR 0013](https://github.com/tett23/adr-guard/blob/main/docs/adr/0013-protect-audit-and-benchmark-records.md)）。

手元の adr-guard はすでに v0.8.0 に更新されている。
このリポジトリのpre-commit hookはオプションを付けていないため、いまはADRの採択でステータスの行を変えるコミットがpre-commit hookで止まる。

現時点で `adr-guard links` を実行すると、リンクの無いADRの参照が6件ある（[ADR 0002](0002-adr-immutability-hooks.md)・0003のステータスの行、[ADR 0005](0005-migrate-to-adr-guard.md)の本文、`docs/specifications.md` のADR一覧）。
`adr-guard check-status` は、すべてのADRがステータスの行をちょうど1つ持つため、違反が無い。
`docs/audit/` と `docs/benchmarks/` は、このリポジトリにはまだ無い。

## 実装すること

- `.githooks/pre-commit` を `exec adr-guard pre-commit --allow-status --allow-links` にする。
  - このADRの実装のコミットでは、[ADR 0005](0005-migrate-to-adr-guard.md) のステータスの行を変え、コミット済みのADRにリンクを付けるため、hookの変更を同じコミットか、それより前のコミットに含める。
- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.0` にし、入力 `allow-status: true` と `allow-links: true` を加える。
  - `actions/checkout@v7`、`fetch-depth: 0`、`persist-credentials: false` はそのままとする。
  - actionの入力 `links` と `check-status` は既定（`true`）のままとし、`adr-guard links` と `adr-guard check-status` を実行させる。
- `adr-guard links --fix --include-committed` で、コミット済みのADRも含めて、リンクの無いADRの参照をリンクにする。`adr-guard links` が終了コード0で終わるようにする。
- `README.md` の開発の節に、adr-guard v0.8.0 以降が必要であること（`cargo install ... --tag v0.8.0`）、ステータスの行の変更とリンクだけの変更がpre-commit hookとCIの `--allow-status` と `--allow-links` で許可されること、ADRの参照にはリンクが要ること、ADRにはステータスの行がちょうど1つ要ること、`docs/audit/` と `docs/benchmarks/` のコミット済みのファイルも変更できないことを記載する。Claude Codeのhookは `adr-guard claude-hook --allow-status` とする。
- `docs/specifications.md` の開発プロセスの記述を v0.8.0 に合わせる（hookのオプション、actionのバージョン、`links` と `check-status` の検査、ステータスの行の規則、記録の保護）。ADR一覧に本ADRを追加し、[ADR 0005](0005-migrate-to-adr-guard.md) のステータスを更新する。
- [ADR 0005](0005-migrate-to-adr-guard.md) のステータスを `採択→指定するバージョンとhookのオプションは[ADR 0008](0008-update-adr-guard-to-v0.8.0.md)` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0005](0005-migrate-to-adr-guard.md) と同じくタグで固定する。
- actionへの `check-status: false` の指定。いまのADRに違反が無いため、移行の手順は要らない。
- `docs/audit/` と `docs/benchmarks/` の作成。記録を置くときに、v0.8.0 の規則に従う。
- Claude Codeのhookの変更。Claude Codeのhookは全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。全体の設定は `adr-guard claude-hook --allow-status` に更新済み（反映は各自の環境で行う）。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## テスト設計

- 手元の adr-guard（v0.8.0）で、次を確かめる（コミットはしない）。
  - `adr-guard links` と `adr-guard check-status` が終了コード0で終わる。
  - 変更をステージして `.githooks/pre-commit` が成功する。ステージには [ADR 0005](0005-migrate-to-adr-guard.md) のステータスの行の変更と、コミット済みのADRへのリンクの付与が含まれる。
  - コミット済みのADRの本文を変えると、pre-commit hookが拒否する。
  - `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。

## トレードオフ

- pre-commit hookに `--allow-status` を付けるため、手元の adr-guard が v0.5.0 より古いと、引数の誤り（終了コード64）でコミットが止まる。また、v0.8.0 より古いと、ステータスの行の規則や記録の保護が手元で検査されず、CIで初めて違反がわかる。READMEに必要なバージョンを書いて補う。
- `--allow-links` により、コミット済みのADRのリンクの付け外しやリンク先の変更は検査されなくなる。リンクの本文以外の変更は引き続き拒否されるため、ADRの内容は守られる。
- `docs/audit/` と `docs/benchmarks/` の記録には `--allow-status` と `--allow-links` が効かないため、誤記を直すにも緊急用の手段（`ADR_GUARD_ALLOW` など）が要る。記録はコミット後は変更不可という方針どおりである。
- コミット済みのADRへのリンクの付与を、ADRの実装と同じプルリクエストに含める。adr-guard のREADMEはリンクの付与をほかの変更と分けた1つのコミットにすることを勧めているため、実装のコミットとは分ける。
