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

// 自由入力欄を持つCardKind。PlayCard.free_textはcore実装上どのkindでも
// 受け付けるが(engine.rs decide_play_card)、UIではDialogue(台詞)と
// Proposal(GMへの提案。domain-model.md「カード」)の2種別のみ表示する
// (ui-visual-design.md「画面ごとのUI方向性」2026-09-12)。
const FREE_TEXT_KINDS: ReadonlySet<CardDef["kind"]> = new Set(["Dialogue", "Proposal"]);

function hasFreeText(kind: CardDef["kind"]): boolean {
  return FREE_TEXT_KINDS.has(kind);
}

// onConfirmへ渡すfreeTextを決める。対象外の種別は常にnull、対象種別は
// 空欄ならnull(free_textを送らない)、入力があればその文字列。
function resolveFreeText(kind: CardDef["kind"], freeText: string): string | null {
  if (!hasFreeText(kind)) return null;
  return freeText === "" ? null : freeText;
}

// Proposalは「タイトル」「内容」の2項目を書いてもらいたいが、
// PlayCard.free_textは単一文字列のため、1つの欄にフォーマット例を
// プレースホルダーとして示す形にする(コマンド構造の変更は伴わない)。
function freeTextPlaceholder(kind: CardDef["kind"]): string {
  if (kind === "Proposal") {
    return "タイトル: 洞窟も調べたい\n内容: 森の外れの洞窟も気になっている";
  }
  return "自由入力(任意。空欄でも出せる)";
}

export function CardLarge({
  def,
  onConfirm,
  onCancel,
}: {
  def: CardDef;
  /** freeTextはDialogue/Proposalのみ非null(自由入力欄の内容、空欄ならnull)。 */
  onConfirm: (freeText: string | null) => void;
  onCancel: () => void;
}) {
  const Icon = CARD_KIND_ICONS[def.kind];
  // Dialogue/Proposalのみ使う自由入力欄の内容。ボタンをカード外に出した
  // ため、確定操作(onConfirm呼び出し)はカードの外で行うがテキスト自体は
  // カード内で保持する。
  const [freeText, setFreeText] = useState("");
  // 基準1.1rem(=17.6px)。改行が必要な長さの時だけ最小11pxまで縮小する
  // (2026-09-12、ユーザー指定)。
  const { ref, fontSize } = useAutoFitTitle(def.name, 17.6, 11);

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
        {hasFreeText(def.kind) && (
          <textarea
            className="tf-card__free-text"
            maxLength={FREE_TEXT_MAX}
            placeholder={freeTextPlaceholder(def.kind)}
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
