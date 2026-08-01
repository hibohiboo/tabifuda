# wasm境界

対象: crates/tabifuda-wasm。tabifuda-core の decide/apply/validate/lint を
wasm-bindgen 経由で TS へ公開する境界の設計。表示・操作ロジックの決定は
[client-conventions.md](client-conventions.md) を参照(本文書は境界の型・API
設計のみ)。

**状態: 確定(2026-08-01、Opus 4.8によるレビュー通過。
docs/tasks/projects/phase3/task.md「着手前にOpus 4.8でwasm境界API設計
レビューを1回」を実施)。**

## 方針

1. **ステートレス関数として公開する。** WASM側でSessionを可変オブジェクトと
   して保持しない。JS側がSessionのJSON文字列を持ち、呼び出しごとに渡す。
   区別すべきは「WASM側に可変状態を持つこと」自体ではなく「イベント以外の
   経路で状態が変わりうること」。ステートレス公開はこれを構造的に避ける
   選択であり、進行が必ず `apply(event)` を通る不変条件(CLAUDE.md最重要
   ルール3)を境界の外でも保つ
2. **Command/Event/Session/Scenario等はJSON文字列で受け渡す**(タスク文書
   C1の指定どおり)。wasm-bindgenのシリアライズ機構(serde-wasm-bindgen)は
   使わず、`serde_json::to_string`/`from_str`を明示的に呼ぶ。JSON文字列で
   あれば冒険記の永続化フォーマット(P3.5でCLIが書き出すセーブファイル)と
   スキーマを完全に共有でき、WASM専用の中間表現を増やさない
3. **ワイヤ形式は serde 既定の外部タグ付けで凍結する**(`shared/scenarios/
   simple-hunt.json` が既にこの形式。例: `{"GotoScene":"mid_travel"}`、
   unit variantは`"AdvancePhase"`)。内部タグ(`{"type":...}`)への移行は
   しない(既存シナリオデータ・goldenフィクスチャの書き直しを要するため)。
   TS側の網羅性は `switch` ではなく **ハンドラマップ**
   (`const handlers: { [K in Event['type_key']]: ... }` 相当)で取る設計とし、
   `#[non_exhaustive]` に新variantが増えたらTS側がコンパイルエラーになる
   形にする(P3 C3「未対応の種別を黙って無視しない」要求を型で満たす。
   詳細はC3で client-conventions.md に反映)
4. **エラーは境界クレート内の封筒型 `WasmError` に統一してJSON化する。**
   `RuleError`/`PatchError`をそのままJSON化せず、種別を含めて包む:
   ```rust
   #[derive(Debug, Serialize)] // TSへ渡すのみのためDeserializeは実装しない
   #[serde(tag = "kind", content = "error", rename_all = "snake_case")]
   enum WasmError {
       Rule(RuleError),
       Patch(PatchError),
       Decode(String), // serde_json::from_str失敗など境界都合のエラー
   }
   ```
   TS側は`kind`で確実に分岐できる(`rule`/`patch`/`decode`の3種のみ。
   `decode`が来たら想定外の入力=バグとして扱ってよい)。
   `console_error_panic_hook`を`#[wasm_bindgen(start)]`関数から
   `set_once()`し、万一のRust panicもコンソールに出す(黙って落ちない)
5. **`BoundedString`超過はdecodeエラーに分類される点に注意。**
   `PlayCard.free_text`等が上限を超えると`serde_json::from_str`が失敗し
   `WasmError::Decode`になる(ドメインエラーの`RuleError`ではない)。
   UI側は入力段階で長さ上限を掛け(cross-cutting.md UGC規律)、
   `decode`エラーをそのままユーザーに見せない(client-conventions.mdへ
   C4で反映)

## 公開関数(確定)

```rust
// すべて #[wasm_bindgen] 付き。engine::decide/apply, patch::validate,
// lint::lint への薄いJSON変換ラッパー。

/// state_json: Option<String>(Noneは未開始セッション)
/// actor: String(UserId)
/// cmd_json: String(Command)
/// 戻り値: Ok(Event列のJSON配列文字列) / Err(WasmErrorのJSON文字列)
#[wasm_bindgen]
pub fn decide(state_json: Option<String>, actor: String, cmd_json: String)
    -> Result<String, JsValue>;

/// state_json: Option<String>、events_json: String(Event配列のJSON)
/// 戻り値: Ok(新しいstateのJSON文字列。Noneはdecide/applyの不正組み合わせ
/// [domain-model.md「stateがOptionになる理由」]、通常経路では起きない)
/// / Err(WasmErrorのJSON文字列)
///
/// **all-or-nothing**: 途中のeventでcoreがNoneを返したら全体をErrにし、
/// 部分適用したstateを返さない。`apply_all(None, events)` がそのまま
/// リプレイになるため、専用のreplay関数は設けない。
/// **UIからはこの関数のみを使う**(単体applyはTSへ公開しない。下記参照)。
#[wasm_bindgen]
pub fn apply_all(state_json: Option<String>, events_json: String)
    -> Result<Option<String>, JsValue>;

/// session_json: String、patch_json: String(ScenarioPatch)
/// 戻り値: Ok(()) / Err(WasmErrorのJSON文字列)
#[wasm_bindgen]
pub fn validate_patch(session_json: String, patch_json: String) -> Result<(), JsValue>;

/// scenario_json: String
/// 戻り値: Ok(LintFindingのJSON配列文字列) / Err(WasmErrorのJSON文字列。
/// lint自体は失敗しないが、scenario_jsonのdecode失敗はありうる)
#[wasm_bindgen]
pub fn lint(scenario_json: String) -> Result<String, JsValue>;
```

