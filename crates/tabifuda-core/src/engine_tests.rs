//! decide/applyのテーブル駆動テスト。docs/design/test-strategy.md §1(a)に対応。
//! 各Commandについて受理/拒否を対で書く。シナリオはテスト用の最小構成
//! (phase1-task.md C2の指示通り、テンプレシナリオはP2で別途作成)。

use std::collections::HashMap;

use crate::card::{CardDef, CardKind, Condition, Effect, Target};
use crate::character::Character;
use crate::command::Command;
use crate::engine::{apply, decide};
use crate::error::RuleError;
use crate::event::Event;
use crate::ids::{CardId, CardInstanceId, CharacterId, ScenarioId, SceneId, StatId, UserId};
use crate::primitives::Outcome;
use crate::scenario::{Deal, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind};
use crate::session::{CardInstance, ScenarioSnapshot, Session, SessionStatus};
use crate::Role;

fn cid(s: &str) -> CardId {
    CardId(s.to_string())
}
fn scn(s: &str) -> SceneId {
    SceneId(s.to_string())
}
fn chr(s: &str) -> CharacterId {
    CharacterId(s.to_string())
}
fn usr(s: &str) -> UserId {
    UserId(s.to_string())
}
fn stat(s: &str) -> StatId {
    StatId(s.to_string())
}
fn inst(s: &str) -> CardInstanceId {
    CardInstanceId(s.to_string())
}

fn card_def(id: &str, kind: CardKind, effects: Vec<Effect>, requires: Vec<Condition>) -> CardDef {
    CardDef {
        id: cid(id),
        name: id.to_string(),
        kind,
        text: String::new(),
        tags: vec![],
        effects,
        requires,
    }
}

fn scene(id: &str, deals: Vec<Deal>) -> SceneDef {
    SceneDef {
        id: scn(id),
        kind: SceneKind::Conversation,
        narration: format!("{id}の描写"),
        deals,
        exits: vec![],
    }
}

/// テスト用の最小シナリオ。カード定義は各Effectの解決経路を1つずつ確認できる分だけ用意する。
fn fixture_scenario() -> Scenario {
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("scenario1".to_string()),
            title: "テスト用シナリオ".to_string(),
            author: "test".to_string(),
            forked_from: None,
        },
        card_defs: vec![
            card_def(
                "advance",
                CardKind::Action,
                vec![Effect::GotoScene(scn("s2"))],
                vec![],
            ),
            card_def(
                "bad_goto",
                CardKind::Action,
                vec![Effect::GotoScene(scn("nowhere"))],
                vec![],
            ),
            card_def(
                "give",
                CardKind::Action,
                vec![Effect::DealCard {
                    card: cid("potion"),
                    to: Target::Party,
                }],
                vec![],
            ),
            card_def(
                "next_phase",
                CardKind::Action,
                vec![Effect::AdvancePhase],
                vec![],
            ),
            card_def(
                "victory",
                CardKind::Scenario,
                vec![Effect::EndSession(Outcome::Victory)],
                vec![],
            ),
            card_def(
                "hit",
                CardKind::Action,
                vec![Effect::ModifyStat {
                    target: Target::Character(chr("ch1")),
                    stat: stat("hp"),
                    delta: -3,
                }],
                vec![],
            ),
            card_def(
                "need_key",
                CardKind::Action,
                vec![],
                vec![Condition::HasCard(cid("key"))],
            ),
            card_def(
                "need_hp",
                CardKind::Action,
                vec![],
                vec![Condition::StatAtLeast(stat("hp"), 5)],
            ),
        ],
        phases: vec![
            PhaseDef {
                phase: Phase::Opening,
                scenes: vec![scene("s1", vec![])],
            },
            PhaseDef {
                phase: Phase::Middle,
                scenes: vec![scene("s2", vec![])],
            },
            PhaseDef {
                phase: Phase::Climax,
                scenes: vec![scene("s3", vec![])],
            },
        ],
    }
}

fn fixture_party() -> Vec<Character> {
    let mut ch1_stats = HashMap::new();
    ch1_stats.insert(stat("hp"), 5);
    vec![
        Character {
            id: chr("ch1"),
            name: "ch1".to_string(),
            stats: ch1_stats,
            deck: vec![],
        },
        Character {
            id: chr("ch2"),
            name: "ch2".to_string(),
            stats: HashMap::new(),
            deck: vec![],
        },
    ]
}

