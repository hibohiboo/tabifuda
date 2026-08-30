# 計画: 1ファイル1責務とSSoTのルール化(書籍「プロフェッショナルAI駆動開発」導入分)

## Context

[プロフェッショナルAI駆動開発.md](../../requirements/プロフェッショナルAI駆動開発.md)
の個人導入チェックリストのうち、まず以下の2原則をプロジェクトのルールとして
明文化する(ユーザー判断、2026-08-31)。

1. **1ファイル1責務**(チェックリスト「フォルダ構成」項目の中核)
2. **SSoT**(1つの事実の情報源は一か所。チェックリスト「ルールへの誘導」を支える前提)

現状認識:

- SSoTは既に**実践されている**が明文のルールが無い。「〜が正」宣言が各所に散在
  (CLAUDE.md「設計文書が正、実装が従」、tasks/README.md「進捗の正はfrontmatter」、
  client-conventions系「正はdocs/design/、スキルは索引のみ」等)。
  慣行を原則として明文化し、新規文書・スキル作成時に強制できる形にする
- 1ファイル1責務は明文ルールも網羅的な実践確認も無い

## スコープ

- **やる**: 2原則のルール文書化と、既存の違反候補の棚卸し(列挙まで)
- **やらない**:
  - 棚卸しで見つかった違反の是正(別チケット化する。
    範囲外の不具合を現タスクに畳み込まない運用に従う)
  - 書籍チェックリストの他項目(ルールファイル3本、テスト1コマンド化等)。
    必要なら別途起票
- コード(crates/・TS側)には一切触れない。docs/ と CLAUDE.md のみの変更

## ルールの置き場所(推奨案。C2着手前に人間が承認)

ADR 0001「ADR化の基準」の基準1(複数フェーズ・複数レイヤに波及)に該当するため:

- **ADR 0007**(新規): 2原則を採用する決定と根拠、書籍由来であること、
  適用範囲(コード・docs両方)を記録する
- **CLAUDE.md**: 「最重要ルール」に短い項を追記(各1〜2行。詳細はADRへ誘導)。
  ルール本文の正はCLAUDE.md、経緯の正はADRという分担(それ自体がSSoTの実例)
- 必要に応じて client-conventions.md / Rust規約への適用例の追記は
  棚卸し結果を見て判断(先回りで書かない)

## ルール内容の骨子(要素名レベル。文面はC2で起草し人間レビュー)

SSoT:

1. 1つの事実の正(情報源)は一か所。他所に載せるときは「正は〜」と明記して参照する
2. スキル・索引・READMEは複製ではなく誘導(既存の client-conventions 方式を一般化)
3. 正と従が食い違ったら正を直してから従を追随させる

1ファイル1責務:

1. 1ファイルには1責務。ファイルの説明が「〜と〜」になったら分割候補
2. どこに何があるか名前と置き場で分かる構成を保つ
3. 行数等の機械的しきい値は設けない(判断基準は責務の数)
4. 既存違反を見つけてもその場で直さず別チケット化

## サイクル分割(実行は低位モデル想定)

### C1: 棚卸し(Haiku/Sonnet)

- docs/ 全体から「〜が正」「正は〜」宣言を一覧化する
- 同じ事実が2か所以上に正の宣言なしで書かれている箇所(SSoT違反候補)を列挙する
- crates/・apps/・packages/ のファイルで複数責務が疑われるもの
  (mod.rs的な寄せ集め、責務の説明が「と」で繋がるファイル)を列挙する
- 成果物: 本ファイル末尾に「棚卸し結果」節として追記。**是正はしない**
- 停止ポイント: 一覧を人間が確認してからC2へ

### C2: ルール文書化(Sonnet。文面は人間レビュー必須)

- ADR 0007 起草(0001の書式に倣う)
- CLAUDE.md「最重要ルール」へ追記
- プロフェッショナルAI駆動開発.md の該当チェックボックスを更新
  (「フォルダ構成」はC3完了までは保留可)
