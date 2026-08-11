---
status: in-progress
cycles:
  C1: done
  C2: done
  C3: done
  D1: done
  D2: done
  D3: done
  D4: done
  D5: done
  D6: done
  D7: planned
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
- ビューア(tools/docs-site/)は表示専用。ゲーム本体(crates/, apps/)
  とはコードを共有しない。**例外**(2026-08-01追加、component-catalogタスク):
  `packages/ui`(`@tabifuda/ui`)のみ依存してよい。同パッケージはwasm
  ランタイム・ビルド成果物を含まない確定型の表示層のみで構成され、
  「ゲーム本体のロジック・ビルド手順に引きずられない」という本原則の
  趣旨(docs-siteのビルドを単純・独立に保つ)を壊さないため
  (詳細: [../component-catalog/task.md](../component-catalog/task.md))

## RDRAレイヤーと既存docsの対応

| RDRAレイヤー | 要素 | 出典文書 |
|---|---|---|
| システム価値 | アクター(プレイヤー/GM/シナリオ作者)、要求 | domain-model.md「アクターと権限」、future-requirements.md、roadmap.md |
| システム外部環境 | 業務フロー(1プレイの流れ)、ビジネスユースケース | domain-guide.md「3. 1プレイの流れ」 |
| システム境界 | ユースケース(=Command)、画面(依頼選択/シナリオ中/シナリオ終了後/作者ページ/GMページ。`status`で未作成を区別) | domain-model.md「コマンドとイベント」「進行の解決規則」、docs/rdra/screens.yaml(D4) |
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
- [x] pnpm workspace 導入、tools 配下に Vite+React+TS 雛形
- [x] docs/rdra/ に最小データ(actors + usecases。出典リンク付き)
- [x] レイヤー4段のボード表示(一覧+出典リンク。GitHub blob URLへ飛べる)
- [x] pages.yml 追加、Pages 有効化
- [x] docs/README.md・CLAUDE.md「リポジトリ構成」・ADR 0003 追記

### D1: 進捗ビュー+docs-site への拡張(完了)
- [x] rdra-viewer → docs-site 改名(タスクディレクトリ・パッケージ・参照)
- [x] ナビ(3タブ)導入。テストビューは D2 までプレースホルダ
- [x] 全 task.md(8本)に frontmatter 導入、docs/tasks/README.md に規約追記、
  roadmap.md に「サイクル粒度の正は task.md frontmatter」の注記
- [x] 進捗ビュー: projects / tools の2セクション、タスクカード(状態バッジ+
  サイクルチップ+task.md への GitHub リンク)、全体サマリ

### D2: テストビュー(完了)
- [x] 全テスト関数名(12ファイル162件)を日本語(検証内容を表す文)へリネーム。
  `cargo test`の出力自体が日本語の説明になるようにし、別途の日英対訳
  マッピングを持たない(正の二重化を避ける)。Command/Event/型名などの
  固有名詞(PlayCard、GmAdvanceなど)はASCIIのまま残しトレーサビリティを
  優先。先頭が大文字ASCIIになる識別子があるため、対象の`mod`宣言
  (lib.rs/各cliモジュール)に`#[allow(non_snake_case)]`を付与
- [x] `tools/docs-site/scripts/gen-test-report.mjs`: `cargo test --workspace`
  を実行し、stdout(test結果行)とstderr(Runningヘッダー)を出現順で
  対応付けてスイート単位に分類 → `src/generated/test-report.json`
  (.gitignore対象。ローカルは`pnpm gen:test-report`で生成。`typecheck`/
  `build`からも自動実行されるvite pluginとして組み込み済み)
- [x] スイート→test-strategy.md分類の対応はスクリプト内`SUITES`に手動定義
  (各テストファイルの冒頭docコメントが実際に引用する節を根拠にした)。
  **未分類のスイートが現れたら生成を失敗させる**(テスト追加時の分類漏れ検知)
- [x] テストビュー: スイートごとにラベル・説明・出典リンク・成否件数を表示し、
  クリックで個々の日本語テスト名一覧を開閉できる
- [x] pages.ymlにdtolnay/rust-toolchain + Swatinem/rust-cacheを追加
  (ADR 0003に追記済み)
