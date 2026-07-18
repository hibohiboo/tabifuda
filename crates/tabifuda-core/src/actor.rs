//! アクターと権限。docs/design/domain-model.md「アクターと権限」節に対応。

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterId, UserId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Gm,
    Player { characters: Vec<CharacterId> },
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub user: UserId,
    pub role: Role,
}