- 停止ポイント: 文面レビュー→マージは人間判断

### C3: 違反の別チケット化(Haiku)

- C1の一覧のうち是正が必要なものを、影響度順に docs/tasks/ へ起票
  (置き場は tasks/README.md の振り分けルールに従う)
- 是正作業自体は各チケットで別途実施

## 終わり方(各サイクル共通)

- 変更は docs/ のみなので cargo / pnpm 系の検証は不要
- 設計文書との乖離チェック(特にC2でCLAUDE.mdと ADR の記述が重複しすぎて
  いないか=このタスク自身がSSoT違反にならないか)
- 誤解があれば agent-journal.md へ1行追記
- 作業ブランチで行い、マージは人間判断

---

## 棚卸し結果(C1完了。2026-08-31)

### 1. SSoT違反候補（「正」宣言なしで複数箇所に同じ事実が記載）

#### 1-1. コアの基本原則の複重記載(優先度: 高)

**事実**: コアの「純粋性」「decide/apply」「イベント経由」「乱数決定性」の4原則

**記載場所**:
- [docs/design/domain-model.md](../../design/domain-model.md) L10-14「基本原則」
- [CLAUDE.md](../../../CLAUDE.md) L9-13「最重要ルール」2・3
- [crates/tabifuda-core/src/lib.rs](../../../crates/tabifuda-core/src/lib.rs) L3-8 rustdoc
- [crates/tabifuda-wasm/src/lib.rs](../../../crates/tabifuda-wasm/src/lib.rs) L4-7 rustdoc

**正はどこか**：明文のルールなし。domain-model.mdが技術的には最詳だが、CLAUDE.mdが「最重要」を名乗っている。実装側(lib.rs)が正を指示していない

**判定**: SSoT違反。正をドメインモデルか CLAUDE.md のいずれかに統一し、他から参照する形に統一する必要あり

#### 1-2. サイクル粒度の進捗の正(優先度: 中)

**事実**: task.md frontmatter に記載されたサイクル進捗

**記載場所**:
- [docs/tasks/README.md](../../README.md) L37「frontmatter が正」
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L40「正は各 task.md の frontmatter」
- [docs/roadmap.md](../../roadmap.md) L5-6「各タスク文書が正」の注記
- [docs/tasks/tools/docs-site/plans/docs-site-progress-plan.md](../../tools/docs-site/plans/docs-site-progress-plan.md) L16
- [docs/tasks/tools/docs-site/plans/d7-checklist.md](../../tools/docs-site/plans/d7-checklist.md) L3(経緯記録のみ)

**正はどこか**：tasks/README.md が「正」を宣言している(複数宣言で重複)

**判定**: SSoT違反ではない(正は宣言済みで、他所は参照が明示されている)。ただし「正」宣言の重複=冗長性が高い

#### 1-3. RDRAモデルの位置づけ(優先度: 低)

**事実**: RDRAYAMLは「手動キュレーションの非規範な索引」

**記載場所**:
- [docs/rdra/README.md](../../rdra/README.md) L10「正は design/ のまま」
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L37-38
- [docs/tasks/tools/docs-site/plans/serene-skipping-gadget.md](../../tools/docs-site/plans/serene-skipping-gadget.md) L15「非規範の索引」

**判定**: 正はrdra/README.mdで宣言済み。他所の記載は参照・確認目的。冗長性あるが違反ではない

#### 1-4. 「規範」の定義と使い分けの曖昧性(優先度: 中)

**事実**: 「規範」「正」「非規範」「索引」という概念の定義と使い分け

**記載場所**:
- [docs/README.md](../../README.md) L50-52「design/の規範文書が正」、L53「解説は非規範」
- [docs/design/domain-model.md](../../design/domain-model.md) L6-7「本文書は現在の仕様のみ」
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L35-40
- [docs/rdra/README.md](../../rdra/README.md) L10「規範は design/ のまま」
- [docs/retrospectives/phase3.md](../../retrospectives/phase3.md) L7「記録文書(非規範)」

