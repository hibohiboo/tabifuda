import { useState } from "react";
import { model } from "../data";
import { relatedIds, sourceUrl, type RdraElement, type UseCase } from "../model";

interface CardProps {
  element: RdraElement;
  refs?: { label: string; ids: string[] }[];
  selected: boolean;
  dimmed: boolean;
  onSelect: (id: string) => void;
}

function ElementCard({ element, refs, selected, dimmed, onSelect }: CardProps) {
  const className = [
    "card",
    selected ? "card--selected" : "",
    dimmed ? "card--dimmed" : "",
  ].join(" ");
  return (
    <button type="button" className={className} onClick={() => onSelect(element.id)}>
      <span className="card__name">{element.name}</span>
      {element.description && <span className="card__desc">{element.description}</span>}
      {refs
        ?.filter((r) => r.ids.length > 0)
        .map((r) => (
          <span key={r.label} className="card__refs">
            <span className="card__refs-label">{r.label}:</span> {r.ids.join(", ")}
          </span>
        ))}
      <a
        className="card__source"
        href={sourceUrl(element.source)}
        target="_blank"
        rel="noreferrer"
        onClick={(e) => e.stopPropagation()}
      >
        出典: {element.source}
      </a>
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

export default function RdraView() {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const related = relatedIds(model, selectedId);
  const toggle = (id: string) => setSelectedId((cur) => (cur === id ? null : id));
  const cardState = (id: string) => ({
    selected: selectedId === id,
    dimmed: related !== null && !related.has(id),
  });

  return (
    <>
      <p className="view-note">
        設計文書の要素を RDRA のレイヤーで一望する非規範の索引。要素をクリックすると
        関係する要素がハイライトされる。規範は docs/ の各文書。
      </p>
      <section className="layer">
        <h2 className="layer__title">システム価値</h2>
        <p className="layer__hint">アクター(要求は C2 で追加)</p>
        <div className="layer__cards">
          {model.actors.map((a) => (
            <ElementCard key={a.id} element={a} onSelect={toggle} {...cardState(a.id)} />
          ))}
        </div>
      </section>
      <section className="layer">
        <h2 className="layer__title">システム外部環境</h2>
        <p className="layer__hint">業務フロー(1プレイの流れ)は C2 で追加</p>
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
        <h2 className="layer__title">システム</h2>
        <p className="layer__hint">情報モデル・状態モデルは C2 で追加</p>
      </section>
    </>
  );
}
