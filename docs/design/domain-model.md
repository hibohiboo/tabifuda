# ドメインモデル叩き台 v0.2

対象: Rustコア(crates/tabifuda-core)。コンソール版ソロプレイMVPの範囲+将来の非同期マルチを見据えた土台。

v0.1からの変更点:
- **アクターと権限を導入**(cross-cutting.md §権限 からの逆輸入)。
  全コマンド実行に認証済み UserId が付き、役割は Session.roles から解決、
  GM専用コマンドの検証を decide 内で行う(「role の信頼モデル」参照)
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
    Party,                     // パーティ全員。台本(シナリオ)執筆時点で書ける役割参照
    Character(CharacterId),    // 実名参照。上演中(セッション実行時)専用
}
```

### Target の意味論(2026-07-18決定。レビューH1)

シナリオは上演前に書かれる台本であり、作者は執筆時点でキャラクターの実名
(CharacterId)を知らない。そこで対象の指し方を2種類に区別する:

| バリアント | 種別 | 使える場所 |
|---|---|---|
| `Party` | 役割参照 | シナリオデータ内(CardDef.effects / SceneDef.deals)、パッチ |
| `Character(id)` | 実名参照 | **上演中専用**: GMのパッチ・実行時の配布。シナリオデータ内での使用は**不正**(解決不能。P2のシナリオlintで検出する) |

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
    title: String,
    author: String,
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
    kind: SceneKind,           // Conversation | Travel | Battle | ...
    narration: String,         // シーン開始時の描写
    deals: Vec<Deal>,          // 入場時に配るカード
    exits: Vec<Transition>,    // 遷移条件(Condition/カード効果)
}

struct Transition {
    condition: Condition,      // これを満たしたら自動遷移
    to: SceneId,
}
```

### コレクションと id の規則(2026-07-18決定。レビューM3)

「台本(作者データ)」と「上演(実行時状態)」で規則を分ける:

| 区分 | 型 | id の正 | 理由 |
|---|---|---|---|
| 作者データ(card_defs / phases / scenes / party) | `Vec<T>` | **構造体内の埋め込み id** | 並び順に意味がある(文書)。シリアライズが決定的(fixture・フォーク差分が安定) |
| 実行時索引(hands / roles) | `HashMap<Id, V>` | **キー**(値に id を持たない=二重化しない) | 順序不問の索引。O(1)参照 |

- id の一意性(card_defs 内の CardId、全 phases を通した SceneId、party 内の
  CharacterId に重複なし)と参照整合性は**不変条件**とする。パッチの validate と
  シナリオlint(P2)で検査し、C5 でプロパティテスト化する
- Vec からの検索はヘルパーメソッド(例: `Scenario::card_def(&CardId)`)で提供する。
  MVP規模(数十枚)では線形探索で十分。性能が問題になったら実行時に別途
  索引(非シリアライズのキャッシュ)を構築する

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

### シナリオファイルの配置(P2 C1決定)

- シナリオデータ(JSON)はリポジトリ直下 `shared/scenarios/` に置く。特定の
  crate(core/cli)やpnpmパッケージに従属させず、将来のweb/apiからも同じ
  パスを参照できるようにするため
- ファイル名は `{ScenarioId}.json`(例: `shared/scenarios/simple-hunt.json`)
- 中身は `Scenario` 構造体をserde_jsonでシリアライズしたもの(トップレベルが
  `Scenario` 1個のJSONオブジェクト)。専用のラッパー形式は設けない
- ファイル読み込みは常にIO層(tabifuda-cli等)が担う。tabifuda-core自体は
  ファイルを読まない(コアの純粋性を維持。シナリオlintは`Scenario`値を
  受け取る純粋関数として実装する。下記「シナリオlint」参照)

## シナリオlint(P2 C1決定)

シナリオデータの静的検証。将来の`scenario-validate`スキルと実装を共有する
前提のため、CLI固有にせず `tabifuda-core::lint` に置く(coreは状態機械の
実行だけでなく「`Scenario`値を検証する純粋関数」も純粋性の条件(IO・時刻・
乱数・グローバル状態なし)を満たすため、置いてよいと判断した)。

```rust
pub fn lint(scenario: &Scenario) -> Vec<LintFinding>;

pub struct LintFinding {
    pub severity: Severity,  // Error | Warning
    pub issue: LintIssue,    // #[non_exhaustive]
}
```

**検査項目と重大度**(test-strategy.md §2「参照解決/到達可能性/詰み検知」に対応):