**判定**: 概念は README.md で一度定義されているが、後発ファイル(phase3ふりかえり等)では独立に「非規範」を再定義している。「規範/非規範」の基準を統一ルール化する際の参考になる

### 2. 1ファイル1責務の疑い

#### 2-1. docs/agent-journal.md(優先度: 低)

**内容**: エージェント失敗ジャーナル(単発の誤り記録) + 傾向分析

**判定**: 1ファイルに「単発記録」と「傾向分析」という2責務が混在している可能性。ただし、ファイルサイズは実際に確認する必要あり(ジャーナルが大規模化すると分割の価値が出る)

#### 2-2. docs/CLAUDE.md(優先度: 中)

**内容**: 技術スタック、ディレクトリ構成、主要コマンド、最重要ルール、Rust規約、やらないことまで、プロジェクト全体の設定・方針が一ファイルに集約

**判定**: 実質的には「プロジェクト設定の中央集約ファイル」という1責務である(複数の観点を含むが、責務は「全体設定」という1つ)。ただし、CLAUDE.md自体の構成が章立てされていて、分割して docs/ 配下にスキャッターすることも可能性として考える価値あり(設定ファイルとしての一元管理vs 領域ごとの分散)

#### 2-3. docs/design/domain-model.md(優先度: 低)

**内容**: 型定義、状態機械、コマンド/イベント定義、シナリオ構造、セッション管理、アクターと権限、進行解決規則、パッチ仕様、lint定義、簡略化記述、決定経緯表、過去文書対応表

**判定**: かなり広い責務が1ファイルに集約されている。ただし、既に一部は分離済み(scenario-lint.mdの分離、test-strategy.mdの存在)。現状では「コアのすべてのドメイン定義」という単一責務ととらえることもできるが、さらなる分割の余地あり(型定義/状態機械/解決規則/決定経緯を別ファイル化するなど)

### 3. 現在SSoT違反ではないが「正」宣言が多い箇所(冗長性注記)

**観察**:
- docs/README.md が「文書の地図」として「どの文書が規範か」を集約している
- 個別文書内での「正は〜」宣言もそこと重複している傾向
- 理由: docs/README.md 自体が非規範(索引)なため、各文書で「自分が規範であることを独立に宣言」する必然性がある(正を二重化回避の工夫)

**判定**: スマートではないが、実装としては正しい。SSoT確立後に「docs/README.mdが真実、他は簡潔に参照」という形に整理できる可能性あり

### 4. コード側の検査結果

#### 4-1. crates/ のファイル構成

Rust クレート内の lib.rs / main.rs は標準的な「ハブファイル」パターン。mod.rs は見当たらず、モジュール宣言がクレート単位で集約されている(責務分離は正当な形)。複数責務が疑われるファイルなし。

#### 4-2. TS側(apps/web、packages/ui) のファイル構成

- `apps/web/src/`: 層ごと(core, scenario, session)に分かれていて清潔
- `packages/ui/src/`: 領域ごと(chronicle, components, core, session)に分かれていて清潔

複数責務が疑われるファイルなし。

---

## C1 の総括

**SSoT関連**:
- 優先度「高」1件：コアの基本原則の複重記載(domain-model.md vs CLAUDE.md vs rustdoc)
- 優先度「中」1件：「規範/非規範」の定義統一
- 優先度「低」その他：既に宣言されているが冗長度が高い

**1ファイル1責務関連**:
- 優先度「低」：現状は責務分離がかなり清潔。気になるのは docs/CLAUDE.md と domain-model.md のサイズと内容幅だが、分割の必然性はまだ不明

**次ステップ**: 人間が C1 の結果を確認し、SSoT違反のうちどれを C2 で扱うかを決定してから C2 へ進む

