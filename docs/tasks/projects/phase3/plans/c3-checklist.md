# P3 C3 チェックリスト(冒険記タイムラインUI)

対象: docs/tasks/projects/phase3/task.md C3。設計計画は
[c3-design-plan.md](c3-design-plan.md)。

**本文書は経緯・作業ログであり、仕様の置き場ではない**(agent-operations.md
「正を二重化しない」の規律)。決定事項の中身は計画文書・各設計文書が正。

1. [x] `core/taggedUnion.ts`追加 + `client-conventions.md`にハンドラマップ
   パターンを追記
2. [x] `core/bindings.ts`拡張、`useGameSession.ts`が`events`を返すよう変更
3. [ ] `chronicle/eventRenderers.tsx` + `chronicle/Timeline.tsx`
   (CardRemoved非描画・カード名解決方針をclient-conventions.mdに追記)
4. [ ] `components/FreeTextInput.tsx` + `Hand.tsx`のDialogue自由入力
5. [ ] `components/ProposalForm.tsx` + `SceneView.tsx`/`App.tsx`配線
6. [ ] `session/gmResponse.ts` + `components/GmJudgePanel.tsx`(y/n/c) +
   `App.tsx`のPaused分岐を本実装に置換。client-conventions.mdにGM裁定UI
   (CLIパリティ)を追記
7. [ ] `App.css`にタイムライン/吹き出し用スタイル追記
8. [ ] 仕上げ: design-syncでの乖離チェック、task.mdのC3をdoneに更新、
   agent-journal.md追記
