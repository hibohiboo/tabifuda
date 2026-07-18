//! decide/apply。docs/design/domain-model.md「C2: decide/applyの解決規則」に対応。
//! CLAUDE.md最重要ルール2・3(coreは純粋・進行は必ずイベント経由)を実装する中核。

use std::collections::HashMap;

use crate::actor::Role;
use crate::card::{CardDef, Condition, Effect, Target};
use crate::character::Character;
use crate::command::Command;
use crate::error::RuleError;
use crate::event::Event;
use crate::ids::{CardInstanceId, CharacterId, SceneId, UserId};
use crate::scenario::{Phase, Scenario};
use crate::session::{CardInstance, ScenarioSnapshot, Session, SessionStatus};

/// state == None が許されるのはStartSessionのみ(domain-model.md「stateが
/// Optionになる理由」参照)。
pub fn decide(
    state: Option<&Session>,
    actor: &UserId,
    cmd: Command,
) -> Result<Vec<Event>, RuleError> {
    match (state, cmd) {
        (None, Command::StartSession { scenario, party }) => {
            decide_start_session(actor, scenario, party)
        }
        (None, _) => Err(RuleError::NoActiveSession),
        (Some(_), Command::StartSession { .. }) => Err(RuleError::SessionAlreadyStarted),
        (
            Some(session),
            Command::PlayCard {
                by,
                card,
                free_text,
            },
        ) => decide_play_card(session, actor, by, card, free_text),
        (Some(session), Command::EndSession { outcome }) => {
            decide_end_session(session, actor, outcome)
        }
    }
}

/// `SessionStarted` を state == None に適用したときだけ新規Sessionを構築する。
/// それ以外の不正な組み合わせ(state==NoneかつSessionStarted以外、または
/// state==SomeかつSessionStartedの二重発行)はpanicせずNoneを返す
/// (「decideの出力は必ずapply可能」という不変条件下では起こらない経路)。
pub fn apply(state: Option<Session>, event: &Event) -> Option<Session> {
    match (state, event) {
        (
            None,
            Event::SessionStarted {
                scenario,
                party,
                roles,
                initial_phase,
                initial_scene,
            },
        ) => {
            let hands = party.iter().map(|c| (c.id.clone(), Vec::new())).collect();
            Some(Session {
                scenario: scenario.clone(),
                party: party.clone(),
                status: SessionStatus::Running,
                roles: roles.clone(),
                phase: initial_phase.clone(),
                scene: initial_scene.clone(),
                hands,
                table: Vec::new(),
                pending_proposal: None,
            })
        }
        (Some(session), event) => Some(apply_to_existing(session, event)),
        (None, _) => None,
    }
}

fn apply_to_existing(mut session: Session, event: &Event) -> Session {
    match event {
        Event::SessionStarted { .. } => {}
        Event::SceneEntered { scene, .. } => {
            session.scene = scene.clone();
        }
        Event::CardDealt { to, card, instance } => {
            session
                .hands
                .entry(to.clone())
                .or_default()
                .push(CardInstance {
                    id: instance.clone(),
                    card: card.clone(),
                });
        }
        Event::CardPlayed { .. } => {}
        Event::EffectApplied { .. } => {}
        Event::PhaseAdvanced { phase } => {
            session.phase = phase.clone();
        }
        Event::SessionEnded { outcome } => {
            session.status = SessionStatus::Ended(outcome.clone());
        }
    }
    session
}

fn decide_start_session(
    actor: &UserId,
    scenario: Scenario,
    party: Vec<Character>,
) -> Result<Vec<Event>, RuleError> {
    let first_phase_def = scenario
        .phases
        .first()
        .ok_or(RuleError::ScenarioHasNoScenes)?;
    let first_scene = first_phase_def
        .scenes
        .first()
        .ok_or(RuleError::ScenarioHasNoScenes)?;
    let initial_phase = first_phase_def.phase.clone();
    let initial_scene = first_scene.id.clone();

    let mut roles = HashMap::new();
    roles.insert(actor.clone(), Role::Gm);

    let mut events = vec![Event::SessionStarted {
        scenario: ScenarioSnapshot(scenario.clone()),
        party: party.clone(),
        roles,
        initial_phase,
        initial_scene: initial_scene.clone(),
    }];
    events.extend(enter_scene(&scenario, &party, 0, &initial_scene)?);
    Ok(events)
}