---

## 対応方針(人間承認済み。2026-08-31)

C1で見つかった3件は全て対応する。以下の方針で C2/C3 に振り分ける。

### 1. コア基本原則の重複(優先度: 高)

正は **domain-model.md「基本原則」**。CLAUDE.md 最重要ルール1
「設計文書が正、実装が従」の帰結そのものであり、新規判断ではない。

- CLAUDE.mdのルール2・3は**削除しない**(毎セッション読み込まれる地図として
  要約を掲示する価値がある)。ただし「正は domain-model.md『基本原則』」の
  参照を1行追記する → **C2で実施**(CLAUDE.md本体の編集のため)
- lib.rs(tabifuda-core)・lib.rs(tabifuda-wasm)のrustdoc原則再掲を、
  domain-model.mdへの1行ポインタに縮める。現状CLAUDE.mdを参照しているが、
  Rust規約(コードから参照してよいのはdocs/design/のみ)に反するため
  参照先もdomain-model.mdに直す → コード変更を伴うため**C3でチケット化**

### 2. 規範/非規範の定義統一(優先度: 中)

正は **docs/README.md「文書間の優先順位」**。規範・非規範・索引・記録の
4分類の定義をそこに一度だけ書く。既存の後発文書(ふりかえり等)の
「位置づけ: 記録文書(非規範)」表記は残し、docs/README.mdへの参照を添える
形に統一する。発生源(agent-operations.md「フェーズ完了時のふりかえり」
手順、retrospectiveエージェント定義)に標準の位置づけ文言を組み込み、
今後作られる文書は自動的に揃うようにする。

- docs/README.mdへの定義追記 → **C2で実施**
- agent-operations.md手順・retrospectiveエージェント定義への文言組み込み、
  既存後発文書への参照追記 → **C3でチケット化**

### 3. CLAUDE.md/domain-model.mdの責務幅(優先度: 低)

**物理的な分割は今はしない。** CLAUDE.mdは「毎セッション読む地図+最重要
ルール」という役割上、幅がある。domain-model.mdはscenario-lint.md分離の
前例があり、肥大化したら次に分離すべき節(進行の解決規則/決定の経緯)の
見当はついているが、今分割する必然性は無い。

- 分割の兆候基準(責務説明が「と」で繋がる、特定節だけへの外部参照が増える等)
  をルール文書に明記する → **C2で実施**
- 実際の分割はこの基準に当てはまった時点で別途起票(**今回はチケット化しない**。
  frozen相当の「着手条件付き保留」であって、今見つかった違反ではないため)

## C2 実施内容(このセクションの方針を反映)

1. ADR 0007新規作成: SSoT・1ファイル1責務の採用決定、上記1〜3の
   「正の所在」判断を含める
2. CLAUDE.md「最重要ルール」への追記: 2原則の要約+ADR 0007への参照、
   ルール2・3への「正はdomain-model.md『基本原則』」参照追記
3. docs/README.md「文書間の優先順位」への規範/非規範定義の明記
4. プロフェッショナルAI駆動開発.mdのチェックボックス更新
   (「フォルダ構成」はC3完了までは保留)

## C3で起票するチケット(このセクションの方針を反映)

1. lib.rs(core/wasm)のrustdoc参照先をdomain-model.mdへ修正
2. 規範/非規範定義の一本化: agent-operations.md手順・retrospectiveエージェント
   定義への文言組み込み+既存後発文書への参照追記
3. (元C1の1-2/1-3): frontmatter正宣言の重複解消、RDRA非規範宣言の重複解消
   (優先度低。時間があれば)

---

## C2 完了記録(2026-08-31)

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  新規作成。SSoT・1ファイル1責務の採用決定、C1で見つかった3件の「正の所在」
  判断、分割の兆候基準を記録
