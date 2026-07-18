//! キャラクター(セッションのparty内。マスターデータの凍結コピー)。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::{CardId, CharacterId, StatId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub stats: HashMap<StatId, i32>,
    pub deck: Vec<CardId>,
}
