// 依頼(シナリオ)選択カード(小サイズ)。docs/design/ui-visual-design.md
// 「カードのビジュアル方針」の白銀比・小サイズ規格をCardDefと同様に流用するが、
// 表示対象はScenarioMeta(id/title/summary)でありCardDefとは型が異なるため
// Card.tsxとは別コンポーネントにする(2026-09-12決定)。
import "./Card.css";
import { SCENARIO_CARD_COLOR, ScenarioIcon } from "./scenarioIcon";
import { useAutoFitTitle } from "./useAutoFitTitle";

export function ScenarioCard({ title }: { title: string }) {
  // 基準0.7rem(=11.2px)。Card.tsxと同じ基準値(2026-09-12)。
  const { ref, fontSize } = useAutoFitTitle(title, 11.2, 7);
  return (
    <div className="tf-card tf-card--small" style={{ borderColor: SCENARIO_CARD_COLOR }}>
      <p className="tf-card__title" ref={ref} style={{ fontSize: `${fontSize}px` }}>
        {title}
      </p>
      <ScenarioIcon className="tf-card__icon" />
    </div>
  );
}