- [CLAUDE.md](../../../CLAUDE.md)最重要ルールに、ルール2・3への
  「正はdomain-model.md『基本原則』」参照追記+ルール5(SSoT・1ファイル
  1責務の要約、ADR 0007への参照)を新設
- [docs/README.md](../../README.md)「文書間の優先順位」の前に「文書区分の
  定義(SSoT)」節を新設。規範/非規範/索引/記録の定義をここに一本化し、
  ADR 0007から参照する形にした。adr/の説明行にADR 0007を含めた
  (ただしADR個別一覧は元々列挙形式ではないため既存書式を踏襲)
- [プロフェッショナルAI駆動開発.md](../../requirements/プロフェッショナルAI駆動開発.md)
  「ルールへの誘導」を完了チェック。「フォルダ構成」はC3完了後まで保留

**停止ポイント**: 文面レビューを人間に依頼する。承認後、C3(3件のチケット
起票)に進む。

---

## C3 完了記録(3件のチケット起票。2026-08-31)

### 1. [rustdoc-references-to-domain-model.md](rustdoc-references-to-domain-model.md)

**内容**: crates/tabifuda-core・crates/tabifuda-wasm の lib.rs rustdoc参照先を
CLAUDE.md から domain-model.md へ統一。全文再掲を1行ポインタに縮める。

**スコープ**: Rust コード(rustdoc コメント)のみ変更。別PRで扱う。

**関連**: SSoT方針・正の所在決定、Rust規約(コードは docs/design/ 参照のみOK)

### 2. [normalize-document-classification-definitions.md](normalize-document-classification-definitions.md)

**内容**: 「規範/非規範/索引/記録」の定義を docs/README.md「文書区分の定義」に統一。
後発文書(ふりかえり等)から参照する形に統一。発生源(agent-operations.md手順、
retrospective エージェント)に標準文言を組み込み、今後の新規文書は自動統一。

**スコープ**: docs/ のみ変更。既存後発文書への参照追記+手順文言の追加。

**関連**: SSoT方針・定義の一本化

### 3. [reduce-redundant-declarations-frontmatter-rdra.md](reduce-redundant-declarations-frontmatter-rdra.md)

**内容**: 「進捗の正は frontmatter」「RDRA は非規範の索引」という正が既に定義
されているにもかかわらず、後発文書で個別に再説明されている箇所を参照形式に統一。

**優先度**: 低。SSoT違反ではなく、冗長度削減の美化項目。

**スコープ**: docs/ のみ。対象4+α の文書の記述短縮+参照追記。

---

## 全体完了

3つのサイクル(C1 棚卸し→C2 ルール文書化→C3 別チケット化)が完了した。

**成果物**:
- ADR 0007 新規作成(SSoT・1ファイル1責務の採用決定・正の所在判断)
- CLAUDE.md 最重要ルール更新(ルール2・3への参照追記、ルール5新設)
- docs/README.md 「文書区分の定義」節新設(規範/非規範/索引/記録の定義一本化)
- プロフェッショナルAI駆動開発.md の「ルールへの誘導」チェック完了
- C3 として3件のチケット起票
  - 優先度高: rustdoc参照修正(コード変更)
  - 優先度中: 規範/非規範定義の一本化(docs)
  - 優先度低: frontmatter/RDRA宣言の重複解消(docs。美化項目)

**停止ポイント**: 人間による最終確認。マージ前に決定ログ/PR作成の判断は人間に仰ぐ。

---

## 追補: フォルダ構成レビュー(2026-08-31。チェックリスト「フォルダ構成」の完了条件)

C3完了後、「どこに何があるか分かる構成」の観点で追加レビューを実施。
チェックリスト「フォルダ構成」のチェック条件は「C3チケット完了」ではなく
**本節4項目の決着**とする(C3はSSoT是正でありフォルダ構成の見直しではないため)。

