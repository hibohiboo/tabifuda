# 計画: docs-site への統合(進捗ビュー+テスト状況ビュー)

## Context

tasks 再編(projects/ と tools/ の分割)で進捗の一覧性が下がった。また、
サイクル粒度の進捗はどこにも構造化されておらず(roadmap の状態列はフェーズ
粒度のみ)、テスト戦略(test-strategy.md)と実際のテスト(162件)の対応も
見えない。既存の RDRA ビューア(tools/rdra-viewer、GitHub Pages 配信済み)を
**docs 総合ビューア「docs-site」に拡張**し、進捗とテスト状況も同じサイトで
見られるようにする。

ユーザー決定(2026-07-20):

- tools/rdra-viewer を **tools/docs-site に改名**し、ナビ付き1SPAに
  3ビュー(RDRA / 進捗 / テスト)を統合
- サイクル粒度の進捗の**正は各 task.md の YAML frontmatter**
  (サイクル名は見出しから抽出し二重化しない)
- テスト状況は **Pages ビルド時に cargo test を実行**してスイート別
  件数・成否を JSON 化して埋め込む
- 今回のスコープ: **タスク文書改訂+進捗ビュー(D1)実装まで**。
  テストビュー(D2)は次セッション

## 実装内容(今回)

### 1. タスク文書の改訂と改名

- `git mv docs/tasks/tools/rdra-viewer docs/tasks/tools/docs-site`。
  task.md を改訂: 名称・目的を「docs 総合ビューア」に拡張し、サイクルを
  再構成 — RDRA系: C1(済)/C2(データ拡充)/C3(CI検証)+ 新規:
  **D1 進捗ビュー**(今回実行)/ **D2 テストビュー**(下記の内容を記載)
- `git mv tools/rdra-viewer tools/docs-site`。package名 `@tabifuda/docs-site`、
  index.html の title 変更。`docs/rdra/` はそのまま(RDRAデータの置き場として妥当)
- 参照更新: [docs/adr/0003-ci-pipeline.md](docs/adr/0003-ci-pipeline.md)、
  [docs/README.md](docs/README.md)、[docs/rdra/README.md](docs/rdra/README.md)、
  CLAUDE.md「リポジトリ構成」、
  [docs/tasks/plans/docs-tasks-restructure.md](docs/tasks/plans/docs-tasks-restructure.md)
  内の実在パスリンク。歴史記録内の言及は直さない(既存方針)
- 本計画ファイルは承認後 `docs/tasks/tools/docs-site/plans/` へ振り分け
  (tasks/README.md の運用)

### 2. 進捗 frontmatter の導入(全 task.md 8本)

```yaml
---
status: done          # done | in-progress | planned
cycles:               # サイクル見出し(### C1: ...)と対応。名前は書かない
  C1: done
  C2: planned
---
```

- 初期値: phase0/1/2 = done(roadmap の状態列より)、phase3/3.5/4/5 = planned、
  docs-site = in-progress(C1: done、D1 は完了時に done へ)
- phase0 はサイクル見出しが無い形式なら `status` のみでよい(実装時に確認)
- [docs/tasks/README.md](docs/tasks/README.md) に frontmatter 規約を追記
  (**サイクル完了と同PRで更新する**。これがサイクル粒度の進捗の正)
- [docs/roadmap.md](docs/roadmap.md) に1行注記: 状態列はフェーズ粒度の索引で、
  サイクル粒度の正は task.md frontmatter(食い違ったら task.md が正 — 既存原則の適用)

### 3. 進捗ビュー(D1)の実装

- ナビ導入: ハッシュルーティング(`#/rdra` `#/progress` `#/tests`)。
  react-router は入れない(3ビューに過剰)。テストビューはD2までプレースホルダ
- データ取得: `import.meta.glob('../../../docs/tasks/{projects,tools}/*/task.md',
  { query: '?raw', eager: true })` で全 task.md をビルド時に取り込み、
  frontmatter(js-yaml)+ 見出し(`### C1: 名前` の正規表現)をパース。
  既存 [model.ts](tools/rdra-viewer/src/model.ts) のパース層の隣に progress.ts を追加
- 表示: projects(P0〜P5)/ tools の2セクション。タスクごとにカード —
  状態バッジ、サイクルチップ(done / in-progress / planned を色分け)、
  task.md への GitHub リンク。全体サマリ(done数/全サイクル数)をヘッダーに
- frontmatter に無いサイクル見出し・見出しに無い frontmatter キーは
  ビルド時エラーにする(парス層で throw。乖離の早期検知)

### 4. pages.yml

今回は変更不要(D1 は Rust 不要)。D2 で Rust toolchain 追加。

## D2 テストビュー(次セッション。task.md に記載する内容)

- Node スクリプト(tools/docs-site/scripts/gen-test-report.mjs):
  `cargo test --workspace`(成否+スイート別件数を stdout からパース)+
  `cargo test --workspace -- --list`(テスト名列挙)→ `src/generated/test-report.json`
- スイート→戦略分類のマッピング(engine/patch/lint_tests→例ベース、
  invariant_tests→プロパティ、golden/replay/roundtrip→ゴールデン、
  play_cli/lint_cli/scenario_lint→CLIスモーク)。未分類スイートは生成失敗で検知
- テストビュー: test-strategy.md の分類ごとに実テスト名・件数・成否・生成時刻を表示
- pages.yml に dtolnay/rust-toolchain + Swatinem/rust-cache とレポート生成ステップを追加
  (ADR 0003 追記が先)

## 検証

- `pnpm -r typecheck && pnpm -r build` 通過
- `vite preview` で進捗ビューが8タスク(projects 7 + tools 1)の状態と
  サイクルチップを表示し、GitHub リンクが正しい
- frontmatter 追加後の task.md が GitHub 上で崩れない(frontmatter は
  GitHub がメタデータ表示するだけで本文に混ざらない)
- 旧パス(`rdra-viewer`)の残存 grep(歴史記録を除きゼロ)
- `cargo fmt/clippy/test` 通過(Rust 無変更の確認)

## リスク・注意

- frontmatter と roadmap 状態列の食い違い → task.md が正(roadmap に注記を追加)
- Pages の URL は変わらない(base `/tabifuda/` のまま)。favicon・title のみ変化
- コミットは master 直("今回はmaster更新でよい"の裁定は前回分。今回どうするかは
  実装開始時のブランチ状態を確認して判断。作業前に `git branch --show-current`)
