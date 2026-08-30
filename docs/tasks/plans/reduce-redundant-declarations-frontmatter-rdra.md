# タスク: 「進捗の正はfrontmatter」「RDRAは非規範」宣言の重複解消

## 概要

[docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
で確定した SSoT 方針に基づき、「進捗の正はfrontmatter」「RDRA は非規範の索引」
という事実が複数の文書で「正は〜」の宣言なしで重複記載されている箇所を統一する。

## 背景

正の宣言が既に存在(tasks/README.md「frontmatter が正」、rdra/README.md「正はdesign/
のまま」)しており、SSoT違反ではない。ただし、後発の計画・タスク文書でも同じ説明が
繰り返されている。冗長度が高いため、正への参照に統一する。

## 作業項目

### 1. 「進捗の正はfrontmatter」の統一

複数箇所で「サイクル粒度の正は task.md frontmatter」と述べられている：

- [docs/tasks/README.md](../../README.md) L37「frontmatter が正」← **これが正**
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L40
- [docs/tasks/tools/docs-site/plans/docs-site-progress-plan.md](../../tools/docs-site/plans/docs-site-progress-plan.md) L16

後発2者を「正は tasks/README.md『frontmatter が正』」への参照に統一。

### 2. 「RDRAは非規範の索引」の統一

複数箇所で「非規範の索引」と述べられている：

- [docs/rdra/README.md](../../rdra/README.md) L10「規範は design/ のまま」← **これが正**
- [docs/README.md](../../README.md) L21「RDRAモデルデータ(非規範の索引)」
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L37-38
- [docs/tasks/tools/docs-site/plans/serene-skipping-gadget.md](../../tools/docs-site/plans/serene-skipping-gadget.md)

後発文書を「正は rdra/README.md『規範は design/』」への参照に統一。

## スコープ

- 変更は docs/ のみ
- 説明を削除ではなく短くして参照を追加
- 対象は上記4+α の文書

## 優先度

**低**。既に正が宣言されており、参照も適切に書かれている。
冗長度を下げるための美化項目であり、今回の SSoT 違反是正の主軸ではない。
時間があれば対応。

## 終わり方

1. 上記4箇所の記述を参照形式に短縮
2. 誤解があれば agent-journal.md に1行記録
3. 作業ブランチで行い、マージは人間判断

## 関連

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  (SSoT方針・冗長度の削減)
- [docs/tasks/README.md](../../README.md)「frontmatter が正」
- [docs/rdra/README.md](../../README.md)「非規範の索引」

---

## 完了記録(2026-08-31。対応不要と判断)

対象4箇所を個別に確認した結果:

- **docs/tasks/tools/docs-site/task.md**: 別チケット
  ([normalize-document-classification-definitions.md](normalize-document-classification-definitions.md))
  で既に docs/README.md・docs/tasks/README.md への参照追記が完了済み
- **docs/tasks/tools/docs-site/plans/docs-site-progress-plan.md**・
  **docs/tasks/tools/docs-site/plans/serene-skipping-gadget.md**: 中身は
  「ユーザー決定事項(2026-07-20)」という特定時点の決定記録であり、
  tasks/README.md「経緯」節が定める既存方針
  (「過去の計画・決定ログ・レビュー記録内の旧パス言及…は歴史記録なので
  直していない」)に該当する性質の文書。今から参照を後付けするのは
  この既存方針と矛盾するため、**編集しない**

結論: 現役の指示文書(task.md)は前チケットで対応済み、歴史記録
(plans/の決定ログ)は既存方針により対象外。本チケットに追加作業なし。
