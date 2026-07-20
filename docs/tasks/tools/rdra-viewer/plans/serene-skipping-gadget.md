# 計画: RDRA風ドキュメント分析ビューア + docs/tasks 再編

## Context

docs/ には規範的な設計文書(domain-model.md ほか)が揃っているが、
「誰が・何をして・何の情報が・どう状態遷移するか」を横断的に一望する手段がない。
これを RDRA(https://www.rdra.jp/)のレイヤー構造(システム価値/システム外部環境/
システム境界/システム)で可視化する静的サイトを作り、GitHub Pages(github.io)で
閲覧できるようにする。

あわせて docs/tasks/ を1階層深くし、`projects/`(フェーズタスク)と
`tools/`(RDRAビューア等のツール系タスク)に分ける再編を**別タスク**として計画する。

ユーザー決定事項(2026-07-20):
- RDRAモデルデータは**手動キュレーションYAML**(docs/rdra/)。非規範の索引と位置づけ、各要素に出典(design文書へのリンク)を持たせる
- ビューアは **Vite+React+TS**(P3の「React+Vite推奨」と整合)。pnpm workspace をこのタイミングで導入
- tasks再編は `docs/tasks/projects/`(現フェーズタスク)+ `docs/tasks/tools/`(RDRA計画)の2ディレクトリ制
- 今回のスコープ: **タスク文書2本の作成 + RDRAビューア C1(基盤+デプロイ)の実装まで**

## 今回の成果物

1. `docs/tasks/tools/rdra-viewer-task.md` — RDRAビューアのタスク文書(C1〜C3)
2. `docs/tasks/plans/docs-tasks-restructure.md` — docs/tasks 再編の実行計画(後日実行。フェーズ横断作業の前例 merry-leaping-tide.md に倣い plans/ に置く)
3. RDRAビューア **C1 の実装**(下記)
4. `docs/README.md`・CLAUDE.md(リポジトリ構成)への追記

作業ブランチ: 現在 phase3 ブランチにいるが本作業はP3と独立のため、
**master から新ブランチ `rdra-viewer-c1` を切って作業する**。

---

## タスク文書1: RDRAビューア(docs/tasks/tools/rdra-viewer-task.md)

### RDRAレイヤーと既存docsの対応(データ設計の骨子)

| RDRAレイヤー | 要素 | 出典文書 |
|---|---|---|
| システム価値 | アクター(プレイヤー/GM/シナリオ作者)、要求 | domain-model.md「アクターと権限」、future-requirements.md、roadmap.md |
| システム外部環境 | 業務フロー(1プレイの流れ)、ビジネスユースケース | domain-guide.md「3. 1プレイの流れ」 |
| システム境界 | ユースケース(=Command: StartSession/PlayCard/Propose/JudgeProposal/GmAdvance/ApplyPatch 等)、画面(CLI、将来Web) | domain-model.md「コマンドとイベント」「進行の解決規則」 |
| システム | 情報モデル(カード/シナリオ/フェーズ/シーン/セッション/パーティ/冒険記/パッチ)、状態モデル(セッション状態機械)、バリエーション(Effect/Condition種別) | domain-model.md「シナリオ構造」「セッション状態」「セッション状態機械」 |

### データ形式(docs/rdra/)

手動キュレーションYAML。**非規範の索引**であり、規範は design/ のまま
(docs/README.md「文書間の優先順位」に整合)。各要素は `id` / `name` /
`description` / `source`(design文書への相対パス+アンカー)を持ち、
関係は参照側に `actors:` `information:` `states:` 等のid配列で持つ。

```yaml
# docs/rdra/usecases.yaml の例
usecases:
  - id: play-card
    name: カードを出す(PlayCard)
    actors: [player]
    information: [card, scene, session]
    states: [in-progress]
    source: design/domain-model.md#playcard-の解決と拒否系
```

ファイル分割: `actors.yaml` `requirements.yaml` `business-flow.yaml`
`usecases.yaml` `information.yaml` `states.yaml` + `README.md`(位置づけ・更新規律)。

### ビューア(tools/rdra-viewer/)

