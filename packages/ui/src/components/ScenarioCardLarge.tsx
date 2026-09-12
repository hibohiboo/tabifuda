// 依頼(シナリオ)選択カード(大サイズ)。CardLarge.tsxと同じ「タップ→
// 展開→確定」の2段階UI(ui-visual-design.md「画面ごとのUI方向性」)を
// 依頼選択でも踏襲するが、表示対象がScenarioMeta(id/title/summary)で
// CardDefとは型が異なるため別コンポーネントにする(2026-09-12決定)。
// 自由入力欄は持たない(依頼選択に自由入力は無い)。
import { SCENARIO_CARD_COLOR, ScenarioIcon } from "./scenarioIcon";
import "./Card.css";
import { useAutoFitTitle } from "./useAutoFitTitle";

export function ScenarioCardLarge({
  title,
  summary,
  onConfirm,
  onCancel,
}: {
  title: string;
  summary: string;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  // 基準1.1rem(=17.6px)。CardLarge.tsxと同じ基準値(2026-09-12)。
  const { ref, fontSize } = useAutoFitTitle(title, 17.6, 11);

  return (
    <div className="tf-card-expand">
      <div className="tf-card tf-card--large" style={{ borderColor: SCENARIO_CARD_COLOR }}>
        <ScenarioIcon className="tf-card__icon" />
        <div className="tf-card__scroll">
          <div className="tf-card__title-row">
            <ScenarioIcon className="tf-card__title-icon" />
            <p className="tf-card__title" ref={ref} style={{ fontSize: `${fontSize}px` }}>
              {title}
            </p>
          </div>
          {summary !== "" && <p className="tf-card__text">{summary}</p>}
        </div>
      </div>
      <div className="tf-card-expand__actions">
        <button type="button" className="tf-card-expand__button" onClick={onConfirm}>
          これで始める
        </button>
        <button type="button" className="tf-card-expand__button" onClick={onCancel}>
          キャンセル
        </button>
      </div>
    </div>
  );
}
