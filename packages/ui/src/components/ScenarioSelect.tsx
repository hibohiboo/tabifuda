// 依頼(シナリオ)選択画面(docs/rdra/screens.yaml「依頼選択」)。
// Hand.tsxと同じ「タップ→大サイズをModal展開→確定」の2段階UI
// (ui-visual-design.md「画面ごとのUI方向性」の操作フローを踏襲)。
import { useRef, useState } from "react";
import { Modal } from "./Modal";
import { ScenarioCard } from "./ScenarioCard";
import { ScenarioCardLarge } from "./ScenarioCardLarge";
import "./ScenarioSelect.css";

/** 依頼選択に必要な最小情報。呼び出し側がzod等でScenarioMetaから検証して渡す。 */
export interface ScenarioOption {
  id: string;
  title: string;
  summary: string;
}

export function ScenarioSelect({
  scenarios,
  onSelect,
}: {
  scenarios: ScenarioOption[];
  onSelect: (id: string) => void;
}) {
  const [selected, setSelected] = useState<string | null>(null);
  const selectedScenario = scenarios.find(({ id }) => id === selected);
  // Modalのフォーカス復帰フォールバック先(Hand.tsxと同じ理由。「これで
  // 始める」を選ぶとこの一覧自体がアンマウントされうるため、フォールバック
  // 先自体が失われるケースも想定されるが、Modal側はその場合何もしないだけで
  // 安全)。
  const listRef = useRef<HTMLUListElement>(null);

  return (
    <>
      {scenarios.length === 0 ? (
        // シナリオ検証が全滅した場合等、依頼が1件も無いと「何も起きていない
        // 白い画面」に見える(edge-case-reviewerで指摘、2026-09-12)。
        <p>選べる依頼がありません。</p>
      ) : (
        <ul className="tf-scenario-select" ref={listRef} tabIndex={-1}>
          {scenarios.map((scenario) => (
            <li key={scenario.id}>
              <button
                type="button"
                className="tf-scenario-select__card-button"
                onClick={() => setSelected(scenario.id)}
              >
                <ScenarioCard title={scenario.title} />
              </button>
            </li>
          ))}
        </ul>
      )}
      {selectedScenario !== undefined && (
        <Modal onClose={() => setSelected(null)} restoreFocusFallbackRef={listRef}>
          <ScenarioCardLarge
            title={selectedScenario.title}
            summary={selectedScenario.summary}
            onConfirm={() => {
              // Hand.tsxと同じく、成否によらずモーダルを閉じる(StartSessionが
              // 拒否された場合、閉じないままだとModalのオーバーレイが画面全体を
              // 覆いErrorBannerが隠れて見えなくなる。edge-case-reviewerで指摘、
              // 2026-09-12)。
              onSelect(selectedScenario.id);
              setSelected(null);
            }}
            onCancel={() => setSelected(null)}
          />
        </Modal>
      )}
    </>
  );
}
