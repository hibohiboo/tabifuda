---
paths:
  - "crates/*/src/**"
---

# crates/ 実装規約(中心は tabifuda-core)

正はCLAUDE.md最重要ルール2(コアの純粋性)・3(すべての進行はイベント)。
本ファイルはその具体化として、crates/ 配下でのRust実装規約を集める。
項目のうち「tabifuda-core」と明記したものは core のみ、それ以外は
cli / wasm を含む全クレートに適用する。

## enum設計

- Effect / Condition / Event / Command / PatchOp の各enumは追加前提。
  `#[non_exhaustive]` を付け、serdeは種別名を含むタグ付き表現にする

## 型設計

- ID型はnewtypeで包む(生Stringを引き回さない)

## エラー処理

- tabifuda-coreの公開APIにpanicを含めない。エラーは `RuleError` / `PatchError` で返す

## コードコメントからのdocs参照(全クレート)

- コードコメントから docs/tasks/(工程文書)を参照しない。参照してよいのは
  docs/design/(規範)のみ、それもコードから読み取れない制約を指す場合に限る。
  置くのはクレート/モジュールの入口(lib.rs先頭等)のみ、内容を再掲しない
  ポインタ1行に留める(docs/adr/0007-ssot-single-responsibility.md)。
  由来・経緯(どのサイクルで書いたか等)はコミットメッセージ/PRに書く

## テスト

- 正は docs/design/test-strategy.md。観点の索引は core-invariants スキル
