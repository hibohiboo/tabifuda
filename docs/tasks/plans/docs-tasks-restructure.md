# 実行記録: docs/tasks の再編(1タスク=1ディレクトリ制)

状態: **実行済み**(2026-07-20、RDRAビューアC1と同ブランチで実施)
経緯: RDRAビューア計画時にユーザー決定
([../tools/docs-site/plans/serene-skipping-gadget.md](../tools/docs-site/plans/serene-skipping-gadget.md))。
当初は「projects/ と tools/ にファイルを移すだけ」の計画だったが、
「どの plans がどのタスクのものか分かりにくい」というユーザー指摘を受け、
**タスクごとにディレクトリを切り、専用 plans/ を同居させる**形に改めた。

現行構造の正は [../README.md](../README.md)(docs/tasks/README.md)。

## 実施した移動

```
docs/tasks/phaseN-task.md            → docs/tasks/projects/phaseN/task.md(N=0..5, 3.5)
docs/tasks/tools/rdra-viewer-task.md → docs/tasks/tools/rdra-viewer/task.md
docs/tasks/plans/phase0-task-3-5.md          → docs/tasks/projects/phase0/plans/
docs/tasks/plans/p1-c1-review-decisions.md   → docs/tasks/projects/phase1/plans/
docs/tasks/plans/serene-skipping-gadget.md   → docs/tasks/tools/rdra-viewer/plans/
```

`docs/tasks/plans/` に残したもの(フェーズ横断):
merry-leaping-tide.md / proposal-id-issuance-decisions.md / 本ファイル。
`.claude/settings.json` の `plansDirectory` は `docs/tasks/plans` のまま
(plan mode の書き込み先。振り分け運用は tasks/README.md と ADR 0004 追記を参照)。

## 更新した参照

- docs/roadmap.md(フェーズリンク7本+冒頭の「正」の記述)
- docs/README.md(tasks 関連の表4行)
- docs/agent-operations.md(決定ログの置き場ルール・実例パス)
- docs/design/domain-model.md・domain-guide.md(決定ログへの参照)
- docs/design/reviews/p1-c1-type-review.md(「進捗の正」のパス)
- docs/retrospectives/phase2.md(申し送り先のパス)
- docs/requirements/future-requirements.md(phase3.5 リンク2本)
- docs/adr/0003-ci-pipeline.md・docs/rdra/README.md(rdra-viewer タスクへのリンク)
- docs/adr/0004-claude-config.md(plansDirectory 節へ振り分け運用を追記)
- .claude/skills/phase-cycle/SKILL.md・decision-log/SKILL.md・
  .claude/agents/retrospective.md
- CLAUDE.md(リポジトリ構成に docs/tasks/ の階層を追記)
- コード内コメント6箇所(tabifuda-cli 3・tabifuda-core 3)
- タスク文書内の相互参照(「開始前の儀式は phase2/task.md 冒頭と同じ」等)

**直していないもの**: 過去の計画・決定ログ・レビュー記録・agent-journal 内の
旧パス言及(歴史記録。tasks/README.md「経緯」参照)。

## 検証

- `grep -rn 'phase[0-9.]*-task\.md|rdra-viewer-task\.md'` で、歴史記録を除き
  旧パス残存ゼロを確認
- `cargo test --workspace` / `clippy` / `fmt` / `pnpm -r build` 通過
