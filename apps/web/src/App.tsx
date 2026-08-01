import { ErrorBanner } from "./components/ErrorBanner";
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

  const handlePlay = (instanceId: string) => {
    dispatch({
      PlayCard: { by: SOLO_CHARACTER_ID, card: instanceId, free_text: null },
    });
  };

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
        />
      )}
      {session !== null &&
        typeof session.status === "object" &&
        "Ended" in session.status && (
          <p>{session.status.Ended === "Victory" ? "勝利" : "敗北"}</p>
        )}
      {session !== null &&
        typeof session.status === "object" &&
        "Paused" in session.status && <p>予期しない状態です(提案の裁定待ち)</p>}
      <ErrorBanner error={error} />
      {session !== null && <Timeline events={events} scenario={session.scenario} />}
    </main>
  );
}

export default App;
