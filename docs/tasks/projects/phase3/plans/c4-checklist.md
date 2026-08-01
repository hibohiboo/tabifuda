# P3 C4 チェックリスト(UGC規律とスモーク)

対象: docs/tasks/projects/phase3/task.md C4。

**本文書は経緯・作業ログであり、仕様の置き場ではない**(agent-operations.md
「正を二重化しない」の規律)。決定事項の中身は設計文書が正。

## 事前確認の結果

- free_text長さ上限(cross-cutting.md「自由入力(UGC)の取り扱い」の段階適用)は
  既に全対象に適用済み: `PlayCard.free_text`/`Propose.text`/`ScenarioPatch.note`が
  core側`BoundedString<4096>`、Web UI側も`session/limits.ts`の
  `FREE_TEXT_MAX`/`CARD_NAME_MAX`/`CARD_TEXT_MAX`で`maxLength`を設定済み
  (P3 C3までに完了)。C4での追加実装は不要、現状維持を確認するのみ
- 残タスクは (1) `dangerouslySetInnerHTML`検出のlint/CI禁止、
  (2) Playwrightスモーク1本の2点

## チェックリスト

1. [x] ESLintで`dangerouslySetInnerHTML`使用を禁止するルールを追加
   (`no-restricted-syntax`。React本体の警告と重複するが、CIで機械的に
   失敗させることが目的)。現状コードに使用箇所が無いことを確認済み。
   検出動作は一時的に違反コードを書いて確認後、削除済み
2. [x] Playwright導入: `playwright.config.ts`作成(`webServer`で
   `pnpm run build && vite preview`を起動)、
   `e2e/simple-hunt.spec.ts`で「単純討伐」を勝利まで1本通す。
   `playwright`パッケージ(devDependency)は`@playwright/test`に置換
3. [x] `package.json`に`test:e2e`スクリプト追加
4. [x] `.gitignore`にPlaywrightの成果物(`playwright-report/`/
   `test-results/`)を追記
5. [x] docs/adr/0003-ci-pipeline.md: `web`ジョブへのPlaywright
   ブラウザインストール+スモーク実行の追加を先に追記(ジョブ構成変更は
   実装前にADR更新、というADR自体の方針に従う)
6. [x] `.github/workflows/ci.yml`の`web`ジョブに上記を実装
7. [x] 仕上げ: `cargo test`/`clippy`/`fmt`通過、design-syncで乖離ゼロを確認
   (ADR記述とci.ymlの実装が一致、demo.mdへの player向け影響なし)、
   task.mdのC4/statusをdoneに更新。今回は特筆すべき誤解なし
   (agent-journal.md追記は見送り)
