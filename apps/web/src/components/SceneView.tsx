import type { HandCard } from "../session/scenarioLookup";
import { Hand } from "./Hand";
import { ProposalForm } from "./ProposalForm";

export function SceneView({
  narration,
  hand,
  onPlay,
  onPropose,
}: {
  narration: string | undefined;
  hand: HandCard[];
  onPlay: (instanceId: string, freeText: string | null) => void;
  onPropose: (text: string) => void;
}) {
  return (
    <section>
      <p>{narration ?? ""}</p>
      <Hand cards={hand} onPlay={onPlay} />
      <ProposalForm onPropose={onPropose} />
    </section>
  );
}
