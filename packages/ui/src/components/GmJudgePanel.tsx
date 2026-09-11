import { useState } from "react";
import type { Proposal, Scenario, ScenarioPatch } from "../core/bindings";
import { buildAnswerPatch } from "../session/gmResponse";
import { CARD_NAME_MAX, CARD_TEXT_MAX } from "../session/limits";
import { Card } from "./Card";

export function GmJudgePanel({
  proposal,
  scenario,
  onJudge,
  onRespond,
}: {
  proposal: Proposal;
  scenario: Scenario;
  onJudge: (accepted: boolean) => void;
  onRespond: (patch: ScenarioPatch) => void;
}) {
  const [responding, setResponding] = useState(false);
  const [cardName, setCardName] = useState("");
  const [answerText, setAnswerText] = useState("");

  if (responding) {
    return (
      <section aria-label="GM裁定(カードで応える)">
        <p>提案:「{proposal.text}」</p>
        <label>
          カード名
          <input
            value={cardName}
            maxLength={CARD_NAME_MAX}
            onChange={(e) => setCardName(e.target.value)}
          />
        </label>
        <label>
          回答文
          <textarea
            value={answerText}
            maxLength={CARD_TEXT_MAX}
            onChange={(e) => setAnswerText(e.target.value)}
          />
        </label>
        {/* 配るカードの見た目を確認できるプレビュー(P6 C3、2026-09-12)。
            配るカードはkind: "Scenario"固定(gmResponse.ts buildAnswerPatch
            参照)。回答文はtextareaで既に見えているため、Cardには反映しない
            (小サイズCardはタイトル+アイコンのみで本文欄を持たない)。 */}
        <Card name={cardName === "" ? "(カード名未入力)" : cardName} kind="Scenario" />
        <button
          type="button"
          onClick={() => {
            onRespond(buildAnswerPatch(scenario, cardName, answerText));
            setCardName("");
            setAnswerText("");
            setResponding(false);
          }}
        >
          配る
        </button>
        <button type="button" onClick={() => setResponding(false)}>
          戻る
        </button>
      </section>
    );
  }

  return (
    <section aria-label="GM裁定">
      <p>提案が届いています:「{proposal.text}」</p>
      <button type="button" onClick={() => onJudge(true)}>
        採用する
      </button>
      <button type="button" onClick={() => onJudge(false)}>
        却下する
      </button>
      <button type="button" onClick={() => setResponding(true)}>
        カードを配って応える
      </button>
    </section>
  );
}
