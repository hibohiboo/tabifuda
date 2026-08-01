---
status: in-progress
cycles:
  C1: done
  C2: planned
---

# ツールタスク: コンポーネントカタログ(packages/ui切り出し)

実行モデル: Sonnet 5。1サイクル=1セッション=1PR(本タスクは規模が小さいため
1セッション内で両サイクルを実施してもよい。その場合もコミットはサイクル単位
に分ける)。**開始前の儀式(全フェーズ共通)**: CLAUDE.md と docs/design/ の
関連文書を読む。どのフェーズにも属さないツール系タスク
(置き場所の経緯: [../../plans/docs-tasks-restructure.md](../../plans/docs-tasks-restructure.md))。

## 目的

apps/webのUIコンポーネントをワークスペースパッケージ`packages/ui`
(`@tabifuda/ui`)へ切り出し、tools/docs-siteから閲覧できる
コンポーネントカタログ(4番目のビュー)を追加する。参考にした構成:
`odyssage/packages/ui`(別プロジェクト、Storybookベース)。本プロジェクトは
docs-siteが既にReact+Vite製で3ビュー構成を持つため、Storybookは導入せず
docs-site自身に軽量な自作ビューを追加する方針とした
(人間との相談で決定。2026-08-01)。

## 前提となる設計決定

- コンポーネントの置き場・切り出し範囲・docs-site例外の扱いは
  [../../../design/client-conventions.md](../../../design/client-conventions.md)
  「UIコンポーネントの置き場(packages/ui)」が正
- ワークスペース構成の変更(`packages/*`追加)は
  [../../../adr/0002-package-manager.md](../../../adr/0002-package-manager.md)
  「追記(2026-08-01)」参照
- docs-siteの「ゲーム本体とコードを共有しない」原則への例外は
  [../docs-site/task.md](../docs-site/task.md)「位置づけ」に追記済み

## サイクル

### C1: packages/ui切り出し

- `packages/ui`パッケージ新設(`@tabifuda/ui`。ビルドレス、srcを直接export)
- apps/webから対象コンポーネント・補助モジュールを`git mv`
  (client-conventions.md記載の対象一覧どおり)
- apps/web側のimportを`@tabifuda/ui`からに書き換え、`App.tsx`が問題なく
  動作すること(`pnpm --filter @tabifuda/web build`のtsc/vite build通過、
  Playwrightスモークが引き続き通ることで確認)
- packages/ui自身のeslint(dangerouslySetInnerHTML禁止ルールを含む)・
  tsconfigをapps/webと同等の設定で新設

### C2: docs-siteへのコンポーネントカタログビュー追加

- `tools/docs-site`に`@tabifuda/ui`への依存を追加
- 4番目のビュー(`#/components`、「コンポーネント」タブ)を追加。
  各コンポーネントを静的サンプルデータ(`shared/scenarios/simple-hunt.json`
  等を流用)で1例ずつ表示する。動的なprops操作(Storybookのcontrols相当)は
  スコープ外
- CI: 新規`ui`ジョブ(wasm32ツールチェーン不要。`@tabifuda/ui`の
  typecheck/lint)を追加。ADR 0003(CIパイプライン)にジョブ追加を
  先に追記してから実装する

## 完了条件

`packages/ui`が独立パッケージとして存在しapps/webがそこから
コンポーネントをimportして動作する(既存のPlaywrightスモークが
引き続き通る) / tools/docs-siteに「コンポーネント」ビューが追加され
主要コンポーネントが閲覧できる / CIで`@tabifuda/ui`のtypecheck/lintが
実行される
