# 計画: 使用済み・不要カードの手札除去(Phase 2ふりかえり起票分)

## Context

Phase 2ふりかえり([docs/retrospectives/phase2.md](../../retrospectives/phase2.md))で、
「出したカードが手札に残り続ける」ことをPOが最初に気づく違和感として起票し
([future-requirements.md §6](../../requirements/future-requirements.md))、
Phase 3着手前に解消することにした。

engine.rsの`total_instance_count`は自ら「除去を導入する将来サイクルでは
要再検討」とコメントで予告しており、`ProposalId`が`proposal_seq`という
永続カウンタ方式へ移行した前例(domain-model.md「C3: decide/applyの解決規則」)
がそのまま踏襲できる。

ユーザーとの確認で、範囲は以下の2点に決定:
1. **消費判定はCardKindから自動導出**する(CardDefにフィールド追加しない)。
   `Scenario`/`Dialogue`は使用時に除去、`Action`/`Item`/`Marker`は除去しない
2. **Markerは「選んだ記録」として実データ上は残すが、CLIの手札表示からは
   隠す**(消費・自動消去のどちらの対象にもしない。UI層のみで非表示)
3. **選ばなかったシーンのカードも自動消去に含める**
   (例: クライマックスで「打ち倒す」を選んだら、シーンを離れる時点で
   未使用の「退く」も手札から消える)

これにより、単純討伐終了時の手札は現状6枚(すべて残留)→実質2枚
(Markerの`quest_accepted`は非表示、`defeat`は自動消去で残らない=**実質0枚
の目視ゴミ**)まで改善する。

## 設計要点

### 1. 消費ルール(CardKind由来、CardDefのスキーマ変更なし)

`card.rs`に`impl CardKind { pub fn is_consumed_on_play(&self) -> bool }`を追加。
`Scenario`/`Dialogue` → `true`、それ以外 → `false`(`Marker`含む)。
`CardKind`は`#[non_exhaustive]`だが定義クレート内なので非網羅`match`は
コンパイルエラーになる=将来バリアント追加時に判定を機械的に強制できる。

### 2. 新イベント`CardRemoved`(追加のみ、既存イベント無変更)

```rust
enum RemovalReason {  // #[non_exhaustive]
    Consumed,   // 使用による消費
    SceneLeft,  // シーンを離れたことによる自動消去(未使用の選択肢カード)
}

Event::CardRemoved {
    from: CharacterId,
    card: CardId,
    instance: CardInstanceId,
    reason: RemovalReason,
}
```
`CardPlayed`は変更しない(instanceフィールドを足すと破壊的)。`decide_play_card`が
既に`instance`(プレイされたCardInstance)を特定済みなのでそこから発行する。

### 3. 「シーンを離れた」を検出するための`SceneEntered`拡張

`Event::SceneEntered`に`local_instances: Vec<CardInstanceId>`を追加
(`#[serde(default)]`でデシリアライズ時は空Vecにフォールバック。既存の
`fixtures/simple_hunt_playthrough.json`は無変更で読める=互換性を壊さない)。
`enter_scene`(engine.rs)が、そのシーンの`scene_def.deals`から実際に配った
CardInstanceIdをここに詰める(カード効果による`DealCard`は含まない=
`quest_accepted`のような「持続する付与」はシーンローカル扱いにしない)。

`Session`に`scene_local_instances: Vec<CardInstanceId>`を追加し、
`apply()`の`SceneEntered`ハンドラで`session.scene_local_instances = local_instances`
と丸ごと差し替える(現在のシーンが持つ「未使用なら消える候補」の一覧)。

### 4. シーン離脱時のクリーンアップ発行

新規シーンへ入場する直前(`decide_play_card`のGotoScene分岐、
`decide_gm_advance`)に、`session.scene_local_instances`のうち
「まだ手札に残っている」「CardKindがMarkerではない」ものを`CardRemoved`
(`reason: SceneLeft`)として発行してから`enter_scene`を呼ぶ。
`decide_play_card`側は、いま出したカード自身のinstanceは除外する
(既にConsumed理由で除去済みのため二重発行を避ける)。

共有ヘルパー(engine.rs):
```rust
fn scene_cleanup_events(
    session: &Session,
    exclude: Option<&CardInstanceId>,
) -> Vec<Event>
```
`session.hands`を`find_map`で走査し、instanceの現在の所持者
(`CharacterId`)を引いて`CardRemoved`を組み立てる。

### 5. `CardInstanceId`発番: `total_instance_count`→`Session.card_instance_seq: usize`

`ProposalId`/`proposal_seq`と同じ設計。`total_instance_count`関数は削除。
`apply()`の`CardDealt`ハンドラで`session.card_instance_seq += 1`
(1イベントにつき+1。`decide`内のローカル連番との整合はPlan agentの
検証で確認済み・変更不要)。

### 6. CLIの手札表示: Markerを非表示に(UI層のみ、データは触らない)

`crates/tabifuda-cli/src/play.rs`の手札一覧構築で、`CardKind::Marker`を
除外してから番号付けする(除外後のインデックスで採番するので歯抜けは出ない)。
`session.hands`自体は変更しない(`Condition::HasCard`判定は影響を受けない)。

