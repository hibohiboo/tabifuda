import type { ScenarioPatch } from "./core/bindings";
import { ErrorBanner } from "./components/ErrorBanner";
import { GmJudgePanel } from "./components/GmJudgePanel";
import { SceneView } from "./components/SceneView";
import { Timeline } from "./chronicle/Timeline";
import { findSceneDef, visibleHand } from "./session/scenarioLookup";
import { createSoloCharacter, SOLO_ACTOR, SOLO_CHARACTER_ID } from "./session/soloParty";
import { useGameSession } from "./session/useGameSession";
import { simpleHunt } from "./scenario/simpleHunt";

function App() {
  const { events, session, error, dispatch } = useGameSession(SOLO_ACTOR);

  const handleStart = () => {
    dispatch({
      StartSession: { scenario: simpleHunt, party: [createSoloCharacter()] },
    });
  };

  const handlePlay = (instanceId: string, freeText: string | null) => {
    dispatch({
      PlayCard: { by: SOLO_CHARACTER_ID, card: instanceId, free_text: freeText },
    });
  };

  const handlePropose = (text: string) => {
    dispatch({ Propose: { by: SOLO_CHARACTER_ID, text } });
  };

  const handleJudge = (proposalId: string, accepted: boolean) => {
    dispatch({ JudgeProposal: { proposal: proposalId, accepted } });
  };

  const handleRespond = (patch: ScenarioPatch) => {
    dispatch({ ApplyPatch: { patch } });
  };

  // session.pending_proposalはstatus === {"Paused":...}の間だけnon-null
  // (domain-model.md「提案と裁定」の状態機械の不変条件)
  const pendingProposal = session?.pending_proposal ?? null;

  return (
    <main>
      <h1>{simpleHunt.meta.title}</h1>
      {session === null && (
        <button type="button" onClick={handleStart}>
          はじめる
        </button>
      )}
      {session !== null && session.status === "Running" && (
        <SceneView
          narration={findSceneDef(session.scenario, session.scene)?.narration}
          hand={visibleHand(session, SOLO_CHARACTER_ID)}
          onPlay={handlePlay}
          onPropose={handlePropose}
        />
      )}
      {session !== null &&
        typeof session.status === "object" &&
        "Ended" in session.status && (
          <p>{session.status.Ended === "Victory" ? "勝利" : "敗北"}</p>
        )}
      {session !== null && pendingProposal !== null && (
        <GmJudgePanel
          proposal={pendingProposal}
          scenario={session.scenario}
          onJudge={(accepted) => handleJudge(pendingProposal.id, accepted)}
          onRespond={handleRespond}
        />
      )}
      <ErrorBanner error={error} />
      {session !== null && <Timeline events={events} scenario={session.scenario} />}
    </main>
  );
}

export default App;