- [x] **CI失敗の修正(2026-07-31)**: CI(dtolnay/rust-toolchainが設定する
  `CARGO_TERM_COLOR=always`)でcargoのRunning見出しにANSI色が付き、
  `gen-test-report.mjs`の見出し正規表現が不マッチになる問題を、spawnSyncの
  envで`CARGO_TERM_COLOR=never`を明示して解決。調査記録は
  [plans/test-report-running-header-race.md](plans/test-report-running-header-race.md)

### C2: RDRAデータ拡充+関係トレース(完了)
- [x] information.yaml(14要素)/ states.yaml(3状態+6遷移)/ requirements.yaml
  (9要求。`status: realized|future`で将来要望メモとの誤認を防ぐ)/
  business-flow.yaml(1プレイの流れ、5ステップ)を追加。domain-model.md の
  「カード」「シナリオ構造」「セッション状態」「シナリオパッチ(構造化)」、
  future-requirements.md、domain-guide.md「1プレイの流れ」から起こした
- [x] usecases.yamlのplay-card/apply-patchの情報・状態参照を実態に合わせて拡充
  (Effect::EndSessionによる終了、patch-opの参照)
- [x] 関係ハイライト: `model.ts`の`relatedIds`をusecase専用から汎用化し、
  usecase・requirement・業務フローステップを「関係を運ぶノード」として
  扱う1ホップグラフに変更。どのレイヤーの要素をクリックしても関連要素が
  ハイライトされる(アクター→関連UC→関連情報、を含む一般化)
- [x] Mermaidで状態遷移図(セッション状態機械)と業務フロー図を描画。
  mermaid本体は動的importでRDRAビュー表示時のみ読み込み、初期バンドルを
  軽く保つ(データはdocs/rdra由来の信頼済み文字列のみ扱う)

### C3: CI検証(完了)
- [x] **一部先行実施(2026-07-22)**: docs/ 内のmarkdown間相対リンクの存在検証
  (`scripts/check-doc-links.mjs`。アンカーまでは検証しない)を
  vite プラグインとしてビルド時ゲート化済み。ADR 0003 に追記
- [x] RDRA YAMLスキーマ検証(zod)+ `source` のリンク先ファイル・アンカー
  存在チェック+参照idの存在チェック+id一意性チェックを追加
  (`scripts/check-rdra-data.mjs`。vite プラグイン `rdraDataCheckPlugin` として
  ビルド時ゲート化。アンカーはGitHubの見出しスラグ生成を簡易再現して照合)
- [x] task.md frontmatter の検証(D1で既にビルド時チェックはあった。今回は
  下記のCIゲート化で PR でも実際に走るようにした)
- [x] PR時の typecheck / build チェックを ci.yml に追加(`docs-site` ジョブ。
  ADR 0003 の表へ追記済み)。これにより frontmatter 検証・docs内リンク切れ
  検証・RDRAデータ検証がまとめて PR ゲート化された

### D3: frozen対応+完了フィルタのタブ

経緯: 2026-08-02の進め方見直し
([../../plans/post-p3.5-replanning-decisions.md](../../plans/post-p3.5-replanning-decisions.md)
Q5)。D3〜D5はこの決定ログの切り出しタスク2にあたる。

- [ ] `docs-site-frozen-status` ブランチの取り込み: 進捗statusに `frozen`
  (progress.ts の型・STATUSES、ProgressView のバッジ「凍結」、styles.css の
  配色ライト/ダーク)。**マージ順: 見直し本体のブランチ
  (`frozen` frontmatter を含む)より先、または同時に master へ入れる**
  (先に文書側だけ入ると Pages ビルドが落ちる)
- [ ] 進捗ビューにフィルタタブ: 既定は未完了(in-progress / planned / frozen)
  のみを表示し、「完了」「全部」タブで切り替える
- [ ] RDRAビューの要求(requirements)にも同様のフィルタ: 既定は `future`
  のみ、「実現済み」「全部」で切り替え(完了が上に積もる問題への対処)

### D4: 画面ビュー段階1(画面一覧+関係)

経緯: 同見直し Q6。**D4完了時に人間が画面一覧をレビューしてから D5 に進む**
(段階ゲート)。

