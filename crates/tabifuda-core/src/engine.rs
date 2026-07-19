//! decide/apply。docs/design/domain-model.md「進行の解決規則」に対応。
//! CLAUDE.md最重要ルール2・3(coreは純粋・進行は必ずイベント経由)を実装する中核。

use std::collections::HashMap;

use crate::actor::Role;
use crate::card::{CardDef, CardKind, Condition, Effect, Target};
use crate::character::Character;
use crate::command::Command;
use crate::error::RuleError;
use crate::event::{Event, RemovalReason};
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, UserId};
use crate::patch::{self, PatchOp, ScenarioPatch};
use crate::scenario::{Phase, Scenario};
use crate::session::{CardInstance, Proposal, ScenarioSnapshot, Session, SessionStatus};

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
        (Some(session), Command::Propose { by, text }) => decide_propose(session, actor, by, text),
        (Some(session), Command::ApplyPatch { patch }) => decide_apply_patch(session, actor, patch),
        (Some(session), Command::JudgeProposal { proposal, accepted }) => {
            decide_judge_proposal(session, actor, proposal, accepted)
        }
        (Some(session), Command::GmAdvance { to }) => decide_gm_advance(session, actor, to),
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
                proposal_seq: 0,
                card_instance_seq: 0,
                scene_local_instances: Vec::new(),
            })
        }
        (Some(session), event) => apply_to_existing(session, event),
        (None, _) => None,
    }
}

