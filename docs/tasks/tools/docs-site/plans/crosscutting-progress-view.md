# タスク: 進捗ビューに tasks/crosscutting/ を表示する

## 背景

[crosscutting-task-structure.md](../../../plans/crosscutting-task-structure.md)
(横断タスクの構造化)で `tasks/crosscutting/` を新設する。docs-site の進捗
ビューに表示されて初めて「何が残っているか一目で分かる」という当初の
目的を達成するため、表示対応をこのチケットで行う。

ルール改定・ファイル移動(crosscutting-task-structure.md)とコード変更
(本チケット)は同PRに畳み込まない
(実例: 2026-08-02 frozen語彙追加時、文書反映のついでにdocs-siteの
ソース修正へ手を伸ばしてユーザー指摘で中断した教訓。agent-journal.md参照)。

## 現状の実装(調査済み)

- [tools/docs-site/src/progress.ts](../../../../../tools/docs-site/src/progress.ts)
  の型 `group: "projects" | "tools"` と、パス抽出の正規表現
  `docs\/(tasks\/(projects|tools)\/([^/]+)\/task\.md)$` に
  `projects`・`tools` がハードコードされている
- [tools/docs-site/src/views/ProgressView.tsx](../../../../../tools/docs-site/src/views/ProgressView.tsx)
  でグループ別に表示している(表示ラベル・並び順の実装は着手時に確認)

## 作業項目

1. `progress.ts` の `group` 型に `"crosscutting"` を追加、正規表現を
   `(projects|tools|crosscutting)` に拡張
2. `ProgressView.tsx` で crosscutting グループの表示(ラベル・並び順)を
   追加。既存の projects/tools と同列か、横断タスクとして別枠にするかは
   実装時に決める(UI上の見やすさを優先。要相談なら人間に確認)
3. crosscutting/ 配下のtask.mdが実際に収集されることを確認
   (crosscutting-task-structure.md完了後、`ssot-single-responsibility`
   タスクが表示されるはずなので実データで検証できる)

## スコープ

- 変更は tools/docs-site/ のみ
- 表示ロジックのみ。frontmatter仕様・crosscutting/のディレクトリ構造は
  crosscutting-task-structure.mdが正

## 終わり方

1. `pnpm --filter docs-site typecheck` / `build`
2. ローカルプレビューで crosscutting タスクが表示されることを目視確認
3. 誤解があれば agent-journal.md に1行記録
4. 作業ブランチで行い、マージは人間判断

## 関連

- [crosscutting-task-structure.md](../../../plans/crosscutting-task-structure.md)
  (前提。crosscutting/が存在しないと表示対象が無い)
- [docs/adr/0007-ssot-single-responsibility.md](../../../../adr/0007-ssot-single-responsibility.md)
