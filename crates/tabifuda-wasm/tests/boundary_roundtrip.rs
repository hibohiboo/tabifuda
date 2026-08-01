#![cfg(target_arch = "wasm32")]
#![allow(non_snake_case)]
//! wasm境界のTS↔Rust型往復テスト。docs/design/test-strategy.md
//! 「3. engine-cli / engine-wasm(薄く)」: ルールの正誤はcoreで検証済みなので、
//! ここではJSONが正しく境界を越えるか(往復・エラー種別の判別)のみを確認する。
//! `wasm-pack test --node` で実行(cargo test --workspaceの対象外。ホスト
//! ターゲットではコンパイルされない。docs/design/wasm-boundary.md参照)。

use tabifuda_core::{
    BoundedString, CardInstanceId, Character, CharacterId, Command, Phase, PhaseDef, Scenario,
    ScenarioId, ScenarioMeta, SceneDef, SceneKind,
};
use wasm_bindgen_test::wasm_bindgen_test;

fn short(s: &str) -> BoundedString<200> {
    BoundedString::try_new(s).unwrap()
}
fn long(s: &str) -> BoundedString<2000> {
    BoundedString::try_new(s).unwrap()
}

fn fixture_scenario() -> Scenario {
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("boundary-test".into()),
            title: short("境界テスト"),
            author: short("test"),
            forked_from: None,
        },
        card_defs: vec![],
        phases: vec![PhaseDef {
            phase: Phase::Opening,
            scenes: vec![SceneDef {
                id: tabifuda_core::SceneId("s1".into()),
                kind: SceneKind::Conversation,
                narration: long("開始"),
                deals: vec![],
                exits: vec![],
            }],
        }],
    }
}

fn fixture_party() -> Vec<Character> {
    vec![Character {
        id: CharacterId("hero".into()),
        name: "勇者".to_string(),
        stats: Default::default(),
        deck: vec![],
    }]
}

#[wasm_bindgen_test]
fn decideはStartSessionを受理しEvent列のJSONを返す() {
    let cmd = Command::StartSession {
        scenario: fixture_scenario(),
        party: fixture_party(),
    };
    let cmd_json = serde_json::to_string(&cmd).unwrap();

    let events_json = tabifuda_wasm::decide(None, "gm".to_string(), cmd_json)
        .expect("StartSession should be accepted");
    let events: Vec<tabifuda_core::Event> = serde_json::from_str(&events_json).unwrap();
    assert!(!events.is_empty());
}

#[wasm_bindgen_test]
fn apply_allはdecideの結果を適用しSessionのJSONを返す() {
    let cmd = Command::StartSession {
        scenario: fixture_scenario(),
        party: fixture_party(),
    };
    let cmd_json = serde_json::to_string(&cmd).unwrap();
    let events_json = tabifuda_wasm::decide(None, "gm".to_string(), cmd_json).unwrap();

    let state_json = tabifuda_wasm::apply_all(None, events_json)
        .expect("apply_all should succeed")
        .expect("state should be Some after SessionStarted");
    let session: tabifuda_core::Session = serde_json::from_str(&state_json).unwrap();
    assert_eq!(session.phase, Phase::Opening);
}

#[wasm_bindgen_test]
fn decideは不正なJSONをdecodeエラーとして返す() {
    let err = tabifuda_wasm::decide(None, "gm".to_string(), "not json".to_string())
        .expect_err("invalid JSON should be rejected");
    let err_str = err.as_string().expect("error should be a JS string");
    assert!(err_str.contains(r#""kind":"decode""#), "got: {err_str}");
}

#[wasm_bindgen_test]
fn decideはルール違反をruleエラーとして返す() {
    // セッション未開始でのPlayCardはRuleError::NoActiveSession。
    let cmd = Command::PlayCard {
        by: CharacterId("hero".into()),
        card: CardInstanceId("x".into()),
        free_text: None,
    };
    let cmd_json = serde_json::to_string(&cmd).unwrap();

    let err = tabifuda_wasm::decide(None, "gm".to_string(), cmd_json)
        .expect_err("PlayCard without a session should be rejected");
    let err_str = err.as_string().expect("error should be a JS string");
    assert!(err_str.contains(r#""kind":"rule""#), "got: {err_str}");
}

#[wasm_bindgen_test]
fn lintはシナリオJSONを受け取りfindingのJSON配列を返す() {
    let scenario_json = serde_json::to_string(&fixture_scenario()).unwrap();

    let result = tabifuda_wasm::lint(scenario_json).expect("lint should succeed");
    let findings: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();
    // fixture_scenarioはEndSessionへ到達しないシーンのみで構成されるため、
    // DeadEndScene警告が最低1件出る(lintそのものの正しさはcore側でテスト済み。
    // ここではJSONが往復できることのみを見る)。
    assert!(!findings.is_empty());
}
