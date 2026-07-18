//! シナリオ構造(Scenario/SceneDef等)。docs/design/domain-model.md「シナリオ構造」節に対応。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::card::{CardDef, Condition, Target};
use crate::ids::{CardId, ScenarioId, SceneId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Opening,
    Middle,
    Climax,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneKind {
    Conversation,
    Travel,
    Battle,
}

/// シーン入場時に配るカード。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deal {
    pub card: CardId,
    pub to: Target,
}

/// Conditionによる自動遷移。カードのGotoScene効果による遷移とは別経路。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub condition: Condition,
    pub to: SceneId,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneDef {
    pub id: SceneId,
    pub kind: SceneKind,
    pub narration: String,
    #[cfg_attr(
        test,
        proptest(strategy = "proptest::collection::vec(proptest::prelude::any::<Deal>(), 0..=3)")
    )]
    pub deals: Vec<Deal>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<Transition>(), 0..=3)"
        )
    )]
    pub exits: Vec<Transition>,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseDef {
    pub phase: Phase,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<SceneDef>(), 0..=2)"
        )
    )]
    pub scenes: Vec<SceneDef>,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioMeta {
    pub id: ScenarioId,
    pub title: String,
    pub author: String,
    pub forked_from: Option<ScenarioId>,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub meta: ScenarioMeta,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<CardId>(), proptest::prelude::any::<CardDef>(), 0..=3)"
        )
    )]
    pub card_defs: HashMap<CardId, CardDef>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<PhaseDef>(), 0..=3)"
        )
    )]
    pub phases: Vec<PhaseDef>,
}
