# ドメインモデル叩き台 v0.2

対象: Rustコア(crates/tabifuda-core)。コンソール版ソロプレイMVPの範囲+将来の非同期マルチを見据えた土台。

v0.1からの変更点:
- **アクターと権限を導入**(cross-cutting.md §権限 からの逆輸入)。
  全CommandにActorが付き、GM専用コマンドの検証をdecide内で行う
- 全EventにActorを記録(監査・冒険記の「誰が」に対応)
- **flags(FlagId/FlagValue)を廃止し、状態表現をカードに統一**。
  「世界のすべてはカード」を徹底し、状態・選択の成立は
  `CardKind::Marker` カードで表現する。グローバルな事実(パーティ/
  シナリオ全体に関わるもの)は共有領域 `table` に、個人の選択は
  担当キャラの `hands` に配る。`Condition::HasCard` は
  アクターの手札と `table` の両方を対象に判定する

v0からの変更点:
- セッション状態機械(Running/Paused)と提案→改修→再開フローを追加
- ScenarioPatchを構造化パッチとして定義
- 勝敗分岐は「プレイヤーが選ぶ勝利/敗北カード」で表現(判定・戦闘システムは次版へ)

将来要望メモ(future-requirements.md)からの逆輸入:
- `CardDef.tags` を空フィールドとして予約(タグシステム用)
- `Session.party` はマスターデータの凍結コピーであると明記(並行プレイ・報酬書き戻し用)
- Effect/Condition は追加前提とし、タグ付きシリアライズ+`#[non_exhaustive]` を方針化

## 基本原則

1. コアは純粋な状態機械。`decide(state, command) -> Result<Vec<Event>, RuleError>` と `apply(state, event) -> State` の2関数が中心。
2. すべての進行はイベントとして記録される。セッションログ=イベント列。リプレイで冒険を振り返れる。
3. 乱数を使う場合、結果をイベントに焼き込む(リプレイの決定性を保証)。
4. シナリオはデータ(JSON/RON)。コアはそれを解釈するだけ。GMの改編=パッチの適用。

## カード

世界のすべてはカード。カード定義(CardDef)はシナリオまたはキャラメイクが供給する。

| 種別 | 出所 | 説明 |
|---|---|---|
| Action | キャラメイク | キャラ固有の技能・行動 |
| Scenario | シーン配布 | シナリオが状況に応じて配る選択肢 |
| Dialogue | 常時/配布 | 台詞カード。自由入力テキストを添えて出せる |
| Proposal | 常時 | 新たな選択肢の提案。GMが採否を裁定 |
| Item | 配布/取得 | 所持品。効果を持つことがある |
| Marker | 配布/取得 | 選択・状態の成立を示す印。効果を持たないことが多い。世界の状態(旧flags)はこのカードで表現する |

```rust
struct CardDef {
    id: CardId,
    name: String,
    kind: CardKind,
    text: String,              // フレーバー/説明
    tags: Vec<Tag>,            // v0.1では常に空。将来のタグシステム用に予約
    effects: Vec<Effect>,      // 出したときの効果
    requires: Vec<Condition>,  // 出せる条件(任意)
}

enum Effect {
    GotoScene(SceneId),
    AdvancePhase,
    DealCard { card: CardId, to: Target },
    ModifyStat { target: Target, stat: StatId, delta: i32 },
    EndSession(Outcome),
}

enum Condition {
    HasCard(CardId),   // アクターの手札 or table に存在するか
    StatAtLeast(StatId, i32),
}

enum Target {
    Character(CharacterId),
}
```

Effect / Condition は今後の追加が前提(タグ条件、シナリオ経験条件など)。
シリアライズは種別名を含むタグ付き形式(serdeの外部タグ等)とし、
`#[non_exhaustive]` を付けて後方互換の追加を許容する。

## シナリオ構造

```
Scenario
 ├ meta (id, title, author, forked_from: Option<ScenarioId>)
 ├ card_defs: シナリオ固有カード辞書(Markerカードの定義含む)
 └ phases: [Opening, Middle, Climax]
     └ scenes: [SceneDef]
```

```rust
struct SceneDef {
    id: SceneId,
    kind: SceneKind,           // Conversation | Travel | Battle | ...
    narration: String,         // シーン開始時の描写
    deals: Vec<Deal>,          // 入場時に配るカード
    exits: Vec<Transition>,    // 遷移条件(Condition/カード効果)
}
```

シーン遷移は原則カードの `GotoScene` 効果か、`Condition`(`HasCard`等)による
自動遷移で表現する。

### 勝敗分岐(v0.1の戦闘表現)

クライマックスの戦闘シーンは、入場時に「勝利」「敗北」2枚のScenarioカードを配る。
プレイヤーがどちらかを選んで出すことで分岐する。

```
scene: climax_battle (kind: Battle)
  deals:
    - card: victory  (effects: [GotoScene(epilogue_win)])
    - card: defeat   (effects: [GotoScene(epilogue_lose)])
```

既存のEffectのみで表現でき、新機構は不要。判定システム・ターン制戦闘は次版で
`CheckResolved { roll, success }` イベント等を追加して拡張する。

## セッション状態

