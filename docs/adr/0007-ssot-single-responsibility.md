# ADR 0007: SSoTと1ファイル1責務の明文化

状態: 採用 / 日付: 2026-08-31

## 文脈

書籍「プロフェッショナルAI駆動開発」の個人導入チェックリスト
([docs/requirements/プロフェッショナルAI駆動開発.md](../requirements/プロフェッショナルAI駆動開発.md))
を導入する最初の一歩として、以下の2原則をルール化する。

- **SSoT(Single Source of Truth)**: 1つの事実の情報源は一か所
- **1ファイル1責務**

いずれも本プロジェクトで既に実践されている慣行(CLAUDE.md「設計文書が正、
実装が従」、docs/README.md「文書間の優先順位」、crates/・apps/・packages/の
層別ディレクトリ構成)の明文化であり、新規導入ではない。棚卸し
([tasks/crosscutting/ssot-single-responsibility/task.md](../tasks/crosscutting/ssot-single-responsibility/task.md))
の結果、コードは概ね健全な一方、docsに以下3件の是正候補が見つかった。

1. コアの基本原則(純粋性・decide/apply・イベント経由・乱数決定性)が
   domain-model.md・CLAUDE.md・crates/tabifuda-core/src/lib.rsの3箇所に
   独立に再掲されている
2. 「規範」「非規範」「索引」「記録」の定義・使い分けが、docs/README.mdでの
   定義に加えて後発文書(ふりかえり等)で個別に再定義されている
3. CLAUDE.md・domain-model.mdが広い責務を1ファイルに集約している
   (地図/最重要ルールとしての役割上の広さであり、現時点では分割の必然性なし)

ADR 0001「ADR化の基準」の基準1(複数フェーズ・複数レイヤに波及する決定)に
該当するため、ADRとして記録する。

## 決定

### SSoT

1. **1つの事実の正(情報源)は一か所に置く。** 他所で言及するときは
   「正は〜」と明記して参照する(複製しない)
2. **スキル・索引・READMEは複製ではなく誘導。** 詳細を持たず、正への
   ポインタに徹する(既存の client-conventions スキル、docs/rdra/README.md
   の方式を一般則とする)
3. **正と従が食い違ったら、正を直してから従を追随させる。**
   実装がコアの原則に反していれば実装を直し、参照側の記述が古ければ
   参照側を直す

適用の第一例として、以下2件の「正の所在」をこのADRで確定する:

- **コアの基本原則(純粋性・decide/apply・イベント・乱数決定性)の正は
  [domain-model.md](../design/domain-model.md)「基本原則」節とする。**
  CLAUDE.mdは要約+本ADRおよびdomain-model.mdへの参照を掲示する
  (地図としての速読性を優先し、要約自体は削除しない)。
  crates/tabifuda-core・crates/tabifuda-wasmのrustdocは、原則の全文再掲を
  やめてdomain-model.mdへの1行参照に縮める(是正はC3でチケット化)
- **「規範/非規範/索引/記録」の定義の正は
  [docs/README.md](../README.md)「文書間の優先順位」節とする。**
  後発文書は個別に再定義せず、docs/README.mdへの参照を添える
  (是正はC3でチケット化。発生源となる作成手順・エージェント定義への
  文言組み込みを含む)

### 1ファイル1責務

1. **1ファイルには1責務。** ファイルの役割説明が「〜と〜」で繋がるように
   なったら分割候補と考える
2. **どこに何があるか、名前と置き場だけで分かる構成を保つ。**
   (既存のcrates/・apps/・packages/の層別構成が実例)
3. **行数等の機械的しきい値は設けない。** 判断基準は責務の数であり、
   サイズではない
4. **既存の違反を見つけてもその場で直さず、別チケットとして起票する**
   ([docs/agent-operations.md](../agent-operations.md)「宣言された依存
   範囲の外で見つかった不具合の扱い」と同じ運用)

CLAUDE.md・domain-model.mdは「毎セッション読む地図+最重要ルール」
「コアの現在仕様の中核」という単一責務を担っており、内容の幅は責務の
広さの反映であって違反ではない。ただし将来、責務の説明が「と」で
繋がるようになった場合や、特定節だけへの外部参照が増えてきた場合は
分割を検討する(domain-model.mdは既にscenario-lint.mdを分離した前例あり)。

## 帰結

- **design/reviews/ は「記録」区分だが design/ 直下に残す(意図的な例外)。**
  crates/ のコードコメントは docs/design/ のみ参照できる(CLAUDE.md
  Rust規約)ため、コードから参照されるレビュー記録(golden_tests.rs →
  p1-c1-type-review.md)を design/ の外へ出すとその規約と衝突する。
  置き場所の一貫性より参照規約の単純さを優先した
- **tasks/plans/ の横断計画は、plan mode の自動生成ファイル名のままにせず
  内容が分かる名前へリネームする。** 自動生成名(例: merry-leaping-tide.md)
  は「名前で分かる構成」原則に反する。歴史記録内の旧名言及は既存方針どおり
  未修正でよい(tasks/README.md「plansの振り分けルール」に反映)
- **コード→docs参照の規律(2026-08-31、人間承認)。** 実装(従)から
  規範(正)への参照は依存の向きとして正しく、許可する。ただし:
  (1) 置くのはクレート/モジュールの入口(lib.rs先頭等)のみ、
  (2) 対象はコードから読み取れない不変条件のみ、
  (3) 内容を再掲しないポインタ1行に留める。
  参照パスの腐敗(改名で黙って壊れる)は design-sync の照合観点
  「コードコメント内の docs/design/ 参照の実在確認」で検出する。
  純粋性のような制約の機械的強制(clippyのdisallowed-methods等)は
  上位の解だが、必要性の判断から別途起票する(先回りしない)
- 新規文書・スキルを作るときは、詳細を複製せず「正はどこか」を1行で
  示す形を既定とする
- 既存の違反是正は本ADRの決定に基づき、
  tasks/crosscutting/ssot-single-responsibility/task.mdのC3として
  チケット化し、コード変更を伴うものは別PRで扱う
- CLAUDE.md・domain-model.mdの物理分割は、本ADRの分割の兆候基準に
  当てはまった時点で別途起票する(frozen相当の保留であり、今回の
  是正対象には含めない)
- **現役文書から実行系タスク(工程文書)への直接リンクを禁じる
  (2026-08-31、人間承認)。** コード側の既存規約「コードコメントから
  docs/tasks/を参照しない」のdocs版。経緯を示したい場合はADR・決定ログ
  経由にする。この禁止は`tasks/plans/`直下のような**追跡機構を持たない
  一時的なチケット**が対象であり、`tasks/crosscutting/<slug>/task.md`
  (frontmatterで追跡される、`projects/phaseN/task.md`と同格の存在)への
  リンクは対象外(roadmap.mdがphaseN/task.mdを指すのと同じ通常運用)。
  発見の経緯: ssot-single-responsibility-rules.mdが`tasks/plans/`直下に
  居座ったことで「終わったか残っているか一目で分からない」状態になった
  (詳細: [tasks/plans/crosscutting-task-structure.md](../tasks/plans/crosscutting-task-structure.md)。
  この文書自体はtasks/構造の再編記録であり、docs-tasks-restructure.mdと
  同じ性質のためtasks/plans/に留める)。対応として`tasks/crosscutting/`を
  新設し、実行系タスクはfrontmatter付きtask.mdとして追跡機構に乗せる
  ことにした(tasks/README.md「横断タスクの振り分けルール」)
