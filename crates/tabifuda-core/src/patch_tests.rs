//! patch::validateの単体テスト。docs/design/domain-model.md「シナリオパッチ
//! (構造化)」節、test-strategy.md 不変条件5(パッチ安全性)に対応。
//!
//! v0.1のPatchOp(5種)に削除系操作が無いため、「現在シーンが存在する」不変条件は
//! 現状どのパッチからも壊せない(防御的チェックのみ。拒否テストはPatchOp拡充まで
//! 保留。projects/phase1/task.md C4着手時にユーザーへ確認済み)。「配布済みカードの定義が
//! 解決可能」は`DealCard`で直接再現できるため受理/拒否対で検証する。

use std::collections::HashMap;

use crate::card::{CardDef, CardKind, Condition, Target};
use crate::ids::{CardId, CharacterId, ProposalId, ScenarioId, SceneId};
use crate::patch::{validate, PatchError, PatchOp, ScenarioPatch};
use crate::primitives::BoundedString;
use crate::scenario::{Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind, Transition};
use crate::session::{CardInstance, ScenarioSnapshot, Session, SessionStatus};

fn cid(s: &str) -> CardId {
    CardId(s.to_string())
}
fn scn(s: &str) -> SceneId {
    SceneId(s.to_string())
}
fn chr(s: &str) -> CharacterId {
    CharacterId(s.to_string())
}

fn short(s: &str) -> BoundedString<200> {
    BoundedString::try_new(s).unwrap()
}
fn long(s: &str) -> BoundedString<2000> {
    BoundedString::try_new(s).unwrap()
}

fn card_def(id: &str) -> CardDef {
    CardDef {
        id: cid(id),
        name: short(id),
        kind: CardKind::Item,
        text: long(""),
        tags: vec![],
        effects: vec![],
        requires: vec![],
    }
}

fn scene(id: &str) -> SceneDef {
    SceneDef {
        id: scn(id),
        kind: SceneKind::Conversation,
        narration: long(""),
        deals: vec![],
        exits: vec![],
    }
}

fn note(s: &str) -> BoundedString<4096> {
    BoundedString::try_new(s).unwrap()
}

fn patch(ops: Vec<PatchOp>) -> ScenarioPatch {
    ScenarioPatch {
        ops,
        note: note("テストパッチ"),
    }
}

/// phases=[Opening: [s1], Middle: [s2]]、card_defs=[existing]、現在シーン=s1、
/// Paused(GmのApplyPatch前提だが、validate自体は状態に依らない)。
fn fixture_session() -> Session {
    let scenario = Scenario {
        meta: ScenarioMeta {
            id: ScenarioId("scenario1".to_string()),
            title: short(""),
            author: short(""),
            forked_from: None,
        },
        card_defs: vec![card_def("existing")],
        phases: vec![
            PhaseDef {
                phase: Phase::Opening,
                scenes: vec![scene("s1")],
            },
            PhaseDef {
                phase: Phase::Middle,
                scenes: vec![scene("s2")],
            },
        ],
    };
    Session {
        scenario: ScenarioSnapshot(scenario),
        party: vec![],
        status: SessionStatus::Paused {
            proposal: ProposalId("p1".to_string()),
        },
        roles: HashMap::new(),
        phase: Phase::Opening,
        scene: scn("s1"),
        hands: HashMap::new(),
        table: vec![],
        pending_proposal: None,
        proposal_seq: 0,
        card_instance_seq: 0,
        scene_local_instances: vec![],
    }
}

#[test]
fn validate_accepts_empty_patch() {
    let session = fixture_session();
    assert!(validate(&session, &patch(vec![])).is_ok());
}

// ---- AddCardDef ----

#[test]
fn validate_accepts_add_card_def_new_id() {
    let session = fixture_session();
    assert!(validate(&session, &patch(vec![PatchOp::AddCardDef(card_def("new"))])).is_ok());
}

#[test]
fn validate_rejects_add_card_def_duplicate() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddCardDef(card_def("existing"))]),
    );
    assert_eq!(result, Err(PatchError::DuplicateCardId));
}

