# P1 C1 型設計レビュー: 判断タスクの進捗管理

元文書: docs/design/reviews/p1-c1-type-review.md(指摘の全文はそちらが正)。
本ファイルは**人間の判断が要る4項目の進捗証跡**。判断のやり取りが
セッションを跨いでも追えるよう、状態変化のたびに本ファイルを更新して
コミットする。

## 運用ルール

- 状態は「未着手 → 議論中 → 決定済み → 反映済み」の4段階
- 状態が変わったら本ファイルを更新し、**その場でコミット**する
  (「議論中」で中断してもどこまで話したかが残るように)
- 「決定済み」時は決定内容と理由を本ファイルに記録し、
  domain-model.md等への反映が済んだら「反映済み」に更新する
- 再開時のエージェントへ: 状態表を見て、最初の未完了項目から
  「①論点提示 → ②人間が決定 → ③文書・型へ反映 → ④本ファイル更新」を続ける

## 状態表(2026-07-18 現在)

| # | 項目 | 状態 | ブロック先 | 決定内容 |
|---|---|---|---|---|
| 1 | H1: Target の意味論 | 反映済み | C2 | Party追加。Characterは上演中専用 |
| 2 | M3: コレクション型と id の正 | 反映済み | C2/C5 | 台本はVec+埋込id、索引はMapキーが正 |
| 3 | L2: 文字列長上限の投入時期 | 議論中 | C2 | — |
| 4 | H2: role 信頼モデルと Actor 形状 | 未着手 | C3 | — |

処理順は上から(早くブロックする順)。C2着手前に #1〜#3、C3着手前に #4 の
決定が必要。

---

## #1 H1: Target の意味論

**論点**: シナリオは「パーティ誕生前に凍結される作者データ」なのに、
`Target::Character(CharacterId)` は作者が書けない実行時IDを要求する。
テンプレシナリオ「単純討伐」の deals すら記述不能。

**選択肢(レビュー文書の案)**:
- (a) `Target::Current`(カードを出した本人/配布文脈では手番キャラ)を追加。
  テンプレシナリオはこれを使う
- (b) Deal から Target を外し、配布は「手番キャラ固定」等のルールにする
- (c) その他(パーティ全員 `Party` の要否も含めて議論)

**決定**(2026-07-18): **(a)の縮小版: `Target::Party` のみ追加**。

- 対象の指し方を「役割参照」と「実名参照」に区別する:
  - `Party`(パーティ全員)= 役割参照。台本執筆時点で書ける。
    シナリオデータ(CardDef.effects / SceneDef.deals)とパッチの両方で使用可
  - `Character(id)` = 実名参照。**上演中専用**(GMのパッチ・実行時配布)。
    シナリオデータ内での使用は不正(解決不能)とし、P2のシナリオlintで検出
- `Current`(カードを出した本人)等の追加役割参照は、必要になったサイクルで
  追加する(`#[non_exhaustive]` のため非破壊)

**理由**: 単純討伐(P2のテンプレシナリオ)の配布はすべて Party で記述でき、
Current は現時点で使い所がない。「未使用機能の先回り実装禁止」(CLAUDE.md)と
整合する最小の拡張を選んだ。

**経緯**:
- 2026-07-18 議論開始。前提理解のため、PO向け解説文書
  docs/design/domain-guide.md を作成(§4 に Target の概念と本論点の背景を平易に記載)。
  文書の地図 docs/README.md も同時に整備
- 2026-07-18 決定・反映(domain-model.md「Target の意味論」節を新設、
  card.rs に Party 追加、golden_tests に固定表現追加、domain-guide.md §4 更新)

**反映先**: ✅ domain-model.md「カード」節+「Target の意味論」節 / ✅ card.rs /
✅ golden_tests.rs / ✅ domain-guide.md §4。
残タスク: P2 シナリオlintに「シナリオデータ内の Character 使用検出」を含める
(test-strategy.md §2 のlint実装時)

---

## #2 M3: コレクション型と id の正

**論点**: `card_defs: HashMap<CardId, CardDef>` はキーと `CardDef.id` が二重
(不整合が可能)。一方 scenes は `Vec<SceneDef>`、party は `Vec<Character>` で、
キーイング方針が3様に割れている。

**選択肢**:
- (a) 全て HashMap に寄せ、「キーが正・構造体内 id を削除」
- (b) 全て HashMap に寄せ、「構造体内 id が正・キー一致を不変条件化」
- (c) 全て Vec に寄せ、id 走査で引く(件数が少ない前提)
- (d) 現状維持し、キー=id 一致を apply の不変条件+プロパティテストで固定

