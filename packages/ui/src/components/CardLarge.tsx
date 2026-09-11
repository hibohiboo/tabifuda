// 大サイズカード(本文・自由入力欄まで含む詳細表示。docs/design/
// ui-visual-design.md「カードのビジュアル方針」)。手札からタップして
// モーダル展開する用途を主眼に、確定(出す)/キャンセルの操作まで持つ
// (Hand.tsx参照)。小サイズ専用のCardとはprops要件が大きく異なるため
// 別コンポーネントとして分離する(P6 C2着手前決定)。
// 「出す」「キャンセル」操作はカードの見た目(枠)の外に置く(カード自体は
// 情報表示に専念させる。2026-09-11、ユーザー指定)。
import { useState } from "react";
import "./Card.css";
import type { CardDef } from "../core/bindings";
import { FREE_TEXT_MAX } from "../session/limits";
import { CARD_KIND_COLORS } from "./cardColors";
import { CARD_KIND_ICONS } from "./cardIcons";
import { useAutoFitTitle } from "./useAutoFitTitle";

// onConfirmへ渡すfreeTextを決める。Dialogue以外は常にnull、Dialogueは
// 空欄ならnull(free_textを送らない)、入力があればその文字列。
function resolveFreeText(kind: CardDef["kind"], freeText: string): string | null {
  if (kind !== "Dialogue") return null;
  return freeText === "" ? null : freeText;
}

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
  // Dialogueのみ使う自由入力欄の内容。ボタンをカード外に出したため、
  // 確定操作(onConfirm呼び出し)はカードの外で行うがテキスト自体は
  // カード内で保持する。
  const [freeText, setFreeText] = useState("");
  // 基準1.1rem(=17.6px)。改行が必要な長さの時だけ最小10pxまで縮小する
  // (2026-09-12、ユーザー指定)。
  const { ref, fontSize } = useAutoFitTitle(def.name, 17.6, 10);

  return (
    <div className="tf-card-expand">
      <div className="tf-card tf-card--large" style={{ borderColor: CARD_KIND_COLORS[def.kind] }}>
        <div className="tf-card__title-row">
          <Icon className="tf-card__title-icon" />
          <p className="tf-card__title" ref={ref} style={{ fontSize: `${fontSize}px` }}>
            {def.name}
          </p>
        </div>
        <Icon className="tf-card__icon" />
        {def.text !== "" && <p className="tf-card__text">{def.text}</p>}
        {def.kind === "Dialogue" && (
          <textarea
            className="tf-card__free-text"
            maxLength={FREE_TEXT_MAX}
            placeholder="自由入力(任意。空欄でも出せる)"
            value={freeText}
            onChange={(event) => setFreeText(event.target.value)}
          />
        )}
      </div>
      <div className="tf-card-expand__actions">
        <button type="button" onClick={() => onConfirm(resolveFreeText(def.kind, freeText))}>
          出す
        </button>
        <button type="button" onClick={onCancel}>
          キャンセル
        </button>
      </div>
    </div>
  );
}