// ---- ReplaceScene ----

#[test]
fn validate_accepts_replace_scene_existing_target() {
    let session = fixture_session();
    let mut replaced = scene("s1");
    replaced.narration = long("改訂後の描写");
    let result = validate(&session, &patch(vec![PatchOp::ReplaceScene(replaced)]));
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_replace_scene_missing_target() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::ReplaceScene(scene("nowhere"))]),
    );
    assert_eq!(result, Err(PatchError::SceneNotFound));
}

// ---- AddScene ----

#[test]
fn validate_accepts_add_scene_new_id_in_existing_phase() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddScene {
            phase: Phase::Opening,
            scene: scene("s1b"),
        }]),
    );
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_add_scene_duplicate_id() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddScene {
            phase: Phase::Middle,
            scene: scene("s1"), // Openingに既存
        }]),
    );
    assert_eq!(result, Err(PatchError::DuplicateSceneId));
}

#[test]
fn validate_rejects_add_scene_phase_not_found() {
    let session = fixture_session(); // Climaxのphaseは元から存在しない
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddScene {
            phase: Phase::Climax,
            scene: scene("s3"),
        }]),
    );
    assert_eq!(result, Err(PatchError::PhaseNotFound));
}

// ---- AddTransition ----

#[test]
fn validate_accepts_add_transition_existing_scenes() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddTransition {
            scene: scn("s1"),
            transition: Transition {
                condition: Condition::HasCard(cid("existing")),
                to: scn("s2"),
            },
        }]),
    );
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_add_transition_missing_owner_scene() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddTransition {
            scene: scn("nowhere"),
            transition: Transition {
                condition: Condition::HasCard(cid("existing")),
                to: scn("s2"),
            },
        }]),
    );
    assert_eq!(result, Err(PatchError::SceneNotFound));
}

#[test]
fn validate_rejects_add_transition_missing_target_scene() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::AddTransition {
            scene: scn("s1"),
            transition: Transition {
                condition: Condition::HasCard(cid("existing")),
                to: scn("nowhere"),
            },
        }]),
    );
    assert_eq!(result, Err(PatchError::SceneNotFound));
}

// ---- DealCard(「配布済みカードの定義が解決可能」の受理/拒否対) ----

#[test]
fn validate_accepts_deal_card_existing_def() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::DealCard {
            card: cid("existing"),
            to: Target::Party,
        }]),
    );
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_deal_card_missing_def() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![PatchOp::DealCard {
            card: cid("nowhere"),
            to: Target::Party,
        }]),
    );
    assert_eq!(result, Err(PatchError::CardNotFound));
}

#[test]
fn validate_accepts_deal_card_defined_earlier_in_same_patch() {
    let session = fixture_session();
    let result = validate(
        &session,
        &patch(vec![
            PatchOp::AddCardDef(card_def("brand_new")),
            PatchOp::DealCard {
                card: cid("brand_new"),
                to: Target::Party,
            },
        ]),
    );
    assert!(result.is_ok());
}

// ---- 複数opの逐次適用 ----

#[test]
fn validate_rejects_second_op_referencing_first_ops_duplicate() {
    let session = fixture_session();
    // 1つ目のopで追加したidに、2つ目のopが重複する。
    let result = validate(
        &session,
        &patch(vec![
            PatchOp::AddCardDef(card_def("brand_new")),
            PatchOp::AddCardDef(card_def("brand_new")),
        ]),
    );
    assert_eq!(result, Err(PatchError::DuplicateCardId));
}

#[test]
fn validate_ignores_unrelated_dealt_card_instances() {
    // 既存の手札が既存card_defを正しく参照している限り、無関係なパッチは影響しない。
    let mut session = fixture_session();
    session.hands.insert(
        chr("ch1"),
        vec![CardInstance {
            id: crate::ids::CardInstanceId("ci1".to_string()),
            card: cid("existing"),
        }],
    );
    let result = validate(&session, &patch(vec![PatchOp::AddCardDef(card_def("new"))]));
    assert!(result.is_ok());
}
