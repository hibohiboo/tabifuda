import type { ReactNode } from "react";
import {
  CARD_KIND_COLORS,
  CARD_KIND_ICONS,
  Card,
  CardLarge,
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
  sampleCardsByKind,
  sampleCardsLargeByKind,
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
          <CatalogItem
            title="CardKind一覧"
            description="CardKind(6種)と既定アイコン・縁取り色の対応(cardIcons.tsx・cardColors.ts)。"
          >
            <div style={{ display: "flex", flexWrap: "wrap", gap: "16px" }}>
              {sampleCardsByKind.map((def) => {
                const Icon = CARD_KIND_ICONS[def.kind];
                return (
                  <div
                    key={def.id}
                    style={{
                      display: "flex",
                      flexDirection: "column",
                      alignItems: "center",
                      gap: "4px",
                    }}
                  >
                    <Icon
                      style={{
                        width: "32px",
                        aspectRatio: "74 / 94",
                        color: CARD_KIND_COLORS[def.kind],
                      }}
                    />
                    <code>{def.kind}</code>
                  </div>
                );
              })}
            </div>
          </CatalogItem>
          <CatalogItem
            title="Card"
            description="CardKind種類別の既定アイコン・縁取り色を持つカード(白銀比縦長・小サイズ)。CardKind全6種の見本。"
          >
            <div style={{ display: "flex", flexWrap: "wrap", gap: "8px" }}>
              {sampleCardsByKind.map((def) => (
                <Card key={def.id} name={def.name} kind={def.kind} />
              ))}
            </div>
          </CatalogItem>
          <CatalogItem
            title="CardLarge"
            description="大サイズカード(カード名・本文まで表示)。Dialogueのみ自由入力欄を内蔵する。CardKind全6種の見本。"
          >
            <div style={{ display: "flex", flexWrap: "wrap", gap: "8px" }}>
              {sampleCardsLargeByKind.map((def) => (
                <CardLarge key={def.id} def={def} onConfirm={noop} onCancel={noop} />
              ))}
            </div>
          </CatalogItem>
          <CatalogItem title="ErrorBanner" description="wasm境界のエラーを表示する。">
            <ErrorBanner error={sampleError} />
          </CatalogItem>
          <CatalogItem
            title="FreeTextInput"
            description="自由入力(台詞・提案)の共通入力欄。長さ上限をmaxLengthで受け取る。"
          >
            <FreeTextInput maxLength={200} placeholder="自由入力の例" submitLabel="出す" onSubmit={noop} />
          </CatalogItem>
          <CatalogItem
            title="Hand"
            description="手札の一覧。カードをタップするとCardLargeをモーダル展開し、確認してから出す(クリックして試せます)。"
          >
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
