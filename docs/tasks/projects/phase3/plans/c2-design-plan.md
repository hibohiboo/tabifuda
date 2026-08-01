# Phase 3 C2「apps/web骨格」実装計画

## Context

Phase 3はWASM+Web版(ローカル)。C0(フロント層設計文書の置き場)とC1(tabifuda-wasm、
wasm境界API)は完了済み(docs/tasks/projects/phase3/plans/c0-c1-progress.md)。
C2はその上にapps/web(React+Vite、ADR 0006で決定済み)の骨格を作り、ブラウザで
「シナリオ読込→セッション開始→カードを出して進行」までを動かす。tabifuda-wasm自体は
まだ一度も`wasm-pack build`されておらず、Web側から呼び出す配線が今回初めて入る。

C2はスコープが広い(pnpm workspace導入・CI拡張・ADR更新・新規アプリ雛形・ゲーム進行
ロジック)が、1サイクル=1PRの原則どおり単一PRとして進める(人間に確認済み)。
コミットは意味のある単位に分割する。

## 前提として確認済みの既存資産

- `pnpm-workspace.yaml`(既存): `packages: ["tools/*"]`のみ。ルート`package.json`は
  `"build": "pnpm -r build"` / `"typecheck": "pnpm -r typecheck"`。
- `.github/workflows/ci.yml`(既存5ジョブ): `lint-test` / `wasm-test`(wasm32ターゲット+
  `jetli/wasm-pack-action@v0.4.0`で`wasm-pack test --node crates/tabifuda-wasm`+TS
  バインディングのドリフト検査) / `gitleaks` / `cargo-audit` / `docs-site`(`pnpm -r
  typecheck` → `pnpm -r build`、wasm32ツールチェーンなし)。
- `docs-site`ジョブと`pages.yml`のbuildジョブは**ワークスペース全体を`pnpm -r`で再帰
  実行**しているため、`apps/*`をworkspaceに追加すると自動的に巻き込まれる。apps/webの
  ビルドはwasm-packを要するため、そのままでは両ジョブが壊れる。
- `crates/tabifuda-wasm/bindings/*.ts`(ts-rs生成、41ファイル、コミット済み)に
  `Command`/`Event`/`Session`/`Scenario`等のTS型が既にある。`pkg/`(wasm-pack出力の
  JS glue)はまだ存在しない別物。
- `docs/design/wasm-boundary.md`のAPI(`decide`/`apply_all`/`validate_patch`/`lint`、
  すべてJSON文字列)は確定済みで変更しない。
- `docs/design/client-conventions.md`: 手札表示から`CardKind::Marker`を除外する規約
  (CLI実装済み、Web版もC2以降で従う)。
- `crates/tabifuda-cli/src/play.rs`の`issue()`(decide→events→apply連鎖)と
  Marker除外フィルタが、Web側で再現すべき参照実装。
- task.md C3(タイムラインUI・自由入力・提案・GM裁定UI)は明確に対象外。C2では
  `free_text`は常にnull、Propose/GmAdvance/JudgeProposalは実装しない。

## 設計決定

### wasm-packの呼び出しと出力先
pnpm workspaceのパッケージにはしない(`pkg/`はビルド前に存在せず、`pnpm install`が
workspace globを解決するタイミングと合わないため)。`crates/tabifuda-wasm/pkg`に
`wasm-pack build --target web`で出力し(`.gitignore`に追記)、apps/web側からは相対
importで参照する。`apps/web/package.json`の`wasm:build`スクリプトとして定義し、
`dev`/`build`/`typecheck`の前段で必ず実行する(tools/docs-siteの
`"typecheck": "npm run gen:test-report && tsc --noEmit"`と同じ「前段生成スクリプト」
パターンを踏襲)。

### TypeScript型解決
手書き`.d.ts`は作らない。`wasm:build`が毎回先に走るためtscは常に実物の生成`.d.ts`を
見る。パスエイリアス(tsconfig paths / vite resolve.alias の二重管理)は導入しない
(参照箇所が少なく、二重設定のドリフトリスクの方が深い相対パスより害が大きいと判断)。
代わりに境界越えのimportを1ファイルに集約する:

- `apps/web/src/core/bindings.ts`: `crates/tabifuda-wasm/bindings/*.ts`から必要な型
  (`Command`/`Event`/`Session`/`Scenario`/`WasmError`等)を`export type { ... } from
  "../../../../crates/tabifuda-wasm/bindings/Xxx"`で再エクスポートするバレルファイル
- `apps/web/src/core/wasmClient.ts`: `../../../../crates/tabifuda-wasm/pkg/tabifuda_wasm`
  (wasm-pack生成物)をimportし、`decide`/`applyAll`/`lint`をJSONの文字列化/パースまで
  済ませた薄い関数として公開(`init()`呼び出しも内包)

