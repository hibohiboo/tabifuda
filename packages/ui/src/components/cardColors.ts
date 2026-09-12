// CardKind(crates/tabifuda-core/src/card.rs)ごとの縁取り色。
// 既定アイコン(cardIcons.tsx)に加えて色でも即座に種別を判別できるようにする
// (docs/design/ui-visual-design.md「配色・フォント・実装手段」2026-09-11決定)。
// cardIcons.tsxと同じ理由でRecord<CardKind, string>により網羅性を保証する
// (新しいCardKind variantが増えるとキー不足でtscエラーになる)。
import type { CardKind } from "../core/bindings";

export const CARD_KIND_COLORS: Record<CardKind, string> = {
  Action: "#d97a3d",
  Scenario: "#4d8fc4",
  Dialogue: "#5aab6b",
  Proposal: "#a675d1",
  Item: "#c2a23a",
  Marker: "#7d7d84",
};
