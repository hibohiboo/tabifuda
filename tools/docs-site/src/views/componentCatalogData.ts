import type { CardInstance, Event, HandCard, Proposal, Scenario, WasmError } from "@tabifuda/ui";
import raw from "../../../../shared/scenarios/simple-hunt.json";

// コンポーネントカタログの表示用サンプルデータ。実プレイの状態遷移を厳密に
// 再現するものではなく、各コンポーネントを1例ずつ見せるための静的な値。
// 「単純討伐」(shared/scenarios/simple-hunt.json)のカード定義を流用する。
export const sampleScenario = raw as Scenario;

function cardInstance(id: string, card: string): CardInstance {
  return { id, card };
}

export const sampleHand: HandCard[] = [
  { instance: cardInstance("inst-1", "reply"), def: sampleScenario.card_defs[0] },
  { instance: cardInstance("inst-2", "arrive"), def: sampleScenario.card_defs[2] },
];

export const sampleProposal: Proposal = {
  id: "proposal-1",
  by: "hunter",
  text: "森の外れにある洞窟も調べてみたい",
};

export const sampleError: WasmError = {
  kind: "decode",
  error: "サンプルエラー(表示例)",
};

export const sampleEvents: Event[] = [
  {
    SessionStarted: {
      scenario: sampleScenario,
      party: [{ id: "hunter", name: "旅人", stats: {}, deck: [], owned_cards: [] }],
      roles: { solo: { Player: { characters: ["hunter"] } } },
      initial_phase: "Opening",
      initial_scene: "op_request",
    },
  },
  {
    SceneEntered: {
      scene: "op_request",
      narration: sampleScenario.phases[0].scenes[0].narration,
      local_instances: ["inst-1"],
    },
  },
  { CardPlayed: { by: "hunter", card: "reply", free_text: "引き受けよう" } },
  { ScenarioPatched: { patch: { ops: [], note: "GMが応答した" } } },
  { SessionEnded: { outcome: "Victory" } },
];