**決定**(2026-07-18): **「台本は Vec、索引は Map」で区分を規則化**。

- 作者データ(card_defs / phases / scenes / party)= `Vec<T>`+**埋め込み id が正**。
  並び順に意味がある文書であり、シリアライズが決定的(fixture・フォーク差分が安定)
- 実行時索引(hands / roles)= `HashMap<Id, V>`+**キーが正**(値に id を持たず
  二重化しない)
- 唯一の例外だった `Scenario.card_defs` を `HashMap<CardId, CardDef>` から
  `Vec<CardDef>` に変更。検索は `Scenario::card_def()` / `scene_def()` ヘルパーで提供
- id の一意性・参照整合性は不変条件とし、validate / シナリオlint(P2)/
  プロパティテスト(C5)で固定する

**理由**: 現状の型は card_defs 以外すでにこの区別に従っており、変更が最小。
HashMap キー方式(a/b)はシーンの並び順の別管理と PatchOp の形の変更を要し、
シリアライズ順序の非決定性も残る。純粋な現状維持(d)は二重の正が永続データに
残り続ける。

**経緯**:
- 2026-07-18 議論開始。「作者データ/実行時索引の区別」「HashMap のシリアライズ
  順序非決定性」「PatchOp が埋め込み id に依存」の3観点を追加提示
- 2026-07-18 決定・反映

**反映先**: ✅ domain-model.md「コレクションと id の規則」節を新設 /
✅ scenario.rs(card_defs を Vec 化、card_def()/scene_def() ヘルパー追加)。
残タスク: C4 の validate と P2 の lint に id 一意性検査を含める。C5 で不変条件を
プロパティテスト化(test-strategy.md の不変条件に追補が必要になったら同時更新)

---

## #3 L2: 文字列長上限の投入時期

**論点**: cross-cutting.md §UGC は「長さ上限を型レベルで設ける」を P1 対応項目に
挙げるが、未着手。`Proposal.text` / `narration` / `name` / C2 で登場する
`free_text` が対象。

**選択肢**:
- (a) C2 で `BoundedText` newtype を導入し、Command の free_text から適用開始
  (既存フィールドも同時に置換)
- (b) C2 では free_text のみ、既存フィールドは P2 以降に段階適用
- (c) 型レベルはやめ、decide 内バリデーションにする(cross-cutting.md の改訂が必要)

**決定**: (未定)

**反映先**: cross-cutting.md §UGC、domain-model.md、card.rs / session.rs 等

---

## #4 H2: role 信頼モデルと Actor 形状

**論点**: `decide(&Session, &Actor, cmd)` の `Actor{user, role}` は呼び出し側が
role を自己申告する形。decide がこれを信頼すると Gm 自称で権限昇格でき、
cross-cutting.md「認可は core に委ねる」に反する。`Session.roles` と真実の源が
二重になっている。

**選択肢**:
- (a) role は `session.roles[actor.user]` から引くのが正。`Actor` は
  `UserId` のみ(または UserId の別名)に縮める
- (b) `Actor{user, role}` の形は保つが、decide 冒頭で `session.roles` と
  照合し不一致は `RuleError::Forbidden`
- (c) 現状維持(ソロMVPでは実害なし)とし、P4 のサーバ実装時に再設計

**決定**: (未定)

**反映先**: domain-model.md「アクターと権限」節(decide シグネチャ)、actor.rs、C3 実装

---

## 更新履歴

- 2026-07-18: 文書作成。全4項目「未着手」
- 2026-07-18: #1 H1 を「議論中」へ。前提資料として domain-guide.md / docs/README.md を作成
- 2026-07-18: #1 H1 決定(Party追加・Characterは上演中専用)→ 反映済み。次は #2 M3
- 2026-07-18: #2 M3 を「議論中」へ。論点整理で「作者データ(順序が意味を持つ)と
  実行時索引(順序不問)で規則を分ける」観点、および HashMap のシリアライズ順序
  非決定性(fixture・フォーク差分への影響)を追加で提示
- 2026-07-18: #2 M3 決定(台本はVec+埋込id、索引はMapキーが正)→ 反映済み。次は #3 L2
- 2026-07-18: #3 L2 を「議論中」へ。対象Stringを「実行時の自由入力(Proposal.text/
  free_text/note)」と「作者データ(name/title/text/narration/author)」に分類。
  機構は const generics の BoundedString<MAX>(custom Deserialize で境界を強制、
  panicなし)を軸に、投入時期(段階適用)を論点として提示
