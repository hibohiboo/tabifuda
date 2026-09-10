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
- [x] design-syncで設計文書との乖離チェック(乖離1件検出・同PRで修正。下記参照)
- [x] task.md frontmatter更新(C1: planned → done)

## 事後確認(2026-09-11)

C1は着手前にprompt-sample.md「1. チェックリストの作成(インタビュー式)」の
対話プロセスを経ずに実装した。そのため、本来着手前に確認すべきだった
判断を事後的に質問し、抜けがないか確認した(実装のやり直しはしない)。

- **Cardコンポーネントの大サイズ拡張方法**(size/variant prop拡張 vs
  別コンポーネント分割): 今は決めず、C2着手時のチェックリスト作成時に
  改めて検討する
- **配色の先取り**(未決事項のはずがapps/web既存のダークモード配色を
  現場判断で踏襲): 許容。C2で配色を正式決定したらCard.cssも合わせて
  更新する前提で進める
- **`tf-`プレフィックス命名規則の明文化**: 確認の結果、既に
  [client-conventions.md](../../../../design/client-conventions.md)
  「クラス名の衝突回避」に反映済みだった(抜けなし)
- **アイコンのa11y(aria-hidden化されておりkind情報がタイトル文字列のみ)・
  Cardコンポーネント自体のユニットテスト不在**: 現状のまま許容
  (モバイル優先・演出よりも視認性優先の既定方針どおり)

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
