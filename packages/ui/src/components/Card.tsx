// docs/design/ui-visual-design.md「カードのビジュアル方針」の実装(小サイズのみ。
// 大サイズはP6 C2で追加)。CardDefのname・kindのみを受け取り、タイトルと
// CardKindごとの既定アイコン(cardIcons.tsx)を表示する。
import "./Card.css";
import type { CardKind } from "../core/bindings";
import { CARD_KIND_COLORS } from "./cardColors";
import { CARD_KIND_ICONS } from "./cardIcons";
import { useAutoFitTitle } from "./useAutoFitTitle";

export function Card({ name, kind }: { name: string; kind: CardKind }) {
  const Icon = CARD_KIND_ICONS[kind];
  // 基準0.7rem(=11.2px)。改行が必要な長さの時だけ最小7pxまで縮小する
  // (2026-09-12、ユーザー指定)。
  const { ref, fontSize } = useAutoFitTitle(name, 11.2, 7);
  return (
    <div className="tf-card tf-card--small" style={{ borderColor: CARD_KIND_COLORS[kind] }}>
      <div className="tf-card__title-row">
        <Icon className="tf-card__title-icon" />
        <p className="tf-card__title" ref={ref} style={{ fontSize: `${fontSize}px` }}>
          {name}
        </p>
      </div>
      <Icon className="tf-card__icon" />
    </div>
  );
}
