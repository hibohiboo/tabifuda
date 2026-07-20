# 実行計画: docs/tasks の再編(projects/ と tools/ の2ディレクトリ制)

状態: **未実行**(計画のみ。1セッション=1PRで後日実行する)
作成: 2026-07-20。経緯: RDRAビューア計画時にユーザー決定
([serene-skipping-gadget.md](serene-skipping-gadget.md))。

## 目的

docs/tasks/ 直下にフェーズタスクとツール系タスクが混在し始めたため、
1階層深くして種別で分ける:

- `docs/tasks/projects/` — フェーズタスク(ゲーム本体の開発フェーズ)
- `docs/tasks/tools/` — ツール系タスク(RDRAビューア等。**新設済み**)
- `docs/tasks/plans/` — 変更なし(plan mode成果物・実行計画・決定ログ。
  `.claude/settings.json` の `plansDirectory` も不変)

## 移動内容

```
docs/tasks/phase0-task.md    → docs/tasks/projects/phase0-task.md
docs/tasks/phase1-task.md    → docs/tasks/projects/phase1-task.md
docs/tasks/phase2-task.md    → docs/tasks/projects/phase2-task.md
docs/tasks/phase3-task.md    → docs/tasks/projects/phase3-task.md
docs/tasks/phase3.5-task.md  → docs/tasks/projects/phase3.5-task.md
docs/tasks/phase4-task.md    → docs/tasks/projects/phase4-task.md
docs/tasks/phase5-task.md    → docs/tasks/projects/phase5-task.md
```

`git mv` で移動する(履歴追跡のため)。

## 参照更新の一覧(2026-07-20 調査。実行時に再grepして網羅確認)

- [../../roadmap.md](../../roadmap.md) — フェーズ一覧のリンク7本
  (`tasks/phaseN-task.md` → `tasks/projects/phaseN-task.md`)
- [../../README.md](../../README.md) — 「進め方視点」表の tasks/ 行
  (projects/ と tools/ の説明に分ける)
- [../../agent-operations.md](../../agent-operations.md) —
  phase-cycle スキルの説明で `docs/tasks/` に言及
- `.claude/skills/phase-cycle/SKILL.md` — `docs/tasks/phaseN-task.md` の
  パス参照(frontmatter の description 含む)
- `.claude/agents/retrospective.md` — `docs/tasks/phaseN-task.md` 参照
- CLAUDE.md — 現状 docs/tasks の階層は書かれていないが、
  リポジトリ構成の docs/ 説明に変更が及ばないか確認
- コード内コメント3箇所(`crates/tabifuda-cli/src/play.rs` /
  `src/chronicle.rs` / `tests/play_cli.rs` の `docs/tasks/phase2-task.md`)
- docs/retrospectives/・design/reviews/・tasks/plans/ 内の相対リンク
  (実行時に grep で列挙して直す)

## 手順

1. 上記の `git mv` を実行
2. `grep -rn 'tasks/phase' --include='*.md' --include='*.rs' .claude docs crates CLAUDE.md`
   でヒットした参照を新パスへ更新(plans/ 内の過去計画は歴史記録なので、
   リンク切れになる箇所のみ直す)
3. docs/README.md「更新の規律」に従い、表の記述を新構成に合わせる

## 検証

- 手順2の grep で旧パス(`docs/tasks/phase`)の残存がゼロ
- `cargo test --workspace` / `clippy` / `fmt` 通過(コメント変更のみだが儀式として)
- phase-cycle スキルが新パスでタスク文書を見つけられることを説明文レベルで確認

## リスク・注意

- 進行中ブランチ(phase3 等)とのコンフリクト: フェーズ作業が動いていない
  タイミングで実行する
- 過去のコミットメッセージ・決定ログ内の旧パス言及は**直さない**
  (歴史記録。追記のみ原則に整合)
