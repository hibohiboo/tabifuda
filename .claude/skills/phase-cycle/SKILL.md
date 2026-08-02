---
name: phase-cycle
description: docs/tasks/projects/phaseN/task.md のフェーズタスクを1サイクル実行するための手順書。「C2をやって」「Phase 1 の次のサイクルを進めて」「フェーズタスクに着手」等、docs/tasks 配下のタスク実行を求められたら、作業を始める前に必ずこのスキルを使うこと。開始の儀式・スコープ規律・停止ポイント・終わり方を含む。
---

# Phase Cycle

docs/tasks/projects/phaseN/task.md の1サイクルを、このリポジトリの規律に沿って実行する手順書。
**仕様はここに書かない。仕様は docs/ が正であり、このスキルは手順の索引である。**

## 0. 開始の儀式(スキップ禁止)

1. 対象サイクルを特定する(例: Phase 1 / C2)。指定が曖昧なら着手前に質問する
2. **作業ブランチを用意する**: `git branch --show-current` でmasterにいないか
   確認する。masterであれば、まず `git branch -a` で対象フェーズの
   `phaseN` ブランチが既に存在しないか確認し、あればそれをcheckoutする
   (無ければ新規作成する)。masterへ直接コミットしない(masterは常に
   ビルド・テストが通る状態を保つ)。既にサイクル用ブランチにいればそれを
   継続してよい。同じフェーズに対して命名違いのブランチ(例: `p3`と
   `phase3`)を並立させない
3. docs/tasks/projects/phaseN/task.md の該当サイクルに加え、**同文書の共通制約・完了条件の
   節も必ず読む**(サイクル節だけ読んで着手しない)
4. CLAUDE.md「必読文書」表に従い、該当する設計文書を読む。
   コアに触れるなら docs/design/domain-model.md、テストを書くなら
   docs/design/test-strategy.md は必須
5. **タスク文書は事前に書かれたもの。** 現状の実装や設計文書と食い違う記述を
   見つけたら、着手せず人間に差分を報告し、タスク文書の改訂を先に行う
6. 型骨格・スキーマ系タスクでは、対象設計文書に登場する型名を全て列挙し、
   **「参照はあるが定義が無い」型を実装着手前に洗い出して人間に報告する**
   (設計文書自体の内部不整合は読んだだけでは気づきにくい)

## 1. スコープ確認

- 1サイクル = 1セッション = 1PR。指定サイクルの範囲外に手を出さない
- 将来要望の先回り実装禁止。docs/requirements/future-requirements.md は
  「実装済み」と誤認しないための文書
- 仕様に影響する変更が必要になったら、実装せず設計文書の更新案を先に提示
  (CLAUDE.md 最重要ルール1)
- Event / Command / PatchOp の追加は実装前に要相談(最重要ルール4)

## 2. 実装

- **着手前にチェックリスト化する**: サイクルで行うタスクをTodoWrite等で
  洗い出し、`docs/tasks/projects/phaseN/plans/<cycle>-checklist.md`
  (フェーズ横断/ハーネス改良は `tasks/plans/`)として書き出してから実装に
  入る(agent-operations.md「開発サイクルの回し方」)。チェックリストは
  経緯メモであり仕様の置き場ではない(正はdocs/design/等の規範文書)
- **チェックリスト項目が完了し、ビルドが壊れていない区切りごとにコミット
  する**。チェックリスト文書のチェック状態更新も同じコミットに含める
  (振り返りが機械的にできる/壊れた時にどこまで戻せばよいかがコミット
  単位で分かる、の2点が目的。全項目1コミットにする必要はなく、意味的な
  まとまりとビルドの健全性を優先する)
- タスク文書が plan mode を指定していればそれに従う
- コアの純粋性を厳守(IO・時刻・乱数・グローバル状態なし)。
  変更は必ず decide / apply を通す
- テストは受理/拒否を対で書く。観点は core-invariants スキルを参照

## 3. 停止ポイント(人間の判断を待つ)

- タスク文書に「人間に報告して止まる」とあるサイクル
  (例: P1 C1 完了後の Opus 型設計レビュー)
- 同じ課題に2回失敗した(2ストライク → Opus エスカレーション。
  docs/agent-operations.md「モデルラダー」)
- コンテキスト使用率が60%に達した → handoff スキルへ
- 停止ポイントで人間の判断が要る論点が複数残った/判断待ちが
  セッションを跨ぎそう → decision-log スキルへ(決定ログをgitに残す)

## 4. 終わり方(CLAUDE.md「作業の終わり方」の実行)

1. `cargo test --workspace` / `cargo clippy --workspace -- -D warnings` /
   `cargo fmt --all` を通す。CI設定(ci.yml等)を変更するサイクルでは、
   ローカル動作確認に使った環境変数・コマンド文字列をそのままCI設定へ
   転記する(再入力すると暗黙の前提が抜け落ちやすい。P3 C1で
   `TS_RS_EXPORT_DIR`未設定のままコミットした教訓)
2. **crates/tabifuda-core・tabifuda-wasmで`#[non_exhaustive]`enum
   (Event/Command/PatchOp/RuleError等)やts-rs対象の構造体フィールドに
   変更を加えたら、そのサイクルが「crates/のみ・Web非対応」を謳っていても
   `crates/tabifuda-wasm/bindings/`を再生成しコミットする**
   (`cargo test -p tabifuda-core --features ts export_bindings` →
   `cargo test -p tabifuda-wasm --features ts export_bindings`。
   wasm-boundary.md「生成コマンド」参照)。可能なら`pnpm -r typecheck`
   まで通す(`packages/ui`のHandlerMap網羅性チェックがTSコンパイルエラー
   として即座に検出する設計のため。client-conventions.md「Event/Command
   の網羅性」)。P3.5でEvent追加時にこれを怠り、フェーズ完了後の別件確認
   まで2サイクル気づかれなかった教訓
3. design-sync スキルで設計文書との乖離チェック(乖離があれば同PRで文書も直す)
4. **非規範文書(domain-guide.md / demo.md)への影響を確認する。**
   遊び方・操作手順に見える変更(カードの挙動、UI操作の追加等)があれば
   同PRで反映する。厳密な同期は求めない(欠落を見つけたら報告し、
   対応要否は人間の判断でよい)
5. 作業中に誤解した点があれば docs/agent-journal.md に1行追記。
   **その場で修正済みでも記録する**
6. コミットは意味単位で分割。改行コードはLF
7. **masterへの統合(マージ・PR作成)は人間の判断を仰ぐ。** 作業ブランチの
   ままユーザーに報告し、エージェントが独断でmasterへマージ・pushしない。
   統合後は、マージ済みの作業ブランチを削除する(`git branch --merged
   master` で確認してから `git branch -d`。放置すると同名パターンの
   ブランチが増殖する)
8. **フェーズの最終サイクルなら、人間の指示を待たずにふりかえりを作成する。**
   手順の正は docs/agent-operations.md「フェーズ完了時のふりかえり」
   (ドラフトは `retrospective` エージェントに任せてよい。保存先は
   docs/retrospectives/phaseN.md、気づきの反映先決定まで行って人間に報告)
