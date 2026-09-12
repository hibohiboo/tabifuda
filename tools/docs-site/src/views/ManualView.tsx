import { CARD_KIND_ICONS } from "@tabifuda/ui";
import { sourceUrl } from "../model";
import { CARD_KIND_MANUAL, CARD_KIND_MANUAL_INTRO, CARD_KIND_MANUAL_SOURCE } from "./manualData";

export default function ManualView() {
  return (
    <>
      <p className="view-note">
        WebUIを使う上で必要な用語の意味をまとめたマニュアル。ここでの説明は
        マニュアル向けの独自要約であり、規範は各項目の「出典」リンク先の設計文書
        (design/domain-model.md 等)。規範文書と食い違ったらこちら側を直す。
      </p>

      <section className="layer">
        <h2 className="layer__title">CardKind(カードの種別)</h2>
        <p className="layer__hint">{CARD_KIND_MANUAL_INTRO}</p>
        <table className="manual-table">
          <thead>
            <tr>
              <th>種別</th>
              <th>出所</th>
              <th>説明</th>
            </tr>
          </thead>
          <tbody>
            {(Object.keys(CARD_KIND_MANUAL) as (keyof typeof CARD_KIND_MANUAL)[]).map((kind) => {
              const Icon = CARD_KIND_ICONS[kind];
              const entry = CARD_KIND_MANUAL[kind];
              return (
                <tr key={kind}>
                  <td className="manual-table__kind">
                    <Icon width={18} height={18} />
                    {kind}
                  </td>
                  <td>{entry.origin}</td>
                  <td>{entry.description}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
        <a className="card__source" href={sourceUrl(CARD_KIND_MANUAL_SOURCE)} target="_blank" rel="noreferrer">
          出典: {CARD_KIND_MANUAL_SOURCE}
        </a>
      </section>
    </>
  );
}
