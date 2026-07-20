---
status: in-progress
cycles:
  C1: done
  C2: planned
  C3: planned
  D1: done
  D2: done
---

# ツールタスク: docs-site(docs 総合ビューア)

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
**開始前の儀式(全フェーズ共通)**: CLAUDE.md と docs/design/ の関連文書を読む。
どのフェーズにも属さないツール系タスク(置き場所の経緯:
[../../plans/docs-tasks-restructure.md](../../plans/docs-tasks-restructure.md))。
旧称 rdra-viewer(RDRAビューア単体)を D1 で docs-site に拡張・改名した
(経緯: [plans/docs-site-progress-plan.md](plans/docs-site-progress-plan.md))。

## 目的

docs/ を GitHub Pages で多面的に可視化する静的サイト。
**公開中: https://hibohiboo.github.io/tabifuda/**(D1完了時点からmaster pushで
自動デプロイ)。3ビューを持つ:

1. **RDRA ビュー**: 設計文書を RDRA(https://www.rdra.jp/)のレイヤー構造で一望
2. **進捗ビュー**: 全タスク(projects/tools)のサイクル粒度の進捗を一望
3. **テストビュー**: テスト戦略(test-strategy.md)と実テストの対応・成否を一望

## 位置づけ(規範との関係)

- RDRAモデルデータ(docs/rdra/*.yaml)は**手動キュレーションの非規範な索引**。
  規範は従来どおり design/ の文書(docs/README.md「文書間の優先順位」)。
  規範文書と食い違ったら**YAML側を直す**(正を二重化しない)
- サイクル粒度の進捗の**正は各 task.md の frontmatter**
  (規約は docs/tasks/README.md)。ビューアはそれを表示するだけ
- ビューア(tools/docs-site/)は表示専用。ゲーム本体(crates/, 将来のapps/)
  とはコードを共有しない

## RDRAレイヤーと既存docsの対応

| RDRAレイヤー | 要素 | 出典文書 |
|---|---|---|
| システム価値 | アクター(プレイヤー/GM/シナリオ作者)、要求 | domain-model.md「アクターと権限」、future-requirements.md、roadmap.md |
| システム外部環境 | 業務フロー(1プレイの流れ)、ビジネスユースケース | domain-guide.md「3. 1プレイの流れ」 |
| システム境界 | ユースケース(=Command)、画面(CLI、将来Web) | domain-model.md「コマンドとイベント」「進行の解決規則」 |
| システム | 情報モデル(カード/シナリオ/セッション/冒険記等)、状態モデル(セッション状態機械)、バリエーション(Effect/Condition種別) | domain-model.md「カード」「シナリオ構造」「セッション状態」「セッション状態機械」 |

## データ形式

- RDRA: docs/rdra/ の YAML(要素の形式・更新規律は
  [../../../rdra/README.md](../../../rdra/README.md)が正)
- 進捗: 各 task.md の frontmatter(`status` + `cycles: {C1: done, ...}`)。
  サイクル名は本文見出し(`### C1: ...`)から抽出し二重化しない。
  frontmatter と見出しの不一致は**ビルド時エラー**にして乖離を早期検知する
- テスト: ビルド時に cargo test の実行結果から生成する JSON(D2参照。コミットしない)

## 技術構成

- ビューア: `tools/docs-site/`。Vite + React + TS。ハッシュルーティング
  (`#/rdra` `#/progress` `#/tests`)の1SPA。react-router は入れない
- データはビルド時取り込み(`?raw` import + js-yaml。task.md 群は
  `import.meta.glob`)
- pnpm workspace(ルート package.json + pnpm-workspace.yaml)。
  ADR 0002(pnpm選定)に沿う。P3 C2 の workspace 導入を本タスクで前倒し
- デプロイ: `.github/workflows/pages.yml`(master push で build →
  actions/deploy-pages)。ADR 0003 に追記済み。Vite `base: '/tabifuda/'`

## サイクル

### C1: 基盤+RDRA最小表示+デプロイ(完了)
- pnpm workspace 導入、tools 配下に Vite+React+TS 雛形
- docs/rdra/ に最小データ(actors + usecases。出典リンク付き)
- レイヤー4段のボード表示(一覧+出典リンク。GitHub blob URLへ飛べる)
- pages.yml 追加、Pages 有効化
- docs/README.md・CLAUDE.md「リポジトリ構成」・ADR 0003 追記

### D1: 進捗ビュー+docs-site への拡張(完了)
- rdra-viewer → docs-site 改名(タスクディレクトリ・パッケージ・参照)
- ナビ(3タブ)導入。テストビューは D2 までプレースホルダ
- 全 task.md(8本)に frontmatter 導入、docs/tasks/README.md に規約追記、
  roadmap.md に「サイクル粒度の正は task.md frontmatter」の注記
- 進捗ビュー: projects / tools の2セクション、タスクカード(状態バッジ+
  サイクルチップ+task.md への GitHub リンク)、全体サマリ

### D2: テストビュー(完了)
- 全テスト関数名(12ファイル162件)を日本語(検証内容を表す文)へリネーム。
  `cargo test`の出力自体が日本語の説明になるようにし、別途の日英対訳
  マッピングを持たない(正の二重化を避ける)。Command/Event/型名などの
  固有名詞(PlayCard、GmAdvanceなど)はASCIIのまま残しトレーサビリティを
  優先。先頭が大文字ASCIIになる識別子があるため、対象の`mod`宣言
  (lib.rs/各cliモジュール)に`#[allow(non_snake_case)]`を付与
- `tools/docs-site/scripts/gen-test-report.mjs`: `cargo test --workspace`
  を実行し、stdout(test結果行)とstderr(Runningヘッダー)を出現順で
  対応付けてスイート単位に分類 → `src/generated/test-report.json`
  (.gitignore対象。ローカルは`pnpm gen:test-report`で生成。`typecheck`/
  `build`からも自動実行されるvite pluginとして組み込み済み)
- スイート→test-strategy.md分類の対応はスクリプト内`SUITES`に手動定義
  (各テストファイルの冒頭docコメントが実際に引用する節を根拠にした)。
  **未分類のスイートが現れたら生成を失敗させる**(テスト追加時の分類漏れ検知)
- テストビュー: スイートごとにラベル・説明・出典リンク・成否件数を表示し、
  クリックで個々の日本語テスト名一覧を開閉できる
- pages.ymlにdtolnay/rust-toolchain + Swatinem/rust-cacheを追加
  (ADR 0003に追記済み)

### C2: RDRAデータ拡充+関係トレース
- information / states / requirements / business-flow のYAML整備
  (domain-model.md の「カード」「シナリオ構造」「セッション状態」、
  future-requirements.md、domain-guide.md「1プレイの流れ」から起こす)
- 要素クリックで関係要素をハイライト(アクター→関連UC→関連情報)
- 状態遷移図・業務フロー図(Mermaid)

### C3: CI検証
- RDRA YAMLスキーマ検証(zod等)+ `source` のリンク先ファイル・アンカー
  存在チェックを CI に追加(設計文書の節名変更に追従漏れがあると落ちる)
- task.md frontmatter の検証(D1のビルド時チェックをCIゲート化)
- PR時の typecheck / build チェックを ci.yml に追加(ADR 0003 の表へ追記)

## 完了条件

- github.io で3ビュー(RDRA全レイヤー / 全タスクの進捗 / テスト分類と成否)が
  閲覧できる
- 各要素から出典(設計文書・task.md)へ飛べる
- CIで RDRAデータ・frontmatter のスキーマ/リンク検証が回る

## やらないこと

- markdown からの自動抽出(RDRA。手動キュレーションが正。将来要望が
  あれば別途検討)
- 進捗の git/PR 履歴からの自動推定(frontmatter を人が更新するのが正)
- ゲーム本体のUI(P3 apps/web)との統合・共有コンポーネント化
- 規範文書の内容を YAML へ複製すること(descriptionは1〜2行の要約に留める)
