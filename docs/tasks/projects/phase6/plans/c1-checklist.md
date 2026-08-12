# P6 C1 チェックリスト: カードコンポーネント新設(小サイズ)

正は [../task.md](../task.md) C1節。本文書は経緯メモ(実施順の記録)。

## 前提決定

- [x] アイコン調達方法の決定(自作SVG)を
      [design/ui-visual-design.md](../../../../design/ui-visual-design.md) に追記
      (2026-08-12)

## 実装

- [x] `CardKind`(6種: Action/Scenario/Dialogue/Proposal/Item/Marker)ごとの
      既定アイコンを自作SVGコンポーネントとして実装(`packages/ui/src/components/cardIcons.tsx`)
- [x] `Card` コンポーネント本体を新設(`packages/ui/src/components/Card.tsx`)。
      白銀比(1:√2)縦長・小サイズ(タイトル+アイコンのみ、CardWirthの74x94相当が
      収まる)
- [x] `packages/ui/src/index.ts` から `Card` をexport
- [x] `tools/docs-site` のコンポーネントカタログ
      (`ComponentsView.tsx`/`componentCatalogData.ts`)に `Card` の見本を追加。
      6種類(Marker含む)を並べる

## 確認

- [x] `pnpm --filter @tabifuda/ui typecheck` / `lint`
- [x] `pnpm --filter docs-site typecheck` / `build`(`lint`スクリプトは
      docs-siteに存在しない)
- [x] 既存コンポーネント(Hand等)は変更しない(このサイクルは新設のみ)
- [x] `pnpm --filter @tabifuda/web typecheck` / `lint`(利用側への影響なし確認)
- [ ] design-syncで設計文書との乖離チェック
- [ ] task.md frontmatter更新(C1: planned → done)

## 実装メモ(経緯)

- CSSの副作用import(`import "./Card.css"`)にtsc型が必要だったため、
  `vite/client`型は導入せず(packages/uiはwasmランタイム非依存の
  ビルドレスパッケージであり依存を増やしたくないため)、
  `packages/ui/src/css.d.ts`に`declare module "*.css"`のみ追加した
- カードのクラス名は`tf-`プレフィックス(`tf-card`等)を採用。
  `tools/docs-site/src/styles.css`に既存の`.card`系クラス
  (タスク一覧カード用、無関係)があり衝突するため
- 配色は`design/ui-visual-design.md`「未決事項」でC2着手前に決定予定のため、
  新しい色を決めず`apps/web/src/App.css`の既存ダークモード配色を踏襲した
- `shared/scenarios/simple-hunt.json`にはAction/Proposal/Itemが
  登場しないため、カタログの6種見本には最小のサンプルCardDefを
  `componentCatalogData.ts`に追加した(`sampleCardsByKind`)
