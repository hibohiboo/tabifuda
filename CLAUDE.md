# CLAUDE.md

**Tabifuda(旅札)**。カード制TRPG(CardWirth風)のモノレポ。Rustコア+TS Web+コンソール版。

## 最重要ルール

1. **設計文書が正、実装が従。** 仕様は docs/ にある。実装前に必ず該当文書を読む。
   仕様を変える実装をする場合、先に設計文書を更新してから実装する。
2. **crates/tabifuda-core は純粋に保つ。** IO・時刻取得・乱数生成・グローバル状態を持ち込まない。
   乱数が必要な場合は結果を引数/イベントとして外から与える(リプレイ決定性のため)。
3. **すべての進行はイベント。** 状態を直接書き換える近道を作らない。
   変更は必ず `decide(state, command) -> Result<Vec<Event>, RuleError>` と
   `apply(state, event) -> State` を通す。
4. 迷ったら実装せず質問する。特に Event / Command / PatchOp の追加は要相談。

## リポジトリ構成

```
crates/
  tabifuda-core/  ルール・状態機械(純粋。serde可、IO不可)
  tabifuda-cli/   コンソール版(tabifuda-coreの薄いフロント)
  tabifuda-wasm/  wasm-bindgenラッパー(P3〜)
apps/
  web/          TS+WASMフロントエンド(P3〜)
  api/          Hono on Lambda(P4〜)
packages/
  schema/       Drizzleスキーマ・共有型(P4〜)
tools/
  rdra-viewer/  docs可視化ビューア(RDRA風。GitHub Pagesへデプロイ。ゲーム本体と非依存)
docs/
  requirements/ 要件(将来要望メモ含む)
  design/       設計文書(domain-model.md が中核)
  adr/          アーキテクチャ決定記録
  rdra/         RDRAモデルデータ(非規範の索引。tools/rdra-viewer が読む)
  tasks/        タスク指示文と計画。projects/(フェーズ)・tools/(ツール)・
                plans/(横断)。構造の正は docs/tasks/README.md
  agent-journal.md  エージェント失敗ジャーナル(1行/件)
```

## 必読文書(タスク種別ごと)

- コアのロジックに触れる → docs/design/domain-model.md
- 新機能の要否判断 → docs/requirements/future-requirements.md(実装済みと誤認しない)
- 運用・進め方 → docs/agent-operations.md
- 横断方針(権限・ログ・UGC・削除)に触れる → docs/design/cross-cutting.md
- 手法・構造の是非を判断する → docs/adr/0001-methodology.md
- CI/ワークフローに触れる → docs/adr/0003-ci-pipeline.md
- .claude/ の設定(settings・plans・memory)に触れる → docs/adr/0004-claude-config.md

## コマンド

```
cargo test --workspace        # テスト(コミット前必須)
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

(pnpm系はP3以降に追記。パッケージマネージャの選定根拠は docs/adr/0002-package-manager.md 参照)

## Rust規約

- Effect / Condition / Event / Command / PatchOp の各enumは追加前提。
  `#[non_exhaustive]` を付け、serdeは種別名を含むタグ付き表現にする
- ID型はnewtypeで包む(生Stringを引き回さない)
- コードコメントから docs/tasks/(工程文書)を参照しない。参照してよいのは
  docs/design/(規範)のみ、それもコードから読み取れない制約を指す場合に限る。
  由来・経緯(どのサイクルで書いたか等)はコミットメッセージ/PRに書く
- tabifuda-coreの公開APIにpanicを含めない。エラーは `RuleError` / `PatchError` で返す
- テスト: decideの各Commandに正常系+拒否系(Paused中のPlayCard等)を必ず対で書く

## 用語(揺らさない)

| 用語 | 意味 |
|---|---|
| シナリオ | 作者が作るデータ。phases > scenes の木構造 |
| セッション | シナリオの1回のプレイ。シナリオとパーティを凍結コピーして持つ |
| パッチ | GMによるシナリオ改編の構造化差分(PatchOp列) |
| 提案 | プレイヤーのProposalカード。セッションをPausedにする |
| 冒険記 | セッションのイベントログ。リプレイ可能 |

## やらないこと

- crates/tabifuda-core への IO・async・乱数の導入
- 設計文書を更新せずに仕様へ影響する変更を入れること
- Event列の過去改変(追記のみ。修正は打ち消しイベントで表現)
- 未使用の将来要望(タグ効果、判定、ターン制戦闘)の先回り実装

## コンテキスト使用率が60%に達したら

新しい作業に着手せず、docs/agent-operations.md の「コンテキスト管理と
ハンドオフ」手順(WIPコミット→docs/handoff/にメモ→新セッション)に従う。

## 作業の終わり方

1. `cargo test` `clippy` `fmt` を通す
2. 設計文書との乖離がないか自己チェック(乖離があれば文書も同PRで直す)
3. 作業中に自分(エージェント)が誤解した点があれば docs/agent-journal.md に1行追記。
   **その場で修正済みでも記録する**(ジャーナルの目的は個別修正ではなく傾向分析)
