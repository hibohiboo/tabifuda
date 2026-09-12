---
paths:
  - "apps/web/e2e/**"
  - "apps/web/playwright.config.ts"
  - "apps/web/vite.config.ts"
  - "apps/web/src/**/*.test.ts"
---

# テストの置き場・書き方(TS側、Playwright/vitest)

正は docs/design/test-strategy.md。「テストの質量はcoreに置く」方針の
具体化として、本ファイルはTS側(Playwright/vitest)固有の運用だけを補足する。
重複した規約は書かない(CLAUDE.md最重要ルール5・SSoT)。

## 実行コマンド

- `pnpm --filter @tabifuda/web test:e2e`(内部は`playwright test`)。
  ルール分岐の検証はcoreに寄せ、apps/web側にルール分岐を生やさない
  (test-strategy.md「重複を作らないためのルール」)
- `pnpm --filter @tabifuda/web test:unit`(内部は`vitest run`。P7 C2、
  2026-09-12導入)。対象は`loadScenarios.ts`のような**ロジックを持つ
  純粋関数のみ**(test-strategy.md「`packages/ui`の表示コンポーネントに
  単体テストは必須としない」の裏返し)。表示コンポーネントの単体テストは
  対象外(Playwrightスモーク+コンポーネントカタログの目視確認で担保する
  既定方針は変わらない)。設定は`apps/web/vite.config.ts`の`test`フィールド
  (別ファイルを増やさない)。environmentは既定の`node`のまま
  (DOM APIを使う対象が増えたら`jsdom`等の追加を検討する)

## 骨抜き禁止

- `expect(...).toBeDefined()` のような何も保証しないassertを書かない
- `.only` / `.skip` を残したままコミットしない
- Web版スモークはtest-strategy.md「E2E/スモーク(最小限)」の方針どおり
  最小本数に留める(テンプレシナリオの通しプレイが対象。ログイン・組織・
  プランのような概念はtabifudaに存在しない)
