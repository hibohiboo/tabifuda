import { useState } from "react";
import type { HandCard } from "../session/scenarioLookup";
import { FREE_TEXT_MAX } from "../session/limits";
import { FreeTextInput } from "./FreeTextInput";

export function Hand({
  cards,
  onPlay,
}: {
  cards: HandCard[];
  onPlay: (instanceId: string, freeText: string | null) => void;
}) {
  // 自由入力待ちのCardInstanceId(CardKind::Dialogueのみこの状態を経由する)
  const [armed, setArmed] = useState<string | null>(null);

  return (
    <ul>
      {cards.map(({ instance, def }) => {
        if (armed === instance.id) {
          return (
            <li key={instance.id}>
              <p>{def?.name ?? instance.card}</p>
              <FreeTextInput
                maxLength={FREE_TEXT_MAX}
                placeholder="自由入力(任意。空欄でも出せる)"
                submitLabel="出す"
                onSubmit={(text) => {
                  onPlay(instance.id, text === "" ? null : text);
                  setArmed(null);
                }}
                onCancel={() => setArmed(null)}
              />
            </li>
          );
        }
        return (
          <li key={instance.id}>
            <button
              type="button"
              onClick={() => {
                if (def?.kind === "Dialogue") {
                  setArmed(instance.id);
                } else {
                  onPlay(instance.id, null);
                }
              }}
            >
              {def?.name ?? instance.card}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
