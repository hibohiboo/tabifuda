---
paths:
  - "apps/web/e2e/**"
  - "apps/web/playwright.config.ts"
---

# テストの置き場・書き方(TS側、Playwright)

正は docs/design/test-strategy.md。「テストの質量はcoreに置く」方針の
具体化として、本ファイルはTS側(Playwright)固有の運用だけを補足する。
重複した規約は書かない(CLAUDE.md最重要ルール5・SSoT)。

## 実行コマンド

`pnpm --filter @tabifuda/web test:e2e`(内部は`playwright test`)。単体テストランナー
(vitest等)は導入していない。TS側の境界値・分岐検証はtest-strategy.md
「重複を作らないためのルール」のとおりcoreのプロパティ/テーブル駆動テストに
寄せ、apps/web側にルール分岐を生やさない。

## 骨抜き禁止

- `expect(...).toBeDefined()` のような何も保証しないassertを書かない
- `.only` / `.skip` を残したままコミットしない
- Web版スモークはtest-strategy.md「E2E/スモーク(最小限)」の方針どおり
  最小本数に留める(テンプレシナリオの通しプレイが対象。ログイン・組織・
  プランのような概念はtabifudaに存在しない)