/// `hands["ch1"]` に指定カード定義のインスタンスを1枚持たせた実行中セッション。
fn fixture_session(hand_card: &str) -> Session {
    let mut roles = HashMap::new();
    roles.insert(
        usr("u1"),
        Role::Player {
            characters: vec![chr("ch1")],
        },
    );
    roles.insert(usr("gm1"), Role::Gm);

    let mut hands = HashMap::new();
    hands.insert(
        chr("ch1"),
        vec![CardInstance {
            id: inst("ci1"),
            card: cid(hand_card),
        }],
    );
    hands.insert(chr("ch2"), vec![]);

    Session {
        scenario: ScenarioSnapshot(fixture_scenario()),
        party: fixture_party(),
        status: SessionStatus::Running,
        roles,
        phase: Phase::Opening,
        scene: scn("s1"),
        hands,
        table: vec![],
        pending_proposal: None,
        proposal_seq: 0,
    }
}

// ---- StartSession ----

#[test]
fn start_session_accepts_and_enters_initial_scene() {
    let scenario = fixture_scenario();
    let party = fixture_party();
    let actor = usr("gm1");

    let events = decide(None, &actor, Command::StartSession { scenario, party }).unwrap();

    assert!(matches!(events[0], Event::SessionStarted { .. }));
    assert!(matches!(
        events[1],
        Event::SceneEntered { ref scene, .. } if *scene == scn("s1")
    ));

    let mut state = None;
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.roles.get(&actor), Some(&Role::Gm));
    assert_eq!(session.phase, Phase::Opening);
    assert_eq!(session.scene, scn("s1"));
    assert_eq!(session.status, SessionStatus::Running);
}

#[test]
fn start_session_deals_initial_scene_cards() {
    let mut scenario = fixture_scenario();
    scenario.phases[0].scenes[0].deals = vec![Deal {
        card: cid("advance"),
        to: Target::Party,
    }];
    let party = fixture_party();
    let actor = usr("gm1");

    let events = decide(None, &actor, Command::StartSession { scenario, party }).unwrap();
    let mut state = None;
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.hands[&chr("ch1")].len(), 1);
    assert_eq!(session.hands[&chr("ch2")].len(), 1);
    // Party解決は宣言順。連番は既存カード総数(0)起点で振られ、重複しない。
    assert_ne!(
        session.hands[&chr("ch1")][0].id,
        session.hands[&chr("ch2")][0].id
    );
}

#[test]
fn start_session_rejects_when_already_started() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::StartSession {
            scenario: fixture_scenario(),
            party: fixture_party(),
        },
    );
    assert_eq!(result, Err(RuleError::SessionAlreadyStarted));
}

#[test]
fn start_session_rejects_scenario_without_scenes() {
    let scenario = Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("empty".to_string()),
            title: String::new(),
            author: String::new(),
            forked_from: None,
        },
        card_defs: vec![],
        phases: vec![],
    };
    let result = decide(
        None,
        &usr("gm1"),
        Command::StartSession {
            scenario,
            party: fixture_party(),
        },
    );
    assert_eq!(result, Err(RuleError::ScenarioHasNoScenes));
}

// ---- 状態が無い場合の共通拒否 ----

#[test]
fn commands_reject_when_no_active_session() {
    let result = decide(
        None,
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::NoActiveSession));

    let result = decide(
        None,
        &usr("gm1"),
        Command::EndSession {
            outcome: Outcome::Victory,
        },
    );
    assert_eq!(result, Err(RuleError::NoActiveSession));
}

// ---- PlayCard: 権限 ----

#[test]
fn play_card_accepts_for_owning_player() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert!(result.is_ok());
}

#[test]
fn play_card_rejects_for_unregistered_actor() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("stranger"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn play_card_rejects_for_player_of_other_character() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch2"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn play_card_accepts_for_gm_on_behalf_of_any_character() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert!(result.is_ok());
}

// ---- PlayCard: セッション状態 ----

#[test]
fn play_card_rejects_when_paused() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Paused {
        proposal: crate::ids::ProposalId("p1".to_string()),
    };
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::SessionPaused));
}

#[test]
fn play_card_rejects_when_ended() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}

// ---- PlayCard: カード解決 ----

#[test]
fn play_card_rejects_when_card_not_in_hand() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("not-here"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::CardNotFound));
}

#[test]
fn play_card_rejects_condition_not_met() {
    let session = fixture_session("need_key");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::ConditionNotMet));
}

#[test]
fn play_card_accepts_has_card_condition_satisfied_via_table() {
    let mut session = fixture_session("need_key");
    session.table.push(CardInstance {
        id: inst("key-0"),
        card: cid("key"),
    });
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert!(result.is_ok());
}

