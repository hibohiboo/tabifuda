import type { HandCard } from "../session/scenarioLookup";
import { Hand } from "./Hand";

export function SceneView({
  narration,
  hand,
  onPlay,
}: {
  narration: string | undefined;
  hand: HandCard[];
  onPlay: (instanceId: string) => void;
}) {
  return (
    <section>
      <p>{narration ?? ""}</p>
      <Hand cards={hand} onPlay={onPlay} />
    </section>
  );
}
