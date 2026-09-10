---
paths:
  - "crates/tabifuda-core/src/**"
---

# crates/tabifuda-core 実装規約

正はCLAUDE.md最重要ルール2(コアの純粋性)・3(すべての進行はイベント)。
本ファイルはその具体化として、crates/tabifuda-core配下でのRust実装規約を集める。

## enum設計

- Effect / Condition / Event / Command / PatchOp の各enumは追加前提。
  `#[non_exhaustive]` を付け、serdeは種別名を含むタグ付き表現にする

## 型設計

- ID型はnewtypeで包む(生Stringを引き回さない)

## エラー処理

- tabifuda-coreの公開APIにpanicを含めない。エラーは `RuleError` / `PatchError` で返す

## コードコメントからのdocs参照

- コードコメントから docs/tasks/(工程文書)を参照しない。参照してよいのは
  docs/design/(規範)のみ、それもコードから読み取れない制約を指す場合に限る。
  置くのはクレート/モジュールの入口(lib.rs先頭等)のみ、内容を再掲しない
  ポインタ1行に留める(docs/adr/0007-ssot-single-responsibility.md)。
  由来・経緯(どのサイクルで書いたか等)はコミットメッセージ/PRに書く

## テスト

- decideの各Commandに正常系+拒否系(Paused中のPlayCard等)を必ず対で書く。
  詳細な観点は docs/design/test-strategy.md・core-invariantsスキル参照