- [ ] `docs/rdra/screens.yaml` 新設: 依頼選択 / シナリオ中(プレイ画面) /
  冒険記タイムライン(プレイ中の別画面) / シナリオ終了後 / 作者ページ /
  GMページ の6画面から開始(冒険記タイムラインはD4レビューで分離。
  2026-08-02)。各画面は id / name / description /
  `status: implemented | future`(**未作成が一目で分かるように**)/
  関連 `usecases:` `actors:` を持つ
- [ ] rdra/README.md のファイル構成表・形式説明に screens.yaml を追記
- [ ] `check-rdra-data.mjs` のスキーマ検証・参照id検証に screens を追加
- [ ] RDRAビューのシステム境界レイヤーに「画面」を表示(関係ハイライトの
  1ホップグラフに参加。future画面はバッジ等で視覚的に区別)
- [ ] 人間レビュー: 画面の過不足・画面↔Command対応の穴(どのCommandを
  どの画面から打つか)を確認し、結果を本タスクの plans/ に記録

### D5: 画面ビュー段階2(ワイヤーフレーム)

- [ ] **人間の事前決定**: ワイヤーフレームのデータ形式(screens.yaml に
  領域・要素の構造を持たせるか、別ファイルか。描画は枠+ラベルの簡易
  ボックスレイアウトを想定)。D4のレビュー結果を踏まえて決める
- [ ] 各画面のワイヤーフレーム表示(スマホ幅での見え方も確認できる形が
  望ましい。Q2の「スマホで選択するには小さい」が発端のため)
- [ ] カード実物のビジュアル(白銀比・アイコン)は本タスクでは作り込まない
  (packages/ui のカードコンポーネント試作=カードUI強化タスクの担当。
  二重投資を避ける)

### D6: アクター要求・業務フローの拡充(GM/作者)を先にやってから画面を見直す

経緯: D4〜D5で画面を直接ブレストして書いたところ、レビューで3周の
手戻りが発生した(もう一度遊ぶボタンの憶測混入、依頼選択のactors誤り、
GMのシナリオ運用フロー丸ごと欠落。d3-d5-checklist.md参照)。
2026-08-03、ユーザー指摘: 「画面一覧構築前に、どんな画面が必要か洗い出す
フレームワークが要る」。方針として合意したのは、**新しい手法を持ち込む
のではなく、このプロジェクトが既に持つRDRAのレイヤー順序
(アクター要求 requirements.yaml → 業務フロー business-flow.yaml →
ユースケース usecases.yaml → 画面 screens.yaml)を、手薄なアクター
(gm・author)にも同じ厳密さで適用する**こと。現状 requirements.yaml は
player中心の9件のみで、business-flow.yaml も player の「1プレイの流れ」
1本のみ。gm・author はいきなり screens.yaml を書いてしまい、この2層を
飛ばしていたのが手戻りの根本原因。

ユーザーが例示した未洗い出しの要求(いずれもrequirements.yamlへの追加候補):
- player: 別パーティを作って遊びたい
- gm: シナリオをカスタマイズしておき、複数セッションに使い回したい
  (D4/D5で発見済み。future-requirements.md §1に暫定記録済み)
- author: 他の作者のシナリオを見たい
- author: 他シナリオのベースをコピーして自分のものを作り始めたい
- author: キャンペーンシナリオを作りたい(future-requirements.md §2と関連)
- author: 同じロケーションを使ってシェアワールドを表現したい(新規論点)

進め方(案。着手時に確定させる):
1. アクターごとに要求を洗い出し requirements.yaml に追加(status: future
   が大半になる見込み。特にauthorは3件中0件しか無い現状)
2. 要求ごとに業務フロー(business-flow.yaml、フローが無い場合は新規)を
   作る。既存の「1プレイの流れ」と同じ形式(ステップ列、branch対応)
3. フローのステップから screens.yaml を見直す・追加する。D4〜D5で作った
   8画面(特にgm-scenario-stock/session-recruitと、author-page)を
   このフローと突き合わせて過不足を確認
4. 画面の再ブレストは行わない(要求→フローから機械的に導出する)

**実施結果(2026-08-11)**: 各層で「名前だけの列挙→確認」を挟んでから作り込む
進め方(post-p3.5-session-retrospective.md「気づきと対応」)を実際に適用した。

