# phase0-task.md タスク3〜5 実行計画

## Context

phase0-task.md のタスク1(docs/配置)・2(CLAUDE.md修正)は完了・コミット済みを確認した
(`docs/` の配置がCLAUDE.mdの構成通り、CLAUDE.mdの「必読文書」にcross-cutting.md /
0001-methodology.mdの2行が既に存在、working treeはclean)。
残りはタスク3(cargo workspace)・4(CI)・5(雑務ファイル)。ドメインロジックは実装しない
(P0の目的は骨格のみ)。受け入れ条件どおり、コミットは
「workspace / CI / 雑務」の意味単位で分割する。

## タスク3: cargo workspace

**ルートに `Cargo.toml`**(workspace定義):
```toml
[workspace]
resolver = "2"
members = ["crates/core", "crates/engine-cli"]
```

**`crates/core/Cargo.toml`**
- package name: `tabifuda-core`(ディレクトリ名は `core` のまま維持。
  Rustの `core` はビルトインクレート名と衝突しうるため、package名は
  プレフィックス付きにする)
- edition: `2021`
- 依存: `serde`(derive feature)、`serde_json`、`thiserror` のみ
- lib crate(`crate-type` 指定なしの通常lib)

**`crates/core/src/lib.rs`**
- クレート先頭のdocコメントに、CLAUDE.md最重要ルール2・3の要約
  (純粋性:IO・時刻・乱数・グローバル状態を持ち込まない/
  すべての進行はイベント:decide/applyを通す)を記載
- ダミー型1つ(例: `pub struct Placeholder;` に「P1で実際のドメイン型に
  置き換わるCI疎通用」の一言doc)
- テスト1本(例: Placeholderの生成・比較程度の自明なテストで、
  `cargo test --workspace` が緑になることを保証)
- Card/Session等ドメイン型は一切書かない(受け入れ条件)

**`crates/engine-cli/Cargo.toml`**
- package name: `tabifuda-engine-cli`
- bin crate、edition `2021`
- 依存なし(coreへの依存はP1でcoreに実APIができてから配線する。
  今つないでも呼び出すものがなく、未使用importになるだけのため)

**`crates/engine-cli/src/main.rs`**
- 挨拶出力のみ。例: `println!("tabifuda engine-cli (P0 scaffold)");`

`apps/` `packages/` は作らない(タスク指示どおり)。

作成後 `cargo fmt --all` / `cargo clippy --workspace -- -D warnings` /
`cargo test --workspace` を実行して全緑を確認してからコミット。

## タスク4: CI (`.github/workflows/ci.yml`)

`on: [push, pull_request]` で単一ワークフロー、3ジョブ構成:

1. **lint-test**(ubuntu-latest)
   - checkout
   - `dtolnay/rust-toolchain@stable`(rust-toolchain.tomlのバージョンを尊重)
     + `Swatinem/rust-cache@v2` でキャッシュ
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo test --workspace`
2. **gitleaks**: `gitleaks/gitleaks-action@v2` でシークレットスキャン
3. **cargo-audit**: `rustsec/audit-check@v2`。
   `continue-on-error: true` を付け、当面は失敗させない
   (cross-cutting.md「依存関係」節: P0は警告ゲート、P4から必須化)

## タスク5: 雑務ファイル

- **`.gitignore`**: Rust(`/target`)、node(`node_modules/`, `dist/`, `*.log`等、
  P3以降のapps/web用に先置き)、`.env` / `.env.*`(ただし `.env.example` は除外しない)
- **`rust-toolchain.toml`**: ローカルの `rustc 1.95.0` に合わせてstableを固定
  ```toml
  [toolchain]
  channel = "1.95.0"
  components = ["rustfmt", "clippy"]
  ```
- **`README.md`**: プロジェクト一行説明+docs/への案内のみ、仕様は書かない
  ```markdown
  # tabifuda

  カード制TRPG(CardWirth風)のRust実装。仕様・設計は docs/ を参照。
  ```

`Cargo.lock` はbinクレート(engine-cli)を含むworkspaceのためコミット対象とする
(gitignoreしない)。

## コミット分割

1. `add cargo workspace (core, engine-cli scaffolds)` — タスク3一式
2. `add CI workflow` — タスク4一式
3. `add misc project files (.gitignore, rust-toolchain, README)` — タスク5一式

各コミット前に fmt/clippy/testを再確認。

## 受け入れ条件チェック(実装後)

- [ ] ローカルで fmt/clippy/test 全緑
- [ ] CLAUDE.mdの「必読文書」参照先が全て実在(既に確認済み、再確認のみ)
- [ ] Card/Session等ドメイン型が存在しない
- [ ] コミットが意味単位で分割されている

## 終わり方

CLAUDE.md「作業の終わり方」に従う:
1. fmt/clippy/test
2. 設計文書との乖離チェック(P0では骨格のみなので通常は乖離なし)
3. 作業中に誤解があれば `docs/agent-journal.md` に1行追記
   (現時点で想定される誤解ポイント: crate名を `tabifuda-core`/
   `tabifuda-engine-cli` とした点はタスク文の字面通りではないため、
   実行後に問題なければジャーナル化不要、ユーザー指摘があれば追記)
