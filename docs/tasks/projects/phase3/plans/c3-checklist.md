# P3 C3 チェックリスト(冒険記タイムラインUI)

対象: docs/tasks/projects/phase3/task.md C3。設計計画は
[c3-design-plan.md](c3-design-plan.md)。

**本文書は経緯・作業ログであり、仕様の置き場ではない**(agent-operations.md
「正を二重化しない」の規律)。決定事項の中身は計画文書・各設計文書が正。

1. [x] `core/taggedUnion.ts`追加 + `client-conventions.md`にハンドラマップ
   パターンを追記
2. [x] `core/bindings.ts`拡張、`useGameSession.ts`が`events`を返すよう変更
3. [x] `chronicle/eventRenderers.tsx` + `chronicle/Timeline.tsx`
   (CardRemoved非描画・カード名解決方針をclient-conventions.mdに追記済み)。
   App.tsxに配線。ハンドラマップのキー不足が実際にtscエラーになることを
   手動確認済み(CardRemovedを一時削除→復元)
4. [x] `components/FreeTextInput.tsx` + `Hand.tsx`のDialogue自由入力
5. [x] `components/ProposalForm.tsx` + `SceneView.tsx`/`App.tsx`配線
6. [x] `session/gmResponse.ts` + `components/GmJudgePanel.tsx`(y/n/c) +
   `App.tsx`のPaused分岐を本実装に置換。client-conventions.mdにGM裁定UI
   (CLIパリティ)を追記済み。Playwrightで提案→カードで応える(複数可)→
   採用→再開→自由入力付きプレイ→最後まで勝利、を通しで手動確認済み
   (コンソールエラー無し)
7. [ ] `App.css`にタイムライン/吹き出し用スタイル追記
8. [ ] 仕上げ: design-syncでの乖離チェック、task.mdのC3をdoneに更新、
   agent-journal.md追記
