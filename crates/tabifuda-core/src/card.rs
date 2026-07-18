//! カード定義(CardDef)と、その効果(Effect)・条件(Condition)。
//! docs/design/domain-model.md「カード」節に対応。

use serde::{Deserialize, Serialize};

use crate::ids::{CardId, CharacterId, SceneId, StatId};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(pub String);

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CardKind {
    Action,
    Scenario,
    Dialogue,
    Proposal,
    Item,
    Marker,
}

/// 効果の対象。意味論は domain-model.md「Target の意味論」参照(2026-07-18決定):
/// - `Party` は役割参照。台本(シナリオ)執筆時点で書ける
/// - `Character` は実名参照で**上演中専用**。シナリオデータ内での使用は不正
///   (解決不能。P2のシナリオlintで検出)
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Target {
    Party,
    Character(CharacterId),
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Effect {
    GotoScene(SceneId),
    AdvancePhase,
    DealCard {
        card: CardId,
        to: Target,
    },
    ModifyStat {
        target: Target,
        stat: StatId,
        delta: i32,
    },
    EndSession(crate::primitives::Outcome),
}

/// アクターの手札 or table に存在するかで判定する(FlagIsはv0.2で廃止。
/// docs/design/domain-model.md参照)。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Condition {
    HasCard(CardId),
    StatAtLeast(StatId, i32),
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardDef {
    pub id: CardId,
    pub name: String,
    pub kind: CardKind,
    pub text: String,
    #[cfg_attr(
        test,
        proptest(strategy = "proptest::collection::vec(proptest::prelude::any::<Tag>(), 0..=3)")
    )]
    pub tags: Vec<Tag>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<Effect>(), 0..=3)"
        )
    )]
    pub effects: Vec<Effect>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<Condition>(), 0..=3)"
        )
    )]
    pub requires: Vec<Condition>,
}
