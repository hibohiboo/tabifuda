import type { CardDef, CardInstance, Event, HandCard, Proposal, Scenario, WasmError } from "@tabifuda/ui";
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

// Cardコンポーネント(CardKind全6種の見本)用。simple-hunt.jsonにはAction/
// Proposal/Itemが登場しないため、種別を示すためだけの最小サンプルを用意する。
function sampleCard(id: string, name: string, kind: CardDef["kind"]): CardDef {
  return { id, name, kind, text: "", tags: [], effects: [], requires: [] };
}

export const sampleCardsByKind: CardDef[] = [
  sampleCard("sample-action", "斬りかかる", "Action"),
  sampleCard("sample-scenario", "獣の巣に到着する", "Scenario"),
  sampleCard("sample-dialogue", "依頼を受ける", "Dialogue"),
  sampleCard("sample-proposal", "洞窟も調べたい", "Proposal"),
  sampleCard("sample-item", "傷薬", "Item"),
  sampleCard("sample-marker", "依頼受諾", "Marker"),
];

// CardLarge(P6 C2、大サイズ)用。本文(text)まで見せるため空文字にはしない。
function sampleCardLarge(id: string, name: string, kind: CardDef["kind"], text: string): CardDef {
  return { id, name, kind, text, tags: [], effects: [], requires: [] };
}

export const sampleCardsLargeByKind: CardDef[] = [
  sampleCardLarge("sample-large-action", "斬りかかる", "Action", "武器を構え、目の前の獣に斬りかかる。"),
  sampleCardLarge(
    "sample-large-scenario",
    "獣の巣に到着する",
    "Scenario",
    "森の奥、獣の巣とおぼしき洞穴の前に辿り着いた。",
  ),
  sampleCardLarge("sample-large-dialogue", "依頼を受ける", "Dialogue", "村長からの依頼を引き受ける。"),
  sampleCardLarge(
    "sample-large-proposal",
    "洞窟も調べたい",
    "Proposal",
    "森の外れにある洞窟も調べてみたい、とGMに提案する。",
  ),
  sampleCardLarge("sample-large-item", "傷薬", "Item", "傷を癒やす軟膏。使うと体力を少し回復する。"),
  sampleCardLarge("sample-large-marker", "依頼受諾", "Marker", "依頼を受けたことを示す印。"),
];

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