| # | 項目 | 状態 |
|---|---|---|
| 1 | docs/design/reviews/(記録)が規範ディレクトリ design/ 配下にある | **対応済み**(下記) |
| 2 | tasks/plans/ 直下の自動生成名 merry-leaping-tide.md | **対応済み**(上記) |
| 3 | CLAUDE.md の地図に未作成ディレクトリ(apps/api・packages/schema)。P4/P5凍結が未反映 | **見送り**(ユーザー判断。凍結反映はやらない) |
| 4 | コアの src/*_tests.rs 配置規約が test-strategy.md に明文なし | **対応済み**(下記) |

### 項目1の対応記録(移動せず「意図的な例外」として明記)

design/reviews/ は区分としては「記録」だが、golden_tests.rs 等の
コードコメントから直接参照されており(例: p1-c1-type-review.md)、
Rust規約「コードコメントは docs/design/ のみ参照可」と衝突するため
design/ の外へ移動できない。移動より参照規約の単純さを優先し、
**物理配置は変えず、意図的な例外として明記する**方針で対応:

- [docs/README.md](../../README.md)「文書区分の定義」に例外注記を追加
- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  「帰結」に本決定+項目2のリネーム決定を追記

### 項目4の対応記録

- [docs/design/test-strategy.md](../../design/test-strategy.md)に
  「テストファイルの置き場所」節を新設。crates/core の `src/*_tests.rs`
  分離パターン(`tests/`結合テストではなくsrc内モジュールである理由含む)
  を明文化

### 副産物: スコープ外の発見(別チケット化)

item 4対応中に、[crates/tabifuda-core/src/lib.rs](../../../crates/tabifuda-core/src/lib.rs)
L42-43のコメントが `docs/tasks/` を参照する既存Rust規約違反を発見。
その場で直さず[fix-lib-rs-docs-tasks-reference.md](fix-lib-rs-docs-tasks-reference.md)
として別チケット化した(コード変更を伴うため)。

### 項目2の対応記録(ユーザー決定: 1件のうちに改名)

- `merry-leaping-tide.md` → `hand-card-removal.md` に git mv
- 生きた参照3件を張り替え(tasks/README.md・domain-model.md「決定の経緯」表・
  roadmap.md)。歴史記録2件(docs-tasks-restructure.md・serene-skipping-gadget.md)
  の旧名言及は既存方針どおり未修正。改名先ファイル冒頭に旧名注記を追加
- 再発防止: tasks/README.md「plansの振り分けルール」に
  「横断として残す場合も内容が分かる名前へリネームする」を追記

---

## 全チケット完了(2026-08-31)

C3で起票した4件すべてに対応した(優先度順)。

| チケット | 優先度 | 結果 |
|---|---|---|
| [rustdoc-references-to-domain-model.md](rustdoc-references-to-domain-model.md) | 高 | 対応済み。crates/2箇所のrustdocをdomain-model.mdへの1行参照に縮小 |
| [normalize-document-classification-definitions.md](normalize-document-classification-definitions.md) | 中 | 対応済み。ふりかえり3件+docs-site/task.mdを参照形式に統一、発生源2箇所に定型文組み込み |
| [fix-lib-rs-docs-tasks-reference.md](fix-lib-rs-docs-tasks-reference.md) | 中(副産物発見分) | 対応済み。テスト関数名規約をtest-strategy.mdへ移設し参照化 |
| [reduce-redundant-declarations-frontmatter-rdra.md](reduce-redundant-declarations-frontmatter-rdra.md) | 低 | 対応不要と判定。現役文書は既に対応済み、歴史記録は既存方針により対象外 |

途中で派生した論点「コード→docsコメント参照の是非」もユーザーと議論の上
ADR 0007「帰結」に規律(入口のみ・不変条件のみ・ポインタ1行のみ)として
明文化し、design-syncスキルに検出観点を追加した。

**本タスクは完了。** `refactor-professional` ブランチに一連のコミットが
積まれている。masterへの統合(PR作成・マージ)は人間の判断を仰ぐ。
