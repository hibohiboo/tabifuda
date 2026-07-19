//! シナリオデータの静的検証(シナリオlint)。
//! docs/design/scenario-lint.md に対応。
//!
//! `lint(scenario)` は純粋関数(IOなし)。将来の`scenario-validate`スキルと
//! 実装を共有する前提のため、CLI固有にせずここに置く。

use std::collections::{HashSet, VecDeque};

use crate::card::{CardDef, Condition, Effect, Target};
use crate::ids::{CardId, SceneId};
use crate::scenario::{Deal, Scenario, SceneDef, Transition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// 検査で見つかった問題。docs/design/scenario-lint.md「検査項目と重大度」の表に対応。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LintIssue {
    DuplicateCardId(CardId),
    DuplicateSceneId(SceneId),
    UnknownCardId(CardId),
    UnknownSceneId(SceneId),
    CharacterTargetInScenarioData,
    NoInitialScene,
    UnreachableScene(SceneId),
    DeadEndScene(SceneId),
}

impl LintIssue {
    pub fn severity(&self) -> Severity {
        match self {
            LintIssue::DuplicateCardId(_)
            | LintIssue::DuplicateSceneId(_)
            | LintIssue::UnknownCardId(_)
            | LintIssue::UnknownSceneId(_)
            | LintIssue::CharacterTargetInScenarioData
            | LintIssue::NoInitialScene => Severity::Error,
            LintIssue::UnreachableScene(_) | LintIssue::DeadEndScene(_) => Severity::Warning,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFinding {
    pub severity: Severity,
    pub issue: LintIssue,
}

impl std::fmt::Display for LintFinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "[{severity}] {:?}", self.issue)
    }
}

fn finding(issue: LintIssue) -> LintFinding {
    LintFinding {
        severity: issue.severity(),
        issue,
    }
}

pub fn lint(scenario: &Scenario) -> Vec<LintFinding> {
    let mut findings = Vec::new();

    check_duplicate_ids(scenario, &mut findings);
    check_references(scenario, &mut findings);

    let all_scene_ids: HashSet<&SceneId> = scenario
        .phases
        .iter()
        .flat_map(|phase| phase.scenes.iter())
        .map(|scene| &scene.id)
        .collect();

    match initial_scene(scenario) {
        None => findings.push(finding(LintIssue::NoInitialScene)),
        Some(initial) => {
            let reachable = reachable_from(scenario, initial);
            for id in &all_scene_ids {
                if !reachable.contains(*id) {
                    findings.push(finding(LintIssue::UnreachableScene((*id).clone())));
                }
            }
            for id in &reachable {
                if !can_reach_end(scenario, id) {
                    findings.push(finding(LintIssue::DeadEndScene((*id).clone())));
                }
            }
        }
    }

    findings
}

fn initial_scene(scenario: &Scenario) -> Option<&SceneId> {
    scenario
        .phases
        .first()
        .and_then(|phase| phase.scenes.first())
        .map(|scene| &scene.id)
}

fn check_duplicate_ids(scenario: &Scenario, findings: &mut Vec<LintFinding>) {
    let mut seen_cards = HashSet::new();
    for def in &scenario.card_defs {
        if !seen_cards.insert(&def.id) {
            findings.push(finding(LintIssue::DuplicateCardId(def.id.clone())));
        }
    }

    let mut seen_scenes = HashSet::new();
    for phase in &scenario.phases {
        for scene in &phase.scenes {
            if !seen_scenes.insert(&scene.id) {
                findings.push(finding(LintIssue::DuplicateSceneId(scene.id.clone())));
            }
        }
    }
}

