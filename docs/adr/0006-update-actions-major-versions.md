# ADR 0006: GitHub Actionsのアクションを最新のメジャー版に更新する

ステータス: 採択

## 文脈

ワークフローで使っているGitHub公式のアクションは、[ADR 0003](0003-ci.md) と [ADR 0004](0004-binary-distribution.md) の時点で選んだ `@v4` のままである。
[ADR 0005](0005-migrate-to-adr-guard.md) で `adr` ジョブだけを `actions/checkout@v7` にしたため、同じワークフローの中でバージョンが混在している。

運用の方針として、GitHub ActionsはNode 24以降で動く最新のメジャー版を使うことになった。
`@v4` はいずれもNode 20で動く。

2026-10-13時点の各アクションの状況は以下のとおり。

| アクション | 現在 | 最新 | 最新の `runs.using` |
| --- | --- | --- | --- |
| `actions/checkout` | `v4`（`adr` ジョブは `v7`） | `v7.0.1` | `node24` |
| `actions/upload-artifact` | `v4` | `v7.0.2` | `node24` |
| `actions/download-artifact` | `v4` | `v8.0.2` | `node24` |

### 間のメジャー版での破壊的変更

各リリースノートから、このリポジトリの使い方に影響しうるものを挙げる。

- `actions/checkout`
  - v5: Node 24で動く。ランナー v2.327.1 以降が必要。
  - v6: 認証情報を `.git/config` ではなく別のファイルに保存する。
  - v7: `pull_request_target` と `workflow_run` でフォークのpull requestをチェックアウトすることを拒否する。ESMへの移行。
- `actions/upload-artifact`
  - v5・v6: Node 24で動く。ランナー v2.327.1 以降が必要。
  - v7: `archive: false` で単一ファイルを圧縮せずにアップロードできる（既定は従来どおり圧縮する）。ESMへの移行。
- `actions/download-artifact`
  - v5: IDで単一のアーティファクトを指定した場合の展開先が、`path/<名前>/` から `path/` に変わる。
  - v6・v7: Node 24で動く。ランナー v2.327.1 以降が必要。
  - v8: ハッシュの不一致を既定でエラーにする。`Content-Type` が圧縮形式でないものは展開しない。ESMへの移行。

このリポジトリは以下の使い方しかしていないため、いずれも影響しないと判断する。

- `actions/checkout` はGitHubがホストするランナー上で、`push`、`pull_request`、`workflow_dispatch`、タグのpushでのみ使う。`pull_request_target` と `workflow_run` は使わない。チェックアウト後にgitの認証情報を使う処理は無い。
- `actions/upload-artifact` は `.tar.gz` をアーティファクトとして圧縮してアップロードする（`archive` は既定のまま）。
- `actions/download-artifact` はIDを指定せず、全アーティファクトを `merge-multiple: true` で `dist/` に展開する。

## 実装すること

- `.github/workflows/ci.yml`
  - `check` ジョブの `actions/checkout@v4` を `actions/checkout@v7` にする。
- `.github/workflows/release.yml`
  - `build` ジョブの `actions/checkout@v4` を `actions/checkout@v7` にする。
  - `actions/upload-artifact@v4` を `actions/upload-artifact@v7` にする。
  - `actions/download-artifact@v4` を `actions/download-artifact@v8` にする。
- `check` ジョブと `build` ジョブのチェックアウトに `persist-credentials: false` を付ける。どちらのジョブもgitの認証情報を使わないため。`adr` ジョブと揃える。
- アクションはメジャー版のタグ（`@v7` など）で指定する。

## 実装しないこと

- Dependabotなどによるアクションの自動更新は導入しない。
- アクションのコミットSHAでの固定は行わない。GitHub公式のアクションであり、メジャー版のタグで追従する。
- `tett23/adr-guard` のバージョンは変えない（[ADR 0005](0005-migrate-to-adr-guard.md) のとおり `v0.2.4`）。
- `upload-artifact` の `archive: false` による直接アップロードは使わない。アーティファクトの形式は変えない。
- 仕様（`docs/specifications.md`）とREADMEに書いている挙動は変わらないため、ADR一覧の追加以外は編集しない。

## テスト設計

- `ci.yml`: 更新のpull requestで `check`（Ubuntu、macOS）と `adr` の全ジョブが成功することを確認する。
- `release.yml` の `build` ジョブ: マージ後に `main` で `workflow_dispatch` を実行し、以下を確認する。
  - 4ターゲットの `build` がすべて成功し、`release` ジョブはスキップされる。
  - `gh run download` で取得したアーカイブの中身（バイナリ、`LICENSE`、`README.md`）が [ADR 0004](0004-binary-distribution.md) のとおりである。
- `release.yml` の `release` ジョブ（`download-artifact`）: タグのpushでしか動かないため、次のリリースで確認する。
  - リリースに4つのアーカイブと `SHA256SUMS` が添付され、`shasum -a 256 -c SHA256SUMS` が通ることを確認する。

## トレードオフ

- `download-artifact@v8` は次のリリースまで実際には動かない。失敗した場合、タグは作られたがリリースが無い状態になる。その場合は原因を直したうえでジョブを再実行する。タグは作り直さない。
- メジャー版のタグで指定するため、アクション側のマイナー・パッチ更新は自動で取り込まれる。再現性よりも、公式アクションの修正に追従することを優先する。
- 最新のメジャー版はランナー v2.327.1 以降を要求する。GitHubがホストするランナーだけを使うため問題にならないが、セルフホストランナーを導入する場合は注意が必要になる。
