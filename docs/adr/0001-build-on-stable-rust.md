# ADR 0001: stable Rustの最新版でビルドできるようにする

ステータス: 採択

## 文脈

`src/main.rs` は `#![feature(box_syntax)]` と `box` 式を使っている。
`box_syntax` はnightlyからも削除済みで、現在はnightly・stableのどちらでもビルドできない（E0557）。
また `edition = "2018"` と `extern crate dirs;` は古い書き方のまま残っている。

## 実装すること

- `#![feature(box_syntax)]` を削除し、`box f` を `Box::new(f)` に置き換える。
- `Cargo.toml` の `edition` を `2024` に上げる。
- 不要になった `extern crate dirs;` を削除する。
- `cargo clippy --all-targets -- -D warnings` が通るように既存コードを修正する。
  - `env::args().last()` を `next_back()` に置き換える。
  - `MAIN_SEPARATOR.to_string()` の参照を `MAIN_SEPARATOR_STR` に置き換える。
  - 末尾の `()`、`print!` 引数の不要な参照、不要な `to_string()` を削除する。
- `cargo fmt` で整形する。
- stable 1.99.0 で `cargo build`、`cargo test`、`cargo clippy` が通ることを確認する。

## 実装しないこと

- コマンドの挙動（ホームディレクトリの `~` 置換、親ディレクトリの頭文字化）は変更しない。
- `dirs` クレートの更新や再実装は行わない。
- `rust-toolchain.toml` によるツールチェインの固定は行わない。

## テスト設計

挙動を変えないことを確認するため、`shorten_dirs` に単体テストを追加する。

- `~/foo/bar/baz` → `~/f/b/baz`
- `~/foo` → `~/foo`
- `~` で始まらないパス（`/usr/local/bin`）はそのまま返す

`replace_home` は実行環境のホームディレクトリに依存するため、`dirs::home_dir()` を基準にした入力で `~` に置換されることを確認する。

## トレードオフ

- edition 2024 はrustc 1.85以降を要求するため、それより古いツールチェインではビルドできなくなる。
- `Box::new` は `box` 式と違い、ヒープ上への直接構築が保証されないが、関数ポインタ程度の大きさなので影響はない。
