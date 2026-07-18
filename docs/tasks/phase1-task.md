# Phase 1 実装タスク(エージェント指示文)

実行モデル: Sonnet 5 / Claude Code。**1サイクルごとに新セッションで渡す**
(コンテキスト管理の一次対策)。各サイクル開始時、まずCLAUDE.mdと
docs/design/domain-model.md を読むこと。

## 目的

crates/core にドメインモデル(v0.2)を実装する。UI・永続化・非同期は
スコープ外。コアは純粋(IO・時刻・乱数なし)を厳守。

## サイクル分割(1サイクル=1セッション=1PR)

### C0: 命名の反映(小タスク。C1と同セッションでよい)
- プロジェクト名は **Tabifuda(旅札)** に決定
- crateを改名: core → tabifuda-core、engine-cli → tabifuda-cli
  (Cargo.toml、ディレクトリ名、workspace members、CI設定を一貫更新)
- CLAUDE.md冒頭・リポジトリ構成、README のプロジェクト名を更新
- 確認: fmt/clippy/test が通ること

### C1: 型の骨格
- ID群(newtype)、Tag、CardDef(tags含む)、Effect、Condition、
  SceneDef、Scenario、Character、Session、SessionStatus、Actor/Role
- serdeは全enumタグ付き表現+`#[non_exhaustive]`(CLAUDE.md規約)
- テスト: 全公開型のserialize/deserialize往復(proptestで一括)
- **完了後、人間に報告して止まる。ここでOpus 4.8の型設計レビューを
  挟む(agent-operations.md のエスカレーション条件2。P1で最重要)**

### C2: decide/apply の基本コマンド
- `decide(&Session, &Actor, Command) -> Result<Vec<Event>, RuleError>`
  `apply(Session, &Event) -> Session`
- StartSession(シナリオ+パーティの凍結コピー、初期シーン入場と配布)、
  PlayCard(free_text対応、requires検証、Effect解決)、EndSession
- Effect: GotoScene / AdvancePhase / DealCard / EndSession
  (ModifyStatは型のみ、解決は後回し可)
- free_text は `BoundedString<MAX>` 型で長さ上限を設ける(型レベル)。
  機構と段階適用は cross-cutting.md §UGC-3 参照(レビューL2の決定)
- テスト: 各Commandに受理/拒否を対で(テーブル駆動)。拒否系は
  test-strategy.md の網羅対象に従う

### C3: 権限と提案フロー
- Actor権限規則(Player/Gm、Forbidden)、Propose→Paused、
  JudgeProposal→Running、Paused中のPlayCard/Propose拒否、GmAdvance
- テスト: 権限とステータスの受理/拒否対、状態機械の全遷移

### C4: シナリオパッチ
- PatchOp 5種(C1でのflags廃止に伴いSetFlagは無し)、
  `validate(&Session, &ScenarioPatch)`、ApplyPatch(Paused中のみ)
- テスト: validate の受理/拒否対(現在シーン削除・配布済カード定義消失の検出)

### C5: プロパティテストによる不変条件の固定
- test-strategy.md の不変条件1〜5:
  リプレイ決定性 / decide出力は必ずapply可能 / 状態機械の合法性 /
  カードの保存則 / パッチ安全性
- ランダム生成: コマンド列、小型シナリオ、パッチ

## 全サイクル共通の制約

- 設計文書と食い違う実装が必要になったら、実装せず文書更新案を先に提示
- 将来要望(タグ効果・判定・ターン制戦闘・報酬)の先回り実装禁止。
  ただし v0.2 に予約済みの範囲(tagsフィールド等)は型として持つ
- テンプレシナリオ「単純討伐」の作成は P2 のスコープ(C2〜C5では
  テスト用の最小シナリオをテストコード内に構築する)
- 終了時は CLAUDE.md「作業の終わり方」に従う(ジャーナル含む)

## Phase 1 の完了条件

- C1〜C5 完了、`cargo test --workspace` 通過、clippy警告ゼロ
- docs/design/domain-model.md と実装の乖離ゼロ
  (乖離があれば文書側も改訂済みであること)