```rust
struct Session {
    scenario: ScenarioSnapshot,   // 開催時点のシナリオを凍結コピー(元シナリオの後編集と独立)
    party: Vec<Character>,        // マスターデータの参照ではなく凍結コピー。
                                  // セッション中の変化はこのコピーにのみ及ぶ。
                                  // マスターへの書き戻しは終了処理でのみ行う(将来要望メモ§1,3)
    status: SessionStatus,        // 状態機械(下記)
    roles: HashMap<UserId, Role>, // 参加者の役割。権限検証の根拠
    phase: Phase,
    scene: SceneId,
    hands: HashMap<CharacterId, Vec<CardInstance>>,
    table: Vec<CardInstance>,     // 場に出たカード。パーティ/シナリオ全体の
                                  // 状態(旧flags)は Marker カードとしてここに置く
    pending_proposal: Option<Proposal>,
}

enum SessionStatus {
    Running,
    Paused { proposal: ProposalId },  // 提案の裁定待ち
    Ended(Outcome),
}

struct Character {
    id: CharacterId,
    name: String,
    stats: HashMap<StatId, i32>,  // MVPはHP程度から
    deck: Vec<CardId>,            // キャラメイク時取得のAction群
}
```

### セッション状態機械

```
Running --ProposalSubmitted-------------------------> Paused
Paused  --ProposalJudged(却下)----------------------> Running
Paused  --ScenarioPatched(0回以上) + ProposalJudged(採用)--> Running
Running --EndSession--------------------------------> Ended
```

- Paused中の `PlayCard` / `Propose` はRuleErrorで拒否する
- 却下の場合もイベントは残る(「提案したが見送られた」も冒険の記録)
- 非同期マルチでは、Pausedは「GMの応答待ち」としてそのまま使える

## シナリオパッチ(構造化)

GMによる改編は構造化パッチの列として表現する。全体書き換えは採用しない。

```rust
enum PatchOp {
    AddCardDef(CardDef),
    ReplaceScene(SceneDef),       // 既存シーンの丸ごと差し替え
    AddScene { phase: Phase, scene: SceneDef },
    AddTransition { scene: SceneId, transition: Transition },
    DealCard { card: CardId, to: Target },  // その場で配る
}

struct ScenarioPatch {
    ops: Vec<PatchOp>,
    note: String,   // GMのコメント。ログカードとして表示
}
```

設計意図:
- **再開の安全性**: 適用前に「現在シーンの削除」「配布済みカード定義の消失」等を
  操作単位で検証できる(`validate(session, patch) -> Result<(), PatchError>`)
- **ログ表示**: パッチが小さいため `ScenarioPatched` を1枚のカードとして
  時系列ログに描画できる(「世界はすべてカード」に一致)
- **フォーク公開**: 改編シナリオ=元シナリオ+パッチ列。由来を追跡できる形で公開可能

## アクターと権限

権限はルールの一部であり、API層ではなく **decide の中で検証する**
(cross-cutting.md 参照。サーバ側でも同じcoreを通すため、検証は一箇所で済む)。

```rust
struct Actor { user: UserId, role: Role }

enum Role {
    Gm,
    Player { characters: Vec<CharacterId> },  // 操作できるキャラ
}

// decideはアクターを受け取る
fn decide(state: &Session, actor: &Actor, cmd: Command)
    -> Result<Vec<Event>, RuleError>;
```

権限規則(v0.2):
- `PlayCard` / `Propose`: そのキャラを担当するPlayer、またはGm
- `JudgeProposal` / `ApplyPatch` / `GmAdvance`: Gmのみ
- 違反は `RuleError::Forbidden` で拒否(テストは受理/拒否を対で書く)
- 全Eventに `actor: UserId` を記録する(冒険記の「誰が」・監査の根拠)

ソロMVPでは、単一ユーザーが Gm と Player{全キャラ} の両ロールを持つ
Actorを常に渡す(素通しだが経路は本番と同一)。

## コマンドとイベント

```rust
enum Command {
    StartSession { scenario, party },
    PlayCard { by: CharacterId, card: CardInstanceId,
               free_text: Option<String> },   // Dialogueの自由入力
    Propose { by: CharacterId, text: String }, // → Paused へ遷移
    ApplyPatch { patch: ScenarioPatch },       // GM。Paused中のみ(v0.1)
    JudgeProposal { proposal: ProposalId, accepted: bool }, // GM裁定 → Running へ
    GmAdvance { to: SceneId },                 // GM強制進行
    EndSession { outcome: Outcome },
}

enum Event {
    SessionStarted { .. },
    SceneEntered { scene: SceneId, narration: String },
    CardDealt { to: CharacterId, card: CardId },
    CardPlayed { by: CharacterId, card: CardId,
                 free_text: Option<String> },
    EffectApplied { effect: Effect },
    ProposalSubmitted { id: ProposalId, by, text },   // → Paused
    ScenarioPatched { patch: ScenarioPatch },
    ProposalJudged { id: ProposalId, accepted: bool }, // → Running
    PhaseAdvanced { phase: Phase },
    SessionEnded { outcome: Outcome },
}
```

ログUI(Web版)は Event 列をそのまま時系列カードとして描画する。`CardPlayed` は
カード画像+free_text の吹き出し、`ScenarioPatched` は「GMがシナリオを改修した」
カード+note、という表示が素直。

## ソロMVPでの簡略化

- プレイヤーがGMを兼任(単一ユーザーがGm+Player両ロールのActorを持つ)
- 判定(ダイス)なし。分岐はMarkerカードとカード選択のみ
- 戦闘は勝利/敗北カードの選択で表現(上記)
- パーティは1人から
- テンプレシナリオ「単純討伐」1本: OP(依頼会話)→ミドル(移動)→
  クライマックス(勝敗選択)→エピローグ(勝利/敗北で分岐)→終了

## 次版(v1)以降に送る事項

1. **判定システム**: `CheckResolved { roll, success }` イベントの追加で拡張
2. **ターン制戦闘**: 敵もカードを持つ戦闘。TurnStateの本格化
3. **パッチ操作の拡充**: 必要になったら `RemoveScene` 等を追加。
   全体置換オペは最後の逃げ道として温存
