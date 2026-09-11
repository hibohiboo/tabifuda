import { useState } from "react";
import type { HandCard } from "../session/scenarioLookup";
import { Card } from "./Card";
import { CardLarge } from "./CardLarge";
import "./Hand.css";
import { Modal } from "./Modal";

export function Hand({
  cards,
  onPlay,
}: {
  cards: HandCard[];
  onPlay: (instanceId: string, freeText: string | null) => void;
}) {
  // タップして展開(モーダル表示)中のCardInstanceId。全種類統一でタップ→
  // 展開→確認の2段階を踏む(P6 C2着手前決定。誤タップでの誤使用を防ぐ)。
  const [selected, setSelected] = useState<string | null>(null);
  const selectedCard = cards.find(({ instance }) => instance.id === selected);

  return (
    <>
      <ul className="tf-hand">
        {cards.map(({ instance, def }) => (
          <li key={instance.id}>
            {def === undefined ? (
              // シナリオにカード定義が見つからない防御的フォールバック
              // (通常到達しない。旧実装から踏襲)
              <button type="button" onClick={() => onPlay(instance.id, null)}>
                {instance.card}
              </button>
            ) : (
              <button
                type="button"
                className="tf-hand__card-button"
                onClick={() => setSelected(instance.id)}
              >
                <Card name={def.name} kind={def.kind} />
              </button>
            )}
          </li>
        ))}
      </ul>
      {selectedCard?.def !== undefined && (
        <Modal onClose={() => setSelected(null)}>
          <CardLarge
            def={selectedCard.def}
            onConfirm={(freeText) => {
              onPlay(selectedCard.instance.id, freeText);
              setSelected(null);
            }}
            onCancel={() => setSelected(null)}
          />
        </Modal>
      )}
    </>
  );
}
