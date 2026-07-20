# ツールタスク: RDRA風ドキュメント分析ビューア

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
**開始前の儀式(全フェーズ共通)**: CLAUDE.md と docs/design/ の関連文書を読む。
どのフェーズにも属さないツール系タスク(置き場所の経緯:
[plans/docs-tasks-restructure.md](../../plans/docs-tasks-restructure.md))。

## 目的

docs/ の設計文書を RDRA(https://www.rdra.jp/)のレイヤー構造で可視化する
静的サイトを作り、GitHub Pages(https://hibohiboo.github.io/tabifuda/)で
閲覧できるようにする。「誰が・何をして・何の情報が・どう状態遷移するか」を
横断的に一望し、文書間の関係を分析する入口にする。

## 位置づけ(規範との関係)

- RDRAモデルデータ(docs/rdra/*.yaml)は**手動キュレーションの非規範な索引**。
  規範は従来どおり design/ の文書(docs/README.md「文書間の優先順位」)
- 各要素は `source`(design文書への相対パス+アンカー)で出典を指す。
  規範文書と食い違ったら**YAML側を直す**(正を二重化しない)
- ビューア(tools/rdra-viewer/)は表示専用。ゲーム本体(crates/, 将来のapps/)
  とはコードを共有しない

## RDRAレイヤーと既存docsの対応

| RDRAレイヤー | 要素 | 出典文書 |
|---|---|---|
| システム価値 | アクター(プレイヤー/GM/シナリオ作者)、要求 | domain-model.md「アクターと権限」、future-requirements.md、roadmap.md |
| システム外部環境 | 業務フロー(1プレイの流れ)、ビジネスユースケース | domain-guide.md「3. 1プレイの流れ」 |
| システム境界 | ユースケース(=Command)、画面(CLI、将来Web) | domain-model.md「コマンドとイベント」「進行の解決規則」 |
| システム | 情報モデル(カード/シナリオ/セッション/冒険記等)、状態モデル(セッション状態機械)、バリエーション(Effect/Condition種別) | domain-model.md「カード」「シナリオ構造」「セッション状態」「セッション状態機械」 |

## データ形式(docs/rdra/)

ファイル分割: `actors.yaml` `requirements.yaml` `business-flow.yaml`
`usecases.yaml` `information.yaml` `states.yaml` + `README.md`(位置づけ・更新規律)。

各要素は `id` / `name` / `description` / `source` を持ち、関係は参照側に
id配列で持つ(例: usecase が `actors:` `information:` `states:` を持つ)。

```yaml
# docs/rdra/usecases.yaml の例
usecases:
  - id: play-card
    name: カードを出す(PlayCard)
    actors: [player]
    information: [card, scene, session]
    states: [running]
    source: design/domain-model.md#playcard-の解決と拒否系
```

## 技術構成

- ビューア: `tools/rdra-viewer/`。Vite + React + TS。
  YAMLはビルド時に取り込む(`js-yaml` + Vite の `?raw` import)
- pnpm workspace(ルート package.json + pnpm-workspace.yaml)。
  ADR 0002(pnpm選定)に沿う。P3 C2 の workspace 導入を本タスクで前倒し
- デプロイ: `.github/workflows/pages.yml`(master push で build →
  actions/deploy-pages)。ADR 0003 に追記済み
- Vite `base: '/tabifuda/'`

## サイクル

### C1: 基盤+最小表示+デプロイ
- pnpm workspace 導入、tools/rdra-viewer 雛形
- docs/rdra/ に最小データ(actors + usecases。出典リンク付き)
- レイヤー4段のボード表示(一覧+出典リンク。GitHub blob URLへ飛べる)
- pages.yml 追加、Pages 有効化(gh api で試み、権限不足なら人間に設定依頼)
- docs/README.md・CLAUDE.md「リポジトリ構成」・ADR 0003 追記

### C2: データ拡充+関係トレース
- information / states / requirements / business-flow のYAML整備
  (domain-model.md の「カード」「シナリオ構造」「セッション状態」、
  future-requirements.md、domain-guide.md「1プレイの流れ」から起こす)
- 要素クリックで関係要素をハイライト(アクター→関連UC→関連情報)
- 状態遷移図・業務フロー図(Mermaid)

### C3: CI検証
- YAMLスキーマ検証(zod等)+ `source` のリンク先ファイル・アンカー存在
  チェックを CI に追加(設計文書の節名変更に追従漏れがあると落ちる)
- PR時の typecheck / build チェックを ci.yml に追加

## 完了条件

- github.io で全レイヤー(価値/外部環境/境界/システム)が閲覧できる
- 各要素から出典の設計文書(GitHub上)へ飛べる
- CIで RDRAデータのスキーマ・リンク切れ検証が回る

## やらないこと

- markdown からの自動抽出(手動キュレーションが正。将来要望が
  あれば別途検討)
- ゲーム本体のUI(P3 apps/web)との統合・共有コンポーネント化
- 規範文書の内容を YAML へ複製すること(descriptionは1〜2行の要約に留める)
