import type { ReactNode } from "react";
import type { Event, Outcome, Scenario } from "../core/bindings";
import type { HandlerMap } from "../core/taggedUnion";
import { findCardDef } from "../session/scenarioLookup";

export interface ChronicleContext {
  scenario: Scenario;
}

function cardName(ctx: ChronicleContext, cardId: string): string {
  return findCardDef(ctx.scenario, cardId)?.name ?? cardId;
}

function outcomeLabel(outcome: Outcome): string {
  return outcome === "Victory" ? "勝利" : "敗北";
}

// docs/design/client-conventions.md「Event/Commandの網羅性」参照。
// キーの網羅はHandlerMap<Event, ...>が型で強制する。CardRemovedのように
// 明示的に扱うが描画しないものは、キーを書いた上でnullを返す。
export const eventRenderers: HandlerMap<Event, ReactNode, ChronicleContext> = {
  SessionStarted: (payload, ctx) => (
    <p>
      冒険『{ctx.scenario.meta.title}』が始まった(参加者:{" "}
      {payload.party.map((character) => character.name).join("、")})
    </p>
  ),
  SceneEntered: (payload) => (
    <div>
      <h3>── {payload.scene} ──</h3>
      <p>{payload.narration}</p>
    </div>
  ),
  CardDealt: (payload, ctx) => (
    <p className="chronicle-minor">
      {payload.to} に『{cardName(ctx, payload.card)}』が配られた
    </p>
  ),
  CardPlayed: (payload, ctx) => (
    <div>
      <p>
        {payload.by} は『{cardName(ctx, payload.card)}』を出した。
      </p>
      {payload.free_text !== null && <blockquote>{payload.free_text}</blockquote>}
    </div>
  ),
  CardRemoved: () => null,
  EffectApplied: () => <p className="chronicle-minor">(未解決の効果が記録された)</p>,
  ProposalSubmitted: (payload) => (
    <p>
      {payload.by} が提案した:『{payload.text}』
    </p>
  ),
  ScenarioPatched: (payload) => <p>GMがシナリオを改修した:『{payload.patch.note}』</p>,
  ProposalJudged: (payload) => <p>GMは提案を{payload.accepted ? "採用" : "却下"}した。</p>,
  PhaseAdvanced: (payload) => <h3>── フェーズが {payload.phase} へ ──</h3>,
  SessionEnded: (payload) => <p>=== 冒険の終わり: {outcomeLabel(payload.outcome)} ===</p>,
  RewardsGranted: (payload) => (
    <p>
      {payload.to} は『{payload.cards.map((card) => card.name).join("』『")}』を持ち帰った。
    </p>
  ),
  CardsDiscarded: (payload, ctx) => (
    <p className="chronicle-minor">
      {payload.from} は『
      {payload.cards.map((card) => cardName(ctx, card)).join("』『")}』を持ち出せなかった。
    </p>
  ),
};
