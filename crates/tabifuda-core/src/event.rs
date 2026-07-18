//! decideの出力・冒険記の構成要素。docs/design/domain-model.md「コマンドとイベント」節に
//! 対応。C2でSessionStarted/SceneEntered/CardDealt/CardPlayed/EffectApplied/
//! PhaseAdvanced/SessionEndedのみ実装(ProposalSubmitted/ScenarioPatched/
//! ProposalJudgedはC3/C4で追加)。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actor::Role;
use crate::card::Effect;
use crate::character::Character;
use crate::ids::{CardId, CardInstanceId, CharacterId, SceneId, UserId};
use crate::primitives::{BoundedString, Outcome};
use crate::scenario::Phase;
use crate::session::ScenarioSnapshot;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    SessionStarted {
        scenario: ScenarioSnapshot,
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<Character>(), 0..=2)"
            )
        )]
        party: Vec<Character>,
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::hash_map(proptest::prelude::any::<UserId>(), proptest::prelude::any::<Role>(), 0..=2)"
            )
        )]
        roles: HashMap<UserId, Role>,
        initial_phase: Phase,
        initial_scene: SceneId,
    },
    SceneEntered {
        scene: SceneId,
        narration: String,
    },
    CardDealt {
        to: CharacterId,
        card: CardId,
        instance: CardInstanceId,
    },
    CardPlayed {
        by: CharacterId,
        card: CardId,
        free_text: Option<BoundedString<4096>>,
    },
    /// 未解決Effect(C2時点ではModifyStatのみ)の監査記録。解決済みEffectは
    /// SceneEntered/CardDealt/PhaseAdvanced/SessionEndedの方に載るため、
    /// これらと重複しては発行されない(domain-model.md「Effect解決」参照)。
    EffectApplied {
        effect: Effect,
    },
    PhaseAdvanced {
        phase: Phase,
    },
    SessionEnded {
        outcome: Outcome,
    },
}
