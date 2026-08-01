// @tabifuda/ui のエントリポイント。ビルドレス(srcをそのままexport)のため、
// apps/web・tools/docs-siteの利用側バンドラ(Vite)がトランスパイルする前提。
// docs/design/client-conventions.md「UIコンポーネントの置き場(packages/ui)」参照。

export { ErrorBanner } from "./components/ErrorBanner";
export { FreeTextInput } from "./components/FreeTextInput";
export { GmJudgePanel } from "./components/GmJudgePanel";
export { Hand } from "./components/Hand";
export { ProposalForm } from "./components/ProposalForm";
export { SceneView } from "./components/SceneView";

export { Timeline } from "./chronicle/Timeline";
export { eventRenderers, type ChronicleContext } from "./chronicle/eventRenderers";

export { findCardDef, findSceneDef, visibleHand, type HandCard } from "./session/scenarioLookup";
export { FREE_TEXT_MAX, CARD_NAME_MAX, CARD_TEXT_MAX } from "./session/limits";
export { nextGmCardId, buildAnswerPatch } from "./session/gmResponse";

export type {
  Command,
  Event,
  Session,
  SessionStatus,
  Scenario,
  ScenarioSnapshot,
  WasmError,
  Character,
  CharacterId,
  CardId,
  CardInstance,
  CardInstanceId,
  CardDef,
  CardKind,
  UserId,
  SceneId,
  SceneDef,
  Outcome,
  RuleError,
  Proposal,
  ProposalId,
  PatchOp,
  ScenarioPatch,
  Target,
} from "./core/bindings";

export type { TagOf, PayloadOf, HandlerMap } from "./core/taggedUnion";
export { tagOf, dispatchTagged } from "./core/taggedUnion";
