import type { CardId, Scenario, ScenarioPatch } from "../core/bindings";

// crates/tabifuda-cli/src/play.rs「c」の発番規則を踏襲(gm-card-{n}、
// scenario.card_defsと衝突しない最小のn)。一意性の最終検証はdecide内の
// validateが担うため、ここでの衝突回避は防御のためだけではなく単なる
// 候補選びであり、ルール分岐は持たない(domain-model.md「提案への応答UI」)。
export function nextGmCardId(scenario: Scenario): CardId {
  const used = new Set(scenario.card_defs.map((def) => def.id));
  let n = 1;
  while (used.has(`gm-card-${n}`)) {
    n += 1;
  }
  return `gm-card-${n}`;
}

/**
 * 提案に「カードを配って応える」ためのパッチを組み立てる。
 * 作られるCardDefはkind: Scenario・effects: []・requires: []固定
 * (出すと消費され、シーンは変わらない。回答はカード使用時のtext表示で開示)。
 */
export function buildAnswerPatch(
  scenario: Scenario,
  cardName: string,
  answerText: string,
): ScenarioPatch {
  const cardId = nextGmCardId(scenario);
  return {
    ops: [
      {
        AddCardDef: {
          id: cardId,
          name: cardName,
          kind: "Scenario",
          text: answerText,
          tags: [],
          effects: [],
          requires: [],
        },
      },
      { DealCard: { card: cardId, to: "Party" } },
    ],
    note: "提案に応えてカードを配布",
  };
}
