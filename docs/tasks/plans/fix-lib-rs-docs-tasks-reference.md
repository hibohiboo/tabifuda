# タスク: lib.rs のコメントが docs/tasks/ を参照している規約違反の是正

## 概要

[crates/tabifuda-core/src/lib.rs](../../../crates/tabifuda-core/src/lib.rs) L42-43
のコメントが `docs/tasks/tools/docs-site/task.md` を直接参照しており、
CLAUDE.md Rust規約「コードコメントから docs/tasks/(工程文書)を参照しない。
参照してよいのは docs/design/(規範)のみ」に反する。

## 発見の経緯

ssot-single-responsibility-rules.md のフォルダ構成レビュー(item 4: テスト
配置の規範化)対応中に偶然発見。同種の違反は2026-07-20に6箇所是正された
記録が agent-journal.md にあるが、本箇所は当時の是正後に追加されたか
見落とされたものと見られる。現在の作業スコープ外のため、その場で直さず
本チケットとして切り出した。

## 該当箇所

```rust
// テスト関数名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md
// D2「テストビュー」。cargo test出力がそのままGitHub Pagesの説明文になる)。
```

## 対応方針

- 「テスト関数名を日本語にする」という制約自体はコードから読み取れない
  外部要請(docs-siteのビューア都合)なので、コメント自体を消すのではなく
  参照先を規約に合う形に直す
- 案: 制約の一次情報を docs/design/test-strategy.md 側に1〜2行追記し、
  lib.rs のコメントはそちらを参照する形にする(2026-07-20の是正時の
  パターンを踏襲)

## スコープ

- crates/tabifuda-core/src/lib.rs のコメント修正
- 必要なら docs/design/test-strategy.md への制約の反映(移設)

## 終わり方

1. 参照先を docs/design/ 配下に統一
2. `cargo test --workspace` で検証(コメントのみの変更のため実質影響なし)
3. 誤解があれば agent-journal.md に1行記録
4. 作業ブランチで行い、マージは人間判断

## 関連

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
- [docs/agent-journal.md](../../agent-journal.md) 2026-07-20エントリ(同種の過去是正)
