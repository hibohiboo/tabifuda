// docs/design/ui-visual-design.md「カードのビジュアル方針」の実装(小サイズのみ。
// 大サイズはP6 C2で追加)。CardDefのname・kindのみを受け取り、タイトルと
// CardKindごとの既定アイコン(cardIcons.tsx)を表示する。
import "./Card.css";
import type { CardKind } from "../core/bindings";
import { CARD_KIND_COLORS } from "./cardColors";
import { CARD_KIND_ICONS } from "./cardIcons";

export function Card({ name, kind }: { name: string; kind: CardKind }) {
  const Icon = CARD_KIND_ICONS[kind];
  return (
    <div className="tf-card tf-card--small" style={{ borderColor: CARD_KIND_COLORS[kind] }}>
      <Icon className="tf-card__icon" />
      <p className="tf-card__title">{name}</p>
    </div>
  );
}
