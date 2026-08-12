// CardKind(crates/tabifuda-core/src/card.rs)ごとの既定アイコン(自作SVG)。
// docs/design/ui-visual-design.md「カードのビジュアル方針」参照
// (調達方法は同文書に2026-08-12決定として追記済み)。
//
// CardKindは単純なリテラル合併型(HandlerMap<U,...>が前提とする単一キー
// オブジェクト形式ではない)のため、網羅性はRecord<CardKind, ...>で取る。
// 新しいCardKind variantが増えるとこのオブジェクトリテラルはキー不足で
// tscエラーになる(docs/design/client-conventions.md「Event/Commandの
// 網羅性」と同じ考え方をCardKindに適用したもの)。
import type { ReactElement, SVGProps } from "react";
import type { CardKind } from "../core/bindings";

type IconProps = SVGProps<SVGSVGElement>;

function IconBase({ children, ...props }: IconProps) {
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
      {children}
    </svg>
  );
}

// Action(行動): 即座の働きかけを示す稲妻
function ActionIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M13 2 4 14h6l-1 8 9-12h-6z" />
    </IconBase>
  );
}

// Scenario(進行): 物語を綴じた本
function ScenarioIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M4 5.5A1.5 1.5 0 0 1 5.5 4H11v16H5.5A1.5 1.5 0 0 1 4 18.5z" />
      <path d="M20 5.5A1.5 1.5 0 0 0 18.5 4H13v16h5.5a1.5 1.5 0 0 0 1.5-1.5z" />
    </IconBase>
  );
}

// Dialogue(台詞): 発言の吹き出し
function DialogueIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M4 4.5h16v11H9l-4 4v-4H4z" />
    </IconBase>
  );
}

// Proposal(提案): ひらめきの電球
function ProposalIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M12 3a6 6 0 0 0-3.6 10.8c.5.4.85 1 .95 1.7h5.3c.1-.7.45-1.3.95-1.7A6 6 0 0 0 12 3z" />
      <path d="M9.5 18.5h5" />
      <path d="M10.3 21h3.4" />
    </IconBase>
  );
}

// Item(持ち物): 道具袋
function ItemIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M8 8V6.5a4 4 0 0 1 8 0V8" />
      <rect x="3.5" y="8" width="17" height="12" rx="2" />
    </IconBase>
  );
}

// Marker(印): 選択・成立を示す旗
function MarkerIcon(props: IconProps) {
  return (
    <IconBase {...props}>
      <path d="M5.5 3v18" />
      <path d="M5.5 4h13l-3.2 4 3.2 4h-13z" />
    </IconBase>
  );
}

/** CardKindごとの既定アイコン。新variant追加時はここに追記する(網羅性チェックは冒頭コメント参照)。 */
export const CARD_KIND_ICONS: Record<CardKind, (props: IconProps) => ReactElement> = {
  Action: ActionIcon,
  Scenario: ScenarioIcon,
  Dialogue: DialogueIcon,
  Proposal: ProposalIcon,
  Item: ItemIcon,
  Marker: MarkerIcon,
};
