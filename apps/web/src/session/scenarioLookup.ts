import type {
  CardDef,
  CardId,
  CardInstance,
  CharacterId,
  Scenario,
  SceneDef,
  SceneId,
  Session,
} from "../core/bindings";

export function findCardDef(scenario: Scenario, cardId: CardId): CardDef | undefined {
  return scenario.card_defs.find((def) => def.id === cardId);
}

export function findSceneDef(scenario: Scenario, sceneId: SceneId): SceneDef | undefined {
  for (const phaseDef of scenario.phases) {
    const scene = phaseDef.scenes.find((s) => s.id === sceneId);
    if (scene !== undefined) return scene;
  }
  return undefined;
}

export type HandCard = { instance: CardInstance; def: CardDef | undefined };

/**
 * 手札からCardKind::Markerを除外する(docs/design/client-conventions.md
 * 「手札表示からのMarker除外」。crates/tabifuda-cli/src/play.rsと同じ規約)。
 */
export function visibleHand(session: Session, characterId: CharacterId): HandCard[] {
  const instances = session.hands[characterId] ?? [];
  return instances
    .map((instance) => ({ instance, def: findCardDef(session.scenario, instance.card) }))
    .filter(({ def }) => def?.kind !== "Marker");
}
