# docs/ の歩き方(文書の地図)

Tabifuda(旅札)の文書一覧。**「設計文書が正、実装が従」**(CLAUDE.md 最重要
ルール1)がこのリポジトリの大原則で、その「正」は本ディレクトリにある。

**公開サイト**: https://hibohiboo.github.io/tabifuda/ で、本ディレクトリの一部を
RDRA風の図・タスク進捗・テスト結果として可視化している(tools/docs-site。
中身は [rdra/](rdra/) と [tasks/tools/docs-site/](tasks/tools/docs-site/) 参照)。

## 読者別の入口

### 企画・PO視点(何を作るか。コードを読まずに確認できるもの)

| 文書 | 内容 |
|---|---|
| [design/domain-guide.md](design/domain-guide.md) | **最初に読む解説文書(非規範)**。ゲームの遊び方・世界観・主要概念を平易に説明。設計判断の論点(例: カード効果の「対象」)の背景もここで掴める |
| [demo.md](demo.md) | **実際に手元で遊ぶ手順(非規範)**。CLIの起動方法・操作方法・勝利/敗北エンドまでの具体的な入力例 |
| [requirements/future-requirements.md](requirements/future-requirements.md) | 将来要望メモ(**将来要望専用**)。今は作らないが壊さないよう意識する要望集。**「実装済み」と誤認しないこと**。「現在の要件」の正は design/ の規範文書(domain-model.md「ソロMVPでの簡略化」等)と tasks/ が兼ねる |
| CLAUDE.md「用語」表(リポジトリ直下) | シナリオ/セッション/パッチ/提案/冒険記の定義。用語を揺らさない |
| [design/reviews/](design/reviews/) | 設計レビューの記録(指摘と対応状況) |
| [rdra/](rdra/) | RDRAモデルデータ(**非規範の索引**)。設計文書のアクター・ユースケース・情報・状態とその関係を構造化YAMLで持ち、tools/docs-site が github.io に可視化する。規範と食い違ったらYAML側を直す |
| tasks/ 配下の `*-decisions.md` | **決定ログ**。人間の判断が要る論点の進捗と決定理由。対象タスクの `plans/`(横断は [tasks/plans/](tasks/plans/))にある。置き場所のルールは [tasks/README.md](tasks/README.md) |

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
| [roadmap.md](roadmap.md) | **フェーズ全体像の索引(非規範)**。P0〜P5(P3.5含む)の目的・完了条件・現在地と、フェーズ対応表3箇所への参照。各フェーズの正は tasks/ |
| [agent-operations.md](agent-operations.md) | 運用の正: モデルラダー、開発サイクル、決定ログ運用、ハンドオフ、コスト |
| [tasks/](tasks/) | タスク指示文と計画。1タスク=1ディレクトリ(`task.md` + 専用 `plans/`)。構造の正は [tasks/README.md](tasks/README.md) |
| [tasks/projects/](tasks/projects/) | フェーズタスク(phase0〜5)。1サイクル=1セッション=1PR |
| [tasks/tools/](tasks/tools/) | ツール系タスク(どのフェーズにも属さない開発支援ツール。例: RDRAビューア) |
| [tasks/plans/](tasks/plans/) | フェーズ横断の計画・決定ログ + plan mode の書き込み先(セッション終了時に対象タスクの plans/ へ振り分け) |
| [agent-journal.md](agent-journal.md) | エージェント失敗ジャーナル(1行/件)。週次棚卸しの材料 |
| handoff/ | ハンドオフ用の一時メモ置き場(通常は空。使い捨て運用。手順は agent-operations.md「コンテキスト管理とハンドオフ」) |
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
