//! lint::lint の単体テスト。docs/design/domain-model.md「シナリオlint(P2 C1決定)」
//! の検査項目表に対応(受理系=findingsが空、各拒否系=該当issueが1件含まれることを検証)。

use crate::card::{CardDef, CardKind, Condition, Effect, Target};
use crate::ids::{CardId, CharacterId, SceneId};
use crate::lint::{lint, LintIssue, Severity};
use crate::primitives::Outcome;
use crate::scenario::{
    Deal, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind, Transition,
};

fn cid(s: &str) -> CardId {
    CardId(s.to_string())
}
fn scn(s: &str) -> SceneId {
    SceneId(s.to_string())
}
fn chr(s: &str) -> CharacterId {
    CharacterId(s.to_string())
}

fn card_def(id: &str) -> CardDef {
    CardDef {
        id: cid(id),
        name: id.to_string(),
        kind: CardKind::Item,
        text: String::new(),
        tags: vec![],
        effects: vec![],
        requires: vec![],
    }
}

fn scene(id: &str) -> SceneDef {
    SceneDef {
        id: scn(id),
        kind: SceneKind::Conversation,
        narration: String::new(),
        deals: vec![],
        exits: vec![],
    }
}

fn scenario(card_defs: Vec<CardDef>, phases: Vec<PhaseDef>) -> Scenario {
    Scenario {
        meta: ScenarioMeta {
            id: crate::ids::ScenarioId("scenario1".to_string()),
            title: String::new(),
            author: String::new(),
            forked_from: None,
        },
        card_defs,
        phases,
    }
}

fn only(phase: Phase, scenes: Vec<SceneDef>) -> Vec<PhaseDef> {
    vec![PhaseDef { phase, scenes }]
}

fn issues(scenario: &Scenario) -> Vec<LintIssue> {
    lint(scenario).into_iter().map(|f| f.issue).collect()
}

/// 単一シーンにEndSessionカードを配る、最小の「壊れていない」シナリオ。
fn minimal_valid_scenario() -> Scenario {
    let mut end_card = card_def("end");
    end_card.effects.push(Effect::EndSession(Outcome::Victory));
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("end"),
        to: Target::Party,
    });
    scenario(vec![end_card], only(Phase::Opening, vec![s1]))
}

#[test]
fn lint_accepts_minimal_valid_scenario() {
    assert_eq!(issues(&minimal_valid_scenario()), vec![]);
}

#[test]
fn lint_rejects_duplicate_card_id() {
    let s = scenario(
        vec![card_def("dup"), card_def("dup")],
        only(Phase::Opening, vec![scene("s1")]),
    );
    let found = issues(&s);
    assert!(found.contains(&LintIssue::DuplicateCardId(cid("dup"))));
}

#[test]
fn lint_rejects_duplicate_scene_id_across_phases() {
    let s = scenario(
        vec![],
        vec![
            PhaseDef {
                phase: Phase::Opening,
                scenes: vec![scene("dup")],
            },
            PhaseDef {
                phase: Phase::Middle,
                scenes: vec![scene("dup")],
            },
        ],
    );
    let found = issues(&s);
    assert!(found.contains(&LintIssue::DuplicateSceneId(scn("dup"))));
}

#[test]
fn lint_rejects_unknown_card_id_in_deal() {
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("nowhere"),
        to: Target::Party,
    });
    let s = scenario(vec![], only(Phase::Opening, vec![s1]));
    assert!(issues(&s).contains(&LintIssue::UnknownCardId(cid("nowhere"))));
}

#[test]
fn lint_rejects_unknown_card_id_in_effect_deal_card() {
    let mut card = card_def("c1");
    card.effects.push(Effect::DealCard {
        card: cid("nowhere"),
        to: Target::Party,
    });
    let s = scenario(vec![card], only(Phase::Opening, vec![scene("s1")]));
    assert!(issues(&s).contains(&LintIssue::UnknownCardId(cid("nowhere"))));
}

#[test]
fn lint_rejects_unknown_card_id_in_requires() {
    let mut card = card_def("c1");
    card.requires.push(Condition::HasCard(cid("nowhere")));
    let s = scenario(vec![card], only(Phase::Opening, vec![scene("s1")]));
    assert!(issues(&s).contains(&LintIssue::UnknownCardId(cid("nowhere"))));
}

#[test]
fn lint_rejects_unknown_card_id_in_transition_condition() {
    let mut s1 = scene("s1");
    s1.exits.push(Transition {
        condition: Condition::HasCard(cid("nowhere")),
        to: scn("s1"),
    });
    let s = scenario(vec![], only(Phase::Opening, vec![s1]));
    assert!(issues(&s).contains(&LintIssue::UnknownCardId(cid("nowhere"))));
}

#[test]
fn lint_rejects_unknown_scene_id_in_goto_scene() {
    let mut card = card_def("c1");
    card.effects.push(Effect::GotoScene(scn("nowhere")));
    let s = scenario(vec![card], only(Phase::Opening, vec![scene("s1")]));
    assert!(issues(&s).contains(&LintIssue::UnknownSceneId(scn("nowhere"))));
}

#[test]
fn lint_rejects_unknown_scene_id_in_transition_to() {
    let mut s1 = scene("s1");
    s1.exits.push(Transition {
        condition: Condition::HasCard(cid("c1")),
        to: scn("nowhere"),
    });
    let s = scenario(vec![card_def("c1")], only(Phase::Opening, vec![s1]));
    assert!(issues(&s).contains(&LintIssue::UnknownSceneId(scn("nowhere"))));
}

