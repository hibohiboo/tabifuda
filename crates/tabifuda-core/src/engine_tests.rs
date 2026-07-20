//! decide/applyのテーブル駆動テスト。docs/design/test-strategy.md §1(a)に対応。
//! 各Commandについて受理/拒否を対で書く。シナリオはテスト用の最小構成。

use std::collections::HashMap;

use crate::card::{CardDef, CardKind, Condition, Effect, Target};
use crate::character::Character;
use crate::command::Command;
use crate::engine::{apply, decide};
use crate::error::RuleError;
use crate::event::{Event, RemovalReason};
use crate::ids::{
    CardId, CardInstanceId, CharacterId, ProposalId, ScenarioId, SceneId, StatId, UserId,
};
use crate::patch::{PatchError, PatchOp, ScenarioPatch};
use crate::primitives::{BoundedString, Outcome};
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
fn short(s: &str) -> BoundedString<200> {
    BoundedString::try_new(s).unwrap()
}
fn long(s: &str) -> BoundedString<2000> {
    BoundedString::try_new(s).unwrap()
}

fn card_def(id: &str, kind: CardKind, effects: Vec<Effect>, requires: Vec<Condition>) -> CardDef {
    CardDef {
        id: cid(id),
        name: short(id),
        kind,
        text: long(""),
        tags: vec![],
        effects,
        requires,
    }
}

fn scene(id: &str, deals: Vec<Deal>) -> SceneDef {
    SceneDef {
        id: scn(id),
        kind: SceneKind::Conversation,
        narration: long(&format!("{id}の描写")),
        deals,
        exits: vec![],
    }
}

/// テスト用の最小シナリオ。カード定義は各Effectの解決経路を1つずつ確認できる分だけ用意する。
fn fixture_scenario() -> Scenario {
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("scenario1".to_string()),
            title: short("テスト用シナリオ"),
            author: short("test"),
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
        card_instance_seq: 1,
        scene_local_instances: vec![],
    }
}

// ---- StartSession ----

