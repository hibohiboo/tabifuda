//! tabifuda-core を wasm-bindgen 経由で TS へ公開する境界。
//! docs/design/wasm-boundary.md に対応。
//!
//! ステートレス: WASM側でSessionを可変オブジェクトとして保持しない。
//! Command/Event/Session/Scenario等はすべてJSON文字列で受け渡す
//! (CLAUDE.md最重要ルール2・3。coreの純粋性・イベント経由の進行を境界の
//! 外でも保つ)。

use serde::{Deserialize, Serialize};
use tabifuda_core::{
    self as core, Command, Event, PatchError, RuleError, Scenario, ScenarioPatch, Session, UserId,
};
use wasm_bindgen::prelude::*;

/// wasmモジュール読み込み時に一度だけ呼ぶ。Rust panic時にconsoleへ
/// スタックトレースを出す(wasm-boundary.md「方針4」。黙って落ちない)。
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// 境界越しのエラー封筒。`RuleError`/`PatchError`をそのままJSON化せず、
/// 種別を含めて包む(TS側が`kind`で確実に分岐できるようにするため。
/// wasm-boundary.md「方針4」)。TSへ渡すのみ(Rust側でデコードしないため
/// `Deserialize`は実装しない)。
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "error", rename_all = "snake_case")]
enum WasmError {
    Rule(RuleError),
    Patch(PatchError),
    /// `serde_json::from_str`失敗など境界都合のエラー(TS側の入力不備)。
    /// `BoundedString`の長さ上限超過もここに分類される
    /// (wasm-boundary.md「方針5」)。
    Decode(String),
}

impl WasmError {
    fn to_js(&self) -> JsValue {
        // WasmError自体のシリアライズは失敗しない(全フィールドが
        // 既知の型のSerialize実装のみで構成されるため)。
        JsValue::from_str(&serde_json::to_string(self).expect("WasmError is always serializable"))
    }
}

fn decode<T: for<'de> Deserialize<'de>>(json: &str, what: &str) -> Result<T, JsValue> {
    serde_json::from_str(json).map_err(|e| WasmError::Decode(format!("{what}: {e}")).to_js())
}

fn encode<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("core types are always serializable")
}

/// state_json: `Option<Session>`のJSON文字列(Noneは未開始セッション)
/// actor: UserId
/// cmd_json: `Command`のJSON文字列
/// 戻り値: Ok(`Vec<Event>`のJSON文字列) / Err(WasmErrorのJSON文字列)
#[wasm_bindgen]
pub fn decide(
    state_json: Option<String>,
    actor: String,
    cmd_json: String,
) -> Result<String, JsValue> {
    let state: Option<Session> = match state_json {
        Some(s) => Some(decode(&s, "state_json")?),
        None => None,
    };
    let actor = UserId(actor);
    let cmd: Command = decode(&cmd_json, "cmd_json")?;

    let events =
        core::decide(state.as_ref(), &actor, cmd).map_err(|e| WasmError::Rule(e).to_js())?;
    Ok(encode(&events))
}

/// 1イベント分の内部適用。`apply_all`が内部でループ呼び出しする
/// (wasm-boundary.md「単体applyはTSへ公開しない」)。
fn apply_one(state: Option<Session>, event: &Event) -> Option<Session> {
    core::apply(state, event)
}

/// state_json: `Option<Session>`のJSON文字列
/// events_json: `Vec<Event>`のJSON文字列
/// 戻り値: Ok(新しいstateのJSON文字列。Noneはdecide/applyの不正組み合わせ、
/// 通常経路では起きない) / Err(WasmErrorのJSON文字列)
///
/// all-or-nothing: 途中のeventでcoreがNoneを返したら全体をErrにし、
/// 部分適用したstateを返さない。`apply_all(None, events)`がそのまま
/// リプレイになる。UIからはこの関数のみを使う。
#[wasm_bindgen]
pub fn apply_all(
    state_json: Option<String>,
    events_json: String,
) -> Result<Option<String>, JsValue> {
    let mut state: Option<Session> = match state_json {
        Some(s) => Some(decode(&s, "state_json")?),
        None => None,
    };
    let events: Vec<Event> = decode(&events_json, "events_json")?;

    for event in &events {
        state = match apply_one(state, event) {
            Some(next) => Some(next),
            None => {
                return Err(WasmError::Decode(
                    "apply produced no state (invalid state/event combination)".to_string(),
                )
                .to_js())
            }
        };
    }

    Ok(state.map(|s| encode(&s)))
}

/// session_json: `Session`のJSON文字列、patch_json: `ScenarioPatch`のJSON文字列
/// 戻り値: Ok(()) / Err(WasmErrorのJSON文字列)
#[wasm_bindgen]
pub fn validate_patch(session_json: String, patch_json: String) -> Result<(), JsValue> {
    let session: Session = decode(&session_json, "session_json")?;
    let patch: ScenarioPatch = decode(&patch_json, "patch_json")?;

    core::validate(&session, &patch).map_err(|e| WasmError::Patch(e).to_js())
}

/// scenario_json: `Scenario`のJSON文字列
/// 戻り値: Ok(`Vec<LintFinding>`のJSON文字列) / Err(WasmErrorのJSON文字列。
/// lint自体は失敗しないが、scenario_jsonのdecode失敗はありうる)
#[wasm_bindgen]
pub fn lint(scenario_json: String) -> Result<String, JsValue> {
    let scenario: Scenario = decode(&scenario_json, "scenario_json")?;
    Ok(encode(&core::lint(&scenario)))
}