他のソースファイルは`core/bindings`・`core/wasmClient`からのみimportし、深い相対パスは
この2ファイルに閉じ込める。

### CIジョブ構成
既存`docs-site`ジョブに便乗させない。理由: `docs-site`ジョブと`pages.yml`のbuildジョブは
どちらもwasm32ツールチェーンを持たず、apps/web追加でそのまま巻き込むと無関係な検証まで
壊れる。

1. `docs-site`ジョブ・`pages.yml`のbuildジョブのコマンドを`pnpm -r ...`から
   `pnpm --filter @tabifuda/docs-site ...`に絞り、apps/webの巻き込みを止める
2. `ci.yml`に新規`web`ジョブを追加: `dtolnay/rust-toolchain@stable`
   (`targets: wasm32-unknown-unknown`)+`Swatinem/rust-cache@v2`+
   `jetli/wasm-pack-action@v0.4.0`(`wasm-test`ジョブと同一設定)+
   `pnpm/action-setup@v6`+`actions/setup-node@v7`(Node24)+
   `pnpm install --frozen-lockfile`→`pnpm --filter @tabifuda/web typecheck`→
   `pnpm --filter @tabifuda/web lint`→`pnpm --filter @tabifuda/web build`
3. **`docs/adr/0003-ci-pipeline.md`をこの変更内容で先に更新する**(CLAUDE.md最重要
   ルール1。ci.ymlのジョブ構成変更は本ADRを先に更新するルールが既に明記されている)

lintはESLint flat config(`@typescript-eslint`+`eslint-plugin-react-hooks`の
recommendedのみ)の基盤導入までとし、UGC専用ルール(`dangerouslySetInnerHTML`検出)は
C4で追加する(task.md C2の「CI拡張(型チェック・lint・build)」はこの基盤導入で満たす)。

### 状態管理
イベント列だけをReact stateとし、Session等はそこからの純粋な導出にする
(`useReducer`でイベント配列を管理、`applyAll`によるリプレイを`useMemo`で行う)。

```
useGameSession(scenario, actorId):
  events: Event[]        ← useReducer((s, add: Event[]) => [...s, ...add], [])
  session: Session | null ← useMemo(() => applyAll(null, events), [events])
  error: WasmError | null ← useState(独立。直近のdecide失敗のみ、履歴に残さない)
  dispatch(command):
    decide(session ? JSON化 : null, actorId, command) が
      成功 → 返ったEvent[]をreducerにappend、error=null
      失敗 → WasmErrorをerrorにセット、eventsは変更しない
```

`session`を独立stateとして持たない(2つのstateの不整合を構造的に排除)。イベント数は
ソロMVPで小さく、dispatchごとの全件replayは無視できるコスト。`wasm-boundary.md`の
「`apply_all(None, events)`がそのままリプレイになるため専用replay関数は設けない」と
整合する。Reduxは導入しない(ADR 0006で不採用と明記済み)。

### UIの最小実装範囲
「見た目は最小」を字義通りに取る。CSSは1ファイル(`App.css`)、ダークモード固定の
背景色/文字色のみ(`docs/design/ui-visual-design.md`のダークモード方針の最小反映)。
コンポーネントは4つ: `App`(オーケストレーション)/`SceneView`(シーン文+手札の入れ物)/
`Hand`(カードボタン列、Marker除外)/`ErrorBanner`(RuleError/WasmErrorの生表示)。
自由入力欄・提案・GM裁定UI・「もう一度遊ぶ」ボタンはC2のスコープに含めない
(task.md完了条件・C2記述のどちらにも無いため。C3以降で必要になれば追加)。

## ディレクトリ構成(追加分)

```
apps/web/
  index.html
  package.json
  tsconfig.json
  vite.config.ts
  eslint.config.js
  src/
    main.tsx
    App.tsx
    App.css
    core/
      bindings.ts       # ts-rs生成型の再エクスポートバレル
      wasmClient.ts      # decide/applyAll/lint の薄いJSONラッパー+init
    session/
      useGameSession.ts  # イベント列を正とする状態管理フック
      soloParty.ts       # ソロキャラ構築(CLIのSOLO_CHARACTER_ID="hunter"を踏襲)
      scenarioLookup.ts  # cardDef()/sceneDef()/visibleHand()(Marker除外)
    scenario/
      simpleHunt.ts      # shared/scenarios/simple-hunt.json の読み込み
    components/
      SceneView.tsx
      Hand.tsx
      ErrorBanner.tsx
```

