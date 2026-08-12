---
name: client-conventions
description: apps/web・packages/ui・tools/docs-site(TS側)の実装時に確認する規約のチェックリスト。「UIコンポーネントを追加」「Web版の表示・操作を実装」「packages/uiに手を入れる」等、CLI/WASM/Web各層のクライアント実装時に使う。正は docs/design/client-conventions.md(このスキルは観点の索引と手順のみ)。
---

# Client Conventions

TS側(apps/web・packages/ui・tools/docs-site)の実装時に確認するチェックリスト。
crates/(コア)側の規約は core-invariants スキルを参照(役割が異なる)。
**詳細と変更は docs/design/client-conventions.md が正。** 文書を変えたら
このスキルも同PRで更新する。

## 手順

1. docs/design/client-conventions.md を通読し、該当する節を確認する
2. 下のチェックリストで実装規約の抜けを確認する
3. 新しい実装パターン(CSS実装方法・ライブラリ導入等)を採用したら、
   docs/design/client-conventions.md に追記する(次に同じ作業をする
   セッションが同じ調査を繰り返さないため)

## チェックリスト

### Event/Commandの網羅性

- [ ] Event/CommandをTS側で分岐処理する箇所はHandlerMapパターン
      (`core/taggedUnion.ts`)を使っているか。switchの`default:`や部分的な
      if連鎖で新variantを黙って無視する経路を作っていないか
- [ ] 「明示的に扱うが処理しない」ケースはキーを書いた上で`null`等の
      no-op値を返しているか(キー自体を省略しない)
- [ ] `CardKind`のような単純なリテラル合併型(単一キーオブジェクト形式
      ではない判別共用体)を網羅する場合は`Record<Kind, ...>`を使う
      (HandlerMapはワイヤ形式が単一キーオブジェクトの型にのみ使える)

### packages/uiの置き場・ビルドレス方針

- [ ] 新規コンポーネントは`packages/ui`に追加し、`src`を直接exportする
      (`dist`ビルドを作らない。client-conventions.md「UIコンポーネントの
      置き場」)
- [ ] `packages/ui`はwasmランタイムに依存しない(`core/wasmClient.ts`等は
      apps/web側に残す)
- [ ] コンポーネントカタログ掲載対象なら`tools/docs-site`の
      `ComponentsView.tsx`/`componentCatalogData.ts`にも見本を追加したか
- [ ] CSSを使う場合、副作用importの型は`vite/client`型を導入せず
      `packages/ui/src/css.d.ts`の`declare module "*.css"`で足りるか確認する
      (パッケージの依存を増やさない)
- [ ] クラス名は`tools/docs-site/src/styles.css`の既存クラスと衝突しない
      プレフィックスを使う(前例: `.tf-card`)

### 実行コマンド

- [ ] 変更したワークスペースに対して `pnpm --filter <pkg> typecheck` /
      `pnpm --filter <pkg> lint --if-present` / 必要なら `build` を実行したか
      (`--if-present`でlintスクリプト未定義のパッケージでも失敗しない。
      事前にpackage.jsonのscriptsを確認する手間を省く)
- [ ] crates/を変更していないサイクルでcargo系コマンドを実行していないか
      (意味のない実行はしない)
