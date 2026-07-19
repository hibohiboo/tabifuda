# ドメインモデル

対象: Rustコア(crates/tabifuda-core)。コンソール版ソロプレイMVPの範囲+
将来の非同期マルチを見据えた土台。

本文書は**現在の仕様のみ**を記述する。版履歴は git 履歴、主要決定の経緯は
末尾「決定の経緯(参照)」から辿る。シナリオデータの静的検証(lint)の規範は
[scenario-lint.md](scenario-lint.md) に分離した。

## 基本原則

1. コアは純粋な状態機械。`decide(state, command) -> Result<Vec<Event>, RuleError>` と `apply(state, event) -> State` の2関数が中心。
2. すべての進行はイベントとして記録される。セッションログ=イベント列。リプレイで冒険を振り返れる。
3. 乱数を使う場合、結果をイベントに焼き込む(リプレイの決定性を保証)。
4. シナリオはデータ(JSON/RON)。コアはそれを解釈するだけ。GMの改編=パッチの適用。

## カード

世界のすべてはカード。カード定義(CardDef)はシナリオまたはキャラメイクが
供給する。「フラグ管理」のような見えない内部変数は持たず、世界の状態・選択の
成立も `CardKind::Marker` カードで表現する。グローバルな事実(パーティ/
シナリオ全体に関わるもの)は共有領域 `table` に、個人の選択は担当キャラの
`hands` に置く。

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
    name: BoundedString<200>,
    kind: CardKind,
    text: BoundedString<2000>, // フレーバー/説明
    tags: Vec<Tag>,            // 現状は常に空。将来のタグシステム用に予約
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
    Party,                     // パーティ全員。台本(シナリオ)執筆時点で書ける役割参照
    Character(CharacterId),    // 実名参照。上演中(セッション実行時)専用
}
```

### Target の意味論

シナリオは上演前に書かれる台本であり、作者は執筆時点でキャラクターの実名
(CharacterId)を知らない。そこで対象の指し方を2種類に区別する
(経緯: 決定ログ #1):

| バリアント | 種別 | 使える場所 |
|---|---|---|
| `Party` | 役割参照 | シナリオデータ内(CardDef.effects / SceneDef.deals)、パッチ |
| `Character(id)` | 実名参照 | **上演中専用**: GMのパッチ・実行時の配布。シナリオデータ内での使用は**不正**(解決不能。シナリオlintで検出する) |

「カードを出した本人(Current)」等の役割参照は、必要になったサイクルで追加する
(`#[non_exhaustive]` のため非破壊)。テンプレシナリオ「単純討伐」の配布は
すべて `Party` で記述できる。

Effect / Condition / Target は今後の追加が前提(タグ条件、シナリオ経験条件、
Party/Actor等の対象追加など)。シリアライズは種別名を含むタグ付き形式
(serdeの外部タグ等)とし、`#[non_exhaustive]` を付けて後方互換の追加を許容する。

## シナリオ構造

```
Scenario
 ├ meta (id, title, author, forked_from: Option<ScenarioId>)
 ├ card_defs: シナリオ固有カード定義リスト(Markerカードの定義含む)
 └ phases: [Opening, Middle, Climax]
     └ scenes: [SceneDef]
```

```rust
struct ScenarioMeta {
    id: ScenarioId,
    title: BoundedString<200>,
    author: BoundedString<200>,
    forked_from: Option<ScenarioId>,
}

struct Scenario {
    meta: ScenarioMeta,
    card_defs: Vec<CardDef>,   // Markerカードの定義含む。検索はヘルパー経由
    phases: Vec<PhaseDef>,
}

struct PhaseDef {
    phase: Phase,
    scenes: Vec<SceneDef>,
}

struct Deal {
    card: CardId,
    to: Target,
}

struct SceneDef {
    id: SceneId,
    kind: SceneKind,               // Conversation | Travel | Battle | ...
    narration: BoundedString<2000>, // シーン開始時の描写
    deals: Vec<Deal>,              // 入場時に配るカード
    exits: Vec<Transition>,        // 遷移条件(Condition/カード効果)
}

struct Transition {
    condition: Condition,      // これを満たしたら自動遷移
    to: SceneId,
}
```

