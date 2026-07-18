//! decideの出力・冒険記の構成要素。docs/design/domain-model.md「コマンドとイベント」節に
//! 対応。C2でSessionStarted/SceneEntered/CardDealt/CardPlayed/EffectApplied/
//! PhaseAdvanced/SessionEnded、C3でProposalSubmitted/ProposalJudged、
//! C4でScenarioPatchedを実装。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actor::Role;
use crate::card::Effect;
use crate::character::Character;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, UserId};
use crate::patch::ScenarioPatch;
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
    /// → Paused へ遷移(C3)。`id`の発番規則はdomain-model.md「C3:
    /// decide/applyの解決規則」参照。
    ProposalSubmitted {
        id: ProposalId,
        by: CharacterId,
        text: BoundedString<4096>,
    },
    /// GMによるシナリオ改編(C4)。ログUIでは1枚のカードとして表示できる
    /// (domain-model.md「シナリオパッチ」節)。
    ScenarioPatched {
        patch: ScenarioPatch,
    },
    /// → Running へ遷移(C3)。
    ProposalJudged {
        id: ProposalId,
        accepted: bool,
    },
    SessionEnded {
        outcome: Outcome,
    },
}
