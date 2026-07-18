//! `tabifuda-cli lint <file>` の翻訳(ファイル読込→lint呼び出し→終了コード)のみを
//! 検証する薄いテスト。lintの検査ロジック自体はtabifuda-core側でテスト済み
//! (docs/design/test-strategy.md §3「ルール分岐はテストしない」に対応)。

use std::io::Write;
use std::process::Command;

use tabifuda_core::{
    CardDef, CardKind, Effect, Outcome, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef,
    SceneKind, Target,
};
use tabifuda_core::{CardId, ScenarioId, SceneId};

fn write_temp_json(name: &str, scenario: &Scenario) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "tabifuda-cli-test-{name}-{}-{}.json",
        std::process::id(),
        name
    ));
    let json = serde_json::to_string(scenario).unwrap();
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    path
}

fn minimal_valid_scenario() -> Scenario {
    let end_card = CardDef {
        id: CardId("end".to_string()),
        name: "end".to_string(),
        kind: CardKind::Marker,
        text: String::new(),
        tags: vec![],
        effects: vec![Effect::EndSession(Outcome::Victory)],
        requires: vec![],
    };
    let s1 = SceneDef {
        id: SceneId("s1".to_string()),
        kind: SceneKind::Conversation,
        narration: String::new(),
        deals: vec![tabifuda_core::Deal {
            card: CardId("end".to_string()),
            to: Target::Party,
        }],
        exits: vec![],
    };
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("scenario1".to_string()),
            title: String::new(),
            author: String::new(),
            forked_from: None,
        },
        card_defs: vec![end_card],
        phases: vec![PhaseDef {
            phase: Phase::Opening,
            scenes: vec![s1],
        }],
    }
}

fn broken_scenario_with_unknown_card_ref() -> Scenario {
    let s1 = SceneDef {
        id: SceneId("s1".to_string()),
        kind: SceneKind::Conversation,
        narration: String::new(),
        deals: vec![tabifuda_core::Deal {
            card: CardId("nowhere".to_string()),
            to: Target::Party,
        }],
        exits: vec![],
    };
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("scenario1".to_string()),
            title: String::new(),
            author: String::new(),
            forked_from: None,
        },
        card_defs: vec![],
        phases: vec![PhaseDef {
            phase: Phase::Opening,
            scenes: vec![s1],
        }],
    }
}

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tabifuda-cli"))
}

#[test]
fn lint_exits_success_on_valid_scenario() {
    let path = write_temp_json("valid", &minimal_valid_scenario());
    let output = bin().arg("lint").arg(&path).output().unwrap();
    std::fs::remove_file(&path).ok();
    assert!(
        output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn lint_exits_failure_on_scenario_with_unknown_reference() {
    let path = write_temp_json("broken", &broken_scenario_with_unknown_card_ref());
    let output = bin().arg("lint").arg(&path).output().unwrap();
    std::fs::remove_file(&path).ok();
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("error"), "stdout: {stdout}");
}

#[test]
fn lint_exits_failure_on_missing_file() {
    let output = bin().arg("lint").arg("no-such-file.json").output().unwrap();
    assert!(!output.status.success());
}
