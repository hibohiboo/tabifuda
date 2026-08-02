# D3・D4 チェックリスト

対象: [../task.md](../task.md) D3(frozen対応+完了フィルタ)・D4(画面ビュー段階1)。
経緯: [../../../plans/post-p3.5-replanning-decisions.md](../../../plans/post-p3.5-replanning-decisions.md) Q5/Q6。

## D3: frozen対応+完了フィルタのタブ

- [x] `docs-site-frozen-status` ブランチをマージ(progress.ts の型・STATUSES、
  ProgressView のバッジ「凍結」、styles.css の配色)
- [ ] 進捗ビュー: フィルタタブ追加。既定は未完了(in-progress/planned/frozen)
  のみ表示、「完了」「全部」で切り替え
- [ ] RDRAビューの要求(requirements): 既定は `future` のみ表示、
  「実現済み」「全部」で切り替え

## D4: 画面ビュー段階1

- [ ] `docs/rdra/screens.yaml` 新設(5画面: 依頼選択/シナリオ中/
  シナリオ終了後/作者ページ/GMページ。`status: implemented | future`)
- [ ] `docs/rdra/README.md` のファイル構成表・形式説明に screens.yaml を追記
- [ ] `check-rdra-data.mjs` に screens のスキーマ検証・参照id検証を追加
- [ ] `model.ts`/`data.ts` に screens を取り込み
- [ ] `RdraView.tsx` のシステム境界セクションに画面カードを表示
  (future画面はバッジで区別、usecases/actors参照でハイライト対象に含める)
- [ ] task.md「RDRAレイヤーと既存docsの対応」表のシステム境界行を実態に更新
- [ ] 人間レビュー: 画面の過不足・画面↔Command対応の穴を確認してもらう
  (レビュー結果は本ファイルに追記)

## 終わり方

- [ ] `pnpm --filter @tabifuda/docs-site typecheck` / `build` /
  `check:rdra-data` / `check:doc-links` を通す
- [ ] design-sync 相当の自己チェック(docs-site は非規範ツールのため簡易)
- [ ] agent-journal.md への追記要否を確認
