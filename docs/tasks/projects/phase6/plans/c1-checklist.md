# P6 C1 チェックリスト: カードコンポーネント新設(小サイズ)

正は [../task.md](../task.md) C1節。本文書は経緯メモ(実施順の記録)。

## 前提決定

- [x] アイコン調達方法の決定(自作SVG)を
      [design/ui-visual-design.md](../../../../design/ui-visual-design.md) に追記
      (2026-08-12)

## 実装

- [ ] `CardKind`(6種: Action/Scenario/Dialogue/Proposal/Item/Marker)ごとの
      既定アイコンを自作SVGコンポーネントとして実装(`packages/ui/src/components/cardIcons.tsx`)
- [ ] `Card` コンポーネント本体を新設(`packages/ui/src/components/Card.tsx`)。
      白銀比(1:√2)縦長・小サイズ(タイトル+アイコンのみ、CardWirthの74x94相当が
      収まる)
- [ ] `packages/ui/src/index.ts` から `Card` をexport
- [ ] `tools/docs-site` のコンポーネントカタログ
      (`ComponentsView.tsx`/`componentCatalogData.ts`)に `Card` の見本を追加。
      6種類(Marker含む)を並べる

## 確認

- [ ] `pnpm --filter @tabifuda/ui typecheck` / `lint`
- [ ] `pnpm --filter docs-site typecheck` / `lint` / `build`
- [ ] 既存コンポーネント(Hand等)は変更しない(このサイクルは新設のみ)
- [ ] design-syncで設計文書との乖離チェック
- [ ] task.md frontmatter更新(C1: planned → done)
