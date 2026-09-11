# P6 C2 チェックリスト: 大サイズ+手札UIへの適用

正は [../task.md](../task.md) C2節。本文書は経緯メモ(実施順の記録)。

## 着手前の質問と回答(2026-09-11、インタビュー式)

C1の反省(質問手順を踏まずに実装した。[c1-checklist.md](c1-checklist.md)
「事後確認」参照)を受け、着手前にユーザーへ質問して以下を決定した。

- **手札レイアウト**: 小サイズCardの一覧(横スクロール)→タップで大サイズ
  (`CardLarge`)をモーダル展開する2段階UI
- **Card APIの拡張方法**: 既存`Card`(小)は変更せず、`CardLarge`を別
  コンポーネントとして新設する(小/大でprops要件が大きく異なるため)
- **自由入力欄の位置**: `CardLarge`内部に統合する(Dialogueのみ表示)
- **操作フロー**: 非Dialogueも含め**全種類統一**でタップ→展開→確認
  (「出す」ボタン)。ワンタップ即時使用の例外は設けない(誤タップ防止)
- **展開表示方法**: モーダル/オーバーレイで画面中央に表示
- **カードの色分け**: `CardKind`ごとに縁取り色を分ける(既定アイコンに
  加えて色でも即座に判別できるようにする)
- **フォント・UIライブラリ・アニメーション手段**: C1の方針(自作CSS・
  ライブラリ未導入)をそのまま拡張。アニメーションはCSS Transitionのみ
  (モーダルのフェード程度)

上記は [design/ui-visual-design.md](../../../../design/ui-visual-design.md)
「配色・フォント・実装手段」「画面ごとのUI方向性」に反映済み。

## 前提決定

- [x] 配色パレット(CardKind6種の縁取り色)をui-visual-design.mdへ追記
- [x] フォント・UIライブラリ・アニメーション手段の方針をui-visual-design.mdへ追記
- [x] 手札選択の操作フロー(タップ→モーダル展開→確認)をui-visual-design.mdへ追記

## 実装

- [x] `packages/ui/src/components/cardIcons.tsx`と対になる、`CardKind`→
      縁取り色のマッピングを追加(`CARD_KIND_COLORS`。網羅性は
      `Record<CardKind, string>`で担保。C1の`CARD_KIND_ICONS`と同じ考え方)
- [x] `Card.css`に`.tf-card`へ`border-color`を種別ごとに適用する仕組みを追加
      (小サイズCardにも縁取り色を反映。実装はコンポーネント側`style`指定)
- [x] `CardLarge`コンポーネントを新設(`packages/ui/src/components/CardLarge.tsx`)。
      カード名・種別アイコン・本文(`CardDef.text`)を表示。スマホ幅で
      読みやすいレイアウトにする
- [x] `CardLarge`にDialogue用の自由入力欄(既存`FreeTextInput`を再利用)を
      内部統合する。他種別では表示しない
- [x] モーダル/オーバーレイの表示に使う仕組みを用意する(`Modal.tsx`を新設。
      Escキー・オーバーレイクリックで閉じる最小実装。ライブラリは導入しない)
- [x] `Hand.tsx`を置き換え:
  - 手札一覧は小`Card`をボタン化して横に並べる(横スクロール)
  - タップで対応する`CardLarge`をモーダル表示する状態管理に変更
  - モーダル内の確定操作は既存同様「出す」ボタン(`onPlay`呼び出し)
  - モーダルを閉じる操作(閉じるボタン/オーバーレイ外タップ)で選択状態を
    リセットする(既存`armed`相当のstateを`selected`に置き換えて流用)
  - Dialogueの場合は自由入力後に「出す」を押して確定する流れを維持する
- [x] `apps/web`側(`App.tsx`・`SceneView.tsx`)は、`Hand`へのprops
      (`cards`/`onPlay`)に変更が無く、手を入れていない(影響確認のみ)

## 確認

- [x] `pnpm --filter @tabifuda/ui typecheck` / `lint`
- [x] `pnpm --filter @tabifuda/web typecheck` / `lint`
- [x] コンポーネントカタログ(`tools/docs-site`)に`CardLarge`の見本を追加
      (6種類)。既存`Card`の見本にも縁取り色反映後の見た目を反映
- [x] `pnpm --filter docs-site typecheck` / `build`
- [x] `apps/web/e2e/simple-hunt.spec.ts`を新フロー(カードをタップ→モーダル
      内「出す」で確定)に書き換える。**現状の「ワンタップ即時使用」を前提に
      書かれたセレクタ(例:`getByRole("button", { name: "獣の巣に到着する" })`
      で即クリック)は全て「タップして展開→出す」の2手順に直す**
- [x] `pnpm --filter @tabifuda/web test:e2e`で通過確認
- [x] design-syncで設計文書との乖離チェック。2件検出し同PRで文書側を修正
      (大サイズカードの白銀比は小サイズ限定の原則と明記/アニメーション
      手段の用語を「CSS Transitionのみ」→「CSSのみ(Transition/Animation)」
      に訂正)。非規範文書(demo.md)のWeb版操作説明も新フローに更新
- [x] task.md frontmatter更新(C2: planned → done)

## 実装順・割り当て(agent-operations.md「タスク種別ごとの割り当て」参照)

Phase6はフェーズ×モデル対応表に未掲載の新しいフェーズのため、既定の
Sonnet 5を主力とする。P3(WASM+Web UI)に準じ、UI文言・体裁のみHaikuに
振れる箇所を分離した。

1. **Sonnet 5(メインセッション)**: `CARD_KIND_COLORS`・`Card.css`縁取り色
   ・`CardLarge`・モーダル実装・`Hand.tsx`置き換え(仕様判断を伴う中心部分)
2. **Sonnet 5(メインセッション)**: `simple-hunt.spec.ts`の新フロー書き換え
   (UI操作フローの変更を伴うためHaikuの雛形生成には向かない)
3. **Haiku 4.5(サブエージェント可)**: コンポーネントカタログへの見本追加
   (`componentCatalogData.ts`・`ComponentsView.tsx`への定型追加)
4. **Haiku 4.5(`design-sync-screen`エージェント)**: 実装完了後の設計文書との
   乖離一次スクリーニング(最終判定はメインセッション)
