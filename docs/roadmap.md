# ロードマップ(フェーズ全体像の索引)

**位置づけ**: 索引文書(非規範)。P0〜P5(P3.5 を含む)の全体像と現在地を
1枚で見るためのもの。**各フェーズの内容の正は tasks/phaseN-task.md**(正を二重化しない)。
本文書の要約と各タスク文書が食い違ったらタスク文書が正しく、本文書を直す。

作成の経緯: docs構造レビュー
([design/reviews/docs-structure-review.md](design/reviews/docs-structure-review.md) H1)。

## フェーズ一覧

| フェーズ | 目的(1行) | 完了条件の要約 | 状態 |
|---|---|---|---|
| [P0 骨格・ハーネス](tasks/phase0-task.md) | リポジトリ骨格・CI・エージェントハーネスを整備し P1 を開始可能にする | fmt/clippy/test 通過。ドメイン型ゼロ | 完了 |
| [P1 コアのドメインモデル](tasks/phase1-task.md) | crates/tabifuda-core に domain-model.md(v0.2)を実装する | C1〜C5 完了。全テスト通過。設計文書と乖離ゼロ | 完了 |
| [P2 コンソール版ソロプレイMVP](tasks/phase2-task.md) | 「単純討伐」を tabifuda-cli で通しプレイ可能にする | 通しプレイ可 / lint・全テスト通過 / fixture が CI 検証済み | 完了([ふりかえり](retrospectives/phase2.md)) |
| [P3 WASM+Web版(ローカル)](tasks/phase3-task.md) | バックエンドなしのローカル Web アプリで遊べ、冒険記タイムライン UI を見られる | ブラウザ通しプレイ可 / タイムライン UI / 生 HTML 挿入の静的検査が CI に | 未着手(着手前に wasm 境界 API 設計レビュー) |
| [P3.5 CLI永続化(中断・再開/パーティ/持ち帰り)](tasks/phase3.5-task.md) | セッションの中断・再開とパーティ持続、portable カードの持ち帰りを CLI で成立させる | 中断→再開で通しプレイ可(Paused 中断含む) / 持ち帰りがテストで固定 / lint 拡張 | 未着手(P3 と独立。先に着手可) |
| [P4 バックエンド(Hono+Drizzle+Neon)](tasks/phase4-task.md) | API 経由で動かし、2ユーザーの非同期セッションを成立させる | API 経由で動作 / 非同期セッション成立 / 楽観ロック・削除フローが実 DB テストで固定 | 未着手(入り口で DB スキーマ+コンテキスト分割の上流判断) |
| [P5 AWSデプロイ](tasks/phase5-task.md) | 本番 URL で公開し、再現可能なパイプラインを持つ | 本番 URL で通しプレイ可 / パイプライン再現可 / セキュリティレビュー対応済み | 未着手 |

P3.5 は後から挿入したフェーズ(2026-07-20)。既存の ADR・決定ログが
「P4=バックエンド」の意味で参照しているため、**挿入時に既存番号は
振り直さない**(以降の挿入も同様)。

フェーズ間の作業(どのフェーズにも属さない改善・再検討)は発生しうる。
実例: P2 完了後のカード消費・除去
([tasks/plans/merry-leaping-tide.md](tasks/plans/merry-leaping-tide.md))と
ProposalId 発番の再検討([adr/0005](adr/0005-proposal-id-issuance.md))。

## フェーズ対応表の所在(前提が変わったらここから辿って点検する)

フェーズ×○○の対応表は以下の3箇所に分散している。あるサイクルの成果で
スコープが前倒し・変更されたときは、**この一覧から各表を点検する**
(更新漏れの実例: 不変条件5の P1 前倒し時に test-strategy.md の表が
古いまま残った。agent-journal.md 2026-07-19)。

| 表 | 場所 | 内容 |
|---|---|---|
| フェーズ×モデル対応 | [agent-operations.md](agent-operations.md)「フェーズ×モデル対応」 | 各フェーズの主力モデルと Opus スポット投入箇所 |
| フェーズ別の導入順 | [design/test-strategy.md](design/test-strategy.md)「フェーズ別の導入順」 | 各フェーズで追加するテスト |
| フェーズ対応 | [design/cross-cutting.md](design/cross-cutting.md)「フェーズ対応」 | 各フェーズで効く横断方針の項目 |

## 更新の規律

- フェーズ完了時のふりかえり(agent-operations.md)の際に、本文書の
  「状態」列を更新する
- タスク文書の完了条件・スコープを変えたら、本文書の要約行も同 PR で直す
