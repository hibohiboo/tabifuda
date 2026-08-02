# D3・D4 チェックリスト

対象: [../task.md](../task.md) D3(frozen対応+完了フィルタ)・D4(画面ビュー段階1)。
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
- [ ] **人間レビュー継続**: 6画面になった現状で過不足を再確認。
  現状の6画面と対応:
  - 依頼選択(future): actors=[player, gm], usecases=[start-session]
  - シナリオ中/プレイ画面(implemented): actors=[player, gm],
    usecases=[play-card, propose, judge-proposal, apply-patch, gm-advance]
  - 冒険記タイムライン(future): actors=[player, gm], usecases=[](表示専用)
  - シナリオ終了後(future): actors=[player, gm], usecases=[end-session]
  - 作者ページ(future): actors=[author], usecases=[](作者向けCommand無し)
  - GMページ(future): actors=[gm],
    usecases=[judge-proposal, apply-patch, gm-advance, end-session]

## 終わり方

- [x] `pnpm --filter @tabifuda/docs-site typecheck` / `build` /
  `check:rdra-data` / `check:doc-links` を通す
- [x] ブラウザでの目視確認(Playwright、apps/web の @playwright/test を一時
  利用。進捗ビューのフィルタタブ・RDRAビューのscreensセクションとも表示・
  コンソールエラー無しを確認。検証用スクリプトは作業後に削除)
- [ ] design-sync 相当の自己チェック(docs-site は非規範ツールのため簡易)
- [ ] agent-journal.md への追記要否を確認