- requirements.yaml: 9件追加(player 1 / gm 3 / author 5)。うちauthorの
  「シナリオを作りたい/編集したい」は、これまで要求としてすら存在しな
  かった最も基本的なギャップだった。会話中に出た「使えることが段階的に
  増えていく」という成長体験の話は、最初は個別要求にせずfuture-requirements.md
  §8(設計原則のメモ)に留めたが、「ルールブックを読むと機能が広がる」という
  具体像が出た時点で要求2件(unlock-authoring-by-rules、
  unlock-character-options-by-rules)として書き戻した
- business-flow.yaml: 4本追加(パーティ編成/GMの遊びたいシナリオリストから
  セッション開始まで/作者のシナリオ執筆/作者間のシナリオ共有・派生)+
  会話中に追加合意した1本(ルールを読んで使える機能が広がる流れ)。当初
  「作者のシナリオ執筆」1本にまとめる案だったが、ユーザー指摘で「執筆」と
  「他作者との関わり」を2本に分割
- screens.yaml: 4画面追加(パーティ編成/シナリオ共有ページ/ワールド設定
  ページ/ルールブックページ)。既存8画面(gm-scenario-stock/session-recruit/
  scenario-select/gm-page/author-page)は新フローで過不足なくカバー済みと
  確認できた(D4/D5時点の発見が正しかったことの裏取りになった)
- 「ロケーション」の扱いはユーザーとの会話で2回方向転換した
  (作者間共有の一部→ルールブック/ワールド設定の文脈→最終的に
  ワールド設定ページとルールブックページに分離)。荒い段階で確認を
  挟んだことで、書き込む前に軌道修正できた
- 目視確認(Playwright)で、追加した要求・フロー・画面がすべてRDRAビューに
  表示されコンソールエラーが無いことを確認。**その過程で、RdraView.tsxが
  `model.flows[0]`しか表示しない(業務フローが1本の間は問題にならなかった
  実装上の制約)ことを発見した。D6のスコープ外のため、D7として別途起票する**

### D7: RDRAビューで業務フローを複数表示する

経緯: D6(2026-08-11)でbusiness-flow.yamlが1本(simple-hunt-playthrough)から
5本に増えたが、`RdraView.tsx`の「システム外部環境」セクションは
`const flow = model.flows[0];`で先頭の1本しか表示しない実装だった
(業務フローが1本だけの間は問題が顕在化しなかった)。データ(business-flow.yaml)
・関係ハイライト(screens等からの1ホップ参照)には新規4本+1本が正しく
投入済みで、表示ロジックのみの問題。

- [ ] `RdraView.tsx`のシステム外部環境セクションを`model.flows`全件を
  ループして表示するよう修正(見出し・Mermaid図・ステップカードをフローごとに
  繰り返す構成に変更。既存の1本表示との見た目の連続性は保つ)
- [ ] Mermaid描画(`flowDiagram`)がフローごとに独立したdiagramになることを確認
  (idの衝突が起きないか。ステップidはファイル横断で一意なため問題ない見込み)
- [ ] 目視確認(Playwright): 5本すべてのフロー名・図・ステップカードが表示され
  コンソールエラーが無いことを確認

## 完了条件

- github.io で3ビュー(RDRA全レイヤー / 全タスクの進捗 / テスト分類と成否)が
  閲覧できる
- 各要素から出典(設計文書・task.md)へ飛べる
- CIで RDRAデータ・frontmatter のスキーマ/リンク検証が回る

D3〜D5追加分(2026-08-02):

- 一覧の既定表示が「これから」中心になり、完了はタブで見る
- 画面一覧(未作成の画面を含む)がRDRAビューで確認でき、未作成が
  一目で分かる
- ワイヤーフレームが画面ごとに閲覧できる

## やらないこと

- markdown からの自動抽出(RDRA。手動キュレーションが正。将来要望が
  あれば別途検討)
- 進捗の git/PR 履歴からの自動推定(frontmatter を人が更新するのが正)
- ゲーム本体のUI(P3 apps/web)との統合・共有コンポーネント化
- 規範文書の内容を YAML へ複製すること(descriptionは1〜2行の要約に留める)