シーン遷移は原則カードの `GotoScene` 効果か、`Condition`(`HasCard`等)による
自動遷移で表現する。

### コレクションと id の規則

「台本(作者データ)」と「上演(実行時状態)」で規則を分ける
(経緯: 決定ログ #2):

| 区分 | 型 | id の正 | 理由 |
|---|---|---|---|
| 作者データ(card_defs / phases / scenes / party) | `Vec<T>` | **構造体内の埋め込み id** | 並び順に意味がある(文書)。シリアライズが決定的(fixture・フォーク差分が安定) |
| 実行時索引(hands / roles) | `HashMap<Id, V>` | **キー**(値に id を持たない=二重化しない) | 順序不問の索引。O(1)参照 |

- id の一意性(card_defs 内の CardId、全 phases を通した SceneId、party 内の
  CharacterId に重複なし)と参照整合性は**不変条件**とする。パッチの validate と
  シナリオlintで検査し、プロパティテスト(test-strategy.md 不変条件)でも固定する
- Vec からの検索はヘルパーメソッド(例: `Scenario::card_def(&CardId)`)で提供する。
  MVP規模(数十枚)では線形探索で十分。性能が問題になったら実行時に別途
  索引(非シリアライズのキャッシュ)を構築する

### 勝敗分岐

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

### シナリオファイルの配置

- シナリオデータ(JSON)はリポジトリ直下 `shared/scenarios/` に置く。特定の
  crate(core/cli)やpnpmパッケージに従属させず、将来のweb/apiからも同じ
  パスを参照できるようにするため
- ファイル名は `{ScenarioId}.json`(例: `shared/scenarios/simple-hunt.json`)
- 中身は `Scenario` 構造体をserde_jsonでシリアライズしたもの(トップレベルが
  `Scenario` 1個のJSONオブジェクト)。専用のラッパー形式は設けない
- ファイル読み込みは常にIO層(tabifuda-cli等)が担う。tabifuda-core自体は
  ファイルを読まない(コアの純粋性を維持。[scenario-lint.md](scenario-lint.md)
  「置き場所と純粋性」参照)

## 文字列の長さ上限(BoundedString)

自由入力・作者データの長さ上限は `BoundedString<const MAX: usize>` newtype で
型レベルに設ける。機構と段階適用の方針の正は cross-cutting.md
「自由入力(UGC)の取り扱い」§3。本モデルでの適用状況:

- **実行時の自由入力**(`PlayCard.free_text` / `Propose.text` /
  `ScenarioPatch.note`): 4096文字
- **作者データ**: 識別的な短いテキスト(`CardDef.name`、`ScenarioMeta.title` /
  `author`)は200文字、説明的な長いテキスト(`CardDef.text`、
  `SceneDef.narration`)は2000文字。実行時の自由入力より小さいのは、
  作者データは一括投入されるコンテンツでありDoS面の切実さが実行時入力
  ほど高くないためだが、無制限にはしない
- `BoundedString` はJSON上は素の文字列としてシリアライズされるため、
  シナリオデータの形式には現れない(型レベルの検証が増えるのみ)

## セッション状態

```rust
struct ScenarioSnapshot(Scenario);  // 開催時点のシナリオを凍結コピーしたもの

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
    proposal_seq: u64,            // ProposalId発番用の単調カウンタ
                                  // (下記「提案と裁定」参照)
    card_instance_seq: usize,     // CardInstanceId発番用の単調カウンタ(下記
                                  // 「カードの消費・除去」節参照。除去してもID
                                  // を再利用しないため巻き戻さない)
    scene_local_instances: Vec<CardInstanceId>, // 現在のシーンが`SceneEntered`
                                  // で配ったカードのうち、まだ手札にあるものの
                                  // 一覧(シーン離脱時のクリーンアップ対象の候補)
}

enum SessionStatus {
    Running,
    Paused { proposal: ProposalId },  // 提案の裁定待ち
    Ended(Outcome),
}

enum Outcome {
    Victory,
    Defeat,
}

struct Proposal {
    id: ProposalId,
    by: CharacterId,
    text: BoundedString<4096>,
}

struct CardInstance {
    id: CardInstanceId,   // 配布された1枚ごとの実体
    card: CardId,         // 元になったCardDef
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

## アクターと権限

権限はルールの一部であり、API層ではなく **decide の中で検証する**
(cross-cutting.md 参照。サーバ側でも同じcoreを通すため、検証は一箇所で済む)。

```rust
enum Role {
    Gm,
    Player { characters: Vec<CharacterId> },  // 操作できるキャラ
}

// decideは認証済みの本人確認(UserId)だけを受け取る
fn decide(state: Option<&Session>, actor: &UserId, cmd: Command)
    -> Result<Vec<Event>, RuleError>;
```

### state が Option になる理由

`StartSession` はセッションがまだ存在しない時点で呼ぶ唯一のコマンドであり、
検証対象の `&Session` を渡せない(そもそも無い)。そこで `decide`/`apply` の
第1引数を `Option<&Session>` / `Option<Session>` とする:

- `state == None` で許されるコマンドは `StartSession` のみ。他のコマンドは
  `RuleError::NoActiveSession` で拒否する
- 逆に `state == Some`(セッションが既に始まっている)で `StartSession` が
  渡された場合は `RuleError::SessionAlreadyStarted` で拒否する
- `StartSession` を呼んだ `actor`(UserId)がそのままセッションの **Gm として
  roles に登録される**(Gm は Player 権限を包含するため、ソロMVPはこれだけで
  成立する。「role の信頼モデル」節参照)
- `apply` は `SessionStarted` イベントを受け取ったとき、渡された `state` の
  中身を使わずに新しい `Session` を構築する。`state.is_some()` で
  `SessionStarted` 以外の通常イベントを扱い、逆の組み合わせ
  (`state == None` で通常イベント、または `state == Some` で二重の
  `SessionStarted`)は**コアの公開APIとしてpanicしない**設計とするため、
  `apply` は `Option<Session>` を返し、不正な組み合わせは `None` を返す
  (「decideの出力は必ずapply可能」という不変条件下では起こらない経路であり、
  呼び出し側であるtabifuda-cli等がここで `unwrap` するのは構わない)

### role の信頼モデル

- **`Session.roles` が役割の唯一の正**。decide は渡された UserId で
  `state.roles` を引き、役割を自分で解決する。roles に未登録のユーザーは
  `RuleError::Forbidden`
- 呼び出し側が役割を自己申告する経路は**型ごと存在しない**(旧
  `Actor { user, role }` 構造体は廃止)。「GMを名乗るだけの権限昇格」が
  型レベルで不可能になり、cross-cutting.md「API層は認証(誰か)のみ、
  認可(何ができるか)は core」の境界と一致する(経緯: 決定ログ #4)
- 権限判定は decide 内で行い、違反は `RuleError::Forbidden` で拒否

### 権限規則

- `StartSession`: 制約なし(呼んだ本人がGmとして登録されるため、事前の
  roles検証対象が存在しない。上記「state が Option になる理由」参照)
- `PlayCard` / `Propose`: そのキャラを担当するPlayer、またはGm
  (**Gm は Player の権限を包含する**)
- `JudgeProposal` / `ApplyPatch` / `GmAdvance` / `EndSession`: Gmのみ
  (`EndSession` は物語上の決着(勝敗カードが起こす `Effect::EndSession`)
  とは別に、GMがセッションを打ち切る強制操作。`GmAdvance` と同格の
  「進行の強制操作」として扱う)
- 違反は `RuleError::Forbidden` で拒否(テストは受理/拒否を対で書く)
- 全Eventに `actor: UserId` を記録する(冒険記の「誰が」・監査の根拠)

ソロMVPでは、単一ユーザーを **Gm として roles に登録**するだけでよい
(Gm が Player 権限を包含するため両ロール登録は不要。経路は本番と同一)。

## コマンドとイベント

```rust
enum Command {
    StartSession { scenario: Scenario, party: Vec<Character> },
    PlayCard { by: CharacterId, card: CardInstanceId,
               free_text: Option<BoundedString<4096>> },   // Dialogueの自由入力
    Propose { by: CharacterId, text: BoundedString<4096> }, // → Paused へ遷移
    ApplyPatch { patch: ScenarioPatch },       // GM。Paused中のみ
    JudgeProposal { proposal: ProposalId, accepted: bool }, // GM裁定 → Running へ
    GmAdvance { to: SceneId },                 // GM強制進行
    EndSession { outcome: Outcome },           // GM専用の強制終了(権限規則参照)
}

enum Event {
    SessionStarted { scenario: ScenarioSnapshot, party: Vec<Character>,
                     roles: HashMap<UserId, Role>,
                     initial_phase: Phase, initial_scene: SceneId },
    SceneEntered { scene: SceneId, narration: String,
                   local_instances: Vec<CardInstanceId> }, // 下記「カードの消費・除去」参照
    CardDealt { to: CharacterId, card: CardId, instance: CardInstanceId },
    CardPlayed { by: CharacterId, card: CardId,
                 free_text: Option<BoundedString<4096>> },
    CardRemoved { from: CharacterId, card: CardId,
                  instance: CardInstanceId, reason: RemovalReason }, // 下記参照
    EffectApplied { effect: Effect },          // 未解決Effect専用(下記参照)
    ProposalSubmitted { id: ProposalId, by: CharacterId, text: BoundedString<4096> },   // → Paused
    ScenarioPatched { patch: ScenarioPatch },
    ProposalJudged { id: ProposalId, accepted: bool }, // → Running
    PhaseAdvanced { phase: Phase },
    SessionEnded { outcome: Outcome },
}

enum RemovalReason {
    Consumed,   // 使用による消費(下記参照)
    SceneLeft,  // シーンを離れたことによる自動消去(下記参照)
}
```

ログUI(Web版)は Event 列をそのまま時系列カードとして描画する。`CardPlayed` は
カード画像+free_text の吹き出し、`ScenarioPatched` は「GMがシナリオを改修した」
カード+note、という表示が素直。

## 進行の解決規則

decide が各コマンドをどう解決し、apply が状態をどう進めるかの規範。

### セッション開始(StartSession)

`StartSession` の `initial_phase`/`initial_scene` は
`scenario.phases[0].phase` / `scenario.phases[0].scenes[0].id`(先頭フェーズの
先頭シーン)。`phases` が空、または先頭フェーズに `scenes` が無い場合は
`RuleError::ScenarioHasNoScenes` で拒否する。

### シーン入場と CardInstanceId の発番

シーン入場は共有ヘルパー `enter_scene`(初期シーン入場・`GotoScene` 効果・
`GmAdvance` が共有)が担い、`SceneEntered{scene, narration}` を発行し、
続けてそのシーンの `deals` を解決した `CardDealt` 群を発行する。

**CardInstanceId の発行**(coreはIO・乱数を持たないため決定的に発行する):
`CardDealt` イベントに `instance: CardInstanceId` を焼き込む(apply 側では
計算しない)。`Session.card_instance_seq: usize` を発番用の単調カウンタとして
持ち(`ProposalId`/`proposal_seq` と同じ設計)、decide は
`{session.card_instance_seq}` を起点にこのdecide呼び出し内で新規発行する
インスタンスへ連番を割り当てる(`{CardId}-{連番}` 形式)。`apply` は
`CardDealt` イベントを1件処理するごとに `card_instance_seq` を+1する。
カードは除去されうる(下記「カードの消費・除去」参照)ため、除去後もこの
カウンタは巻き戻さない(一度発行したIDを再利用しない)。

### PlayCard の解決と拒否系

**Effect解決**(`CardDef.effects` を先頭から順に解決):
- `GotoScene(id)`: `id` がシナリオに存在しなければ `RuleError::SceneNotFound`。
  存在すれば `enter_scene`(上記)で入場する(初期シーン入場と同じ解決経路)
- `AdvancePhase`: 現在の `Phase` の次(`Opening→Middle→Climax`)へ進める
  `PhaseAdvanced` を発行。既に `Climax` なら次が無いため `RuleError::NoNextPhase`
- `DealCard{card, to}`: `Target` を対象キャラID群へ解決し、各キャラへ
  `CardDealt` を発行。`Target::Party` はパーティ全員、`Target::Character(id)`
  はそのキャラ1人(シナリオデータ内での `Character` 使用はシナリオlintで
  別途検出される想定であり、decideはどちらも同じ規則で解決してよい)
- `EndSession(outcome)`: `SessionEnded{outcome}` を発行(以降のEffectは解決しない)
- `ModifyStat{..}`: **型のみ、解決は後回し**(スコープ外)。状態は変更せず
  `EffectApplied{effect}` のみ発行し、監査ログ上に「効果が存在した」ことだけ残す。
  `EffectApplied` はこの「未解決Effect」専用であり、上記の解決済みEffectとは
  重複して発行しない

**拒否系**: `requires` 未達は `RuleError::ConditionNotMet`
(`HasCard` は実行者キャラの手札+`table`、`StatAtLeast` は実行者キャラの
`stats` を見る)。`card`(CardInstanceId)が実行者キャラの手札に無ければ
`RuleError::CardNotFound`。

### カードの消費・除去

**消費ルール(`CardKind`から自動導出。`CardDef`のスキーマ変更なし)**:
`CardKind::is_consumed_on_play(&self) -> bool` が `Scenario`/`Dialogue` を
`true`(使用時に除去)、それ以外(`Action`/`Item`/`Marker`)を `false` と判定する。
作者がカード単位で上書きする機構は導入しない(需要が具体化したら
future-requirements.mdへ)。

**`PlayCard`での除去**: `decide_play_card`は`CardPlayed`発行の直後
(`effects`解決の前)、`is_consumed_on_play(card_def.kind)`が`true`なら
`CardRemoved{from: by, card: card_def.id, instance, reason: Consumed}`を発行する。

**シーン離脱時のクリーンアップ**: `SceneEntered`は、そのシーンの
`scene_def.deals`から実際に配ったカードの`CardInstanceId`一覧を
`local_instances`に積む(カード効果由来の`DealCard`は含まない。
`quest_accepted`のような「持続する付与」と区別するため)。`apply`は
`Session.scene_local_instances`をこの一覧で丸ごと差し替える。

新しいシーンへ入場する直前(`decide_play_card`の`GotoScene`分岐、
`decide_gm_advance`)、`session.scene_local_instances`のうち
「まだ手札にある」かつ「`CardKind`が`Marker`ではない」ものを
`CardRemoved{reason: SceneLeft}`として発行してから`enter_scene`を呼ぶ
(選ばなかった側の選択肢カードが、シーンを離れた時点で自動的に消える)。
`decide_play_card`側は今出したカード自身の`instance`を対象から除く
(`Consumed`で既に除去済みのため)。

**Markerは除去対象外**: 消費ルール・シーン離脱クリーンアップのどちらの
対象にもならない(「選んだ記録」として`Session.hands`に残り続ける。
`Condition::HasCard`の判定にも影響しない)。ただし**CLIの手札表示からは
`CardKind::Marker`を除外する**(domain-guide.md「世界はすべてカード」の
Markerの用途どおり、プレイヤーが選ぶ対象ではなく世界の状態を示す印である
ため)。これは規範(decide/apply)ではなくCLI(tabifuda-cli)の表示ロジックの
決定であり、`session.hands`のデータ自体は変更しない。

**冒険記(chronicle)**: `CardRemoved`は明示的に扱うが、テキストとしては
描画しない(プレイヤー行動の物語的な流れを主役にする。housekeeping detail
は省く)。運用ログ(oplog)には種別(`CardRemoved`)のみ記録する。

### 提案と裁定(Propose / JudgeProposal)

**ProposalIdの発番**: `pending_proposal`は裁定のたびに`None`へ戻る
(=除去が起きる)ため、現在状態からの逆算(総数起点の連番)では一意性を
保てない。そこで`Session.proposal_seq: u64`を単調カウンタとして持ち、
`decide`は`{session.proposal_seq}`を埋め込んだ`proposal-{seq}`形式のIDを発行、
`apply`(`ProposalSubmitted`)がインクリメントする。外部発行UUIDの添付は
純粋性には抵触しないが不採用(一意性の検証責務をcoreに閉じることと、
CardInstanceIdとの発番戦略統一のため。ADR 0005参照)。

**Propose**: `by`を担当するPlayerまたはGmのみ受理(`PlayCard`と同じ
`check_player_or_gm`)。`ProposalSubmitted{id, by, text}`を発行し、
`apply`側で`status`を`Paused{proposal: id}`にし`pending_proposal`を設定する。

**JudgeProposal**: Gm専用。`status`が`Paused{proposal}`かつ`proposal`が
コマンドの`proposal`と一致する場合のみ受理し、`ProposalJudged{id, accepted}`
を発行する。不一致(`status == Running`で裁定対象が無い場合を含む)は
`RuleError::ProposalNotFound`。`accepted`の真偽によらず`apply`は
`status`を`Running`に戻し`pending_proposal`を`None`にする(採用時に挟まる
`ScenarioPatched`は別コマンド`ApplyPatch`が担い、`JudgeProposal`
自体の解決規則には影響しない)。

### GmAdvance(強制進行)

Gm専用。`enter_scene`(上記)を直接呼び出し、カードの`requires`や
`Condition`を経由せず`SceneEntered`+`deals`解決分の`CardDealt`を発行する。
状態機械図に載らない進行の強制操作のため、`Running`/`Paused`いずれでも許可し
(状態は変えない)、共通の拒否系(`Ended`)にのみ従う。遷移先シーンが存在
しなければ`PlayCard`の`GotoScene`と同じ`RuleError::SceneNotFound`。

### 共通の拒否系

`status == Ended` の間の全Commandは `RuleError::SessionEnded`。
`status == Paused` の間の `PlayCard` / `Propose` / `EndSession` は
`RuleError::SessionPaused`(状態機械図に `Paused --EndSession-->`等が無いことに対応)。
逆に`ApplyPatch`は`status == Paused`の間**のみ**許可され、それ以外は
`RuleError::SessionNotPaused`(status != Pausedでの拒否。既存の
`RuleError::SessionPaused`とは逆方向)。

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
    note: BoundedString<4096>,   // GMのコメント。ログカードとして表示
}
```

設計意図:
- **再開の安全性**: 適用前に「現在シーンの削除」「配布済みカード定義の消失」等を
  操作単位で検証できる(`validate(session, patch) -> Result<(), PatchError>`)
- **ログ表示**: パッチが小さいため `ScenarioPatched` を1枚のカードとして
  時系列ログに描画できる(「世界はすべてカード」に一致)
- **フォーク公開**: 改編シナリオ=元シナリオ+パッチ列。由来を追跡できる形で公開可能

### validate の解決規則

**PatchOpごとの検証**(`validate`が`ScenarioPatch.ops`を先頭から順に、直前までの
opを反映した状態に対して適用する。同一パッチ内の後続opは先行opの結果を参照できる):
- `AddCardDef(def)`: `def.id`が既存`card_defs`と重複していれば`PatchError::DuplicateCardId`
- `ReplaceScene(new_def)`: `new_def.id`と一致する既存シーンが無ければ
  `PatchError::SceneNotFound`(「追加」ではなく「置換」のため、新規idはAddSceneが担う)
- `AddScene{phase, scene}`: `scene.id`が既存シーン(全phase通して)と重複していれば
  `PatchError::DuplicateSceneId`。`phase`に対応する`PhaseDef`が無ければ
  `PatchError::PhaseNotFound`(既存フェーズへの追加のみを許可し、新規フェーズの
  作成はしない)
- `AddTransition{scene, transition}`: `scene`(遷移の所有シーン)または
  `transition.to`(遷移先)がシナリオに無ければ`PatchError::SceneNotFound`
- `DealCard{card, ..}`: `card`のCardDefがシナリオに無ければ`PatchError::CardNotFound`

**事後不変条件**(test-strategy.md 不変条件5。全op適用後の結果に対して検証):
- 現在シーン(`session.scene`)が解決できることの確認。現行のPatchOp(5種)には
  削除系操作が無いため実際にはこのチェックが失敗する経路は存在しない
  (将来`RemoveScene`等が追加された時のための防御的チェック。拒否テストは
  PatchOp拡充サイクルまで保留。経緯: agent-journal.md 2026-07-19 P1 C4)
- 配布済みカード(`session.hands`+`table`の全`CardInstance.card`)のCardDefが
  シナリオに解決できることの確認。`DealCard{card: 未定義のCardId, ..}`で
  再現できるため受理/拒否対でテストする

### ApplyPatch の実行

Gm専用・`status == Paused`の間のみ許可(`RuleError::SessionNotPaused`
はstatus != Pausedでの拒否、既存の`RuleError::SessionPaused`とは逆方向)。
`validate`を通ったパッチは`ScenarioPatched{patch}`を発行し、`PatchOp::DealCard`分は
`enter_scene`と同じ連番起点で`CardDealt`を追加発行する(その場で配る、
入場時配布とは別経路)。`apply`側は`ScenarioPatched`の適用時に`patch::apply_ops`を
Sessionのシナリオへ直接適用する。

### 提案への応答UI(CLIの決定。規範ではない)

提案は「採用/却下」の二値だけでは足りない。プレイヤーの提案が
「獣の姿は?被害は?」のような**確認・質問**である場合、GMの自然な応答は
「答えを引き出す選択肢(カード)をシナリオに足してから再開する」ことであり、
これは状態機械の `Paused --ScenarioPatched(0回以上) + ProposalJudged(採用)-->
Running` がそのまま担う。コアの変更は不要で、以下はtabifuda-cliの
操作系の決定である。

- Paused画面の選択肢は `y`(採用)/`n`(却下)/`c`(カードを配って応える)
- `c` はGMに **カード名** と **回答文** を入力させ、
  `ApplyPatch { ops: [AddCardDef(def), DealCard{card, to: Party}], note }`
  を1回で発行する。作られる`CardDef`は `kind: Scenario`・`text: 回答文`・
  `effects: []`・`requires: []`(出すと消費され、シーンは変わらない。
  回答はカード使用時のtext表示(下記)で開示される)
- `CardId`はCLIが `gm-card-{n}` 形式で発番する(既存`card_defs`と重複しない
  最小の連番を探す)。発番の一意性検証は従来どおり`validate`
  (`PatchError::DuplicateCardId`)が担い、CLIはルール分岐を持たない
- `note`はCLIが「提案に応えてカードを配布」の定型文で自動生成する
  (回答文自体はカードを出すまで開示しない。冒険記に載るのはnoteのみ)
- パッチ適用後もPausedのまま(状態機械どおり)。`c`を繰り返して複数枚
  配ってから、最後に`y`/`n`で裁定して再開する

### カード使用時のtext表示(CLIの決定。規範ではない)

`PlayCard`受理後、CLIは出したカードの`CardDef.text`を表示する
(カード本文の開示。上記の質問カードでは「出すと回答が読める」となる)。
冒険記(chronicle)には含めない(全カードのフレーバー文で冒険記が
埋まるのを避ける。回答文自体は`ScenarioPatched`の`AddCardDef`として
イベントログに残るため、情報は失われない)。

冒険記のカード名解決は開始時スナップショットに加えて
`ScenarioPatched`の`ops`中の`AddCardDef`を反映する(パッチで足された
カードの名前がID表示に落ちないようにする。表示用の翻訳であり、
`patch::apply_ops`の再実装ではない)。

## シナリオlint

シナリオデータの静的検証(検査項目・重大度・到達可能性/詰み検知の探索範囲)の
規範は [scenario-lint.md](scenario-lint.md) に分離した。実装は
`tabifuda-core::lint`(純粋関数)と `tabifuda-cli lint` サブコマンド。

## ソロMVPでの簡略化

- プレイヤーがGMを兼任(単一ユーザーを Gm として roles に登録。
  Gm は Player 権限を包含する)
- 判定(ダイス)なし。分岐はMarkerカードとカード選択のみ
- 戦闘は勝利/敗北カードの選択で表現(上記「勝敗分岐」)
- パーティは1人から
- テンプレシナリオ「単純討伐」1本: OP(依頼会話)→ミドル(移動)→
  クライマックス(勝敗選択)→エピローグ(勝利/敗北で分岐)→終了

## 次版(v1)以降に送る事項

1. **判定システム**: `CheckResolved { roll, success }` イベントの追加で拡張
2. **ターン制戦闘**: 敵もカードを持つ戦闘。TurnStateの本格化
3. **パッチ操作の拡充**: 必要になったら `RemoveScene` 等を追加。
   全体置換オペは最後の逃げ道として温存

## 決定の経緯(参照)

本文書は現在の仕様のみを保持する(正を二重化しない)。主要決定の「なぜ」は
以下から辿る。「決定ログ」= tasks/plans/p1-c1-review-decisions.md。

| 決定 | 経緯の記録 |
|---|---|
| flags 廃止(状態表現を Marker カードへ統一) | agent-journal.md 2026-07-18 / git履歴(旧v0.2冒頭の変更点) |
| Target の意味論(役割参照/実名参照) | design/reviews/p1-c1-type-review.md H1 / 決定ログ #1 |
| コレクションと id の規則 | 同 M3 / 決定ログ #2 |
| BoundedString の機構・段階適用・上限値 | cross-cutting.md §UGC / 決定ログ #3 / agent-journal.md 2026-07-19(P2でのretrofit) |
| role の信頼モデル(Actor 廃止) | 同 H2 / 決定ログ #4 |
| state の Option 化・CardInstanceId 発番・EndSession 権限 | agent-journal.md 2026-07-19(P1 C2) |
| ProposalId の連番発番(UUID不採用の根拠訂正含む) | adr/0005-proposal-id-issuance.md / tasks/plans/proposal-id-issuance-decisions.md |
| validate の「現在シーン削除」拒否テスト保留 | agent-journal.md 2026-07-19(P1 C4) |
| カードの消費・除去 | retrospectives/phase2.md / tasks/plans/merry-leaping-tide.md |
| シナリオファイル配置と lint 仕様 | tasks/phase2-task.md C1 / git履歴 |

### 旧節名との対応(過去文書からの参照用)

過去の決定ログ・ジャーナル・ADR・計画文書は、本文書の旧節名(サイクル番号軸)
を参照している。それらの文書は凍結された記録なので更新しない。対応:

| 旧節名 | 現在の置き場 |
|---|---|
| C2: decide/applyの解決規則 | 「進行の解決規則」の各節(StartSession / シーン入場 / PlayCard / 共通の拒否系) |
| C3: decide/applyの解決規則 | 「提案と裁定」「GmAdvance(強制進行)」 |
| C4: decide/applyの解決規則 | 「validate の解決規則」「ApplyPatch の実行」 |
| 作者データへのBoundedString適用(P2 C3前決定) | 「文字列の長さ上限(BoundedString)」 |
| シナリオlint(P2 C1決定) | [scenario-lint.md](scenario-lint.md) |
| 冒頭の「v0.1/v0からの変更点」 | git 履歴 |