#[test]
fn StartSessionは受理され初期シーンへ入場する() {
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
fn StartSessionは初期シーンのdealsでカードを配布する() {
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
fn StartSessionは開始済みセッションでは拒否される() {
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
fn StartSessionはシーンが無いシナリオでは拒否される() {
    let scenario = Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("empty".to_string()),
            title: short(""),
            author: short(""),
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
fn セッション未開始時は各種コマンドが拒否される() {
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
fn PlayCardは担当プレイヤーには受理される() {
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
fn PlayCardは未登録アクターでは拒否される() {
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
fn PlayCardは他キャラ担当プレイヤーでは拒否される() {
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
fn PlayCardはGMなら任意キャラの代理で受理される() {
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
fn PlayCardはPaused中は拒否される() {
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
fn PlayCardはEnded後は拒否される() {
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
fn PlayCardは手札に無いカードでは拒否される() {
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
fn PlayCardは条件未達では拒否される() {
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
fn PlayCardはtable上のカードでHasCard条件を満たせば受理される() {
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
fn PlayCardはStatAtLeast未達では拒否される() {
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
fn PlayCardはStatAtLeast達成で受理される() {
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
fn PlayCardはGotoScene効果でシーン遷移する() {
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
                narration: "s2の描写".to_string(),
                local_instances: vec![]
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
fn PlayCardは存在しないシーンへのGotoScene効果は拒否される() {
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
fn PlayCardはDealCard効果でPartyにカードを配る() {
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
fn PlayCardはAdvancePhase効果で次フェーズへ進む() {
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
fn PlayCardはClimaxでのAdvancePhase効果は拒否される() {
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
fn PlayCardはEndSession効果でセッションを終了する() {
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
            Event::CardRemoved {
                from: chr("ch1"),
                card: cid("victory"),
                instance: inst("ci1"),
                reason: crate::event::RemovalReason::Consumed,
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
fn PlayCardはModifyStat効果をEffectAppliedとして記録し状態は変更しない() {
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
fn Proposeは担当プレイヤーに受理されPausedへ遷移する() {
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
fn ProposeはGMなら任意キャラの代理で受理される() {
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
fn Proposeは他キャラ担当プレイヤーでは拒否される() {
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
fn Proposeは既にPaused中は拒否される() {
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
fn ProposeはEnded後は拒否される() {
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
fn JudgeProposalは却下でもRunningに戻る() {
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
fn JudgeProposalは採用でRunningに戻る() {
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
fn JudgeProposalはPlayerでは拒否される() {
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
fn JudgeProposalは提案が無い時は拒否される() {
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
fn JudgeProposalはIDが一致しない提案では拒否される() {
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
fn JudgeProposalはEnded後は拒否される() {
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
fn GmAdvanceはGMに受理されシーンへ入場する() {
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
            local_instances: vec![],
        }]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    assert_eq!(state.unwrap().scene, scn("s2"));
}

#[test]
fn GmAdvanceはPaused中でも許可される() {
    let session = paused_session("p1");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("s2") },
    );
    assert!(result.is_ok());
}

#[test]
fn GmAdvanceはPlayerでは拒否される() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::GmAdvance { to: scn("s2") },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn GmAdvanceは存在しないシーンでは拒否される() {
    let session = fixture_session("advance");
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("nowhere") },
    );
    assert_eq!(result, Err(RuleError::SceneNotFound));
}

#[test]
fn GmAdvanceはEnded後は拒否される() {
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
fn 状態機械はRunningからProposeでPausedへ却下でRunningへ戻る() {
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
fn 状態機械はRunningからProposeでPausedへ採用でRunningへ戻る() {
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

// ---- ApplyPatch ----

fn scenario_patch(ops: Vec<PatchOp>) -> ScenarioPatch {
    ScenarioPatch {
        ops,
        note: text("GMメモ"),
    }
}

fn paused_for_patch() -> Session {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Paused {
        proposal: ProposalId("p1".to_string()),
    };
    session
}

#[test]
fn ApplyPatchはPaused中のGMに受理されAddCardDefを反映する() {
    let session = paused_for_patch();
    let new_card = card_def("brand_new", CardKind::Item, vec![], vec![]);
    let patch = scenario_patch(vec![PatchOp::AddCardDef(new_card)]);

    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: patch.clone(),
        },
    )
    .unwrap();
    assert_eq!(events, vec![Event::ScenarioPatched { patch }]);

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert!(session.scenario.0.card_def(&cid("brand_new")).is_some());
}

#[test]
fn ApplyPatchのDealCardは即座にカードを配布する() {
    let session = paused_for_patch();
    let patch = scenario_patch(vec![PatchOp::DealCard {
        card: cid("advance"),
        to: Target::Party,
    }]);

    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: patch.clone(),
        },
    )
    .unwrap();
    assert_eq!(events[0], Event::ScenarioPatched { patch });
    let dealt: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, Event::CardDealt { .. }))
        .collect();
    assert_eq!(dealt.len(), 2); // ch1, ch2

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    assert_eq!(session.hands[&chr("ch1")].len(), 2); // 元のadvance + 新規配布
    assert_eq!(session.hands[&chr("ch2")].len(), 1);
}

#[test]
fn ApplyPatchはPlayerでは拒否される() {
    let session = paused_for_patch();
    let result = decide(
        Some(&session),
        &usr("u1"),
        Command::ApplyPatch {
            patch: scenario_patch(vec![]),
        },
    );
    assert_eq!(result, Err(RuleError::Forbidden));
}

#[test]
fn ApplyPatchはRunning中は拒否される() {
    let session = fixture_session("advance"); // 既定でRunning
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: scenario_patch(vec![]),
        },
    );
    assert_eq!(result, Err(RuleError::SessionNotPaused));
}

#[test]
fn ApplyPatchはEnded後は拒否される() {
    let mut session = fixture_session("advance");
    session.status = SessionStatus::Ended(Outcome::Victory);
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: scenario_patch(vec![]),
        },
    );
    assert_eq!(result, Err(RuleError::SessionEnded));
}

#[test]
fn ApplyPatchはvalidate不合格のパッチでは拒否される() {
    let session = paused_for_patch();
    let duplicate = card_def("advance", CardKind::Action, vec![], vec![]);
    let result = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: scenario_patch(vec![PatchOp::AddCardDef(duplicate)]),
        },
    );
    assert_eq!(
        result,
        Err(RuleError::InvalidPatch(PatchError::DuplicateCardId))
    );
}

// ---- 状態機械: Propose→Paused→ApplyPatch(0回以上)→JudgeProposal(採用)→Running ----

#[test]
fn 状態機械はPaused中にApplyPatchを挟んでもJudgeProposal採用でRunningへ戻る() {
    let session = fixture_session("advance");
    let propose_events = decide(
        Some(&session),
        &usr("u1"),
        Command::Propose {
            by: chr("ch1"),
            text: text("近道を提案"),
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

    let patch_events = decide(
        Some(&session),
        &usr("gm1"),
        Command::ApplyPatch {
            patch: scenario_patch(vec![PatchOp::AddCardDef(card_def(
                "brand_new",
                CardKind::Item,
                vec![],
                vec![],
            ))]),
        },
    )
    .unwrap();
    let mut state = Some(session);
    for event in &patch_events {
        state = apply(state, event);
    }
    let session = state.unwrap();
    // パッチ適用中もPausedのまま(JudgeProposalとは独立)。
    assert!(matches!(session.status, SessionStatus::Paused { .. }));

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
    let session = state.unwrap();
    assert_eq!(session.status, SessionStatus::Running);
    assert!(session.scenario.0.card_def(&cid("brand_new")).is_some());
}

// ---- EndSession ----

#[test]
fn EndSessionはGMに受理される() {
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
fn EndSessionはPlayerでは拒否される() {
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
fn EndSessionはPaused中は拒否される() {
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
fn EndSessionは既にEnded後は拒否される() {
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

// ---- カードの消費・除去(CardRemoved) ----
// domain-model.md「カードの消費・除去」参照。simple-hunt.jsonの
// climax_battle(victory/defeat両方Scenario配布、片方だけ選ぶ)と同じ形の
// 最小シナリオで検証する。

/// s1がchosen(Scenario)/unchosen(Scenario)/marker_local(Marker)の3枚を配る。
/// chosenの効果はGotoScene(s2)。
fn removal_test_scenario() -> Scenario {
    Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("removal-test".to_string()),
            title: short(""),
            author: short(""),
            forked_from: None,
        },
        card_defs: vec![
            card_def(
                "chosen",
                CardKind::Scenario,
                vec![Effect::GotoScene(scn("s2"))],
                vec![],
            ),
            card_def("unchosen", CardKind::Scenario, vec![], vec![]),
            card_def("marker_local", CardKind::Marker, vec![], vec![]),
        ],
        phases: vec![
            PhaseDef {
                phase: Phase::Opening,
                scenes: vec![scene(
                    "s1",
                    vec![
                        Deal {
                            card: cid("chosen"),
                            to: Target::Party,
                        },
                        Deal {
                            card: cid("unchosen"),
                            to: Target::Party,
                        },
                        Deal {
                            card: cid("marker_local"),
                            to: Target::Party,
                        },
                    ],
                )],
            },
            PhaseDef {
                phase: Phase::Middle,
                scenes: vec![scene("s2", vec![])],
            },
        ],
    }
}

fn start_removal_test_session() -> Session {
    let scenario = removal_test_scenario();
    let party = vec![Character {
        id: chr("ch1"),
        name: "ch1".to_string(),
        stats: HashMap::new(),
        deck: vec![],
    }];
    let events = decide(None, &usr("gm1"), Command::StartSession { scenario, party }).unwrap();
    let mut state = None;
    for event in &events {
        state = apply(state, event);
    }
    state.unwrap()
}

fn find_instance(session: &Session, character: &CharacterId, card: &CardId) -> CardInstanceId {
    session.hands[character]
        .iter()
        .find(|ci| &ci.card == card)
        .unwrap()
        .id
        .clone()
}

#[test]
fn PlayCardのGotoScene遷移は使用カードを消費し選ばなかった同室カードを除去する() {
    let session = start_removal_test_session();
    let chosen = find_instance(&session, &chr("ch1"), &cid("chosen"));
    let unchosen = find_instance(&session, &chr("ch1"), &cid("unchosen"));
    let marker = find_instance(&session, &chr("ch1"), &cid("marker_local"));

    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::PlayCard {
            by: chr("ch1"),
            card: chosen.clone(),
            free_text: None,
        },
    )
    .unwrap();

    let removed: Vec<(CardInstanceId, RemovalReason)> = events
        .iter()
        .filter_map(|e| match e {
            Event::CardRemoved {
                instance, reason, ..
            } => Some((instance.clone(), reason.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(
        removed,
        vec![
            (chosen, RemovalReason::Consumed),
            (unchosen.clone(), RemovalReason::SceneLeft),
        ]
    );

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let hand = &state.unwrap().hands[&chr("ch1")];
    assert!(
        !hand.iter().any(|ci| ci.id == unchosen),
        "選ばなかった側は消えるはず"
    );
    assert!(hand.iter().any(|ci| ci.id == marker), "Markerは残るはず");
}

#[test]
fn GmAdvanceは未使用のシーン限定Scenarioカードを除去しMarkerは残す() {
    let session = start_removal_test_session();
    let chosen = find_instance(&session, &chr("ch1"), &cid("chosen"));
    let unchosen = find_instance(&session, &chr("ch1"), &cid("unchosen"));
    let marker = find_instance(&session, &chr("ch1"), &cid("marker_local"));

    let events = decide(
        Some(&session),
        &usr("gm1"),
        Command::GmAdvance { to: scn("s2") },
    )
    .unwrap();

    let removed: Vec<CardInstanceId> = events
        .iter()
        .filter_map(|e| match e {
            Event::CardRemoved { instance, .. } => Some(instance.clone()),
            _ => None,
        })
        .collect();
    // GmAdvanceは何も出さないので、まだ手札にあるScenario2枚とも対象になる。
    assert_eq!(removed, vec![chosen.clone(), unchosen.clone()]);

    let mut state = Some(session);
    for event in &events {
        state = apply(state, event);
    }
    let hand = &state.unwrap().hands[&chr("ch1")];
    assert!(!hand.iter().any(|ci| ci.id == chosen));
    assert!(!hand.iter().any(|ci| ci.id == unchosen));
    assert!(hand.iter().any(|ci| ci.id == marker), "Markerは残るはず");
}