fn check_references(scenario: &Scenario, findings: &mut Vec<LintFinding>) {
    let check_card = |card: &CardId, findings: &mut Vec<LintFinding>| {
        if scenario.card_def(card).is_none() {
            findings.push(finding(LintIssue::UnknownCardId(card.clone())));
        }
    };
    let check_scene = |scene: &SceneId, findings: &mut Vec<LintFinding>| {
        if scenario.scene_def(scene).is_none() {
            findings.push(finding(LintIssue::UnknownSceneId(scene.clone())));
        }
    };
    let check_target = |target: &Target, findings: &mut Vec<LintFinding>| {
        if matches!(target, Target::Character(_)) {
            findings.push(finding(LintIssue::CharacterTargetInScenarioData));
        }
    };
    let check_condition = |condition: &Condition, findings: &mut Vec<LintFinding>| {
        if let Condition::HasCard(card) = condition {
            check_card(card, findings);
        }
    };

    for def in &scenario.card_defs {
        check_card_def(def, &check_card, &check_scene, &check_target, findings);
    }
    for phase in &scenario.phases {
        for scene in &phase.scenes {
            check_scene_def(
                scene,
                &check_card,
                &check_scene,
                &check_target,
                &check_condition,
                findings,
            );
        }
    }
}

fn check_card_def(
    def: &CardDef,
    check_card: &impl Fn(&CardId, &mut Vec<LintFinding>),
    check_scene: &impl Fn(&SceneId, &mut Vec<LintFinding>),
    check_target: &impl Fn(&Target, &mut Vec<LintFinding>),
    findings: &mut Vec<LintFinding>,
) {
    for effect in &def.effects {
        match effect {
            Effect::GotoScene(scene) => check_scene(scene, findings),
            Effect::DealCard { card, to } => {
                check_card(card, findings);
                check_target(to, findings);
            }
            Effect::ModifyStat { target, .. } => check_target(target, findings),
            Effect::AdvancePhase | Effect::EndSession(_) => {}
        }
    }
    for condition in &def.requires {
        if let Condition::HasCard(card) = condition {
            check_card(card, findings);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn check_scene_def(
    scene: &SceneDef,
    check_card: &impl Fn(&CardId, &mut Vec<LintFinding>),
    check_scene: &impl Fn(&SceneId, &mut Vec<LintFinding>),
    check_target: &impl Fn(&Target, &mut Vec<LintFinding>),
    check_condition: &impl Fn(&Condition, &mut Vec<LintFinding>),
    findings: &mut Vec<LintFinding>,
) {
    for deal in &scene.deals {
        let Deal { card, to } = deal;
        check_card(card, findings);
        check_target(to, findings);
    }
    for transition in &scene.exits {
        let Transition { condition, to } = transition;
        check_condition(condition, findings);
        check_scene(to, findings);
    }
}

/// シーン`scene`から直接辿れるシーンID(scenario-lint.md「到達可能性・詰み検知の
/// 探索範囲」参照。DealCardで後から配られたカードの効果は追わない)。
fn direct_successors(scenario: &Scenario, scene: &SceneDef) -> Vec<SceneId> {
    let mut out: Vec<SceneId> = scene.exits.iter().map(|t| t.to.clone()).collect();
    for deal in &scene.deals {
        if let Some(def) = scenario.card_def(&deal.card) {
            for effect in &def.effects {
                if let Effect::GotoScene(target) = effect {
                    out.push(target.clone());
                }
            }
        }
    }
    out
}

fn reachable_from(scenario: &Scenario, start: &SceneId) -> HashSet<SceneId> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start.clone());
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        let Some(scene) = scenario.scene_def(&current) else {
            continue;
        };
        for next in direct_successors(scenario, scene) {
            if visited.insert(next.clone()) {
                queue.push_back(next);
            }
        }
    }

    visited
}

fn scene_has_terminal_deal(scenario: &Scenario, scene: &SceneDef) -> bool {
    scene.deals.iter().any(|deal| {
        scenario.card_def(&deal.card).is_some_and(|def| {
            def.effects
                .iter()
                .any(|e| matches!(e, Effect::EndSession(_)))
        })
    })
}

fn can_reach_end(scenario: &Scenario, start: &SceneId) -> bool {
    reachable_from(scenario, start).iter().any(|id| {
        scenario
            .scene_def(id)
            .is_some_and(|scene| scene_has_terminal_deal(scenario, scene))
    })
}