| 区分 | 内容 | 重大度 |
|---|---|---|
| 参照解決 | `card_defs` 内 `CardId` の重複 | Error |
| 参照解決 | 全phase通して `SceneId` の重複 | Error |
| 参照解決 | `Deal.card` / `Effect::DealCard.card` / `Condition::HasCard` が指す `CardId` が `card_defs` に無い | Error |
| 参照解決 | `Transition.to` / `Effect::GotoScene` が指す `SceneId` がどのphaseにも無い | Error |
| 参照解決 | シナリオデータ内(`Deal.to` / `Effect::DealCard.to` / `Effect::ModifyStat.target`)に `Target::Character(_)` が使われている(domain-model.md「Targetの意味論」で上演中専用と規定済み。シナリオ作者データでは不正) | Error |
| 構造 | 先頭シーンが解決できない(`phases`が空、または先頭phaseに`scenes`が無い。`StartSession`が`RuleError::ScenarioHasNoScenes`で拒否する条件と同一) | Error |
| 到達可能性 | オープニング先頭シーンから到達できないシーンがある | Warning |
| 詰み検知 | そのシーンから`Effect::EndSession`を持つカードに到達する経路が無い | Warning |

Error系はlintとして「シナリオが壊れている」ことを意味し、`tabifuda-cli lint`は
Errorが1件でもあれば非ゼロ終了・テストは失敗として扱う。Warning系(到達可能性・
詰み検知)はtest-strategy.mdの「到達不能=警告」表現どおり検出のみ行い、
失敗扱いにはしない。

**到達可能性・詰み検知の探索範囲(シーン単位の直接辺のみ)**: グラフの辺は
以下の2種のみとし、`Effect::DealCard`で後から配られたカードの効果は追わない
(不動点閉包は取らない)。テンプレシナリオ「単純討伐」相当の構成(勝利/敗北
カードをシーン入場時に配って選ばせる、C2「勝敗分岐」節参照)を検証するには
このシーン単位の辺で十分であり、閉包計算より実装がシンプルなため:
- シーン`S`の`exits[].to`(`Transition.condition`の充足可能性は見ない。
  常に辿れるとみなす楽観的判定)
- シーン`S`の`deals[].card`が指す`CardDef`の`effects`に含まれる
  `Effect::GotoScene(target)` → `S → target`

