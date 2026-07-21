import { useState } from "react";
import { model } from "../data";
import Mermaid from "../Mermaid";
import { flowDiagram, stateDiagram } from "../diagrams";
import {
  relatedIds,
  sourceUrl,
  type FlowStep,
  type Information,
  type Requirement,
  type RdraElement,
  type UseCase,
} from "../model";

type CardElement = Omit<RdraElement, "source"> & { source?: string };

interface CardProps {
  element: CardElement;
  refs?: { label: string; ids: string[] }[];
  badge?: string;
  selected: boolean;
  dimmed: boolean;
  onSelect: (id: string) => void;
}

function ElementCard({ element, refs, badge, selected, dimmed, onSelect }: CardProps) {
  const className = ["card", selected ? "card--selected" : "", dimmed ? "card--dimmed" : ""].join(" ");
  return (
    <button type="button" className={className} onClick={() => onSelect(element.id)}>
      <span className="card__name">
        {element.name}
        {badge && <span className="card__badge">{badge}</span>}
      </span>
      {element.description && <span className="card__desc">{element.description}</span>}
      {refs
        ?.filter((r) => r.ids.length > 0)
        .map((r) => (
          <span key={r.label} className="card__refs">
            <span className="card__refs-label">{r.label}:</span> {r.ids.join(", ")}
          </span>
        ))}
      {element.source && (
        <a
          className="card__source"
          href={sourceUrl(element.source)}
          target="_blank"
          rel="noreferrer"
          onClick={(e) => e.stopPropagation()}
        >
          出典: {element.source}
        </a>
      )}
    </button>
  );
}

function usecaseRefs(uc: UseCase) {
  return [
    { label: "アクター", ids: uc.actors ?? [] },
    { label: "情報", ids: uc.information ?? [] },
    { label: "状態", ids: uc.states ?? [] },
  ];
}

function informationRefs(info: Information) {
  return [{ label: "状態", ids: info.states ?? [] }];
}

function requirementRefs(req: Requirement) {
  return [{ label: "アクター", ids: req.actors ?? [] }];
}

function flowStepRefs(step: FlowStep) {
  return [
    { label: "アクター", ids: step.actors ?? [] },
    { label: "ユースケース", ids: step.usecases ?? [] },
  ];
}

const REQUIREMENT_BADGE: Record<Requirement["status"], string> = {
  realized: "実装済み",
  future: "将来要望",
};

export default function RdraView() {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const related = relatedIds(model, selectedId);
  const toggle = (id: string) => setSelectedId((cur) => (cur === id ? null : id));
  const cardState = (id: string) => ({
    selected: selectedId === id,
    dimmed: related !== null && !related.has(id),
  });
  const flow = model.flows[0];

  return (
    <>
      <p className="view-note">
        設計文書の要素を RDRA のレイヤーで一望する非規範の索引。要素をクリックすると
        関係する要素がハイライトされる。規範は docs/ の各文書。
      </p>

      <section className="layer">
        <h2 className="layer__title">システム価値</h2>
        <p className="layer__hint">アクターと要求。将来要望は「将来要望」バッジで区別する(未実装)</p>
        <div className="layer__cards">
          {model.actors.map((a) => (
            <ElementCard key={a.id} element={a} onSelect={toggle} {...cardState(a.id)} />
          ))}
          {model.requirements.map((r) => (
            <ElementCard
              key={r.id}
              element={r}
              refs={requirementRefs(r)}
              badge={REQUIREMENT_BADGE[r.status]}
              onSelect={toggle}
              {...cardState(r.id)}
            />
          ))}
        </div>
      </section>

      <section className="layer">
        <h2 className="layer__title">システム外部環境</h2>
        <p className="layer__hint">
          業務フロー: {flow.name} —{" "}
          <a href={sourceUrl(flow.source)} target="_blank" rel="noreferrer">
            出典
          </a>
        </p>
        {flow.steps.length > 0 && (
          <div className="layer__diagram">
            <Mermaid definition={flowDiagram(flow)} />
          </div>
        )}
        <div className="layer__cards">
          {flow.steps.map((s) => (
            <ElementCard key={s.id} element={s} refs={flowStepRefs(s)} onSelect={toggle} {...cardState(s.id)} />
          ))}
        </div>
      </section>

      <section className="layer">
        <h2 className="layer__title">システム境界</h2>
        <p className="layer__hint">ユースケース(= Command 1つに1ユースケース)</p>
        <div className="layer__cards">
          {model.usecases.map((uc) => (
            <ElementCard
              key={uc.id}
              element={uc}
              refs={usecaseRefs(uc)}
              onSelect={toggle}
              {...cardState(uc.id)}
            />
          ))}
        </div>
      </section>

      <section className="layer">
        <h2 className="layer__title">システム: 情報モデル</h2>
        <div className="layer__cards">
          {model.information.map((info) => (
            <ElementCard
              key={info.id}
              element={info}
              refs={informationRefs(info)}
              onSelect={toggle}
              {...cardState(info.id)}
            />
          ))}
        </div>
      </section>

      <section className="layer">
        <h2 className="layer__title">システム: 状態モデル(セッション状態機械)</h2>
        <div className="layer__diagram">
          <Mermaid definition={stateDiagram(model)} />
        </div>
        <div className="layer__cards">
          {model.states.map((s) => (
            <ElementCard key={s.id} element={s} onSelect={toggle} {...cardState(s.id)} />
          ))}
        </div>
      </section>
    </>
  );
}
