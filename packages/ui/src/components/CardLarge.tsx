// 大サイズカード(本文・自由入力欄まで含む詳細表示。docs/design/
// ui-visual-design.md「カードのビジュアル方針」)。手札からタップして
// モーダル展開する用途を主眼に、確定(出す)/キャンセルの操作まで持つ
// (Hand.tsx参照)。小サイズ専用のCardとはprops要件が大きく異なるため
// 別コンポーネントとして分離する(P6 C2着手前決定)。
import "./Card.css";
import type { CardDef } from "../core/bindings";
import { FREE_TEXT_MAX } from "../session/limits";
import { CARD_KIND_COLORS } from "./cardColors";
import { CARD_KIND_ICONS } from "./cardIcons";
import { FreeTextInput } from "./FreeTextInput";

export function CardLarge({
  def,
  onConfirm,
  onCancel,
}: {
  def: CardDef;
  /** freeTextはCardKind::Dialogueのみ非null(自由入力欄の内容、空欄ならnull)。 */
  onConfirm: (freeText: string | null) => void;
  onCancel: () => void;
}) {
  const Icon = CARD_KIND_ICONS[def.kind];

  return (
    <div className="tf-card tf-card--large" style={{ borderColor: CARD_KIND_COLORS[def.kind] }}>
      <Icon className="tf-card__icon" />
      <p className="tf-card__title tf-card__title--large">{def.name}</p>
      {def.text !== "" && <p className="tf-card__text">{def.text}</p>}
      {def.kind === "Dialogue" ? (
        <FreeTextInput
          maxLength={FREE_TEXT_MAX}
          placeholder="自由入力(任意。空欄でも出せる)"
          submitLabel="出す"
          onSubmit={(text) => onConfirm(text === "" ? null : text)}
          onCancel={onCancel}
        />
      ) : (
        <div className="tf-card__actions">
          <button type="button" onClick={() => onConfirm(null)}>
            出す
          </button>
          <button type="button" onClick={onCancel}>
            キャンセル
          </button>
        </div>
      )}
    </div>
  );
}
