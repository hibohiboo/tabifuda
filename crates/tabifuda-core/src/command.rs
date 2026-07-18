//! decideへの入力。docs/design/domain-model.md「コマンドとイベント」節に対応。
//! C2でStartSession/PlayCard/EndSession、C3でPropose/JudgeProposal/GmAdvance、
//! C4でApplyPatchを実装。

use serde::{Deserialize, Serialize};

use crate::character::Character;
use crate::ids::{CardInstanceId, CharacterId, ProposalId, SceneId};
use crate::patch::ScenarioPatch;
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
    /// → Paused へ遷移(C3)。
    Propose {
        by: CharacterId,
        text: BoundedString<4096>,
    },
    /// GM専用。Paused中のみ(C4)。
    ApplyPatch {
        patch: ScenarioPatch,
    },
    /// GM裁定 → Running へ遷移(C3)。
    JudgeProposal {
        proposal: ProposalId,
        accepted: bool,
    },
    /// GM強制進行(C3)。
    GmAdvance {
        to: SceneId,
    },
    EndSession {
        outcome: Outcome,
    },
}
