# P1 C1 型設計レビュー(Opus 4.8)

日付: 2026-07-18 / レビュー対象: crates/tabifuda-core の C1 型骨格
(ids / card / scenario / character / session / actor)
位置づけ: phase1-task.md C1 完了後の停止ポイントで実施した型設計レビュー
(agent-operations.md エスカレーション条件2。P1で最重要)。

各項目の「対処方針」は提案であって決定ではない。判断が要る項目は、採否を
決めてから domain-model.md 等へ反映する。

**進捗(2026-07-18)**: **全9項目クローズ**。安全修正(M1・M2・L1・H3)は
適用済み。人間の判断が要る項目(H1・H2・M3・L2)も決定・反映済み
(経緯は docs/tasks/plans/p1-c1-review-decisions.md)。L3(IDスマート
コンストラクタ)のみ「記録のみ・対応不要」として残置。

総評: C1の型骨格は設計文書とよく整合している。serde・`#[non_exhaustive]`・
newtype ID の方針も概ね守られている。以下は C2/C3 で実装が詰まる、または
セキュリティ posture に関わる論点を重要度順に並べたもの。

---

## 🔴 High(C2/C3 着手前に判断が要る)

### H1. `Target::Character(CharacterId)` はシナリオデータ内で表現不能 ✅ 対応済み(C1)

**対応**: `Target::Party` を追加し「役割参照/実名参照」の意味論を
domain-model.md「Target の意味論」に明記。決定の経緯は決定ログ #1
(docs/tasks/plans/p1-c1-review-decisions.md)参照。


シナリオは「パーティが存在する前に凍結される作者データ」。ところが
`CardDef.effects` や `SceneDef.deals` の中の `Target::Character(CharacterId)`
は、セッション開始時まで生成されない `CharacterId` を作者が名指しすることを
要求する。

- MVPテンプレシナリオ「単純討伐」の勝敗カード配布(`deals`)ですら、対象の
  `CharacterId` を作者が書けない。
- よく使う「このカードを出した本人」「現在の手番」「パーティ全員」が、現在の
  単一バリアントでは一つも表現できない。

`Target` は `#[non_exhaustive]` なので後から `Current`/`Actor`/`Party` を足すのは
非破壊。ただし**既存の `Character` バリアントがシナリオデータ内でどう解決される
のか**の意味論を、C2 で DealCard/ModifyStat を実装する前に決める必要がある。

対処方針(案): C2着手前に「シナリオ由来の効果における Target 解決規則」を
決める。候補は (a) `Target` に `Current`(カードを出した本人)を追加し、
テンプレシナリオはそれを使う / (b) 配布は「手番キャラ」固定にして Deal から
Target を外す、等。domain-model.md にも解決規則を明記する。

### H2. ロールの真実の源が二重(`Actor.role` vs `Session.roles`) ✅ 対応済み(C1)

**対応**: decide は認証済み UserId のみ受け取り、`Session.roles` を唯一の正に。
Actor 構造体は廃止(役割の自己申告経路を型ごと排除)。domain-model.md
「role の信頼モデル」節を新設。決定ログ #4 参照。


`decide(&Session, &Actor, cmd)` の `Actor{user, role}` に呼び出し側が role を
載せて渡す一方、`Session.roles: HashMap<UserId, Role>` も保持している。もし
decide が渡された `Actor.role` を信頼するなら、呼び出し側が `Gm` を自称する
だけで権限昇格でき、cross-cutting.md「クライアントの検証は信頼しない/認可は
core に委ねる」に反する。

- ソロMVPでは単一ユーザーが両ロールを持つため実害は出ないが、型設計が
  昇格バグを誘発する形になっている。
- domain-model.md は明示的に `Actor{user, role}` と書いており、**文書の記述と
  セキュリティ原則の衝突**。

対処方針(案): C3(権限)着手前に「role は `session.roles[actor.user]` から
引くのが正。`Actor` は実質 `UserId`(認証済みプリンシパル)だけを運ぶ」と
決め、domain-model.md の decide シグネチャと `Actor` 定義を直す。