### 7. 冒険記・運用ログ

`chronicle.rs`: `CardRemoved`は明示アームを追加するが**何も描画しない**
(プレイヤー行動の物語的な流れを主役にする。housekeeping detailは省く)。
アーム自体は必須(`Event`は`#[non_exhaustive]`なので`match`にワイルドカードが
あっても、無いと「未知の出来事」表示に落ちて読者に劣化して見えるため)。
`oplog.rs`: `event_kind`に`"CardRemoved"`アームを追加。

## 変更ファイル

**crates/tabifuda-core/src/**
- `card.rs`: `CardKind::is_consumed_on_play`
- `event.rs`: `CardRemoved`変体、`RemovalReason` enum、`SceneEntered.local_instances`
  (`#[serde(default)]`)
- `session.rs`: `Session.card_instance_seq: usize`、`Session.scene_local_instances: Vec<CardInstanceId>`
- `engine.rs`: `enter_scene`(deals計算→`SceneEntered`構築の順序を入れ替えて
  `local_instances`を詰める)、`scene_cleanup_events`ヘルパー新設、
  `decide_play_card`(Consumed発行+GotoScene分岐でのクリーンアップ呼び出し)、
  `decide_gm_advance`(クリーンアップ呼び出し)、`apply_to_existing`
  (`CardDealt`のseq加算、`CardRemoved`ハンドラ、`SceneEntered`の
  `scene_local_instances`更新)、`total_instance_count`削除、3箇所の
  呼び出し元を`session.card_instance_seq`に置換
- テスト: `engine_tests.rs`(`fixture_session`にフィールド追加、既存の
  `SceneEntered`期待値リテラルに`local_instances`追加、consumption/cleanup
  の受理系テストを新規追加)、`patch_tests.rs`(`fixture_session`にフィールド追加)、
  `invariant_tests.rs`(`small_session_strategy`にフィールド追加、
  不変条件4に`CardRemoved`アーム追加とドキュメントコメント更新)、
  `golden_tests.rs`(`SceneEntered`固定JSON更新、`CardRemoved`の
  goldenケース追加)
- `fixtures/simple_hunt_playthrough.final_state.json`: `Session`スキーマ変更に
  伴い再生成(`replay_tests.rs`の`--nocapture`実測→貼り替え、C4と同じ手順)。
  `simple_hunt_playthrough.json`(イベント列本体)は無変更で読める想定

**crates/tabifuda-cli/src/**
- `play.rs`: 手札一覧構築でMarkerを除外
- `chronicle.rs`: `CardRemoved`の明示(無描画)アーム追加
- `oplog.rs`: `CardRemoved`の`event_kind`アーム追加

**docs/**
- `docs/design/domain-model.md`: `Session`/`Event`コードブロック更新、
  「CardInstanceId の発行」節を`card_instance_seq`方式に書き換え、
  新設「カードの消費・除去」節(消費ルール表、`CardRemoved`、
  シーンクリーンアップ規則、CLI非表示は規範に含めずUI実装として言及)
- `docs/requirements/future-requirements.md`: §6を削除(実装済みのため
  「将来要望」から外す)
- `docs/tasks/phase3-task.md`: 「P2からの申し送り」の「カード残留仕様」
  項目を削除/更新
- `docs/demo.md`: 実装後に実際に`cargo run -p tabifuda-cli -- play ...`
  を通しプレイし直し、手札インデックスが変わる(除去・Marker非表示で
  番号がずれる)ため手順表を全面的に取り直す

## 実行順序

1. domain-model.md先行更新(CLAUDE.md最重要ルール1)
2. core型追加(event.rs/session.rs/card.rs)→ビルドが通る最小単位まで
3. engine.rsのロジック実装
4. 既存テストのコンパイルエラーを解消(fixture_session等へのフィールド追加)
5. 新規テスト追加(consumption受理系、cleanup受理系、Marker非除去、
   invariant 4のCardRemovedアーム)
6. golden_tests.rs更新
7. fixtureの`final_state.json`再生成(C4のreplay_tests.rsと同じ手順:
   一時的に`println!`で実測→貼り替え)
8. CLI側(play.rs/chronicle.rs/oplog.rs)更新
9. `cargo test --workspace` / `clippy` / `fmt`
10. design-syncチェック
11. 実際に`tabifuda-cli play`を手動で通しプレイし、Marker非表示・カード
    除去・シーン離脱クリーンアップを目視確認
12. docs/demo.mdの手順を実測に基づき更新
13. future-requirements.md §6削除、phase3-task.md更新
14. 意味単位でコミット分割(設計文書→core実装→CLI→デモ手順更新、を目安に)

## 検証方法

- `cargo test --workspace`(既存153件が通り、新規テストが追加される)
- `cargo clippy --workspace -- -D warnings` / `cargo fmt --all -- --check`
- 手動プレイ: `cargo run -p tabifuda-cli -- play shared/scenarios/simple-hunt.json`
  で以下を目視確認
  - `quest_accepted`が手札の番号付きリストに出ない
  - `reply`/`arrive`を出した直後、手札からその番号が消える
  - クライマックスで`victory`を選んだ直後、`defeat`が手札から消える
    (`epilogue_win`のシーン表示時点で手札に残っていない)
