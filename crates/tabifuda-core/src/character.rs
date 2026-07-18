//! キャラクター(セッションのparty内。マスターデータの凍結コピー)。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::{CardId, CharacterId, StatId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<StatId>(), proptest::prelude::any::<i32>(), 0..=3)"
        )
    )]
    pub stats: HashMap<StatId, i32>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<CardId>(), 0..=3)"
        )
    )]
    pub deck: Vec<CardId>,
}
