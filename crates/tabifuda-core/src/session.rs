//! セッション状態。docs/design/domain-model.md「セッション状態」節に対応。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actor::Role;
use crate::character::Character;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, UserId};
use crate::scenario::{Phase, Scenario};

/// 開催時点のシナリオを凍結コピーしたもの。元シナリオの後編集と独立。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioSnapshot(pub Scenario);

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Victory,
    Defeat,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Proposal {
    pub id: ProposalId,
    pub by: CharacterId,
    pub text: String,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInstance {
    pub id: CardInstanceId,
    pub card: CardId,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionStatus {
    Running,
    Paused { proposal: ProposalId },
    Ended(Outcome),
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub scenario: ScenarioSnapshot,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<Character>(), 0..=2)"
        )
    )]
    pub party: Vec<Character>,
    pub status: SessionStatus,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<UserId>(), proptest::prelude::any::<Role>(), 0..=2)"
        )
    )]
    pub roles: HashMap<UserId, Role>,
    pub phase: Phase,
    pub scene: SceneId,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<CharacterId>(), proptest::collection::vec(proptest::prelude::any::<CardInstance>(), 0..=3), 0..=2)"
        )
    )]
    pub hands: HashMap<CharacterId, Vec<CardInstance>>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<CardInstance>(), 0..=3)"
        )
    )]
    pub table: Vec<CardInstance>,
    pub pending_proposal: Option<Proposal>,
}
