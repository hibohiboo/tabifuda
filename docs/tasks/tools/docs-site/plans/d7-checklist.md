# D7 チェックリスト: RDRAビューで業務フローを複数表示する

正は [../task.md](../task.md) の D7 節。本メモは経緯・作業単位の記録。

- [x] `RdraView.tsx`のシステム外部環境セクションを`model.flows`全件をループして
  表示するよう修正(見出し・Mermaid図・ステップカードをフローごとに繰り返す構成に変更。
  既存の1本表示との見た目の連続性は保つ)
- [x] Mermaid描画(`flowDiagram`)がフローごとに独立したdiagramになることを確認
  (idの衝突が起きないか。ステップidはファイル横断で一意なため問題ない見込み)
  - コード調査: `Mermaid.tsx`は`useId()`でコンポーネントインスタンスごとに一意な
    mermaid idを発行するため、React側でも衝突しない。`business-flow.yaml`の
    ステップidをgrepし重複なしを確認済み
  - 実装: フローごとに`.flow-block`(見出しh3+出典リンク+図+カード)を繰り返す
    構成にし、`styles.css`に`.flow-block`/`.flow-block__title`(ライト/ダーク)を追加
- [x] 目視確認(Playwright): 6本すべてのフロー名・図・ステップカードが表示され
  コンソールエラーが無いことを確認(`@playwright/test`のchromiumを直接ドライブ。
  `pnpm preview`起動→`#/rdra`へnavigate→`.flow-block__title`6件・
  `.flow-block .layer__diagram svg`6件・console error 0件・スクリーンショット
  目視で確認。使ったスクリプトは一時ファイルで検証後に削除)
