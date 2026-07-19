//! ゴールデンJSONテスト。永続化される主要enum/型のワイヤ形式(serdeデフォルトの
//! 外部タグ表現)を固定し、破壊的変更を検出する。冒険記の決定的リプレイのため、
//! シリアライズ形式は永続契約である(docs/design/test-strategy.md §1(d)、
//! docs/design/reviews/p1-c1-type-review.md H3 に対応)。
//!
//! roundtripテストは T→JSON→T の同一性しか見ないため、タプル→構造体バリアントへの
//! 整形のようなワイヤ形式の変更を検出できない。ここでは JSON 文字列そのものを固定する。

use crate::card::{CardDef, CardKind, Condition, Effect, Target};
use crate::error::RuleError;
use crate::event::Event;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, StatId};
use crate::patch::{PatchError, PatchOp, ScenarioPatch};
use crate::primitives::{BoundedString, Outcome};

fn short(s: &str) -> BoundedString<200> {
    BoundedString::try_new(s).unwrap()
}
fn long(s: &str) -> BoundedString<2000> {
    BoundedString::try_new(s).unwrap()
}
use crate::scenario::{Phase, SceneDef, SceneKind};
use crate::session::{CardInstance, Proposal, SessionStatus};

/// 値 → JSON文字列が固定表現と一致し、かつその文字列 → 値で往復することを検証する
/// (両方向を固定することで、シリアライズ・デシリアライズどちらの破壊も検出する)。
fn assert_golden<T>(value: T, expected_json: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(
        json, expected_json,
        "シリアライズ形式が固定表現から変化した"
    );
    let restored: T = serde_json::from_str(expected_json).unwrap();
    assert_eq!(
        restored, value,
        "固定表現からのデシリアライズが元の値に戻らない"
    );
}

fn card(id: &str) -> CardId {
    CardId(id.to_string())
}
fn scene(id: &str) -> SceneId {
    SceneId(id.to_string())
}
fn character(id: &str) -> CharacterId {
    CharacterId(id.to_string())
}
fn stat(id: &str) -> StatId {
    StatId(id.to_string())
}

