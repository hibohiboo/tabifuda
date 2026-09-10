# **Tabifuda(旅札)**。
カード制TRPGのルールブック兼プレイ環境。本ファイルはAIコーディングエージェント向けの地図である。

## 技術スタック
モノレポ。Rustコア+TS Web+コンソール版。

## 最重要ルール

1. **設計文書が正、実装が従。** 仕様は docs/ にある。実装前に必ず該当文書を読む。
   仕様を変える実装をする場合、先に設計文書を更新してから実装する。
2. **crates/tabifuda-core は純粋に保つ。** IO・時刻取得・乱数生成・グローバル状態を持ち込まない。
   乱数が必要な場合は結果を引数/イベントとして外から与える(リプレイ決定性のため)。
   正は docs/design/domain-model.md「基本原則」(docs/adr/0007-ssot-single-responsibility.md)。
3. **すべての進行はイベント。** 状態を直接書き換える近道を作らない。
   変更は必ず `decide(state, command) -> Result<Vec<Event>, RuleError>` と
   `apply(state, event) -> State` を通す。
   正は docs/design/domain-model.md「基本原則」(docs/adr/0007-ssot-single-responsibility.md)。
4. 迷ったら実装せず質問する。特に Event / Command / PatchOp の追加は要相談。
5. **SSoT(1つの事実の正は一か所)。** 他所で言及するときは「正は〜」と明記して参照し、複製しない。1ファイル1責務(役割説明が「と」で繋がったら分割候補。行数等の機械的しきい値は設けない)。詳細は docs/adr/0007-ssot-single-responsibility.md。

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
  ui/           共有UIコンポーネント(TS+React。apps/web・tools/docs-siteが利用。
                wasmランタイム非依存・ビルドレスでソースを直接消費)
  eslint-config/ ESLint共通設定(base.js+frontend.js。apps/web・packages/uiが利用)
  schema/       Drizzleスキーマ・共有型(P4〜)
tools/
  docs-site/    docs可視化サイト(RDRA/進捗/テスト/コンポーネントの4ビュー。
                packages/ui以外はゲーム本体と非依存)。
                公開中: https://hibohiboo.github.io/tabifuda/
docs/
  requirements/ 要件(将来要望メモ含む)
  design/       設計文書(domain-model.md が中核)
  adr/          アーキテクチャ決定記録
  rdra/         RDRAモデルデータ(非規範の索引。tools/docs-site が読む)
  tasks/        タスク指示文と計画。projects/(フェーズ)・tools/(ツール)・
                plans/(横断)。構造の正は docs/tasks/README.md
  agent-journal.md  エージェント失敗ジャーナル(1行/件)
```

## 開発ルールの適用

ファイルを読む・変更する・レビューするときは、対象パスに一致するルールを先に読む。
複数一致した場合はすべて適用する。

### 必読文書(タスク種別ごと)

- コアのロジックに触れる → docs/design/domain-model.md
- TS側(apps/web・packages/ui・tools/docs-site)の表示・操作に触れる →
  docs/design/client-conventions.md(手順の索引は client-conventions スキル)
- 新機能の要否判断 → docs/requirements/future-requirements.md(実装済みと誤認しない)
- 運用・進め方 → docs/agent-operations.md
- 横断方針(権限・ログ・UGC・削除)に触れる → docs/design/cross-cutting.md
- 手法・構造の是非を判断する → docs/adr/0001-methodology.md
- CI/ワークフローに触れる → docs/adr/0003-ci-pipeline.md
- .claude/ の設定(settings・plans・memory)に触れる → docs/adr/0004-claude-config.md

ルールと仕様書が矛盾した場合は、推測で進めず作業を止めて矛盾を報告する

## コマンド

```
cargo test --workspace        # テスト(コミット前必須。crates/に変更がある場合)
cargo clippy --workspace -- -D warnings
cargo fmt --all

pnpm --filter <pkg> typecheck       # TS側(変更のあったworkspaceに対して)
pnpm --filter <pkg> lint --if-present  # lintスクリプトが無いパッケージでも失敗しない
pnpm --filter <pkg> build
```

crates/とTS側は独立して変更されうるため、**変更が無い側のコマンドは実行しない**
(意味のない実行はしない。判定は `git diff <base>...HEAD --stat` で対象ディレクトリを見る)。
パッケージマネージャの選定根拠は docs/adr/0002-package-manager.md 参照。

## Rust規約(crates/)

正は `.claude/rules/core-architecture.md`(`paths: crates/*/src/**`
で対象ファイルを読むときに自動適用)。api-architecture.md(TS/APIレイヤー版)と
同じ位置づけの、レイヤー固有実装規約の置き場(docs/adr/0004-claude-config.md
「ルール置き場」)。

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
- masterブランチへの直接コミット(作業は専用ブランチで行う。
  agent-operations.md「開発サイクルの回し方」参照)

## コンテキスト使用率が60%に達したら

新しい作業に着手せず、docs/agent-operations.md の「コンテキスト管理と
ハンドオフ」手順(WIPコミット→docs/handoff/にメモ→新セッション)に従う。

## 作業の終わり方

1. 変更範囲に応じた検証コマンドを通す(詳細な条件分岐は phase-cycle スキル
   「終わり方」参照。crates/変更ならcargo3点セット、TS側変更ならpnpm系)
2. 設計文書との乖離がないか自己チェック(乖離があれば文書も同PRで直す)
3. 作業中に自分(エージェント)が誤解した点があれば docs/agent-journal.md に1行追記。
   **その場で修正済みでも記録する**(ジャーナルの目的は個別修正ではなく傾向分析)
4. masterへの統合(マージ・PR作成)は人間の判断を仰ぐ
