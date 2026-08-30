# ロードマップ(フェーズ全体像の索引)

**位置づけ**: 索引文書(非規範)。P0〜P7(P3.5 を含む)の全体像と現在地を
1枚で見るためのもの。**各フェーズの内容の正は tasks/projects/phaseN/task.md**(正を二重化しない)。
状態列はフェーズ粒度の索引であり、サイクル粒度の進捗の正は各 task.md の
frontmatter([tasks/README.md](tasks/README.md)「進捗 frontmatter」。
可視化: https://hibohiboo.github.io/tabifuda/#/progress )。
本文書の要約と各タスク文書が食い違ったらタスク文書が正しく、本文書を直す。

作成の経緯: docs構造レビュー
([design/reviews/docs-structure-review.md](design/reviews/docs-structure-review.md) H1)。

## フェーズ一覧

| フェーズ | 目的(1行) | 完了条件の要約 | 状態 |
|---|---|---|---|
| [P0 骨格・ハーネス](tasks/projects/phase0/task.md) | リポジトリ骨格・CI・エージェントハーネスを整備し P1 を開始可能にする | fmt/clippy/test 通過。ドメイン型ゼロ | 完了 |
| [P1 コアのドメインモデル](tasks/projects/phase1/task.md) | crates/tabifuda-core に domain-model.md(v0.2)を実装する | C1〜C5 完了。全テスト通過。設計文書と乖離ゼロ | 完了 |
| [P2 コンソール版ソロプレイMVP](tasks/projects/phase2/task.md) | 「単純討伐」を tabifuda-cli で通しプレイ可能にする | 通しプレイ可 / lint・全テスト通過 / fixture が CI 検証済み | 完了([ふりかえり](retrospectives/phase2.md)) |
| [P3 WASM+Web版(ローカル)](tasks/projects/phase3/task.md) | バックエンドなしのローカル Web アプリで遊べ、冒険記タイムライン UI を見られる | ブラウザ通しプレイ可 / タイムライン UI / 生 HTML 挿入の静的検査が CI に | 完了([ふりかえり](retrospectives/phase3.md)) |
| [P3.5 CLI永続化(中断・再開/パーティ/持ち帰り)](tasks/projects/phase3.5/task.md) | セッションの中断・再開とパーティ持続、portable カードの持ち帰りを CLI で成立させる | 中断→再開で通しプレイ可(Paused 中断含む) / 持ち帰りがテストで固定 / lint 拡張 | 完了([ふりかえり](retrospectives/phase3.5.md)) |
| [P4 バックエンド(Hono+Drizzle+Neon)](tasks/projects/phase4/task.md) | API 経由で動かし、2ユーザーの非同期セッションを成立させる | API 経由で動作 / 非同期セッション成立 / 楽観ロック・削除フローが実 DB テストで固定 | **凍結**(2026-08-02。再開条件は下記) |
| [P5 AWSデプロイ](tasks/projects/phase5/task.md) | 本番 URL で公開し、再現可能なパイプラインを持つ | 本番 URL で通しプレイ可 / パイプライン再現可 / セキュリティレビュー対応済み | **凍結**(2026-08-02。再開条件は下記) |
| [P6 カードUI改善](tasks/projects/phase6/task.md) | カードのビジュアル(白銀比・種類別既定アイコン・2サイズ)を実装し、手札・冒険記等のカード表示を差し替える | 手札・冒険記・GM裁定パネルが新カードコンポーネントで表示 / コンポーネントカタログに掲載 / Q4(身近な人に見せるか)再判断済み | 計画中(2026-08-12起票) |
| [P7 依頼(シナリオ)選択画面](tasks/projects/phase7/task.md) | 複数シナリオから選んで開始できるようにし、選択画面を張り紙カードUIで実装する(ソロプレイ範囲。P6完了が前提) | 複数シナリオからStartSessionできる / 選択画面がP6のカードコンポーネントで表示される | 計画中(2026-08-12起票。サイクル未設計) |

### P4・P5 の凍結(2026-08-02)

P4・P5 は凍結中(経緯:
[tasks/plans/post-p3.5-replanning-decisions.md](tasks/plans/post-p3.5-replanning-decisions.md) Q1)。
再開は次の2条件を**両方**満たしたとき:

1. 非同期で遊びたい相手が具体的に現れる
2. DB スキーマ(PostgreSQL のデータ構造)を確定できるほど要件が安定する
   (凍結時点ではデータ構造を決めること自体が時期尚早という判断)

P3.5 は後から挿入したフェーズ(2026-07-20)。既存の ADR・決定ログが
「P4=バックエンド」の意味で参照しているため、**挿入時に既存番号は
振り直さない**(以降の挿入も同様)。

フェーズ間の作業(どのフェーズにも属さない改善・再検討)は発生しうる。
実例: P2 完了後のカード消費・除去
([tasks/plans/hand-card-removal.md](tasks/plans/hand-card-removal.md))と
ProposalId 発番の再検討([adr/0005](adr/0005-proposal-id-issuance.md))。

## 今・次・いつか(Now / Next / Later)

フェーズ横断の「これからやること」の**優先度の正**(2026-08-02新設。経緯:
[tasks/plans/post-p3.5-replanning-decisions.md](tasks/plans/post-p3.5-replanning-decisions.md)
Q3)。各項目の内容の正は従来どおりタスク文書・規範文書にあり、本節は
順番だけを持つ。項目の状態が変わったら同PRで本節も更新する。

### 今(Now)

- **P6 カードUI改善**: [tasks/projects/phase6/task.md](tasks/projects/phase6/task.md)
  C1〜C3(決定ログ Q2)。完了時に「身近な人に見せるか」を再判断(決定ログ Q4)

### 次(Next)

- (2026-08-12時点で未選定。P6完了時に決める)

### いつか(Later)

- **P7 依頼(シナリオ)選択画面**: P6完了後の有力候補
  (P6起票時にスコープ外として先送り。2026-08-12)
- [future-requirements.md](requirements/future-requirements.md) の各項目
  (作者定義報酬・タグシステム・条件付きテキスト・キャンペーン・
  フォーク還流・Web版フォーク保存 等)
- P4 バックエンド / P5 AWSデプロイ(**凍結**。再開条件は上記「P4・P5 の凍結」)

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