#[test]
fn play_card_rejects_stat_at_least_not_met() {
    let mut session = fixture_session("need_hp");
    session.party[0].stats.insert(stat("hp"), 4);
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::ConditionNotMet));
}

#[test]
fn play_card_accepts_stat_at_least_met() {
    let session = fixture_session("need_hp"); // ch1.hp == 5
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert!(result.is_ok());
}

// ---- Effect解決 ----

#[test]
fn play_card_resolves_goto_scene() {
    let session = fixture_session("advance");
    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![
            Event::CardPlayed {
                by: chr("ch1"),
                card: cid("advance"),
                free_text: None
            },
            Event::SceneEntered {
                scene: scn("s2"),
                narration: "s2の描写".to_string()
            },
        ]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().scene, scn("s2"));
}

#[test]
fn play_card_rejects_goto_scene_to_missing_scene() {
    let session = fixture_session("bad_goto");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::SceneNotFound));
}

#[test]
fn play_card_resolves_deal_card_to_party() {
    let session = fixture_session("give");
    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    )
    .unwrap();

    let dealt: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, Event::CardDealt { .. }))
        .collect();
    assert_eq!(dealt.len(), 2); // ch1, ch2の2人

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.hands[&chr("ch1")].len(), 2); // 元のadvance + potion
    assert_eq!(session.hands[&chr("ch2")].len(), 1);
}

#[test]
fn play_card_resolves_advance_phase() {
    let session = fixture_session("next_phase");
    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![
            Event::CardPlayed {
                by: chr("ch1"),
                card: cid("next_phase"),
                free_text: None
            },
            Event::PhaseAdvanced {
                phase: Phase::Middle
            },
        ]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().phase, Phase::Middle);
}

#[test]
fn play_card_rejects_advance_phase_at_climax() {
    let mut session = fixture_session("next_phase");
    session.phase = Phase::Climax;
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    );
    assert_eq!(result, Err(RuleError::NoNextPhase));
}

#[test]
fn play_card_resolves_end_session_effect() {
    let session = fixture_session("victory");
    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![
            Event::CardPlayed {
                by: chr("ch1"),
                card: cid("victory"),
                free_text: None
            },
            Event::SessionEnded {
                outcome: Outcome::Victory
            },
        ]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(
        state.unwrap().status,
        SessionStatus::Ended(Outcome::Victory)
    );
}

#[test]
fn play_card_records_modify_stat_as_effect_applied_without_mutating_state() {
    let session = fixture_session("hit");
    let stats_before = session.party[0].stats.clone();

    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: inst("ci1"),
            free_text: None,
        },
    )
    .unwrap();
    assert!(matches!(
        events[1],
        Event::EffectApplied {
            effect: Effect::ModifyStat { .. }
        }
    ));

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    // ModifyStatの数値解決はC2スコープ外。stateは変化しない。
    assert_eq!(state.unwrap().party[0].stats, stats_before);
}

// ---- Propose ----

fn text(s: &str) -> crate::primitives::BoundedString<4096> {
    crate::primitives::BoundedString::try_new(s).unwrap()
}

#[test]
fn propose_accepts_for_owning_player_and_transitions_to_paused() {
    let session = fixture_session("advance");
    let events = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("もっと近道を探したい"),
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![Event::ProposalSubmitted {
            id: crate::ids::ProposalId("proposal-0".to_string()),
            by: chr("ch1"),
            text: text("もっと近道を探したい"),
        }]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(
        session.status,
        SessionStatus::Paused {
            proposal: crate::ids::ProposalId("proposal-0".to_string())
        }
    );
    assert_eq!(session.pending_proposal.unwrap().by, chr("ch1"));
    assert_eq!(session.proposal_seq, 1);
}

#[test]
fn propose_accepts_for_gm_on_behalf_of_any_character() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("提案"),
        },
    );
    assert!(result.is_ok());
}

#[test]
fn propose_rejects_for_player_of_other_character() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch2"),
            text: text("提案"),
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn propose_rejects_when_paused() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Paused {
        proposal: crate::ids::ProposalId("p1".to_string()),
    };
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("提案"),
        },
    );
    assert_eq!(result, Err(RuleError::SessionPaused));
}

#[test]
fn propose_rejects_when_ended() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("提案"),
        },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}

// ---- JudgeProposal ----

fn paused_session(proposal_id: &str) -> Session {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Paused {
        proposal: crate::ids::ProposalId(proposal_id.to_string()),
    };
    session.pending_proposal = Some(crate::session::Proposal {
        id: crate::ids::ProposalId(proposal_id.to_string()),
        by: chr("ch1"),
        text: text("提案"),
    });
    session
}

