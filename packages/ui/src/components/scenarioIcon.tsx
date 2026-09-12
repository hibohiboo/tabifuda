// 依頼(シナリオ)選択カード専用アイコン・縁取り色(張り紙をイメージした
// 自作SVG)。CardKind(cardIcons.tsx/cardColors.ts)とは無関係の別概念
// (ScenarioMetaの表示用)のため別ファイルに分離する(1ファイル1責務)。
// CardKindのような複数バリアントの網羅マップではなく、依頼カードは常に
// この1種のみを使う。
import type { SVGProps } from "react";

// docs/design/ui-visual-design.md「配色・フォント・実装手段」参照。
export const SCENARIO_CARD_COLOR = "#8b6b4a";

export function ScenarioIcon(props: SVGProps<SVGSVGElement>) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.5}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      {...props}
    >
      {/* 張り紙本体+四隅の画鋲跡 */}
      <rect x="4" y="3" width="16" height="18" rx="1" />
      <path d="M8 8h8M8 12h8M8 16h5" />
    </svg>
  );
}
