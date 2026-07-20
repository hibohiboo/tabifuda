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
    rdra-viewer/
      task.md
      plans/
  plans/         フェーズ横断の計画・決定ログ + plan mode の書き込み先(下記)
```

## plans の振り分けルール

- **特定のフェーズ/ツールに対する**計画・決定ログ → その `projects/phaseN/plans/`
  または `tools/<name>/plans/` に置く
- **フェーズ横断**(どのタスクにも属さない改善・再検討。例:
  [plans/merry-leaping-tide.md](plans/merry-leaping-tide.md)、
  [plans/proposal-id-issuance-decisions.md](plans/proposal-id-issuance-decisions.md))
  → `tasks/plans/` 直下に置く
- plan mode の自動生成ファイルは `.claude/settings.json` の `plansDirectory`
  (= `tasks/plans/`)に作られる。**セッションの終わりに、対象タスクの
  `plans/` へ `git mv` する**(横断ならそのまま)
- 決定ログ(`*-decisions.md`)の書式・運用は
  [../agent-operations.md](../agent-operations.md)「人間の判断が要る論点の進め方」が正

## 経緯

再編の経緯と旧構造からの対応は
[plans/docs-tasks-restructure.md](plans/docs-tasks-restructure.md)。
過去の計画・決定ログ・レビュー記録内の旧パス言及(`phaseN-task.md` 等)は
歴史記録なので直していない。