#[test]
fn lint_rejects_character_target_in_deal() {
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("c1"),
        to: Target::Character(chr("ch1")),
    });
    let s = scenario(vec![card_def("c1")], only(Phase::Opening, vec![s1]));
    assert!(issues(&s).contains(&LintIssue::CharacterTargetInScenarioData));
}

#[test]
fn lint_rejects_character_target_in_effect_deal_card() {
    let mut card = card_def("c1");
    card.effects.push(Effect::DealCard {
        card: cid("c1"),
        to: Target::Character(chr("ch1")),
    });
    let s = scenario(vec![card], only(Phase::Opening, vec![scene("s1")]));
    assert!(issues(&s).contains(&LintIssue::CharacterTargetInScenarioData));
}

#[test]
fn lint_rejects_character_target_in_modify_stat() {
    let mut card = card_def("c1");
    card.effects.push(Effect::ModifyStat {
        target: Target::Character(chr("ch1")),
        stat: crate::ids::StatId("hp".to_string()),
        delta: -1,
    });
    let s = scenario(vec![card], only(Phase::Opening, vec![scene("s1")]));
    assert!(issues(&s).contains(&LintIssue::CharacterTargetInScenarioData));
}

#[test]
fn lint_rejects_empty_phases_as_no_initial_scene() {
    let s = scenario(vec![], vec![]);
    assert_eq!(issues(&s), vec![LintIssue::NoInitialScene]);
}

#[test]
fn lint_rejects_first_phase_without_scenes_as_no_initial_scene() {
    let s = scenario(
        vec![],
        vec![
            PhaseDef {
                phase: Phase::Opening,
                scenes: vec![],
            },
            PhaseDef {
                phase: Phase::Middle,
                scenes: vec![scene("s2")],
            },
        ],
    );
    assert!(issues(&s).contains(&LintIssue::NoInitialScene));
}

#[test]
fn lint_accepts_reachable_scene_via_transition() {
    let mut end_card = card_def("end");
    end_card.effects.push(Effect::EndSession(Outcome::Victory));
    let mut s1 = scene("s1");
    s1.exits.push(Transition {
        condition: Condition::HasCard(cid("end")),
        to: scn("s2"),
    });
    let mut s2 = scene("s2");
    s2.deals.push(Deal {
        card: cid("end"),
        to: Target::Party,
    });
    let s = scenario(vec![end_card], only(Phase::Opening, vec![s1, s2]));
    let found = issues(&s);
    assert!(!found.contains(&LintIssue::UnreachableScene(scn("s2"))));
}

#[test]
fn lint_warns_unreachable_scene() {
    let mut end_card = card_def("end");
    end_card.effects.push(Effect::EndSession(Outcome::Victory));
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("end"),
        to: Target::Party,
    });
    let s2 = scene("s2"); // s1からの辺が無いため到達不能
    let s = scenario(vec![end_card], only(Phase::Opening, vec![s1, s2]));
    let finding = lint(&s)
        .into_iter()
        .find(|f| f.issue == LintIssue::UnreachableScene(scn("s2")))
        .expect("s2 should be reported unreachable");
    assert_eq!(finding.severity, Severity::Warning);
}

#[test]
fn lint_accepts_scene_that_can_reach_end_via_goto_scene() {
    // s1はvictoryカードを配り、それがGotoSceneでs2へ、s2がEndSessionカードを配る。
    let mut victory = card_def("victory");
    victory.effects.push(Effect::GotoScene(scn("s2")));
    let mut end_card = card_def("end");
    end_card.effects.push(Effect::EndSession(Outcome::Victory));
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("victory"),
        to: Target::Party,
    });
    let mut s2 = scene("s2");
    s2.deals.push(Deal {
        card: cid("end"),
        to: Target::Party,
    });
    let s = scenario(vec![victory, end_card], only(Phase::Opening, vec![s1, s2]));
    assert!(!issues(&s).contains(&LintIssue::DeadEndScene(scn("s1"))));
}

#[test]
fn lint_warns_dead_end_scene() {
    // s1は何のEndSessionにも到達できない(deals/exits無し)。
    let s1 = scene("s1");
    let s = scenario(vec![], only(Phase::Opening, vec![s1]));
    let finding = lint(&s)
        .into_iter()
        .find(|f| f.issue == LintIssue::DeadEndScene(scn("s1")))
        .expect("s1 should be reported as dead end");
    assert_eq!(finding.severity, Severity::Warning);
}

#[test]
fn lint_does_not_warn_dead_end_for_unreachable_scene() {
    // s2は到達不能(既にUnreachableSceneで報告される)。詰み検知は到達可能な
    // シーンのみを対象とするため、DeadEndSceneは重複報告しない。
    let mut end_card = card_def("end");
    end_card.effects.push(Effect::EndSession(Outcome::Victory));
    let mut s1 = scene("s1");
    s1.deals.push(Deal {
        card: cid("end"),
        to: Target::Party,
    });
    let s2 = scene("s2");
    let s = scenario(vec![end_card], only(Phase::Opening, vec![s1, s2]));
    assert!(!issues(&s).contains(&LintIssue::DeadEndScene(scn("s2"))));
}