詰み検知は、到達可能な各シーンを起点に同じ辺で探索した閉包内に
`Effect::EndSession`を持つカードが配られるシーンが1つも無い場合に警告する
(到達不能シーンは既に別の警告で報告済みのため対象外)。

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
    proposal_seq: u64,            // ProposalId発番用の単調カウンタ(C3決定。下記参照)
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
    text: BoundedString<4096>,  // cross-cutting.md §UGC-3 段階適用(C3)
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
    note: BoundedString<4096>,   // GMのコメント。ログカードとして表示。
                                 // cross-cutting.md §UGC-3 段階適用(C4)
}
```

設計意図:
- **再開の安全性**: 適用前に「現在シーンの削除」「配布済みカード定義の消失」等を
  操作単位で検証できる(`validate(session, patch) -> Result<(), PatchError>`)
- **ログ表示**: パッチが小さいため `ScenarioPatched` を1枚のカードとして
  時系列ログに描画できる(「世界はすべてカード」に一致)
- **フォーク公開**: 改編シナリオ=元シナリオ+パッチ列。由来を追跡できる形で公開可能

### C4: decide/applyの解決規則(2026-07-19決定)

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
- 現在シーン(`session.scene`)が解決できることの確認。v0.1のPatchOp(5種)には
  削除系操作が無いため実際にはこのチェックが失敗する経路は存在しない
  (将来`RemoveScene`等が追加された時のための防御的チェック。2026-07-19、
  phase1-task.md C4着手時にユーザーへ確認の上、拒否テストは保留しPatchOp拡充
  サイクルへ先送り)
- 配布済みカード(`session.hands`+`table`の全`CardInstance.card`)のCardDefが
  シナリオに解決できることの確認。`DealCard{card: 未定義のCardId, ..}`で
  再現できるため受理/拒否対でテストする

**ApplyPatchの実行**: Gm専用・`status == Paused`の間のみ許可(`RuleError::SessionNotPaused`
はstatus != Pausedでの拒否、既存の`RuleError::SessionPaused`とは逆方向)。
`validate`を通ったパッチは`ScenarioPatched{patch}`を発行し、`PatchOp::DealCard`分は
`enter_scene`と同じ連番起点で`CardDealt`を追加発行する(その場で配る、
入場時配布とは別経路)。`apply`側は`ScenarioPatched`の適用時に`patch::apply_ops`を
Sessionのシナリオへ直接適用する。

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

### state が Option になる理由(2026-07-19決定。C2)

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

### role の信頼モデル(2026-07-18決定。レビューH2)

- **`Session.roles` が役割の唯一の正**。decide は渡された UserId で
  `state.roles` を引き、役割を自分で解決する。roles に未登録のユーザーは
  `RuleError::Forbidden`
- 呼び出し側が役割を自己申告する経路は**型ごと存在しない**(旧
  `Actor { user, role }` 構造体は廃止)。「GMを名乗るだけの権限昇格」が
  型レベルで不可能になり、cross-cutting.md「API層は認証(誰か)のみ、
  認可(何ができるか)は core」の境界と一致する
- 権限判定は decide 内で行い、違反は `RuleError::Forbidden` で拒否

権限規則(v0.2):
- `StartSession`: 制約なし(呼んだ本人がGmとして登録されるため、事前の
  roles検証対象が存在しない。上記「state が Option になる理由」参照)
- `PlayCard` / `Propose`: そのキャラを担当するPlayer、またはGm
  (**Gm は Player の権限を包含する**)
- `JudgeProposal` / `ApplyPatch` / `GmAdvance` / `EndSession`: Gmのみ
  (2026-07-19決定。`EndSession` は物語上の決着(勝敗カードが起こす
  `Effect::EndSession`)とは別に、GMがセッションを打ち切る強制操作。
  `GmAdvance` と同格の「進行の強制操作」として扱う)
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
    Propose { by: CharacterId, text: BoundedString<4096> }, // → Paused へ遷移(C3)
    ApplyPatch { patch: ScenarioPatch },       // GM。Paused中のみ(v0.1、C4)
    JudgeProposal { proposal: ProposalId, accepted: bool }, // GM裁定 → Running へ(C3)
    GmAdvance { to: SceneId },                 // GM強制進行(C3)
    EndSession { outcome: Outcome },           // GM専用の強制終了(下記参照)
}

enum Event {
    SessionStarted { scenario: ScenarioSnapshot, party: Vec<Character>,
                     roles: HashMap<UserId, Role>,
                     initial_phase: Phase, initial_scene: SceneId },
    SceneEntered { scene: SceneId, narration: String },
    CardDealt { to: CharacterId, card: CardId, instance: CardInstanceId },
    CardPlayed { by: CharacterId, card: CardId,
                 free_text: Option<BoundedString<4096>> },
    EffectApplied { effect: Effect },          // 下記「Effect解決」参照
    ProposalSubmitted { id: ProposalId, by: CharacterId, text: BoundedString<4096> },   // → Paused(C3)
    ScenarioPatched { patch: ScenarioPatch },   // C4
    ProposalJudged { id: ProposalId, accepted: bool }, // → Running(C3)
    PhaseAdvanced { phase: Phase },
    SessionEnded { outcome: Outcome },
}
```

ログUI(Web版)は Event 列をそのまま時系列カードとして描画する。`CardPlayed` は
カード画像+free_text の吹き出し、`ScenarioPatched` は「GMがシナリオを改修した」
カード+note、という表示が素直。

### C2: decide/apply の解決規則(2026-07-19決定)

**初期シーンの決定**: `StartSession` の `initial_phase`/`initial_scene` は
`scenario.phases[0].phase` / `scenario.phases[0].scenes[0].id`(先頭フェーズの
先頭シーン)。`phases` が空、または先頭フェーズに `scenes` が無い場合は
`RuleError::ScenarioHasNoScenes` で拒否する。

**CardInstanceId の発行**(coreはIO・乱数を持たないため決定的に発行する):
`CardDealt` イベントに `instance: CardInstanceId` を焼き込む(apply 側では
計算しない)。decide は「現在の hands+table に存在するカード総数」を起点に、
このdecide呼び出し内で新規発行するインスタンスへ連番を割り当てる
(`{CardId}-{連番}` 形式)。C2の範囲ではカードは配布のみで手札から除去され
ないため、この総数は単調増加し、Session側に別途カウンタを持たなくても
一意性が保たれる(除去を導入する将来サイクルでは要再検討)。