**単体`apply`(1イベント)は`#[wasm_bindgen]`を付けない境界クレート内部
関数として残し、`apply_all`の実装が内部でループ呼び出しする形にする。**
wasm-bindgen-testの型往復テストはこの内部関数に対しても書ける(同一crate
内)が、TS側のAPI表面には出さない(表面積を最小化し、部分適用の事故経路を
物理的に塞ぐ)。

## 決定した論点

1. **TS型定義の同期方法(実施済み)**: `ts-rs`をcore/wasmクレートに
   `feature = "ts"`で追加した。撤退条件(手書きimplでC1実装timeの半分以上を
   溶かす)には該当せず、そのまま採用した。
   - **出力先**: `crates/tabifuda-wasm/bindings/`(`TS_RS_EXPORT_DIR`環境変数で
   指定)。git管理対象とする(apps/webがC2以降でここからimportする配布物)
   - **エクスポート対象**: WASM境界の入出力に直接現れる最上位型のみに
     `#[ts(export)]`を付ける: `Command`/`Event`/`Session`/`Scenario`/
     `RuleError`/`PatchError`/`ScenarioPatch`/`LintFinding`(core)、
     `WasmError`(wasmクレート)。依存する下位の型(`CardDef`/`Effect`等)は
     ts-rsが依存解決で自動的にbindings/へ書き出す
   - **生成コマンド**: `cargo test -p tabifuda-core --features ts
     export_bindings` → `cargo test -p tabifuda-wasm --features ts
     export_bindings`(2回に分かれるのは`WasmError`が依存する`RuleError`/
     `PatchError`をcore側が先に生成しないため。両方実行すれば全ファイルが揃う)
   - **落とし穴(発生・対処済み)**: `BoundedString<const MAX: usize>`は
     custom `Serialize`/`Deserialize`のため`derive(TS)`が付かず、
     `impl<const MAX: usize> TS for BoundedString<MAX>`(`string`として
     出力。primitives.rs参照)を手書きした
   - **検証**: `BTreeMap<UserId, Role>`のような「キーがstruct newtype」の
     フィールドは`{ [key in UserId]: Role }`という形で出力される。
     `UserId`はJSON上は素の文字列(`type UserId = string`)なので、
     これは`{ [key: string]: Role }`と等価に解決されることをTypeScript
     コンパイラで確認済み(mapped typeの`in`右辺がstring型の場合の挙動)。
     `HashMap`から`BTreeMap`化(下記「既知の課題」参照)した後も
     ts-rsの出力形は変わらないことを`export_bindings`再実行で確認済み
   - CIでのドリフト検査は adr/0003-ci-pipeline.md「wasm-testジョブ」参照
2. **`LintFinding`/`LintIssue`/`Severity`へのSerialize/Deserialize追加**:
   C1スコープに含める。既存の`#[non_exhaustive]`・外部タグ付け方針と整合する。
   - **注意**: `LintFinding{severity, issue}`は`severity`が`issue.severity()`
     から導出可能な値であり、`Deserialize`を素朴にderiveすると矛盾した
     組み合わせ(例: `issue`は`DuplicateCardId`なのに`severity: Warning`)が
     型として作れてしまう。`LintFinding`を外部入力として扱わない旨を
     doc commentに明記し、`finding.severity == finding.issue.severity()`を
     確認する単体テストを1本追加する
   - `Severity`に`#[non_exhaustive]`を今のうちに付ける(将来`Info`追加を
     見込む。TS側バインド後に付けると破壊的変更になる)
3. **crateの物理配置(実施済み)**: `crates/tabifuda-wasm`を新設し、
   ルート`Cargo.toml`の`members`に追加(core/cli/wasmの3つ)。
   `crate-type = ["cdylib", "rlib"]`、依存は`wasm-bindgen`+`serde`+
   `serde_json`+`tabifuda-core`+`console_error_panic_hook`(方針4)+
   `ts-rs`(optional、論点1)、devは`wasm-bindgen-test`
   - `#[wasm_bindgen_test]`を含むテストファイル先頭に
     `#![cfg(target_arch = "wasm32")]`を置く(`cargo test --workspace`は
     ホストターゲットで走るため、これを忘れると境界テストが1本も実行
     されずにCIが緑になる)
   - CIに`wasm-pack test --node`(または headless chrome)ステップを別途
     追加する(adr/0003-ci-pipeline.md更新が必要。C1実装時に反映)
   - `wasm-bindgen`クレートと`wasm-bindgen-cli`/`wasm-pack`のバージョン
     不一致は典型的な突然死要因のため、CIでバージョンを固定する

## 既知の課題(解決済み)

`Session.roles`/`hands`等が`HashMap`だとキー順が非決定的なため、同じ状態でも
JSON文字列が実行ごとに変わりうる問題があった(ソロMVP規模では実害が出にくいが、
セーブファイルの差分が無意味に発生する等につながる)。P3.5 C1でID型に`Ord`を
deriveして`HashMap`→`BTreeMap`へ置換済み(経緯:
[wasm-boundary-decisions.md](../tasks/projects/phase3/plans/wasm-boundary-decisions.md)
論点1)。domain-model.md「セッション状態」参照。
