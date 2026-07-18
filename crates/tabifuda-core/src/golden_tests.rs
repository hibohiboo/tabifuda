//! ゴールデンJSONテスト。永続化される主要enum/型のワイヤ形式(serdeデフォルトの
//! 外部タグ表現)を固定し、破壊的変更を検出する。冒険記の決定的リプレイのため、
//! シリアライズ形式は永続契約である(docs/design/test-strategy.md §1(d)、
//! docs/design/reviews/p1-c1-type-review.md H3 に対応)。
//!
//! roundtripテストは T→JSON→T の同一性しか見ないため、タプル→構造体バリアントへの
//! 整形のようなワイヤ形式の変更を検出できない。ここでは JSON 文字列そのものを固定する。

use crate::card::{Condition, Effect, Target};
use crate::ids::{CardId, CharacterId, ProposalId, SceneId, StatId};
use crate::primitives::Outcome;
use crate::session::{CardInstance, SessionStatus};

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
