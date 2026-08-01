# component-catalog C2 チェックリスト(docs-siteへのカタログビュー追加)

対象: docs/tasks/tools/component-catalog/task.md C2。

1. [x] `tools/docs-site/package.json`に`@tabifuda/ui`を`workspace:*`で追加
2. [x] `vite.config.ts`に`server.fs.allow`を追加(apps/webと同じ理由。
   `packages/ui`・`shared/scenarios/`をdocs-siteの外から読めるようにする)
3. [x] `src/views/componentCatalogData.ts`: サンプルデータ
   (`shared/scenarios/simple-hunt.json`のカード定義を流用したHandCard・
   Proposal・Event列・WasmError)
4. [x] `src/views/ComponentsView.tsx`: 7コンポーネント
   (ErrorBanner・FreeTextInput・Hand・ProposalForm・SceneView・
   GmJudgePanel・Timeline)を1例ずつ表示。`App.tsx`に4番目のタブ
   (`#/components`)として配線
5. [x] `styles.css`に`.catalog__preview`を追加(既存の`.task`/`.task-list`/
   `.card__desc`を再利用)
6. [x] 検証: `pnpm --filter @tabifuda/docs-site typecheck/build`通過。
   `vite preview`を起動しPlaywright(apps/web環境から一時テストで)で
   実際にレンダリングを確認(コンソールエラー無し、7コンポーネント全て表示、
   スクリーンショットで目視確認済み)
7. [x] CI: ADR 0003に`ui`ジョブ追加を先に追記してから`ci.yml`に実装
   (`@tabifuda/ui`のtypecheck/lint。wasm32ツールチェーン不要)