fn decide_play_card(
    session: &Session,
    actor: &UserId,
    by: CharacterId,
    card: CardInstanceId,
    free_text: Option<crate::primitives::BoundedString<4096>>,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_not_paused(session)?;
    check_player_or_gm(session, actor, &by)?;

    let hand = session.hands.get(&by).ok_or(RuleError::CardNotFound)?;
    let instance = hand
        .iter()
        .find(|ci| ci.id == card)
        .ok_or(RuleError::CardNotFound)?;
    let card_def: &CardDef = session
        .scenario
        .0
        .card_def(&instance.card)
        .ok_or(RuleError::CardNotFound)?;

    check_requires(session, &by, &card_def.requires)?;

    let mut events = vec![Event::CardPlayed {
        by: by.clone(),
        card: card_def.id.clone(),
        free_text,
    }];

    let mut instance_count = total_instance_count(session);
    let mut current_phase = session.phase.clone();
    let mut ended = false;
    for effect in &card_def.effects {
        if ended {
            break;
        }
        match effect {
            Effect::GotoScene(scene_id) => {
                let scene_events = enter_scene(
                    &session.scenario.0,
                    &session.party,
                    instance_count,
                    scene_id,
                )?;
                instance_count += scene_events
                    .iter()
                    .filter(|e| matches!(e, Event::CardDealt { .. }))
                    .count();
                events.extend(scene_events);
            }
            Effect::AdvancePhase => {
                let next = next_phase(&current_phase).ok_or(RuleError::NoNextPhase)?;
                current_phase = next.clone();
                events.push(Event::PhaseAdvanced { phase: next });
            }
            Effect::DealCard {
                card: dealt_card,
                to,
            } => {
                for character_id in resolve_target(to, &session.party) {
                    let instance_id =
                        CardInstanceId(format!("{}-{}", dealt_card.0, instance_count));
                    instance_count += 1;
                    events.push(Event::CardDealt {
                        to: character_id,
                        card: dealt_card.clone(),
                        instance: instance_id,
                    });
                }
            }
            Effect::EndSession(outcome) => {
                events.push(Event::SessionEnded {
                    outcome: outcome.clone(),
                });
                ended = true;
            }
            Effect::ModifyStat { .. } => {
                events.push(Event::EffectApplied {
                    effect: effect.clone(),
                });
            }
        }
    }
    Ok(events)
}

fn decide_end_session(
    session: &Session,
    actor: &UserId,
    outcome: crate::primitives::Outcome,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_not_paused(session)?;
    check_gm(session, actor)?;
    Ok(vec![Event::SessionEnded { outcome }])
}

/// シーン入場+入場時配布(初期シーン入場・GotoScene効果の両方から共有)。
/// `existing_instance_count` はCardInstanceId発行の起点(domain-model.md参照)。
fn enter_scene(
    scenario: &Scenario,
    party: &[Character],
    existing_instance_count: usize,
    scene_id: &SceneId,
) -> Result<Vec<Event>, RuleError> {
    let scene_def = scenario
        .scene_def(scene_id)
        .ok_or(RuleError::SceneNotFound)?;
    let mut events = vec![Event::SceneEntered {
        scene: scene_id.clone(),
        narration: scene_def.narration.clone(),
    }];
    let mut next_seq = existing_instance_count;
    for deal in &scene_def.deals {
        for character_id in resolve_target(&deal.to, party) {
            let instance = CardInstanceId(format!("{}-{}", deal.card.0, next_seq));
            next_seq += 1;
            events.push(Event::CardDealt {
                to: character_id,
                card: deal.card.clone(),
                instance,
            });
        }
    }
    Ok(events)
}

fn resolve_target(target: &Target, party: &[Character]) -> Vec<CharacterId> {
    match target {
        Target::Party => party.iter().map(|c| c.id.clone()).collect(),
        Target::Character(id) => vec![id.clone()],
    }
}

fn next_phase(current: &Phase) -> Option<Phase> {
    match current {
        Phase::Opening => Some(Phase::Middle),
        Phase::Middle => Some(Phase::Climax),
        Phase::Climax => None,
    }
}

fn total_instance_count(session: &Session) -> usize {
    session.hands.values().map(|v| v.len()).sum::<usize>() + session.table.len()
}

fn check_not_ended(session: &Session) -> Result<(), RuleError> {
    if matches!(session.status, SessionStatus::Ended(_)) {
        Err(RuleError::SessionEnded)
    } else {
        Ok(())
    }
}

fn check_not_paused(session: &Session) -> Result<(), RuleError> {
    if matches!(session.status, SessionStatus::Paused { .. }) {
        Err(RuleError::SessionPaused)
    } else {
        Ok(())
    }
}

fn check_gm(session: &Session, actor: &UserId) -> Result<(), RuleError> {
    match session.roles.get(actor) {
        Some(Role::Gm) => Ok(()),
        _ => Err(RuleError::Forbidden),
    }
}

fn check_player_or_gm(
    session: &Session,
    actor: &UserId,
    character: &CharacterId,
) -> Result<(), RuleError> {
    match session.roles.get(actor) {
        Some(Role::Gm) => Ok(()),
        Some(Role::Player { characters }) if characters.contains(character) => Ok(()),
        _ => Err(RuleError::Forbidden),
    }
}

fn check_requires(
    session: &Session,
    character: &CharacterId,
    requires: &[Condition],
) -> Result<(), RuleError> {
    for cond in requires {
        let ok = match cond {
            Condition::HasCard(card_id) => {
                let in_hand = session
                    .hands
                    .get(character)
                    .is_some_and(|hand| hand.iter().any(|ci| &ci.card == card_id));
                let on_table = session.table.iter().any(|ci| &ci.card == card_id);
                in_hand || on_table
            }
            Condition::StatAtLeast(stat_id, min) => session
                .party
                .iter()
                .find(|c| &c.id == character)
                .and_then(|c| c.stats.get(stat_id))
                .is_some_and(|v| v >= min),
        };
        if !ok {
            return Err(RuleError::ConditionNotMet);
        }
    }
    Ok(())
}
