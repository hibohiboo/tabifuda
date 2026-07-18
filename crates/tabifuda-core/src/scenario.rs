//! シナリオ構造(Scenario/SceneDef等)。docs/design/domain-model.md「シナリオ構造」節に対応。
//!
//! コレクション規則(同文書「コレクションと id の規則」): 作者データは
//! `Vec<T>`+埋め込み id が正(並び順に意味があり、シリアライズが決定的)。
//! id の一意性は validate / シナリオlint(P2)/ プロパティテスト(C5)で固定する。

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
#[non_exhaustive]
pub enum SceneKind {
    Conversation,
    Travel,
    Battle,
}

/// シーン入場時に配るカード。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deal {
    pub card: CardId,
    pub to: Target,
}

/// Conditionによる自動遷移。カードのGotoScene効果による遷移とは別経路。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub condition: Condition,
    pub to: SceneId,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioMeta {
    pub id: ScenarioId,
    pub title: String,
    pub author: String,
    pub forked_from: Option<ScenarioId>,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub meta: ScenarioMeta,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<CardDef>(), 0..=3)"
        )
    )]
    pub card_defs: Vec<CardDef>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<PhaseDef>(), 0..=3)"
        )
    )]
    pub phases: Vec<PhaseDef>,
}

impl Scenario {
    /// カード定義を id で引く(作者データは Vec+埋め込み id が正。線形探索で十分)。
    pub fn card_def(&self, id: &CardId) -> Option<&CardDef> {
        self.card_defs.iter().find(|def| &def.id == id)
    }

    /// シーン定義を全 phase を通して id で引く。
    pub fn scene_def(&self, id: &SceneId) -> Option<&SceneDef> {
        self.phases
            .iter()
            .flat_map(|phase| phase.scenes.iter())
            .find(|scene| &scene.id == id)
    }
}