/// `ScenarioPatched`の適用はpatch::apply_opsが失敗しうる(型上)ため`Option`を返す。
/// decideが検証済みのパッチしか`ScenarioPatched`として出力しない不変条件の下では
/// 失敗経路には入らない(「decideの出力は必ずapply可能」)。
fn apply_to_existing(mut session: Session, event: &Event) -> Option<Session> {
    match event {
        Event::SessionStarted { .. } => {}
        Event::SceneEntered {
            scene,
            local_instances,
            ..
        } => {
            session.scene = scene.clone();
            session.scene_local_instances = local_instances.clone();
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
            session.card_instance_seq += 1;
        }
        Event::CardPlayed { .. } => {}
        Event::CardRemoved { from, instance, .. } => {
            if let Some(hand) = session.hands.get_mut(from) {
                hand.retain(|ci| &ci.id != instance);
            }
        }
        Event::EffectApplied { .. } => {}
        Event::PhaseAdvanced { phase } => {
            session.phase = phase.clone();
        }
        Event::ProposalSubmitted { id, by, text } => {
            session.status = SessionStatus::Paused {
                proposal: id.clone(),
            };
            session.pending_proposal = Some(Proposal {
                id: id.clone(),
                by: by.clone(),
                text: text.clone(),
            });
            session.proposal_seq += 1;
        }
        Event::ScenarioPatched { patch } => {
            patch::apply_ops(&mut session.scenario.0, &patch.ops).ok()?;
        }
        Event::ProposalJudged { .. } => {
            session.status = SessionStatus::Running;
            session.pending_proposal = None;
        }
        Event::SessionEnded { outcome } => {
            session.status = SessionStatus::Ended(outcome.clone());
        }
    }
    Some(session)
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
    if card_def.kind.is_consumed_on_play() {
        events.push(Event::CardRemoved {
            from: by.clone(),
            card: card_def.id.clone(),
            instance: instance.id.clone(),
            reason: RemovalReason::Consumed,
        });
    }

    let mut instance_count = session.card_instance_seq;
    let mut current_phase = session.phase.clone();
    let mut ended = false;
    for effect in &card_def.effects {
        if ended {
            break;
        }
        match effect {
            Effect::GotoScene(scene_id) => {
                events.extend(scene_cleanup_events(session, Some(&instance.id)));
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

fn decide_propose(
    session: &Session,
    actor: &UserId,
    by: CharacterId,
    text: crate::primitives::BoundedString<4096>,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_not_paused(session)?;
    check_player_or_gm(session, actor, &by)?;

    let id = ProposalId(format!("proposal-{}", session.proposal_seq));
    Ok(vec![Event::ProposalSubmitted { id, by, text }])
}

/// GM専用。Paused中のみ(domain-model.md「シナリオパッチ」節)。`patch::validate`を
/// 通ったパッチのみ`ScenarioPatched`として発行し、`PatchOp::DealCard`分は
/// `enter_scene`と同じ連番起点でその場の`CardDealt`を追加発行する(即時配布)。
fn decide_apply_patch(
    session: &Session,
    actor: &UserId,
    patch: ScenarioPatch,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_gm(session, actor)?;
    check_paused(session)?;
    patch::validate(session, &patch)?;

    let mut events = vec![Event::ScenarioPatched {
        patch: patch.clone(),
    }];
    let mut instance_count = session.card_instance_seq;
    for op in &patch.ops {
        if let PatchOp::DealCard { card, to } = op {
            for character_id in resolve_target(to, &session.party) {
                let instance = CardInstanceId(format!("{}-{}", card.0, instance_count));
                instance_count += 1;
                events.push(Event::CardDealt {
                    to: character_id,
                    card: card.clone(),
                    instance,
                });
            }
        }
    }
    Ok(events)
}

fn decide_judge_proposal(
    session: &Session,
    actor: &UserId,
    proposal: ProposalId,
    accepted: bool,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_gm(session, actor)?;
    match &session.status {
        SessionStatus::Paused { proposal: pending } if *pending == proposal => {
            Ok(vec![Event::ProposalJudged {
                id: proposal,
                accepted,
            }])
        }
        _ => Err(RuleError::ProposalNotFound),
    }
}

/// GM強制進行。カードのrequires/GotoScene遷移条件を経由せず直接シーンへ入場する
/// (`enter_scene`を共有)。状態機械図に載らない操作のため、Running/Paused双方で
/// 許可し、共通の拒否系(Ended)のみに従う(domain-model.md「GmAdvance(強制進行)」参照)。
fn decide_gm_advance(
    session: &Session,
    actor: &UserId,
    to: SceneId,
) -> Result<Vec<Event>, RuleError> {
    check_not_ended(session)?;
    check_gm(session, actor)?;
    let mut events = scene_cleanup_events(session, None);
    events.extend(enter_scene(
        &session.scenario.0,
        &session.party,
        session.card_instance_seq,
        &to,
    )?);
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
/// `scene_def.deals`から配ったCardInstanceIdを`SceneEntered.local_instances`へ
/// 詰める(カード効果由来の`DealCard`は含まない。domain-model.md
/// 「カードの消費・除去」参照)。
fn enter_scene(
    scenario: &Scenario,
    party: &[Character],
    existing_instance_count: usize,
    scene_id: &SceneId,
) -> Result<Vec<Event>, RuleError> {
    let scene_def = scenario
        .scene_def(scene_id)
        .ok_or(RuleError::SceneNotFound)?;

    let mut deal_events = Vec::new();
    let mut local_instances = Vec::new();
    let mut next_seq = existing_instance_count;
    for deal in &scene_def.deals {
        for character_id in resolve_target(&deal.to, party) {
            let instance = CardInstanceId(format!("{}-{}", deal.card.0, next_seq));
            next_seq += 1;
            local_instances.push(instance.clone());
            deal_events.push(Event::CardDealt {
                to: character_id,
                card: deal.card.clone(),
                instance,
            });
        }
    }

    let mut events = vec![Event::SceneEntered {
        scene: scene_id.clone(),
        narration: scene_def.narration.as_str().to_string(),
        local_instances,
    }];
    events.extend(deal_events);
    Ok(events)
}

/// シーンを離れる直前に呼ぶ。現在のシーンが配ったカード
/// (`session.scene_local_instances`)のうち、まだ手札にあり
/// `CardKind::Marker`ではないものを`CardRemoved{reason: SceneLeft}`として
/// 発行する(選ばなかった側の選択肢カードの自動消去。domain-model.md
/// 「カードの消費・除去」参照)。`exclude`は今出したカード自身のinstance
/// (`Consumed`で既に除去済みのため対象から外す)。
fn scene_cleanup_events(session: &Session, exclude: Option<&CardInstanceId>) -> Vec<Event> {
    session
        .scene_local_instances
        .iter()
        .filter(|id| exclude != Some(id))
        .filter_map(|id| {
            let (from, card) = find_in_hands(session, id)?;
            let kind = &session.scenario.0.card_def(&card)?.kind;
            if matches!(kind, CardKind::Marker) {
                return None;
            }
            Some(Event::CardRemoved {
                from,
                card,
                instance: id.clone(),
                reason: RemovalReason::SceneLeft,
            })
        })
        .collect()
}

/// 指定したCardInstanceIdを現在保持しているキャラクターとCardIdを引く。
fn find_in_hands(session: &Session, instance: &CardInstanceId) -> Option<(CharacterId, CardId)> {
    session.hands.iter().find_map(|(character_id, cards)| {
        cards
            .iter()
            .find(|ci| &ci.id == instance)
            .map(|ci| (character_id.clone(), ci.card.clone()))
    })
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

/// ApplyPatch専用。他コマンドの`check_not_paused`とは逆に、Paused**である**
/// ことを要求する(domain-model.md「シナリオパッチ」節「Paused中のみ」)。
fn check_paused(session: &Session) -> Result<(), RuleError> {
    if matches!(session.status, SessionStatus::Paused { .. }) {
        Ok(())
    } else {
        Err(RuleError::SessionNotPaused)
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
