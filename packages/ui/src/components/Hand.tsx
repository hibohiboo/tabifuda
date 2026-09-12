import { useRef, useState } from "react";
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
  // 「出す」で選んだカードが手札から除去されると、モーダルを開く前に
  // フォーカスしていたボタン自体がDOMから消える。Modalのフォーカス復帰が
  // 空振りした場合のフォールバック先として手札一覧自体を渡す
  // (edge-case-reviewerで指摘、2026-09-12)。
  const handRef = useRef<HTMLUListElement>(null);

  return (
    <>
      <ul className="tf-hand" ref={handRef} tabIndex={-1}>
        {cards.map(({ instance, def }) => (
          <li key={instance.id}>
            {def === undefined ? (
              // シナリオにカード定義が見つからないのはシナリオデータの
              // 不整合(パッチ適用ミス等)であり異常系。「全種類統一で
              // タップ→展開→確認」の例外として確認なしに即時使用させず、
              // 無効化して使用不能にする(edge-case-reviewerで指摘、
              // 2026-09-11。通常到達しない経路のため実害は無いが、異常系
              // でこそ確認省略・即時実行を避ける)
              <button type="button" disabled title="カード定義を解決できません">
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
        <Modal onClose={() => setSelected(null)} restoreFocusFallbackRef={handRef}>
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
