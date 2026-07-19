# docs/ の歩き方(文書の地図)

Tabifuda(旅札)の文書一覧。**「設計文書が正、実装が従」**(CLAUDE.md 最重要
ルール1)がこのリポジトリの大原則で、その「正」は本ディレクトリにある。

## 読者別の入口

### 企画・PO視点(何を作るか。コードを読まずに確認できるもの)

| 文書 | 内容 |
|---|---|
| [design/domain-guide.md](design/domain-guide.md) | **最初に読む解説文書(非規範)**。ゲームの遊び方・世界観・主要概念を平易に説明。設計判断の論点(例: カード効果の「対象」)の背景もここで掴める |
| [demo.md](demo.md) | **実際に手元で遊ぶ手順(非規範)**。CLIの起動方法・操作方法・勝利/敗北エンドまでの具体的な入力例 |
| [requirements/future-requirements.md](requirements/future-requirements.md) | 将来要望メモ。今は作らないが壊さないよう意識する要望集。**「実装済み」と誤認しないこと** |
| CLAUDE.md「用語」表(リポジトリ直下) | シナリオ/セッション/パッチ/提案/冒険記の定義。用語を揺らさない |
| [design/reviews/](design/reviews/) | 設計レビューの記録(指摘と対応状況) |
| [tasks/plans/](tasks/plans/) 内の `*-decisions.md` | **決定ログ**。人間の判断が要る論点の進捗と決定理由。判断待ちの案件はここを見る |

### 開発者視点(どう作るか。実装前に必ず読む「正」)

| 文書 | 内容 |
|---|---|
| [design/domain-model.md](design/domain-model.md) | **中核**。型・状態機械・コマンド/イベントの規範的定義。コアに触れるなら必読 |
| [design/scenario-lint.md](design/scenario-lint.md) | シナリオlint(静的検証)の規範: 検査項目・重大度・探索範囲。lintに触れるなら必読 |
| [design/cross-cutting.md](design/cross-cutting.md) | 横断方針: 権限・ログ・UGC(自由入力)・削除・シークレット |
| [design/test-strategy.md](design/test-strategy.md) | テストの置き場所と書き方。不変条件1〜5。コアのテストを書くなら必読 |
| [adr/](adr/) | アーキテクチャ決定記録(手法・パッケージマネージャ・CI・.claude設定・ID発番)。「なぜこうなっているか」を遡る |

### 進め方視点(タスクと運用。エージェント・人間の共通ルール)

| 文書 | 内容 |
|---|---|
| [roadmap.md](roadmap.md) | **フェーズ全体像の索引(非規範)**。P0〜P5の目的・完了条件・現在地と、フェーズ対応表3箇所への参照。各フェーズの正は tasks/ |
| [agent-operations.md](agent-operations.md) | 運用の正: モデルラダー、開発サイクル、決定ログ運用、ハンドオフ、コスト |
| [tasks/](tasks/) | フェーズ別タスク指示文(phase0〜5)。1サイクル=1セッション=1PR |
| [tasks/plans/](tasks/plans/) | plan mode成果物と実行計画・決定ログ(gitで進捗を追う) |
| [agent-journal.md](agent-journal.md) | エージェント失敗ジャーナル(1行/件)。週次棚卸しの材料 |
| [retrospectives/](retrospectives/) | フェーズ完了時のふりかえり(1フェーズ1ファイル、非規範)。作成手順は agent-operations.md「フェーズ完了時のふりかえり」 |

## 文書間の優先順位(矛盾したとき)

1. **design/ の規範文書**(domain-model.md、scenario-lint.md、cross-cutting.md、test-strategy.md)が正
2. adr/ は「決定の経緯」。決定内容が design/ と食い違ったら design/ を直してから実装
3. **解説文書(domain-guide.md)・レビュー記録・決定ログ・タスク文書は非規範**。
   規範文書と食い違いを見つけたら、それは修正すべきバグ(発見者は報告する)
4. 実装と文書が食い違ったら原則実装側の誤り(design-sync スキル参照)

## 更新の規律

- 仕様に影響する変更は、**先に規範文書を更新してから**実装する
- 決定ログで「決定済み」になった内容は規範文書へ反映し、ログには経緯を残す
  (正を二重化しない。agent-operations.md「決定ログ」参照)
- 文書を追加したら本ファイルの表にも1行追加する
