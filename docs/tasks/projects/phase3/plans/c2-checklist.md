# P3 C2 チェックリスト(apps/web骨格)

対象: docs/tasks/projects/phase3/task.md C2。設計計画は
[c2-design-plan.md](c2-design-plan.md)。

**本文書は経緯・作業ログであり、仕様の置き場ではない**(agent-operations.md
「正を二重化しない」の規律)。決定事項の中身は計画文書・各設計文書が正。

1. [ ] `docs/adr/0003-ci-pipeline.md`更新(webジョブ新設・docs-site/pages.ymlの`--filter`化を先に文書化)
2. [ ] インフラ: `pnpm-workspace.yaml`に`apps/*`追加、`.gitignore`に`pkg/`追記、
   `ci.yml`にwebジョブ追加、`docs-site`ジョブ/`pages.yml`のコマンドを`--filter`化
3. [ ] インフラ: `apps/web`骨格(package.json/tsconfig/vite.config/eslint.config/
   index.html/main.tsx+空のApp.tsx)を追加し、`pnpm install`→
   `pnpm --filter @tabifuda/web build`が通ることを確認
4. [ ] 機能: `core/bindings.ts`+`core/wasmClient.ts`+`session/useGameSession.ts`+
   `session/scenarioLookup.ts`+`session/soloParty.ts`+`scenario/simpleHunt.ts`
5. [ ] 機能: `components/`(SceneView/Hand/ErrorBanner)+`App.tsx`本実装+`App.css`
6. [ ] 仕上げ: design-syncでの乖離チェック、`client-conventions.md`反映(必要なら)、
   agent-journal.md追記、task.mdのC2をdoneに更新