- Vite + React + TS。YAMLはビルド時に取り込む(`js-yaml` + Vite の `?raw` import)
- 画面: レイヤー4段のボード表示。要素クリックで関係要素をハイライト(関係トレース)。各要素カードに出典リンク(GitHub blob URL → docs へ)
- 状態モデル・業務フローは Mermaid で図示(C2以降)
- `base: '/tabifuda/'`(https://hibohiboo.github.io/tabifuda/ で公開)

### モノレポ基盤(C1で導入)

- ルートに `package.json` + `pnpm-workspace.yaml`(`tools/*`。将来 `apps/*` `packages/*` を追加)— ADR 0002(pnpm選定)に沿う
- Node 22 / pnpm(corepack)。`.gitignore` に node_modules 等を追記
- `.github/workflows/pages.yml`: master push(+ workflow_dispatch)で build → `actions/upload-pages-artifact` → `actions/deploy-pages`
- ADR 0003(CIパイプライン)に Pages デプロイ追加の追記(同PR)

### サイクル分割

- **C1: 基盤+最小表示+デプロイ**(今回実行)
  - pnpm workspace 導入、tools/rdra-viewer 雛形
  - docs/rdra/ に最小データ(actors + usecases、出典リンク付き)
  - レイヤーボードの最小UI(一覧表示+出典リンク。ハイライトは任意)
  - pages.yml 追加、Pages 有効化(`gh api` で Source=GitHub Actions を試み、権限不足なら人間に設定依頼)
  - docs/README.md・CLAUDE.md「リポジトリ構成」・ADR 0003 追記
- **C2: データ拡充+関係トレース**
  - information / states / requirements / business-flow のYAML整備
  - クリック連動ハイライト、状態遷移図・業務フロー図(Mermaid)
- **C3: CI検証**
  - YAMLスキーマ検証(zod等)+ `source` のリンク先ファイル・アンカー存在チェックをCIに追加
  - PR時の typecheck / build チェックを ci.yml に追加
- 完了条件: github.io で全レイヤー閲覧可 / 各要素から出典docへ飛べる / CIでデータ検証が回る

---

## タスク文書2: docs/tasks 再編(docs/tasks/plans/docs-tasks-restructure.md)

後日実行する実行計画として作成(今回は文書のみ。ただし `docs/tasks/tools/` は
タスク文書1の配置により今回新設される)。

### 移動内容

```
docs/tasks/phase{0,1,2,3,3.5,4,5}-task.md → docs/tasks/projects/ へ git mv
docs/tasks/tools/   (今回新設済み。rdra-viewer-task.md 等ツール系タスク)
docs/tasks/plans/   (変更なし。.claude/settings.json の plansDirectory も不変)
```

### 参照更新の一覧(調査済み。実行時に再grepして網羅確認)

- [docs/roadmap.md](docs/roadmap.md) — フェーズ一覧のリンク7本
- [docs/README.md](docs/README.md) — tasks/ の説明行
- [docs/agent-operations.md](docs/agent-operations.md) — phase-cycle 説明等
- [.claude/skills/phase-cycle/SKILL.md](.claude/skills/phase-cycle/SKILL.md) — `docs/tasks/phaseN-task.md` 参照(description含む)
- [.claude/agents/retrospective.md](.claude/agents/retrospective.md)
- CLAUDE.md「リポジトリ構成」の docs/tasks 説明(あれば)
- コード内コメント3箇所(play.rs / chronicle.rs / play_cli.rs の `docs/tasks/phase2-task.md` 参照)
- docs/retrospectives・reviews・plans 内の相対リンク(実行時 grep で列挙)

### 検証

`grep -rn 'tasks/phase' --include='*.md' --include='*.rs' .claude docs crates` で
旧パス残存ゼロを確認。`cargo test --workspace` 通過(コメントのみだが儀式として)。
docs/README.md「更新の規律」(文書追加時は表に1行追加)に従う。

---

## C1 実装手順(今回実行分)

1. `git checkout master && git checkout -b rdra-viewer-c1`
2. タスク文書2本 + docs/README.md 追記(tasks/tools/ と rdra/ の行追加)+ CLAUDE.md「リポジトリ構成」に `tools/` 追記
3. ルート `package.json` / `pnpm-workspace.yaml` / `.gitignore` 追記
4. `tools/rdra-viewer/`: vite雛形(react-ts)、`js-yaml` で docs/rdra/*.yaml を読む層、レイヤーボードUI(最小)
5. `docs/rdra/`: README.md(非規範の索引である旨・更新規律)+ actors.yaml + usecases.yaml(domain-model.md「アクターと権限」「コマンドとイベント」から起こす)
6. `.github/workflows/pages.yml` 作成、ADR 0003 追記
7. Pages 有効化を `gh api repos/{owner}/{repo}/pages` で試行(失敗時は人間へ設定手順を提示)
8. 検証(下記)後、コミット。push・PR作成は人間に確認してから

## 検証方法

- `pnpm install && pnpm -r build && pnpm -r typecheck` がローカルで通る
- `pnpm -r preview`(または `vite preview`)でレイヤーボードが表示され、出典リンクが正しいGitHub URLを指す
- `cargo test --workspace` / `clippy` / `fmt` が引き続き通る(Rust側は無変更のはずの確認)
- push後: pages.yml の Actions 成功と https://hibohiboo.github.io/tabifuda/ の表示確認(Pages有効化が済んでいれば)

## 前提・リスク

- GitHub Pages の有効化(Source: GitHub Actions)はリポジトリ設定が必要。`gh` で自動化を試みるが、権限次第で人間の1操作が要る
- リポジトリがprivateの場合、無料プランではPages不可(その場合は公開可否を人間に確認)
- pnpm workspace 導入は P3 C2 の前倒しに当たるため、phase3-task.md C2 の記述と重複しないよう、実行時に phase3-task.md へ「workspace導入済み」の注記を入れる(正の二重化を避ける)