#[test]
fn golden_target() {
    assert_golden(Target::Party, r#""Party""#);
    assert_golden(
        Target::Character(character("ch1")),
        r#"{"Character":"ch1"}"#,
    );
}

#[test]
fn golden_outcome() {
    assert_golden(Outcome::Victory, r#""Victory""#);
    assert_golden(Outcome::Defeat, r#""Defeat""#);
}

#[test]
fn golden_effect() {
    assert_golden(Effect::GotoScene(scene("s1")), r#"{"GotoScene":"s1"}"#);
    assert_golden(Effect::AdvancePhase, r#""AdvancePhase""#);
    assert_golden(
        Effect::DealCard {
            card: card("c1"),
            to: Target::Character(character("ch1")),
        },
        r#"{"DealCard":{"card":"c1","to":{"Character":"ch1"}}}"#,
    );
    assert_golden(
        Effect::ModifyStat {
            target: Target::Character(character("ch1")),
            stat: stat("hp"),
            delta: -3,
        },
        r#"{"ModifyStat":{"target":{"Character":"ch1"},"stat":"hp","delta":-3}}"#,
    );
    assert_golden(
        Effect::EndSession(Outcome::Victory),
        r#"{"EndSession":"Victory"}"#,
    );
}

#[test]
fn golden_condition() {
    assert_golden(Condition::HasCard(card("c1")), r#"{"HasCard":"c1"}"#);
    assert_golden(
        Condition::StatAtLeast(stat("hp"), 5),
        r#"{"StatAtLeast":["hp",5]}"#,
    );
}

#[test]
fn golden_session_status() {
    assert_golden(SessionStatus::Running, r#""Running""#);
    assert_golden(
        SessionStatus::Paused {
            proposal: ProposalId("p1".to_string()),
        },
        r#"{"Paused":{"proposal":"p1"}}"#,
    );
    assert_golden(
        SessionStatus::Ended(Outcome::Defeat),
        r#"{"Ended":"Defeat"}"#,
    );
}

#[test]
fn golden_card_instance() {
    assert_golden(
        CardInstance {
            id: crate::ids::CardInstanceId("ci1".to_string()),
            card: card("c1"),
        },
        r#"{"id":"ci1","card":"c1"}"#,
    );
}

#[test]
fn golden_rule_error() {
    assert_golden(RuleError::Forbidden, r#""Forbidden""#);
    assert_golden(RuleError::SessionPaused, r#""SessionPaused""#);
    assert_golden(RuleError::SessionNotPaused, r#""SessionNotPaused""#);
    assert_golden(RuleError::SceneNotFound, r#""SceneNotFound""#);
    assert_golden(RuleError::ProposalNotFound, r#""ProposalNotFound""#);
    assert_golden(
        RuleError::InvalidPatch(PatchError::DuplicateCardId),
        r#"{"InvalidPatch":"DuplicateCardId"}"#,
    );
}

#[test]
fn golden_patch_error() {
    assert_golden(PatchError::DuplicateCardId, r#""DuplicateCardId""#);
    assert_golden(PatchError::DuplicateSceneId, r#""DuplicateSceneId""#);
    assert_golden(PatchError::SceneNotFound, r#""SceneNotFound""#);
    assert_golden(PatchError::PhaseNotFound, r#""PhaseNotFound""#);
    assert_golden(PatchError::CardNotFound, r#""CardNotFound""#);
}

#[test]
fn golden_patch_op() {
    assert_golden(
        PatchOp::AddCardDef(CardDef {
            id: card("c1"),
            name: short("c1"),
            kind: CardKind::Item,
            text: long(""),
            tags: vec![],
            effects: vec![],
            requires: vec![],
        }),
        r#"{"AddCardDef":{"id":"c1","name":"c1","kind":"Item","text":"","tags":[],"effects":[],"requires":[]}}"#,
    );
    assert_golden(
        PatchOp::ReplaceScene(SceneDef {
            id: scene("s1"),
            kind: SceneKind::Conversation,
            narration: long("改訂後の描写"),
            deals: vec![],
            exits: vec![],
        }),
        r#"{"ReplaceScene":{"id":"s1","kind":"Conversation","narration":"改訂後の描写","deals":[],"exits":[]}}"#,
    );
    assert_golden(
        PatchOp::DealCard {
            card: card("c1"),
            to: Target::Party,
        },
        r#"{"DealCard":{"card":"c1","to":"Party"}}"#,
    );
}

#[test]
fn golden_scenario_patch() {
    assert_golden(
        ScenarioPatch {
            ops: vec![PatchOp::DealCard {
                card: card("c1"),
                to: Target::Party,
            }],
            note: BoundedString::<4096>::try_new("バランス調整".to_string()).unwrap(),
        },
        r#"{"ops":[{"DealCard":{"card":"c1","to":"Party"}}],"note":"バランス調整"}"#,
    );
}

#[test]
fn golden_proposal() {
    assert_golden(
        Proposal {
            id: ProposalId("proposal-0".to_string()),
            by: character("ch1"),
            text: BoundedString::<4096>::try_new("近道を探したい").unwrap(),
        },
        r#"{"id":"proposal-0","by":"ch1","text":"近道を探したい"}"#,
    );
}

#[test]
fn golden_event() {
    assert_golden(
        Event::SceneEntered {
            scene: scene("s1"),
            narration: "門が開いた".to_string(),
            local_instances: vec![CardInstanceId("c1-0".to_string())],
        },
        r#"{"SceneEntered":{"scene":"s1","narration":"門が開いた","local_instances":["c1-0"]}}"#,
    );
    assert_golden(
        Event::CardDealt {
            to: character("ch1"),
            card: card("c1"),
            instance: CardInstanceId("c1-0".to_string()),
        },
        r#"{"CardDealt":{"to":"ch1","card":"c1","instance":"c1-0"}}"#,
    );
    assert_golden(
        Event::CardPlayed {
            by: character("ch1"),
            card: card("c1"),
            free_text: None,
        },
        r#"{"CardPlayed":{"by":"ch1","card":"c1","free_text":null}}"#,
    );
    assert_golden(
        Event::CardRemoved {
            from: character("ch1"),
            card: card("c1"),
            instance: CardInstanceId("c1-0".to_string()),
            reason: crate::event::RemovalReason::Consumed,
        },
        r#"{"CardRemoved":{"from":"ch1","card":"c1","instance":"c1-0","reason":"Consumed"}}"#,
    );
    assert_golden(
        Event::PhaseAdvanced {
            phase: Phase::Middle,
        },
        r#"{"PhaseAdvanced":{"phase":"Middle"}}"#,
    );
    assert_golden(
        Event::ProposalSubmitted {
            id: ProposalId("proposal-0".to_string()),
            by: character("ch1"),
            text: BoundedString::<4096>::try_new("近道を探したい").unwrap(),
        },
        r#"{"ProposalSubmitted":{"id":"proposal-0","by":"ch1","text":"近道を探したい"}}"#,
    );
    assert_golden(
        Event::ScenarioPatched {
            patch: ScenarioPatch {
                ops: vec![PatchOp::DealCard {
                    card: card("c1"),
                    to: Target::Party,
                }],
                note: BoundedString::<4096>::try_new("バランス調整".to_string()).unwrap(),
            },
        },
        r#"{"ScenarioPatched":{"patch":{"ops":[{"DealCard":{"card":"c1","to":"Party"}}],"note":"バランス調整"}}}"#,
    );
    assert_golden(
        Event::ProposalJudged {
            id: ProposalId("proposal-0".to_string()),
            accepted: true,
        },
        r#"{"ProposalJudged":{"id":"proposal-0","accepted":true}}"#,
    );
    assert_golden(
        Event::SessionEnded {
            outcome: Outcome::Victory,
        },
        r#"{"SessionEnded":{"outcome":"Victory"}}"#,
    );
}
