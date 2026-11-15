# ADR 0013: adr-guard を v0.13.1 に上げ、ADRのステータスの行を新形式に移す

Status: accepted

## 文脈

[ADR 0012](0012-update-adr-guard-to-v0.10.0.md) で、adr-guard を v0.10.0 に上げた。
CIの `adr` ジョブのactionは `tett23/adr-guard@v0.10.0` に固定し、pre-commit hookは `adr-guard pre-commit --allow-status --allow-links` を実行している。
ADRのステータスの行は、すべて `ステータス: 採択` のような日本語の形（以下、旧形式）で書いてある。

その後、adr-guard は v0.11.0、v0.12.0、v0.13.0、v0.13.1 に更新された（[v0.11.0](https://github.com/tett23/adr-guard/releases/tag/v0.11.0)、[v0.12.0](https://github.com/tett23/adr-guard/releases/tag/v0.12.0)、[v0.13.0](https://github.com/tett23/adr-guard/releases/tag/v0.13.0)、[v0.13.1](https://github.com/tett23/adr-guard/releases/tag/v0.13.1) のリリース）。変更点は次のとおりである。

- v0.11.0（tett23/adr-guard の [ADR 0021: Codex の PreToolUse hook で、コミット済みのADRと記録の編集を拒否する](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0021-codex-hook.md)）
  - Codex のPreToolUse hookとして `adr-guard codex-hook` が加わった。
- v0.12.0（tett23/adr-guard の [ADR 0022: 状態と遷移を別々の代数的データ型で表す](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0022-status-transitions.md)、[ADR 0023: trace で元の決定から修正ADRをたどる](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0023-trace-amendments.md)）
  - ステータスの行の新しい形（以下、新形式）が加わった。新形式は `Status: <トークン>` で、トークンは `proposed`、`accepted`、`accepted-pending`、`rejected`、`discarded`、`deprecated`、または `superseded [<変更の JSON 配列>]` である。変更は `{"part":<変更した部分の名前か null>,"by":[<修正したADRのリポジトリのルートからのパス>]}` の形で書く。
  - 新しいADRは、最初に `Status: proposed` でコミットしなければならない。追加したADRと、ステータスの行を変えたADRには、新形式が要る。旧形式の行は、変えなければそのまま読める。
  - pre-commit と `check-range` は、ステータスの行の状態遷移（`proposed` から `accepted` へ、`accepted` から `superseded` へ、など）を検査する。この検査は `--allow-status`、環境変数 `ADR_GUARD_ALLOW`、コミットのトレーラーでは免除されない。
  - 旧形式を新形式に書き換える移行スクリプト `scripts/migrate-adr-status` が adr-guard のソースに加わった。リリースのアーカイブには入っていない。
  - 元の決定から修正したADRをたどる `adr-guard trace` が加わった。
- v0.13.0（tett23/adr-guard の [ADR 0024: 移行スクリプトに --dry-run を設け、失敗を最後まで数える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0024-migration-dry-run-and-error-count.md)、[ADR 0025: 新形式のステータス行に、実装待ちの修正、補足、注記、別のリポジトリの修正先を加える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0025-extend-canonical-status-amendments.md)、[ADR 0026: 旧形式のステータス行の修正先を、リンク先と裸の番号からも読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0026-read-more-legacy-references.md)、[ADR 0027: 旧形式の `置換（…）` を、修正を受けた状態として読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0027-read-legacy-replaced-status.md)）
  - 移行スクリプトに、書き換えずに差分だけを示す `--dry-run` が加わった。移行できないADRを最後まで数え、`移行できる: N件、移行済み: K件、移行できない: M件` を標準エラーに表示する。
  - 新形式に `superseded-pending`、変更の `kind` と `note`、`https://` で始まる修正先、`trace --list` の6列が加わった。
  - 旧形式の修正先の読み方が広がった。
- v0.13.1（tett23/adr-guard の [ADR 0028: 旧形式の部分名から修飾子と区切りの記号を除き、決められない部分名を移行しない](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md)）
  - 移行スクリプトが作る部分名の不具合が直った。v0.13.0 の移行スクリプトは、部分名に修飾子（`ADR NNNN` の直前の語）、先頭の「。」、末尾の「→」を残すことがあった。
  - 部分名を決められない行は、「部分名を決められません」と報告して書かない。

手元の adr-guard は v0.13.1 になっている。
手元のpre-commit hookは、新しいADRを旧形式で書くと拒否し、コミット済みのADRのステータスの行を旧形式のまま書き換えても拒否する（「調査した事項」）。
そのため、ADRを足すたびに、またステータスの行を変えるたびに、新形式と旧形式が混ざっていく。
CIと手元の版をそろえ、すべてのADRのステータスの行を一度に新形式に移す。

## 調査した事項

次のことを、手元で実行して確かめた。

- `adr-guard -V` は `adr-guard 0.13.1` を表示した。
- 移行スクリプトは、adr-guard の v0.13.1 のソースの `scripts/migrate-adr-status` を使った。このスクリプトは、`cargo run --example migrate_adr_status` で example の `migrate_adr_status` をビルドして実行する。以下では、ビルドした `migrate_adr_status` を直接実行した。
- このリポジトリのルート（origin/main の 67c2063）で `migrate_adr_status --dry-run` を実行すると、終了コード0で終わり、標準エラーの最後の行は `移行できる: 12件、移行済み: 0件、移行できない: 0件` だった。作業ツリーは書きかわらなかった。
  - 本ADRの草稿を置いた作業ツリーでは、最後の行は `移行できる: 12件、移行済み: 1件、移行できない: 0件` だった。本ADRは新形式なので、移行済みに数えられる。
  - 差分の行は、v0.13.0（`0096636`）の移行スクリプトを同じコミットで実行したときと同じだった。このリポジトリの部分名は、tett23/adr-guard の [ADR 0028](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md) の修正の影響を受けない。
  - [ADR 0012](0012-update-adr-guard-to-v0.10.0.md) の行は `Status: accepted` に、[ADR 0011](0011-update-adr-guard-to-v0.8.3.md) の `ステータス: 採択→指定するバージョンは[ADR 0012](...)` は `Status: superseded [{"by":["docs/adr/0012-update-adr-guard-to-v0.10.0.md"],"part":"指定するバージョン"}]` に移る。12件のうち、`accepted` が5件、`superseded` が7件である。
- 同じコミットの複製で `migrate_adr_status --write` を実行すると、12件のADRのステータスの行だけが書きかわった（`12 files changed, 12 insertions(+), 12 deletions(-)`）。その後の `--dry-run` は `移行できる: 0件、移行済み: 12件、移行できない: 0件` を表示した。
  - 書き換えた後の複製で、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` は終了コード0で終わった。`adr-guard unimplemented` は何も表示しなかった。
  - 書き換えをステージして `adr-guard pre-commit --allow-status --allow-links` は終了コード0で終わった。コミットした後、`adr-guard check-range HEAD~1 HEAD --allow-status --allow-links` も終了コード0で終わった。
- 移行していない複製で、手元の adr-guard（v0.13.1）の pre-commit は次を拒否した。
  - `ステータス: 提案` の新しいADR。診断は `` 新形式のステータス行は `Status: ` で始めてください `` だった。
  - `Status: accepted` の新しいADR。診断は `許可されていない状態遷移: Absent -> accepted` だった。
  - [ADR 0012](0012-update-adr-guard-to-v0.10.0.md) の `ステータス: 採択` を、旧形式のまま `ステータス: 採択→指定するバージョンは[ADR 0011](...)` に書き換えた変更。診断は `` 新形式のステータス行は `Status: ` で始めてください `` で、終了コードは1だった。
  - `Status: proposed` の新しいADRは許可した。
- 実装の手順を、origin/main の複製で次のとおりに試した。どの段階でも pre-commit（`--allow-status --allow-links`）は終了コード0で終わった。
  1. 本ADRを `Status: proposed` のまま足してコミットした。
  2. 移行スクリプトの `--write` を実行した。最後の行は `移行できる: 12件、移行済み: 1件、移行できない: 0件` で、本ADRは移行済みに数えられた。
  3. 本ADRを `Status: accepted` に、[ADR 0012](0012-update-adr-guard-to-v0.10.0.md) を「実装すること」に書いた `superseded` の行にして、移行とあわせて一つのコミットにした。
  - その後、`adr-guard check-range <origin/main> HEAD --allow-status --allow-links`、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` は終了コード0で終わった。
  - 移行を一つのコミットにし、本ADRの採択と [ADR 0012](0012-update-adr-guard-to-v0.10.0.md) の変更を次のコミットに分けた場合も、同じ結果だった。このとき `adr-guard trace docs/adr/0012-update-adr-guard-to-v0.10.0.md` は、[ADR 0012](0012-update-adr-guard-to-v0.10.0.md) から本ADRへの、部分「指定するバージョン」の辺を表示した。
- v0.13.1 の `action.yml` の入力は `base`、`head`、`version`、`allow-status`、`allow-links`、`links`、`check-status`、`check-numbers`、`allow-renumber` である。`version` を指定しないと、action自身の `Cargo.toml` の版（`v0.13.1`）のアーカイブ `adr-guard-v0.13.1-x86_64-unknown-linux-gnu.tar.gz` を取得する。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.13.1` にする。
  - `base`、`head`、`allow-status: true`、`allow-links: true` の入力と、`actions/checkout` の設定はそのままとする。ほかの入力は付けない。
- このリポジトリのルートで、adr-guard の v0.13.1 の移行スクリプトを `--dry-run` で実行し、`移行できない: 0件` で終わることを確かめてから、`--write` で全ADRのステータスの行を新形式に書き換える。adr-guard のソースから実行するときは `scripts/migrate-adr-status --dry-run` と `scripts/migrate-adr-status --write` である。書き換えの結果は手で直さない。
- [ADR 0012](0012-update-adr-guard-to-v0.10.0.md) のステータスの行を `Status: superseded [{"by":["docs/adr/0013-update-adr-guard-to-v0.13.1.md"],"part":"指定するバージョン"}]` にする。部分の名前は、移行した後の [ADR 0008](0008-update-adr-guard-to-v0.8.0.md) から [ADR 0011](0011-update-adr-guard-to-v0.8.3.md) のステータスの行と同じ「指定するバージョン」とする。
- `README.md` の開発の節を v0.13.1 に合わせる。
  - 必要なバージョンを v0.13.1 以降（`cargo install ... --tag v0.13.1`）にする。v0.12.0より古いと、新形式のステータスの行と状態遷移の検査が手元では行われないことを書く。
  - ステータスの行の説明を新形式（行頭が `Status:`）にし、新しいADRは `Status: proposed` でコミットすること、採択されたがまだ実装されていないADRは `Status: accepted-pending` とすること、修正されたADRは `superseded` とすることを書く。
- `docs/specifications.md` の開発プロセスの記述を v0.13.1 に合わせる。
  - 使うバージョンとactionのバージョンを v0.13.1 にし、本ADRへリンクする。
  - コミット済みのADRで変えてよい行を `ステータス:` の行から `Status:` の行にし、ステータスの行の形、新しいADRを `Status: proposed` でコミットすること、pre-commit hookとCIが状態遷移を検査すること、未実装のADRのステータスを `accepted-pending` とすることを書く。
  - ADR一覧に本ADRを加え、[ADR 0012](0012-update-adr-guard-to-v0.10.0.md) の行のステータスを `採択→指定するバージョンは[ADR 0013](...)` にする。ADR一覧のステータスの列は、これまでどおり日本語で書く。

## 実装しないこと

- `.githooks/pre-commit` の変更。コマンドとオプションは v0.13.1 でも変わらない。
- Codex のhook（`adr-guard codex-hook`）の導入。Claude Codeのhookと同じく、このリポジトリでは管理しない。
- Claude Codeのhookの変更。全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。
- 移行スクリプトの結果を超えた、ステータスの行の書き直し（`kind` や `note` を足すことなど）。
- ADRの本文と、ステータスの行以外の行の変更。
- 手元の adr-guard の更新やインストールの自動化。
- `check` ジョブと `.github/workflows/release.yml` の変更。

## 不変条件

なし。設定の値と移行の手順を決めるADRで、入力によって変わる振る舞いを足さない。

## テスト設計

設定と文書の変更なので、自動テストは書かない。実装の後に、次を確かめる。

- 書き換える前に移行スクリプトの `--dry-run` が終了コード0で終わり、`移行できない: 0件` を表示する。書き換えた後の `--dry-run` は `移行できる: 0件` を表示する。
- 手元の adr-guard（v0.13.1）で、`adr-guard links --fix` を実行したあと、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` が終了コード0で終わる。
- `adr-guard unimplemented` の出力が、書き換える前と同じである（いまは空）。
- `adr-guard trace docs/adr/0012-update-adr-guard-to-v0.10.0.md` が、本ADRへの部分「指定するバージョン」の辺を表示する。
- 変更をステージして `.githooks/pre-commit` が成功する。ステージには全ADRの移行と、[ADR 0012](0012-update-adr-guard-to-v0.10.0.md) の `superseded` への変更と、本ADRの `proposed` から `accepted` への変更が含まれる。
- コミット後、`adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
- `.github/workflows/ci.yml` がYAMLとして読み込める。
- 変更のプルリクエストのCIで、`adr` ジョブが v0.13.1 のactionで実行され、成功する。ログで `adr-guard-v0.13.1-x86_64-unknown-linux-gnu.tar.gz` を取得していることを確かめる。

## トレードオフ

- ADRのステータスの行が英語のトークンと JSON になり、日本語の形より読みにくい。代わりに、状態遷移の検査と `adr-guard trace` が使え、旧形式と新形式が混ざらない。
- 一度に12件のADRのステータスの行を書き換えるので、差分が大きい。書き換えは移行スクリプトの結果だけで、手で直さないので、`--dry-run` の差分と比べて確かめられる。
- 手元の adr-guard が v0.12.0 より古いと、新形式の行と状態遷移が手元で検査されず、CIで初めて失敗する。READMEに必要なバージョンを書いて補う。
- ADR一覧のステータスの列は日本語のままなので、ADRのステータスの行と表記が異なる。列は一覧を読む人のための要約として残す。

## 参考文献

- tett23/adr-guard [ADR 0021: Codex の PreToolUse hook で、コミット済みのADRと記録の編集を拒否する](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0021-codex-hook.md)
- tett23/adr-guard [ADR 0022: 状態と遷移を別々の代数的データ型で表す](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0022-status-transitions.md)
- tett23/adr-guard [ADR 0023: trace で元の決定から修正ADRをたどる](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0023-trace-amendments.md)
- tett23/adr-guard [ADR 0024: 移行スクリプトに --dry-run を設け、失敗を最後まで数える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0024-migration-dry-run-and-error-count.md)
- tett23/adr-guard [ADR 0025: 新形式のステータス行に、実装待ちの修正、補足、注記、別のリポジトリの修正先を加える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0025-extend-canonical-status-amendments.md)
- tett23/adr-guard [ADR 0026: 旧形式のステータス行の修正先を、リンク先と裸の番号からも読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0026-read-more-legacy-references.md)
- tett23/adr-guard [ADR 0027: 旧形式の `置換（…）` を、修正を受けた状態として読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0027-read-legacy-replaced-status.md)
- tett23/adr-guard [ADR 0028: 旧形式の部分名から修飾子と区切りの記号を除き、決められない部分名を移行しない](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md)
- tett23/adr-guard [リリース v0.11.0](https://github.com/tett23/adr-guard/releases/tag/v0.11.0)
- tett23/adr-guard [リリース v0.12.0](https://github.com/tett23/adr-guard/releases/tag/v0.12.0)
- tett23/adr-guard [リリース v0.13.0](https://github.com/tett23/adr-guard/releases/tag/v0.13.0)
- tett23/adr-guard [リリース v0.13.1](https://github.com/tett23/adr-guard/releases/tag/v0.13.1)
