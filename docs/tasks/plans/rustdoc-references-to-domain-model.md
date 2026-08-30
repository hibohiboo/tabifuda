# タスク: crates/ lib.rs のrustdoc参照先を domain-model.md へ統一

## 概要

[docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
で確定した方針に基づき、crates/ のrustdocコメントから基本原則への参照先を
CLAUDE.md から domain-model.md へ変更する。

## 背景

コアの基本原則(純粋性・decide/apply・イベント経由・乱数決定性)が複数の文書に
再掲されている SSoT違反が見つかった。正は domain-model.md「基本原則」に統一し、
CLAUDE.md と lib.rs は参照による誘導に変える対応の一部。

同時に、Rust規約(CLAUDE.md「コードコメントから docs/tasks/ を参照しない。
参照してよいのは docs/design/(規範)のみ」)に照らすと、現状の lib.rs が
CLAUDE.md を参照しているのは規約違反。domain-model.md は設計(規範)文書なので参照OK。

## 変更対象

- [crates/tabifuda-core/src/lib.rs](../../../crates/tabifuda-core/src/lib.rs) L3-8
  rustdocのコメント。CLAUDE.md を参照する部分を domain-model.md へ変更し、
  全文再掲をやめて1行ポインタに縮める
- [crates/tabifuda-wasm/src/lib.rs](../../../crates/tabifuda-wasm/src/lib.rs) L4-7
  同上。同様に参照先をCLAUDE.md から domain-model.md へ変更

## スコープ

- Rust コード(lib.rs)の rustdoc コメント部分のみ変更
- docs/ は一切変更しない
- テスト等の検証は cargo test でOK(rustdoc自体は仕様に影響しない)

## 終わり方

1. 2つの lib.rs の該当箇所を修正
2. `cargo test --workspace` で検証
3. コード変更を伴うため、作業ブランチ→PR→人間判断でマージ
4. 誤解があれば agent-journal.md に1行記録

## 関連

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  (SSoT方針・正の所在決定)
- [docs/design/domain-model.md](../../design/domain-model.md)L10-14
  (正の所在：基本原則)
