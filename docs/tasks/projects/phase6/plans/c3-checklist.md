# P6 C3 チェックリスト: 残箇所への適用+振り返り

正は [../task.md](../task.md) C3節。本文書は経緯メモ(実施順の記録)。

## 着手の経緯

「Timelineに選んだ小カードを表示するようにして」という依頼を受けたが、
これはtask.md C3(冒険記・GM裁定パネルへの適用)のスコープそのものであり、
C2は既にdone。ユーザー確認の上、C3として正式に着手する
(2026-09-12。phase6ブランチを継続)。

## 着手前の質問と回答(インタビュー式、2026-09-12)

- **対象イベント**: カードが登場する全イベント(`CardDealt`/`CardPlayed`/
  `RewardsGranted`/`CardsDiscarded`)に小Cardを添える
- **レイアウト**: 小Cardを文の左に添え、既存の文章(『カード名』表記)は
  残す(二重表示になるが、CardDefが解決できない場合のフォールバック
  (テキストのみ)と一貫した作りにするため)
- **山札構造**: ui-visual-design.md「冒険記」の「フェーズを山(束)として
  表示し、開くと中のイベントが展開される」という初期案は、今回は含めない
  (task.mdの完了条件にも無い。将来必要になったら別サイクルで検討)
- **GmJudgePanel**: 表示すべき既存カードの一覧が無いため(GMがこれから
  作るカードの名前・回答文を入力するフォームのみ)、入力中の
  `cardName`/`answerText`を小Cardでリアルタイムプレビュー表示する形で
  適用する(`kind: "Scenario"`固定。`gmResponse.ts`の実装通り)

## 前提確認

- `RewardsGranted.cards`は`CardDef[]`(フルオブジェクト)なので直接
  `Card`に渡せる。`CardDealt.card`/`CardPlayed.card`/`CardsDiscarded.cards`
  は`CardId`(定義)/`CardId[]`のため`findCardDef`で解決する
- `chronicle/`(Timeline・eventRenderers)は`packages/ui`が対象
  (client-conventions.md「UIコンポーネントの置き場」)だが、現状の
  `.timeline`/`.chronicle-minor`CSSは`apps/web/src/App.css`に残っている
  (移行時の名残)。今回追加するレイアウトCSSはC1以来のパターン
  (`packages/ui`内に`tf-`プレフィックスのCSSファイルを新設し
  `import "./X.css"`)を踏襲し、既存CSSの移設は対象外とする

## 実装

- [x] `eventRenderers.tsx`: `CardDealt`/`CardPlayed`/`RewardsGranted`/
      `CardsDiscarded`で、`findCardDef`が解決できた場合に小`Card`を
      文の左に添える。解決できない場合は現状通りテキストのみ
      (`cardName`のフォールバック`?? cardId`と一貫)
- [x] 複数カードのイベント(`RewardsGranted`/`CardsDiscarded`)は、
      小Cardを横並び(`flex-wrap`)で並べる
- [x] レイアウト用CSSを新設(`packages/ui/src/chronicle/eventRenderers.css`
      等、`tf-`プレフィックス)
- [x] `GmJudgePanel.tsx`: `responding`状態のフォームに、入力中の
      `cardName`を反映した`Card`(`kind: "Scenario"`)のプレビューを追加
      (未入力時は「(カード名未入力)」表示。回答文はtextareaで既に見える
      ため小Cardの本文欄には反映しない)
- [x] `componentCatalogData.ts`の`sampleEvents`に`CardDealt`・
      `RewardsGranted`・`CardsDiscarded`を追加し、カタログの
      Timeline見本でも新しい表示を確認できるようにした

## 確認

- [x] `pnpm --filter @tabifuda/ui typecheck` / `run --if-present lint`
- [x] `pnpm --filter @tabifuda/web typecheck`(docs-siteのtypecheckも実行)
- [x] コンポーネントカタログでTimeline/GmJudgePanelの見た目を確認
- [x] `pnpm --filter docs-site typecheck` / `build`
- [x] `apps/web/e2e/simple-hunt.spec.ts`のPlaywrightスモークが
      引き続き通ることを確認。加えて実プレイ(apps/webビルド+preview)で
      通しプレイし、Timelineの見た目・コンソールエラー無しを確認
- [ ] design-syncで設計文書との乖離チェック
- [ ] **Q4再訪**(post-p3.5-replanning-decisions.md): 身近な1〜2人に
      見せるかをユーザーに確認し、結果を本ファイルに記録
- [ ] task.md frontmatter更新(C3: planned → done)

## 実装順・割り当て(agent-operations.md「タスク種別ごとの割り当て」参照)

C1/C2と同じくPhase6はフェーズ×モデル対応表に未掲載のため、既定の
Sonnet 5を主力とする。

1. **Sonnet 5(メインセッション)**: eventRenderers/GmJudgePanelへの
   カード適用(仕様判断を伴う中心部分)
2. **Haiku 4.5(サブエージョント可)**: `componentCatalogData.ts`への
   サンプルイベント追加(定型作業)
3. **Haiku 4.5(`design-sync-screen`エージェント)**: 実装完了後の
   設計文書との乖離一次スクリーニング(最終判定はメインセッション)
