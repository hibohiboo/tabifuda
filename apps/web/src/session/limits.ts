// 実行時の自由入力の長さ上限(cross-cutting.md「自由入力(UGC)の取り扱い」、
// domain-model.md「文字列の長さ上限」。PlayCard.free_text/Propose.text/
// ScenarioPatch.noteはいずれもBoundedString<4096>)。
export const FREE_TEXT_MAX = 4096;
