// 長さ上限(cross-cutting.md「自由入力(UGC)の取り扱い」、
// domain-model.md「文字列の長さ上限」)。

// 実行時の自由入力。PlayCard.free_text/Propose.text/ScenarioPatch.noteは
// いずれもBoundedString<4096>。
export const FREE_TEXT_MAX = 4096;

// 作者データ(GM裁定UIの「カードを配って応える」で作られるCardDef)。
// CardDef.nameは識別的な短いテキスト(BoundedString<200>)、CardDef.textは
// 説明的な長いテキスト(BoundedString<2000>)。
export const CARD_NAME_MAX = 200;
export const CARD_TEXT_MAX = 2000;
