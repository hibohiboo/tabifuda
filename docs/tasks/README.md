# tasks/ の構造(タスクと計画の置き場)

1タスク=1ディレクトリ。タスクの指示文(`task.md`)と、そのタスク専用の
計画・決定ログ(`plans/`)を同じディレクトリに置く。
**どの plans がどのタスクのものかはディレクトリで判断する。**

```
tasks/
  README.md      このファイル
  projects/      ゲーム本体の開発フェーズ(1フェーズ=1ディレクトリ)
    phase0/
      task.md    フェーズタスクの正(旧 phase0-task.md)
      plans/     このフェーズ専用の計画・決定ログ
    phase1/ ... phase5/(phase3.5 含む)
  tools/         どのフェーズにも属さない開発支援ツール
    docs-site/
      task.md
      plans/
  crosscutting/  どのフェーズ・ツールにも属さない横断の実行系タスク
    ssot-single-responsibility/
      task.md    タスクの正(1タスク=1ディレクトリ+frontmatterはprojects/と同じ)
      plans/     このタスク専用の子チケット・決定ログ
  plans/         決定ログ・記録・plan mode の書き込み先専用(下記)
```

`crosscutting/` と `plans/` の違い: **ライフサイクル(進捗を追跡する必要)が
あるものは crosscutting/**(task.md+frontmatterでdocs-siteの進捗ビューに
乗る)。**決定の経緯・過去の記録は plans/**(進捗の概念を持たない)。
実行して終わるタスクを plans/ に置くと「終わったか残っているか一目で
分からない」状態になる(2026-08-31、ssot-single-responsibility タスクの
運用で発覚)。

## 進捗 frontmatter(サイクル粒度の進捗の正)

各 `task.md` は先頭に YAML frontmatter を持つ:

```yaml
---
status: in-progress   # done | in-progress | planned | frozen
cycles:               # 本文のサイクル見出し(### C1: ...)と1対1。名前は書かない
  C1: done
  C2: planned
---
```

- **サイクル完了と同じPRで frontmatter を更新する**。ここがサイクル粒度の
  進捗の正であり、roadmap.md の状態列(フェーズ粒度の索引)と食い違ったら
  frontmatter が正
- サイクル見出しの無いタスク(phase0)は `status` のみでよい
- `frozen` は凍結(着手予定から意図的に外した状態。planned との違いは
  「次にやる含みが無い」こと)。再開条件は roadmap.md の該当節に書く
  (初出: P4/P5 の凍結。[../roadmap.md](../roadmap.md)「P4・P5 の凍結」)
- frontmatter のキーと本文見出しの不一致は docs-site のビルドが検出する
  (tools/docs-site。進捗は https://hibohiboo.github.io/tabifuda/#/progress で見る)

## 横断タスクの振り分けルール(crosscutting/ vs plans/)

- **やって終わる実行系タスク**(例: ルールの是正チケット)で、
  どのフェーズ・ツールにも属さないもの → `tasks/crosscutting/<slug>/task.md`
  を新設する(`projects/phaseN/`・`tools/<name>/`と同じ1タスク=1ディレクトリ+
  frontmatter。進捗 frontmatter節の規約をそのまま適用)。関連する子チケット・
  決定ログはそのタスクの `plans/` に置く
- **決定の経緯・過去の記録**(決定ログ、plan mode出力、後から参照するための
  記録。例: [plans/hand-card-removal.md](plans/hand-card-removal.md)、
  [plans/proposal-id-issuance-decisions.md](plans/proposal-id-issuance-decisions.md))
  → `tasks/plans/` 直下に置く。**実行系タスクは置かない**
  (2026-08-31、crosscutting/新設の理由)
- plan mode の自動生成ファイルは `.claude/settings.json` の `plansDirectory`
  (= `tasks/plans/`)に作られる。**セッションの終わりに、対象タスクの
  `plans/` へ `git mv` する**(記録・決定ログとして残す場合はそのまま
  `tasks/plans/` に置く。実行系タスクなら `crosscutting/<slug>/` へ)。
  自動生成名のままにせず**内容が分かる名前へリネームする**
  (どこに何があるか名前で分かる構成を保つ。adr/0007)
- 決定ログ(`*-decisions.md`)の書式・運用は
  [../agent-operations.md](../agent-operations.md)「人間の判断が要る論点の進め方」が正
- **現役文書(規範・索引・README等)から実行系タスク(工程文書)へ直接
  リンクしない。** 経緯を示したいときは ADR・決定ログ経由にする
  (コード側の既存規約「コードコメントからdocs/tasks/を参照しない」の
  docs版。詳細は [../adr/0007-ssot-single-responsibility.md](../adr/0007-ssot-single-responsibility.md)
  「帰結」)。task.md は完了後も**追跡対象であり続ける**ため、
  この禁止の対象外(roadmap.mdがprojects/phaseN/task.mdを指すのと同様、
  crosscutting/の完了タスクへのリンクも通常運用)

## 経緯

再編の経緯と旧構造からの対応は
[plans/docs-tasks-restructure.md](plans/docs-tasks-restructure.md)。
過去の計画・決定ログ・レビュー記録内の旧パス言及(`phaseN-task.md` 等)は
歴史記録なので直していない。
