//! アクターと権限。docs/design/domain-model.md「アクターと権限」節に対応。

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterId, UserId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Gm,
    Player {
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<CharacterId>(), 0..=3)"
            )
        )]
        characters: Vec<CharacterId>,
    },
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub user: UserId,
    pub role: Role,
}
