import type { HandCard } from "../session/scenarioLookup";

export function Hand({
  cards,
  onPlay,
}: {
  cards: HandCard[];
  onPlay: (instanceId: string) => void;
}) {
  return (
    <ul>
      {cards.map(({ instance, def }) => (
        <li key={instance.id}>
          <button type="button" onClick={() => onPlay(instance.id)}>
            {def?.name ?? instance.card}
          </button>
        </li>
      ))}
    </ul>
  );
}