### H3. serde のワイヤ表現が「暗黙のデフォルト」依存 + ゴールデン欠如 ✅ 対応済み(C1)

**対応**: `golden_tests.rs` を追加。`Effect`(全バリアント)/`Condition`/`Target`/
`Outcome`/`SessionStatus`/`CardInstance` の JSON 文字列を固定し、値→JSON と
JSON→値 の両方向を検証(serde デフォルトの外部タグ表現を永続契約として固定)。
外部タグ採用の意図も同ファイル冒頭コメントに明記した。
残: 明示的な `#[serde(tag=...)]` への切替や ADR 化は行わず、デフォルト外部タグを
採用する判断のまま(タプルバリアントを持つため内部タグは使えない)。


このcrateの中核価値は「冒険記の決定的リプレイ」で、シリアライズ形式そのものが
永続契約。現状:

- 各 enum は `#[serde(tag=...)]` 等の明示注釈がなく、serde のデフォルト外部タグに
  依存(結果自体は妥当: `{"DealCard":{...}}` 等)。domain-model.md は「外部タグ等」を
  許容しているので選択は妥当だが、**決定が型に明文化されていない**。
- roundtrip テストは `T→JSON→T` の同一性しか保証せず、**ワイヤ形式の破壊的変更を
  検出できない**(タプル→構造体バリアントへの整形で roundtrip は通るが旧データが
  読めなくなる)。test-strategy.md §1(d) が求める「スナップショット+旧データ読込」が
  未実施。

対処方針(案): `Effect`/`Condition`/`SessionStatus`/`CardInstance` 等の永続化される
主要型にゴールデンJSONスナップショットを最低1本ずつ追加し、外部タグ採用を型コメント
か ADR で明示。C1 の範囲で足せる最も価値の高いテスト。

---

## 🟡 Medium

### M1. `CardKind`/`SceneKind` に `#[non_exhaustive]` が無い ✅ 対応済み(C1)

**対応**: 両 enum に `#[non_exhaustive]` を付与。


- `CardKind` はこのサイクルで 5→6 種(Marker 追加)に増えた実績があり、明確に
  成長する enum。
- `SceneKind` はコメントが `Conversation | Travel | Battle | ...` と「...」で増加を
  明示している。

いずれも「追加前提 enum は `#[non_exhaustive]`」方針の対象とすべきで、現状は方針との
不整合(`Target`/`Effect`/`Condition` には付いている)。

対処方針(案): 両 enum に `#[non_exhaustive]` を付ける。安全な修正。

### M2. `Outcome` の配置が概念的なモジュール循環を作る ✅ 対応済み(C1)

**対応**: 新設 `primitives.rs` に `Outcome` を移動。`card`/`session` は
`crate::primitives::Outcome` を参照し、依存を DAG にした。


`card.rs` の `Effect::EndSession` が `crate::session::Outcome` を参照 → card 依存
session 依存 scenario 依存 card。Rust はモジュール循環を許すのでコンパイルは通るが、
**低レベルの `card` が高レベルの集約 `session` に依存する向きが逆**。

対処方針(案): `Outcome`(単なる結果値)を `ids.rs` か新設の `primitives`/`common`
モジュールへ移し、依存を DAG にする。安全な修正。

### M3. ID の二重保持とコレクション型の不統一 ✅ 対応済み(C1)

**対応**: 「台本は Vec+埋め込み id が正、実行時索引は HashMap キーが正」で
規則化し、card_defs を Vec 化。domain-model.md「コレクションと id の規則」節を
新設。決定ログ #2 参照。


- `card_defs: HashMap<CardId, CardDef>` はキーと `CardDef.id` が二重で、キー≠value.id の
  不整合が起こりうる。
- 一方 `scenes` は `Vec<SceneDef>`(埋め込み id を走査)で、カードは HashMap/シーンは
  Vec と方針が割れている。
- `party: Vec<Character>` も `hands`/`roles`(CharacterId/UserId キー)と揃っていない。

