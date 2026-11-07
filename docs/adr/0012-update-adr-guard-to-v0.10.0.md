# ADR 0012: adr-guard を v0.10.0 に上げる

ステータス: 採択

## 文脈

[ADR 0011](0011-update-adr-guard-to-v0.8.3.md) で、adr-guard を v0.8.3 に上げた。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.8.3` に固定し、pre-commit hookは `adr-guard pre-commit --allow-status --allow-links` を実行している。

その後、adr-guard は v0.9.0、v0.9.1、v0.10.0 に更新された（[v0.9.0](https://github.com/tett23/adr-guard/releases/tag/v0.9.0)、[v0.9.1](https://github.com/tett23/adr-guard/releases/tag/v0.9.1)、[v0.10.0](https://github.com/tett23/adr-guard/releases/tag/v0.10.0) のリリース）。変更点は次のとおりである。

- v0.9.0（tett23/adr-guard の [重複したADRの番号の修復](https://github.com/tett23/adr-guard/blob/main/docs/adr/0018-repair-duplicate-adr-numbers.md)）
  - 新しいサブコマンド `adr-guard check-numbers` が、同じディレクトリで番号が重複したADRを報告する。actionは既定でこれを実行し、重複があればCIが失敗する（入力 `check-numbers`）。
  - 新しいサブコマンド `adr-guard renumber [--dry-run]` が、重複した番号を振りなおす。振りなおしのコミットには `--allow-renumber`（`pre-commit` と `check-range`。actionの入力は `allow-renumber`、既定は false）が要る。
- v0.9.1: `renumber` の誤報の修正。
- v0.10.0（tett23/adr-guard の [モジュールで修飾したADRの参照](https://github.com/tett23/adr-guard/blob/main/docs/adr/0019-module-qualified-adr-references.md) と [シンボリックリンクの文書を飛ばす](https://github.com/tett23/adr-guard/blob/main/docs/adr/0020-skip-symlinked-documents.md)）
  - ADRの参照の直前の語（空白1つを挟む）を修飾子として解釈する。`/` を含む語はディレクトリ、`-` を含む語はモジュール（`docs/<モジュール>/adr/`）、それ以外の語はそのモジュールが存在するときだけ修飾子になる。修飾子はリンクのテキストに含める。
  - `links` とpre-commit hookが、ディレクトリやADRが存在しない修飾子付きの参照、リンク先が修飾子のADRと異なるリンク、リンクの外に出た修飾子を報告する。`links --fix` で直せるものは直す。
  - シンボリックリンクの `.md` は検査しない。

このリポジトリのADRは `docs/adr/` だけにあり、モジュールのADRは無い。
手元の adr-guard（v0.10.0）で `adr-guard links --fix --include-committed` を実行したところ、修正は無く、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` はすべて終了コード0で終わった。

pre-commit hookのコマンドとオプションは変わらない。手元の adr-guard はすでに v0.10.0 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.10.0` にする。
  - `base`、`head`、`allow-status: true`、`allow-links: true` の入力と、`actions/checkout` の設定はそのままとする。`check-numbers` と `allow-renumber` の入力は付けず、既定（番号の重複を検査し、振りなおしは許可しない）に従う。
- `README.md` の開発の節を v0.10.0 に合わせる。必要なバージョンを v0.10.0 以降（`cargo install ... --tag v0.10.0`）にし、ADRの番号が重複するとCIでエラーになること（`adr-guard check-numbers`）と、修飾子付きの参照の規則を書く。
- `docs/specifications.md` の開発プロセスの記述を v0.10.0 に合わせる（使うバージョン、actionのバージョン、CIが `check-numbers` も実行すること、修飾子付きの参照の規則）。ADR一覧に本ADRを追加し、[ADR 0011](0011-update-adr-guard-to-v0.8.3.md) のステータスを更新する。
- [ADR 0011](0011-update-adr-guard-to-v0.8.3.md) のステータスを `採択→指定するバージョンは[ADR 0012](0012-update-adr-guard-to-v0.10.0.md)` にする。

## 実装しないこと

- `.githooks/pre-commit` の変更。コマンドとオプションは v0.10.0 でも変わらない。`--allow-renumber` は付けない。番号の重複が起きたときに、振りなおしのコミットでだけ一時的に付ける。
- CIでの `check-numbers` の無効化（`check-numbers: false`）。
- Claude Codeのhookの変更。全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## テスト設計

- 手元の adr-guard（v0.10.0）で、次を確かめる。
  - `adr-guard links --fix --include-committed` を実行したあと、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` が終了コード0で終わる。
  - 変更をステージして `.githooks/pre-commit` が成功する。ステージには [ADR 0011](0011-update-adr-guard-to-v0.8.3.md) のステータスの行の変更が含まれる。
  - コミット後、`adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
  - `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.10.0のactionで実行され（`check-numbers` を含む）、成功することを確かめる。

## トレードオフ

- 番号が重複したADRがあるとCIが失敗するようになる。並行するブランチで同じ番号のADRを作ると、後からマージした側で失敗する。重複は `adr-guard renumber` で直せるので、検査を止めずに受け入れる。
- 修飾子付きの参照の検査が厳しくなり、ほかのリポジトリのADRを `ADR NNNN` の形で書くとエラーになりうる。ほかのリポジトリのADRは、リンクのテキストごと外部リンクにして書く。
- 手元の adr-guard が v0.10.0 より古いと、修飾子付きの参照の検査が手元では行われず、CIで初めて失敗する。READMEに必要なバージョンを書いて補う。

## 参考文献

- tett23/adr-guard [重複したADRの番号の修復](https://github.com/tett23/adr-guard/blob/main/docs/adr/0018-repair-duplicate-adr-numbers.md)
- tett23/adr-guard [モジュールで修飾したADRの参照](https://github.com/tett23/adr-guard/blob/main/docs/adr/0019-module-qualified-adr-references.md)
- tett23/adr-guard [シンボリックリンクの文書を飛ばす](https://github.com/tett23/adr-guard/blob/main/docs/adr/0020-skip-symlinked-documents.md)
- tett23/adr-guard [リリース v0.9.0](https://github.com/tett23/adr-guard/releases/tag/v0.9.0)
- tett23/adr-guard [リリース v0.9.1](https://github.com/tett23/adr-guard/releases/tag/v0.9.1)
- tett23/adr-guard [リリース v0.10.0](https://github.com/tett23/adr-guard/releases/tag/v0.10.0)