既存/変更対象:
```
pnpm-workspace.yaml          # packages に "apps/*" 追加
.gitignore                   # crates/tabifuda-wasm/pkg/ を追記
.github/workflows/ci.yml     # web ジョブ新設。docs-site ジョブを --filter 化
.github/workflows/pages.yml  # build ジョブを --filter 化
docs/adr/0003-ci-pipeline.md # 実装前に追記
docs/design/client-conventions.md  # Marker除外をWeb版でも実装した旨を反映(design-syncで要否判断)
docs/tasks/projects/phase3/task.md # frontmatterのC2をdoneに更新(完了時)
```

## Reactコンポーネント構成

```
App
 ├─ session === null
 │     → 「はじめる」ボタン1つ。onClick: dispatch(StartSession{scenario, party:[soloParty]})
 ├─ session.status === "Running"
 │     → SceneView(narration) + Hand(visibleHand(session, characterId))
 │        カード = ボタン。onClick: dispatch(PlayCard{by, card: instance.id, free_text: null})
 ├─ session.status === {"Ended": outcome}
 │     → Victory/Defeat表示のみ
 ├─ session.status === {"Paused": ...}
 │     → C2の操作フローでは到達しない想定のフォールバック表示のみ(C3で本実装)
 └─ ErrorBanner(error)  # RuleError/WasmErrorをそのままテキスト表示
```

`Hand`のMarker除外は`scenarioLookup.visibleHand(session, characterId)`に集約する
(`crates/tabifuda-cli/src/play.rs`の`hand`構築フィルタと同じロジックをTSで再実装)。

## チェックリスト分割案(コミット単位、1PR)

1. `docs/adr/0003-ci-pipeline.md`更新(web job新設・docs-site/pages.ymlの`--filter`化を先に文書化)
2. インフラ: `pnpm-workspace.yaml`に`apps/*`追加、`.gitignore`に`pkg/`追記、
   `ci.yml`にwebジョブ追加、`docs-site`ジョブ/`pages.yml`のコマンドを`--filter`化
3. インフラ: `apps/web`骨格(package.json/tsconfig/vite.config/eslint.config/
   index.html/main.tsx+空のApp.tsx)を追加し、`pnpm install`→
   `pnpm --filter @tabifuda/web build`が通ることだけを確認するコミット(白紙ページ)
4. 機能: `core/bindings.ts`+`core/wasmClient.ts`+`session/useGameSession.ts`+
   `session/scenarioLookup.ts`+`session/soloParty.ts`+`scenario/simpleHunt.ts`
   (イベント列駆動の状態管理層)
5. 機能: `components/`(SceneView/Hand/ErrorBanner)+`App.tsx`本実装+
   `App.css`(ダークモード最小スタイル)
6. 仕上げ: design-syncでの乖離チェック、`client-conventions.md`反映(必要なら)、
   agent-journal.md追記、task.mdのC2をdoneに更新

## 検証手順

1. `pnpm install`(ルート)→ `apps/web`がworkspaceに認識されることを確認
2. `pnpm --filter @tabifuda/web typecheck` → `wasm-pack build`が走り`pkg/`生成→
   `tsc --noEmit`が通ることを確認
3. `pnpm --filter @tabifuda/web lint` → ESLintが通ることを確認
4. `pnpm --filter @tabifuda/web dev` → ブラウザで開発サーバを開き:
   - `shared/scenarios/simple-hunt.json`の読み込みが解決できるか
     (Viteのworkspace root自動検出がcrates/を含むかを確認。403が出れば
     `vite.config.ts`に`server.fs.allow`を明示追加する)
   - 「はじめる」→シーン文+手札表示(Markerカードが出ていないこと)
   - カードを1枚出す→画面が更新される(devtoolsコンソールにRust panicが出ていないこと)
   - `simple-hunt.json`を最後まで進めてVictory/Defeat表示に到達できることを確認
     (task.md完了条件「ブラウザで通しプレイ可能」の基礎部分)
5. `pnpm --filter @tabifuda/web build`→`dist/`生成、`preview`で同じ通しプレイを確認
6. `pnpm --filter @tabifuda/docs-site typecheck`/`build`が単独で引き続き通ること
   (apps/web追加による巻き込み事故が無いことの確認)
7. PR上でCIの`web`/`docs-site`/`wasm-test`ジョブがすべて緑になることを確認

## Critical Files
- `pnpm-workspace.yaml`, `.github/workflows/ci.yml`, `.github/workflows/pages.yml`
- `docs/adr/0003-ci-pipeline.md`
- `crates/tabifuda-wasm/bindings/*.ts`(参照する型定義)
- `crates/tabifuda-cli/src/play.rs`(decide/apply連鎖・Marker除外の参照実装)
- `docs/design/wasm-boundary.md`, `docs/design/client-conventions.md`