**Effect解決**(`PlayCard` 実行時、`CardDef.effects` を先頭から順に解決):
- `GotoScene(id)`: `id` がシナリオに存在しなければ `RuleError::SceneNotFound`。
  存在すれば `SceneEntered{scene: id, narration}` を発行し、続けてそのシーンの
  `deals` を解決した `CardDealt` 群を発行する(初期シーン入場と同じ解決経路)
- `AdvancePhase`: 現在の `Phase` の次(`Opening→Middle→Climax`)へ進める
  `PhaseAdvanced` を発行。既に `Climax` なら次が無いため `RuleError::NoNextPhase`
- `DealCard{card, to}`: `Target` を対象キャラID群へ解決し、各キャラへ
  `CardDealt` を発行。`Target::Party` はパーティ全員、`Target::Character(id)`
  はそのキャラ1人(シナリオデータ内での `Character` 使用はP2のシナリオlintで
  別途検出される想定であり、C2のdecideはどちらも同じ規則で解決してよい)
- `EndSession(outcome)`: `SessionEnded{outcome}` を発行(以降のEffectは解決しない)
- `ModifyStat{..}`: **型のみ、解決は後回し**(C2スコープ外)。状態は変更せず
  `EffectApplied{effect}` のみ発行し、監査ログ上に「効果が存在した」ことだけ残す。
  `EffectApplied` はこの「未解決Effect」専用であり、上記の解決済みEffectとは
  重複して発行しない

**PlayCardの拒否系**: `requires` 未達は `RuleError::ConditionNotMet`
(`HasCard` は実行者キャラの手札+`table`、`StatAtLeast` は実行者キャラの
`stats` を見る)。`card`(CardInstanceId)が実行者キャラの手札に無ければ
`RuleError::CardNotFound`。

**共通の拒否系**: `status == Ended` の間の全Commandは `RuleError::SessionEnded`。
`status == Paused` の間の `PlayCard` / `Propose` / `EndSession` は
`RuleError::SessionPaused`(状態機械図に `Paused --EndSession-->`等が無いことに対応)。
逆に`ApplyPatch`は`status == Paused`の間**のみ**許可され、それ以外は
`RuleError::SessionNotPaused`(C4)。

### C3: decide/applyの解決規則(2026-07-19決定)

**ProposalIdの発番**: `CardInstanceId`と異なり、`pending_proposal`は裁定の
たびに`None`へ戻る(=除去が起きる)ため、現在状態からの逆算(総数起点の連番)
では一意性を保てない(`CardInstanceId`のコメントが予告していた「除去を導入
する将来サイクル」に該当)。そこで`Session.proposal_seq: u64`を追加し、
`decide`は`{session.proposal_seq}`を埋め込んだ`proposal-{seq}`形式のIDを発行、
`apply`(`ProposalSubmitted`)がインクリメントする。UUID(v4等)はcoreの純粋性
(乱数禁止)に抵触するため不採用(検討の上、連番方式を選択)。

**Propose**: `by`を担当するPlayerまたはGmのみ受理(`PlayCard`と同じ
`check_player_or_gm`)。`ProposalSubmitted{id, by, text}`を発行し、
`apply`側で`status`を`Paused{proposal: id}`にし`pending_proposal`を設定する。

**JudgeProposal**: Gm専用。`status`が`Paused{proposal}`かつ`proposal`が
コマンドの`proposal`と一致する場合のみ受理し、`ProposalJudged{id, accepted}`
を発行する。不一致(`status == Running`で裁定対象が無い場合を含む)は
`RuleError::ProposalNotFound`。`accepted`の真偽によらず`apply`は
`status`を`Running`に戻し`pending_proposal`を`None`にする(採用時に挟まる
`ScenarioPatched`はC4スコープで別コマンド`ApplyPatch`が担い、`JudgeProposal`
自体の解決規則には影響しない)。

**GmAdvance**: Gm専用。`enter_scene`(初期シーン入場・`GotoScene`効果解決と
共有するシーン入場ヘルパー)を直接呼び出し、カードの`requires`や
`Condition`を経由せず`SceneEntered`+`deals`解決分の`CardDealt`を発行する。
状態機械図に載らない進行の強制操作のため、`Running`/`Paused`いずれでも許可し
(状態は変えない)、共通の拒否系(`Ended`)にのみ従う。遷移先シーンが存在
しなければ`PlayCard`の`GotoScene`と同じ`RuleError::SceneNotFound`。

## ソロMVPでの簡略化

- プレイヤーがGMを兼任(単一ユーザーを Gm として roles に登録。
  Gm は Player 権限を包含する)
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