対処方針(案): 「id の正はどこか」「キー付きコレクションか Vec か」を統一し、apply 時の
不変条件(hands 全キーが party メンバー等)として固定する方針を決める。C5 の
プロパティテストが書きやすくなる。

---

## 🟢 Low(記録のみ。今すぐでなくてよい)

### L1. `Eq` の derive 漏れ ✅ 対応済み(C1)

**対応**: `Effect`/`CardDef`/`Deal`/`Transition`/`SceneDef`/`PhaseDef`/
`ScenarioMeta`/`Scenario`/`ScenarioSnapshot`/`Proposal`/`SessionStatus`/
`Session`/`Character` に `Eq` を追加(全て純粋な値型で float を含まない)。


`Effect` とそれを推移的に含む型が `PartialEq` のみ。全フィールド Eq 可能(float なし)
なので、`Effect` に `Eq` を足せばカスケードで解消。`Condition` は付いているのに
`Effect` は無く、意図的でなく見える。

### L2. 型レベルの文字列長上限(cross-cutting P1 項目)が未着手 ✅ 方針決定済み(実装はC2〜)

**対応**: 機構は `BoundedString<const MAX>`、C2 で導入し実行時入力
(free_text→text→note)から段階適用、作者データは P2。cross-cutting.md
§UGC-3 に明記。決定ログ #3 参照。


`Proposal.text`/`narration`/`name` 等に上限 newtype が無い。cross-cutting.md §3 の
「長さ上限を型レベルで設ける(free_text 等)」は P1 対応項目。Command の `free_text` が
出る C2 で `BoundedText` 導入を計画するのが自然。

### L3. ID のスマートコンストラクタ/Display 無し

`pub` インナーで空文字 ID も作れる。当面可だが、後で非空バリデーション+`Display`+
`From<&str>` があるとテストと安全性が向上。

---

## 対処の切り分け(提案)

| 区分 | 項目 | 性質 |
|---|---|---|
| 安全に即修正可 | M1, M2, L1, H3(ゴールデン追加) | 仕様判断を伴わない |
| 人間の判断が要る | H1(Target 意味論), H2(role 信頼モデル/Actor 形状), M3(コレクション/id の正), L2(長さ上限の投入時期) | domain-model.md の改訂を伴う |

決定した項目は domain-model.md(と必要なら cross-cutting.md / ADR)へ反映し、
本ファイルには「対応済み/対応サイクル」を追記する。

---

## 判断タスクリスト(人間の判断が要るもの。上から一つずつ処理する)

**進捗の正は docs/tasks/plans/p1-c1-review-decisions.md**(状態表・決定内容・
更新履歴をそちらで管理し、状態変化のたびにコミットする)。本節は処理順の
一覧のみ残す。

各項目は「①論点を提示 → ②人間が方針決定 → ③文書(と必要なら型)へ反映 →
④進捗文書を更新」の順で進める。1項目ずつ区切って合意を取る。

処理順は「早くブロックするもの優先」。H1/M3/L2 は C2、H2 は C3 をブロックする。

1. **H1(Target の意味論)** — C2 の DealCard/ModifyStat 実装をブロック。
   シナリオデータ内で対象をどう名指すか(`Current` 追加 / Deal から Target を外す 等)。
2. **M3(コレクション型と id の正)** — C2 の apply / C5 の不変条件をブロック。
   card_defs/scenes/party のキーイング統一と「id の正はどこか」。
3. **L2(文字列長上限の投入時期)** — C2 で Command.free_text が出るタイミング。
   `BoundedText` newtype を導入するか、いつ入れるか。
4. **H2(role 信頼モデルと Actor 形状)** — C3 の権限実装をブロック。
   `Actor.role` を信頼せず `session.roles` を正とするか。`Actor` を `UserId` のみにするか。

> 備考: この4項目は「決定した時点で domain-model.md を改訂 → 該当サイクルで実装」
> という流れ。C1(型骨格)では実装を変更しない。
