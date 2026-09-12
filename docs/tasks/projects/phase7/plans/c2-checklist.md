# P7 C2 チェックリスト: 依頼選択画面の実装

対象: [tasks/projects/phase7/task.md](../task.md) C2。
経緯: grillingスキルでの着手前確認(2026-09-12、task.md「着手前の検討結果」)
+ C2着手時の追加確認(依頼カードのコンポーネント方針)。
本ファイルは経緯メモ。仕様の正はdocs/design/domain-model.md・
docs/design/client-conventions.md。

## 着手前に確認した実装方針

- 依頼カードは`Card`/`CardLarge`(CardDef前提)を流用せず、`ScenarioMeta`
  (id/title/summary)専用の新規コンポーネントを作る(2026-09-12確認)
- データ取得(`import.meta.glob`・zod検証)は`apps/web`側に置く
  (client-conventions.md「対象外(apps/webに残す)」の`scenario/simpleHunt.ts`
  と同じ位置づけ。packages/uiはwasmランタイム非依存の原則を保つ)
- 表示コンポーネント(`ScenarioSelect`/`ScenarioCard`/`ScenarioCardLarge`)は
  `packages/ui`に置く。zodで検証済みの最小情報型(`id`/`title`/`summary`)
  のみに依存させ、`Scenario`全体を知らない設計にする
- 操作フローはP6のHand.tsxと同じ「タップ→大サイズをモーダル展開→確定」の
  2段階(screens.yaml「依頼カード一覧」→「選択して始めるボタン」に対応)

## packages/ui

- [x] `components/scenarioIcon.tsx`: 依頼カード専用アイコン(張り紙イメージ)を
      1種追加(`CARD_KIND_ICONS`とは別。CardKindの網羅マップに無関係な
      概念を混ぜない)。縁取り色`SCENARIO_CARD_COLOR`(`#8b6b4a`)も同ファイルに
- [x] `components/ScenarioCard.tsx`(小): `{ title: string }`を受け取り
      `Card`と同じ`.tf-card--small`の見た目で表示
- [x] `components/ScenarioCardLarge.tsx`(大): `{ title: string; summary:
      string; onConfirm: () => void; onCancel: () => void }`。
      `.tf-card--large`を流用し、本文位置にsummaryを表示
- [x] `components/ScenarioSelect.tsx`: `{ scenarios: ScenarioOption[];
      onSelect: (id: string) => void }`。`ScenarioOption = { id: string;
      title: string; summary: string }`を同ファイルでexport。
      Hand.tsxと同じ構造(小一覧+タップで大をModal展開+確定)
- [x] `index.ts`に上記のexportを追加
- [x] コンポーネントカタログ(tools/docs-site `#/components`)に追加
      (simple-hunt/lost-catのScenarioMetaをサンプルに使用)

## apps/web

- [x] `zod`(^4.6.2)を依存追加
- [x] `scenario/loadScenarios.ts`: `import.meta.glob`で動的検出。各シナリオの
      `meta`部分を`zod`スキーマ(`id`/`title`/`summary`の3フィールドのみ)で
      検証し、検証済みのものだけ一覧に含める(壊れたシナリオはコンソール
      警告+除外。落とすのは全体ではなく該当1件のみ)。`Scenario`全体は
      ts-rs生成の型へ`as`キャストで委ねる(zod限定案)。id順ソートで
      表示順を決定的にする
- [x] 既存の`scenario/simpleHunt.ts`は削除し、`loadScenarios.ts`に統合
- [x] `App.tsx`: `session === null`時に`ScenarioSelect`を表示。`onSelect`で
      対応する`Scenario`を見つけ`StartSession`をdispatch(パーティは既存の
      `createSoloCharacter()`のまま変更なし)
- [x] `e2e/simple-hunt.spec.ts`: 冒頭の`はじめる`ボタン操作を、依頼カード
      (「単純討伐」)をタップ→「これで始める」で確定する操作に置き換え

## 設計文書・カタログ側の反映

- [x] `docs/rdra/screens.yaml`の`scenario-select`エントリの`status`を
      `future`から`implemented`へ更新
- [x] `docs/design/client-conventions.md`にzod導入を実装パターンとして追記
- [x] `docs/design/ui-visual-design.md`に依頼カード縁取り色(`#8b6b4a`)を追記

## 検証

- [x] `pnpm --filter @tabifuda/ui typecheck`
- [x] `pnpm --filter @tabifuda/web typecheck`
- [x] `pnpm --filter @tabifuda/web run --if-present lint`
- [x] `pnpm --filter @tabifuda/web test:e2e`(Playwrightスモーク。全1件パス)
- [x] `pnpm --filter @tabifuda/web build`
- [x] `pnpm -r typecheck`(全体通過)

## 終わり方

- [x] design-syncで乖離チェック(コードコメントに`docs/tasks/`参照が
      混入していた1件を発見しその場で修正。それ以外は乖離ゼロ)
- [x] `run`スキルで実際に動かし目視確認(vite preview起動+Playwright操作+
      スクリーンショット。表示崩れ無し、コンソールエラー無し)
- [x] agent-journal.mdへ2件追記(既存デモ用サンプルとの重複表示に
      気づけなかった件・stale previewサーバーによるテスト誤判定)
- [ ] フェーズ最終サイクル: docs/agent-operations.md「フェーズ完了時の
      ふりかえり」に従いふりかえりを作成(retrospectiveエージェント)
- [ ] `/code-review ultra`をユーザーに提案(フェーズ最終サイクルのため)
