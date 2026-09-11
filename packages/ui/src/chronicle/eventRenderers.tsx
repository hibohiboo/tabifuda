import type { ReactNode } from "react";
import { Card } from "../components/Card";
import type { CardDef, Event, Outcome, Scenario } from "../core/bindings";
import type { HandlerMap } from "../core/taggedUnion";
import { findCardDef } from "../session/scenarioLookup";
import "./eventRenderers.css";

export interface ChronicleContext {
  scenario: Scenario;
}

function cardName(ctx: ChronicleContext, cardId: string): string {
  return findCardDef(ctx.scenario, cardId)?.name ?? cardId;
}

function outcomeLabel(outcome: Outcome): string {
  return outcome === "Victory" ? "勝利" : "敗北";
}

// カードが登場するイベントの文章の左に小Cardを添える(P6 C3、2026-09-12)。
// CardDefが解決できない場合は何も描画しない(cardName関数の`?? cardId`と
// 同じフォールバック方針: 小Cardは省略しテキストのみで表示を続ける)。
function CardThumbs({ defs }: { defs: (CardDef | undefined)[] }) {
  const found = defs.filter((def): def is CardDef => def !== undefined);
  if (found.length === 0) return null;
  return (
    <div className="tf-chronicle-cards">
      {found.map((def) => (
        <Card key={def.id} name={def.name} kind={def.kind} />
      ))}
    </div>
  );
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
    <div className="tf-chronicle-event">
      <CardThumbs defs={[findCardDef(ctx.scenario, payload.card)]} />
      <p className="chronicle-minor">
        {payload.to} に『{cardName(ctx, payload.card)}』が配られた
      </p>
    </div>
  ),
  CardPlayed: (payload, ctx) => (
    <div className="tf-chronicle-event">
      <CardThumbs defs={[findCardDef(ctx.scenario, payload.card)]} />
      <div>
        <p>
          {payload.by} は『{cardName(ctx, payload.card)}』を出した。
        </p>
        {payload.free_text !== null && <blockquote>{payload.free_text}</blockquote>}
      </div>
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
    <div className="tf-chronicle-event">
      <CardThumbs defs={payload.cards} />
      <p>
        {payload.to} は『{payload.cards.map((card) => card.name).join("』『")}』を持ち帰った。
      </p>
    </div>
  ),
  CardsDiscarded: (payload, ctx) => (
    <div className="tf-chronicle-event">
      <CardThumbs defs={payload.cards.map((card) => findCardDef(ctx.scenario, card))} />
      <p className="chronicle-minor">
        {payload.from} は『
        {payload.cards.map((card) => cardName(ctx, card)).join("』『")}』を持ち出せなかった。
      </p>
    </div>
  ),
};
