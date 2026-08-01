# component-catalog C1 チェックリスト(packages/ui切り出し)

対象: docs/tasks/tools/component-catalog/task.md C1。

**本文書は経緯・作業ログであり、仕様の置き場ではない**。切り出し対象・
理由の正は docs/design/client-conventions.md「UIコンポーネントの置き場」。

1. [x] `packages/ui`パッケージ新設(`package.json`/`tsconfig.json`/
   `eslint.config.js`。ビルドレス、`src/index.ts`が唯一の公開エントリ)
2. [x] `git mv`でapps/webから対象一式を移動:
   `components/`(ErrorBanner・FreeTextInput・GmJudgePanel・Hand・
   ProposalForm・SceneView)、`chronicle/`(Timeline・eventRenderers)、
   `core/bindings.ts`・`core/taggedUnion.ts`、
   `session/scenarioLookup.ts`・`limits.ts`・`gmResponse.ts`
3. [x] apps/web側のimportを`@tabifuda/ui`に書き換え
   (App.tsx、wasmClient.ts、useGameSession.ts、soloParty.ts、
   scenario/simpleHunt.ts)。`@tabifuda/ui`をapps/web/package.jsonへ
   `workspace:*`で追加
4. [x] `pnpm install`でワークスペースリンクを確認
   (`apps/web/node_modules/@tabifuda/ui` → `packages/ui/`)
5. [x] 検証: `packages/ui`のtypecheck/lint、apps/webのtypecheck/lint/build、
   Playwrightスモーク(単純討伐)がすべて無変更で通過することを確認
