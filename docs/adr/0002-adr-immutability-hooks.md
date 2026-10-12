# ADR 0002: コミット済みADRの変更をhookで禁止する

ステータス: 採択→ADR 0005

## 文脈

ADRはコミット後、ステータスの行を除いて変更不可とする運用である。
人の注意だけに頼ると、ClaudeやコミッタがADRを書き換えてしまう可能性がある。
そこで、ファイル編集の時点（Claude Code）とコミットの時点（git）の二か所で機械的に検査する。

## 実装すること

### 検査の定義

- 対象は、比較元のリビジョンに存在する `docs/adr/*.md` とする。
- 比較元と比較先の内容から `ステータス:` で始まる行を除いた結果が一致しなければ違反とする。
- 比較先で対象ファイルが削除またはリネームされている場合も違反とする。
- 新規ADRの追加は許可する。

### `scripts/check-adr-immutable.sh`

- `scripts/check-adr-immutable.sh [比較元] [比較先]` の形式で呼ぶPOSIX shスクリプト。
- 比較元の既定値は `HEAD`、比較先の既定値はgitのインデックス（ステージ済みの内容）とする。
- 違反したファイルを標準エラー出力に列挙し、終了コード1で終了する。
- 比較元が存在しない場合（初回コミット前）は何もせず成功とする。

### git pre-commit hook

- `.githooks/pre-commit` から上記スクリプトを引数なしで呼ぶ。
- `git config core.hooksPath .githooks` で有効化する。手順はREADMEに記載する。

### Claude Code hook

- `.claude/settings.json` に `PreToolUse` hook（matcher: `Edit|Write`）を登録する。
- `.claude/hooks/protect-adr.sh` が標準入力のJSONを `jq` で読み、`HEAD` に存在する `docs/adr/*.md` への操作を検査する。
  - `Write`: 書き込む内容と `HEAD` の内容を、`ステータス:` 行を除いて比較する。
  - `Edit`: `old_string` と `new_string` を、`ステータス:` 行を除いて比較する。一致すれば変更はステータス行に限られるとみなす。
- 違反時は理由を標準エラー出力に書き、終了コード2で操作を拒否する。

## 実装しないこと

- Bashツール経由（`sed` や `rm` など）の変更はClaude Code hookでは検査しない。git pre-commit hookとCIで検出する。
- 監査や性能計測のスナップショットなど、`docs/adr/` 以外の変更不可ファイルは対象にしない。該当ファイルが追加された時点で別のADRで扱う。
- ステータス行の値（`採択` など）の妥当性は検査しない。

## テスト設計

hookはシェルスクリプトのため、手動で以下を確認する。

- pre-commit
  - 新規ADRの追加はコミットできる。
  - コミット済みADRのステータス行だけの変更はコミットできる。
  - コミット済みADRの本文の変更はコミットできない。
  - コミット済みADRの削除とリネームはコミットできない。
- Claude Code hook
  - 未コミットのADRは自由に編集できる。
  - コミット済みADRのステータス行のEditは許可される。
  - コミット済みADRの本文のEdit、Writeは拒否される。
  - `docs/adr/` 以外のファイルは影響を受けない。

CI（[ADR 0003](0003-ci.md)）でも同じスクリプトを実行し、hookを有効化していない環境からの変更を検出する。

## トレードオフ

- git hookはclone後に `core.hooksPath` を設定しないと有効にならない。CIで同じ検査を行うことで補う。
- Claude Code hookは `jq` に依存する。
- `Edit` の検査は `old_string` と `new_string` の比較で行うため、ステータス行の一部だけを置換するEditは、ステータス行全体を含めないと拒否される。
- `git commit --no-verify` でpre-commit hookは回避できる。
