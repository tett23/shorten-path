# ADR 0010: adr-guard を v0.8.2 に上げる

ステータス: 採択

## 文脈

[ADR 0009](0009-update-adr-guard-to-v0.8.1.md) で、adr-guard を v0.8.1 に上げた。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.8.1` に固定し、pre-commit hookは `adr-guard pre-commit --allow-status --allow-links` を実行している。

その後、adr-guard は v0.8.2 に更新された（[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.2)）。変更点は次のとおりである（tett23/adr-guard の [ADR 0015](https://github.com/tett23/adr-guard/blob/main/docs/adr/0015-allow-link-changes.md)）。

- `--allow-links` が、ADRの参照のリンクに限らず、コミット済みADRの「リンクだけの変更」全般を許可する。
  - 許可される: インラインリンクと参照形式のリンクの追加・削除・リンク先の変更、画像のリンク先の変更、リンクの参照定義の行（`[1]: url`）の変更。
  - 引き続き拒否される: リンクのテキストの変更、画像の追加・削除、自動リンク（`<url>`）や本文のURLの変更。
- 記録（`docs/audit/` など）は、引き続き `--allow-status` と `--allow-links` が効かず、完全に保護される。

このリポジトリのpre-commit hookとCIはすでに `--allow-links` を付けているため、コマンドやactionの入力を変えずに、許可される変更の範囲だけが広がる。

手元の adr-guard はすでに v0.8.2 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.2` にする。
  - `base`、`head`、`allow-status: true`、`allow-links: true` の入力と、`actions/checkout@v7` の設定はそのままとする。
- `README.md` の開発の節を v0.8.2 に合わせる。必要なバージョンを v0.8.2 以降（`cargo install ... --tag v0.8.2`）にし、`--allow-links` の説明を「ADRの参照のリンクだけの変更」から「リンクだけの変更」にする。
- `docs/specifications.md` の開発プロセスの記述を v0.8.2 に合わせる（使うバージョン、actionのバージョン、`--allow-links` で許可される変更）。ADR一覧に本ADRを追加し、[ADR 0009](0009-update-adr-guard-to-v0.8.1.md) のステータスを更新する。
- [ADR 0009](0009-update-adr-guard-to-v0.8.1.md) のステータスを `採択→指定するバージョンは[ADR 0010](0010-update-adr-guard-to-v0.8.2.md)` にする。

## 実装しないこと

- `.githooks/pre-commit` の変更。コマンドとオプションは v0.8.2 でも変わらない。
- `--allow-links` を外すこと。リンクのテキストや本文の変更は引き続き拒否されるため、ADRの内容の不変は保たれる。
- Claude Codeのhookの変更。全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## テスト設計

- 手元の adr-guard（v0.8.2）で、次を確かめる。
  - `adr-guard links --fix --include-committed` を実行したあと、`adr-guard links` と `adr-guard check-status` が終了コード0で終わる。
  - 変更をステージして `.githooks/pre-commit` が成功する。ステージには [ADR 0009](0009-update-adr-guard-to-v0.8.1.md) のステータスの行の変更が含まれる。
  - コミット後、`adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
  - `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.2のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.2-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。

## トレードオフ

- コミット済みADRのリンク先を書きかえられるようになる。リンク先の変更で参照の意味が変わりうるが、リンクのテキストと本文は変えられないため、判断の記録としての内容は保たれる。リンク切れの修正がADRを作り直さずにできる利点のほうが大きい。
- 手元の adr-guard が v0.8.2 より古いと、v0.8.2 で許可されるリンクの変更が手元のpre-commit hookで拒否される。READMEに必要なバージョンを書いて補う。

## 参考文献

- tett23/adr-guard [ADR 0015: allow link changes](https://github.com/tett23/adr-guard/blob/main/docs/adr/0015-allow-link-changes.md)
- tett23/adr-guard [リリース v0.8.2](https://github.com/tett23/adr-guard/releases/tag/v0.8.2)
