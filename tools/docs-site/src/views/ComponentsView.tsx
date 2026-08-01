import type { ReactNode } from "react";
import {
  ErrorBanner,
  FreeTextInput,
  GmJudgePanel,
  Hand,
  ProposalForm,
  SceneView,
  Timeline,
} from "@tabifuda/ui";
import { sourceUrl } from "../model";
import {
  sampleError,
  sampleEvents,
  sampleHand,
  sampleProposal,
  sampleScenario,
} from "./componentCatalogData";

const noop = () => {};

function CatalogItem({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: ReactNode;
}) {
  return (
    <article className="task catalog__item">
      <header className="task__header">
        <span className="task__title">{title}</span>
      </header>
      <p className="card__desc">{description}</p>
      <div className="catalog__preview">{children}</div>
    </article>
  );
}

export default function ComponentsView() {
  return (
    <>
      <p className="view-note">
        コンポーネントの置き場・切り出し方針の正は{" "}
        <a href={sourceUrl("design/client-conventions.md")} target="_blank" rel="noreferrer">
          docs/design/client-conventions.md
        </a>
        。ここでは<code>packages/ui</code>(<code>@tabifuda/ui</code>
        )の主要コンポーネントを静的サンプルデータで1例ずつ表示する
        (動的なprops操作は対象外)。ボタン等は操作できるが結果は画面に反映されない。
      </p>
      <section className="layer">
        <h2 className="layer__title">コンポーネント一覧</h2>
        <div className="task-list catalog">
          <CatalogItem title="ErrorBanner" description="wasm境界のエラーを表示する。">
            <ErrorBanner error={sampleError} />
          </CatalogItem>
          <CatalogItem
            title="FreeTextInput"
            description="自由入力(台詞・提案)の共通入力欄。長さ上限をmaxLengthで受け取る。"
          >
            <FreeTextInput maxLength={200} placeholder="自由入力の例" submitLabel="出す" onSubmit={noop} />
          </CatalogItem>
          <CatalogItem title="Hand" description="手札の一覧。Dialogueカードは自由入力欄を挟んでから出す。">
            <Hand cards={sampleHand} onPlay={noop} />
          </CatalogItem>
          <CatalogItem title="ProposalForm" description="プレイヤーがGMへ提案するためのフォーム。">
            <ProposalForm onPropose={noop} />
          </CatalogItem>
          <CatalogItem
            title="SceneView"
            description="シーンの語り・手札・提案フォームをまとめた進行中画面。"
          >
            <SceneView
              narration={sampleScenario.phases[0].scenes[0].narration}
              hand={sampleHand}
              onPlay={noop}
              onPropose={noop}
            />
          </CatalogItem>
          <CatalogItem
            title="GmJudgePanel"
            description="提案へのGM裁定UI(採用/却下/カードを配って応える)。"
          >
            <GmJudgePanel
              proposal={sampleProposal}
              scenario={sampleScenario}
              onJudge={noop}
              onRespond={noop}
            />
          </CatalogItem>
          <CatalogItem title="Timeline" description="イベント列を時系列の冒険記として描画する。">
            <Timeline events={sampleEvents} scenario={sampleScenario} />
          </CatalogItem>
        </div>
      </section>
    </>
  );
}
