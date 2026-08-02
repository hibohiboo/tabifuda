# D3・D4・D5 チェックリスト

対象: [../task.md](../task.md) D3(frozen対応+完了フィルタ)・D4(画面ビュー段階1)・
D5(画面ビュー段階2、ワイヤーフレーム)。
経緯: [../../../plans/post-p3.5-replanning-decisions.md](../../../plans/post-p3.5-replanning-decisions.md) Q5/Q6。

## D3: frozen対応+完了フィルタのタブ(完了)

- [x] `docs-site-frozen-status` ブランチをマージ(progress.ts の型・STATUSES、
  ProgressView のバッジ「凍結」、styles.css の配色)
- [x] 進捗ビュー: フィルタタブ追加。既定は未完了(in-progress/planned/frozen)
  のみ表示、「完了」「全部」で切り替え
- [x] RDRAビューの要求(requirements): 既定は `future` のみ表示、
  「実現済み」「全部」で切り替え

## D4: 画面ビュー段階1(完了。人間レビュー1周目の指摘を反映済み)

- [x] `docs/rdra/screens.yaml` 新設(当初5画面: 依頼選択/シナリオ中/
  シナリオ終了後/作者ページ/GMページ。`status: implemented | future`)
- [x] `docs/rdra/README.md` のファイル構成表・形式説明に screens.yaml を追記
- [x] `check-rdra-data.mjs` に screens のスキーマ検証・参照id検証を追加
- [x] `model.ts`/`data.ts` に screens を取り込み
- [x] `RdraView.tsx` のシステム境界セクションに画面カードを表示
  (future画面はバッジで区別、usecases/actors参照でハイライト対象に含める)
- [x] task.md「RDRAレイヤーと既存docsの対応」表のシステム境界行を実態に更新
- [x] **人間レビュー1周目(2026-08-02)**: 「冒険記タイムラインはプレイ中の
  別画面として」との指摘を受け、in-scenario-play(手札選択・GM裁定のみ)と
  chronicle-timeline(冒険記タイムライン専用、future)に分離。usecasesは
  紐付けず、information.yamlのchronicleを表示する画面として説明文に明記
- [x] **人間レビュー継続(2026-08-02)**: 6画面の過不足を再確認。
  「いったんこれで」と承認され、D4はここで完了とする(以降の追加・分割は
  D5以降または再訪で扱う)。
  現状の6画面と対応:
  - 依頼選択(future): actors=[player, gm], usecases=[start-session]
  - シナリオ中/プレイ画面(implemented): actors=[player, gm],
    usecases=[play-card, propose, judge-proposal, apply-patch, gm-advance]
  - 冒険記タイムライン(future): actors=[player, gm], usecases=[](表示専用)
  - シナリオ終了後(future): actors=[player, gm], usecases=[end-session]
  - 作者ページ(future): actors=[author], usecases=[](作者向けCommand無し)
  - GMページ(future): actors=[gm],
    usecases=[judge-proposal, apply-patch, gm-advance, end-session]

## D5: 画面ビュー段階2(ワイヤーフレーム)

- [x] **人間の事前決定(2026-08-02)**: ワイヤーフレームのデータは
  screens.yaml に `layout: [{ label, sizeHint }]` として追記する
  (別ファイル wireframes.yaml との比較で、この案を選択)
- [ ] 6画面すべてに `layout` を追記(枠+ラベルの簡易ボックス。スマホ幅を
  想定した縦積み)
- [ ] `check-rdra-data.mjs` の ScreenSchema に `layout` のスキーマ検証を追加
- [ ] `model.ts` の `Screen` に `layout?: LayoutBlock[]` を追加
- [ ] ワイヤーフレーム表示コンポーネント(スマホ幅の枠。sizeHintで高さ比を
  変える簡易ボックス)を screens カードに追加
- [ ] カード実物のビジュアル(白銀比・アイコン)は作り込まない
  (カードUI強化タスクの担当。二重投資を避ける)

## 終わり方

- [x] `pnpm --filter @tabifuda/docs-site typecheck` / `build` /
  `check:rdra-data` / `check:doc-links` を通す(D3・D4時点)
- [x] ブラウザでの目視確認(Playwright、apps/web の @playwright/test を一時
  利用。進捗ビューのフィルタタブ・RDRAビューのscreensセクションとも表示・
  コンソールエラー無しを確認。検証用スクリプトは作業後に削除。D3・D4時点)
- [ ] D5実装後、上記の通し検証・目視確認を再実施
- [ ] design-sync 相当の自己チェック(docs-site は非規範ツールのため簡易)
- [ ] agent-journal.md への追記要否を確認