#[test]
fn judge_proposal_rejected_returns_to_running() {
    let session = paused_session("p1");
    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("p1".to_string()),
            accepted: false,
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![Event::ProposalJudged {
            id: crate::ids::ProposalId("p1".to_string()),
            accepted: false,
        }]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.status, SessionStatus::Running);
    assert!(session.pending_proposal.is_none());
}

#[test]
fn judge_proposal_accepted_returns_to_running() {
    let session = paused_session("p1");
    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("p1".to_string()),
            accepted: true,
        },
    )
    .unwrap();

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.status, SessionStatus::Running);
    assert!(session.pending_proposal.is_none());
}

#[test]
fn judge_proposal_rejects_for_player() {
    let session = paused_session("p1");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("p1".to_string()),
            accepted: true,
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn judge_proposal_rejects_when_no_pending_proposal() {
    let session = fixture_session("advance"); // Running、pending無し
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("p1".to_string()),
            accepted: true,
        },
    );
    assert_eq!(result, Err(RuleError::ProposalNotFound));
}

#[test]
fn judge_proposal_rejects_mismatched_id() {
    let session = paused_session("p1");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("other".to_string()),
            accepted: true,
        },
    );
    assert_eq!(result, Err(RuleError::ProposalNotFound));
}

#[test]
fn judge_proposal_rejects_when_ended() {
    let mut session = paused_session("p1");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal: crate::ids::ProposalId("p1".to_string()),
            accepted: true,
        },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}

// ---- GmAdvance ----

#[test]
fn gm_advance_accepts_for_gm_and_enters_scene() {
    let session = fixture_session("advance");
    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("s2") },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![Event::SceneEntered {
            scene: scn("s2"),
            narration: "s2の描写".to_string(),
        }]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().scene, scn("s2"));
}

#[test]
fn gm_advance_allowed_while_paused() {
    let session = paused_session("p1");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("s2") },
    );
    assert!(result.is_ok());
}

#[test]
fn gm_advance_rejects_for_player() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::GmAdvance { to: scn("s2") },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn gm_advance_rejects_missing_scene() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("nowhere") },
    );
    assert_eq!(result, Err(RuleError::SceneNotFound));
}

#[test]
fn gm_advance_rejects_when_ended() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("s2") },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}

// ---- 状態機械: 全遷移の通し ----

#[test]
fn state_machine_running_propose_paused_judge_reject_running() {
    let session = fixture_session("advance");
    assert_eq!(session.status, SessionStatus::Running);

    let propose_events = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("提案"),
        },
    )
    .unwrap();
    let mut state = Some(session);
    for event in &propose_events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert!(matches!(session.status, SessionStatus::Paused { .. }));

    let SessionStatus::Paused { proposal } = session.status.clone() else {
        unreachable!()
    };
    let judge_events = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal,
            accepted: false,
        },
    )
    .unwrap();
    let mut state = Some(session);
    for event in &judge_events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().status, SessionStatus::Running);
}

#[test]
fn state_machine_running_propose_paused_judge_accept_running() {
    let session = fixture_session("advance");
    let propose_events = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("提案"),
        },
    )
    .unwrap();
    let mut state = Some(session);
    for event in &propose_events {
        state = apply(state, event);
    }
    let session = state.unwrap();

    let SessionStatus::Paused { proposal } = session.status.clone() else {
        unreachable!()
    };
    let judge_events = decide(
        Some(&session),
        &usr("gm1"),
        Command::JudgeProposal {
            proposal,
            accepted: true,
        },
    )
    .unwrap();
    let mut state = Some(session);
    for event in &judge_events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().status, SessionStatus::Running);
}

// ---- EndSession ----

#[test]
fn end_session_accepts_for_gm() {
    let session = fixture_session("advance");
    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::EndSession {
            outcome: Outcome::Defeat,
        },
    )
    .unwrap();
    assert_eq!(
        events,
        vec![Event::SessionEnded {
            outcome: Outcome::Defeat
        }]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().status, SessionStatus::Ended(Outcome::Defeat));
}

#[test]
fn end_session_rejects_for_player() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::EndSession {
            outcome: Outcome::Defeat,
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn end_session_rejects_when_paused() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Paused {
        proposal: crate::ids::ProposalId("p1".to_string()),
    };
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::EndSession {
            outcome: Outcome::Defeat,
        },
    );
    assert_eq!(result, Err(RuleError::SessionPaused));
}

#[test]
fn end_session_rejects_when_already_ended() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::EndSession {
            outcome: Outcome::Defeat,
        },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}
