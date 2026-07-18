//! decideへの入力。docs/design/domain-model.md「コマンドとイベント」節に対応。
//! C2でStartSession/PlayCard/EndSessionのみ実装(Propose/ApplyPatch/JudgeProposal/
//! GmAdvanceはC3/C4で追加)。

use serde::{Deserialize, Serialize};

use crate::character::Character;
use crate::ids::{CardInstanceId, CharacterId};
use crate::primitives::{BoundedString, Outcome};
use crate::scenario::Scenario;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Command {
    StartSession {
        scenario: Scenario,
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<Character>(), 0..=2)"
            )
        )]
        party: Vec<Character>,
    },
    PlayCard {
        by: CharacterId,
        card: CardInstanceId,
        free_text: Option<BoundedString<4096>>,
    },
    EndSession {
        outcome: Outcome,
    },
}
