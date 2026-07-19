//! セッション状態。docs/design/domain-model.md「セッション状態」節に対応。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actor::Role;
use crate::character::Character;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, UserId};
use crate::primitives::{BoundedString, Outcome};
use crate::scenario::{Phase, Scenario};

/// 開催時点のシナリオを凍結コピーしたもの。元シナリオの後編集と独立。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioSnapshot(pub Scenario);

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub id: ProposalId,
    pub by: CharacterId,
    pub text: BoundedString<4096>,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInstance {
    pub id: CardInstanceId,
    pub card: CardId,
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    Running,
    Paused { proposal: ProposalId },
    Ended(Outcome),
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub scenario: ScenarioSnapshot,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<Character>(), 0..=2)"
        )
    )]
    pub party: Vec<Character>,
    pub status: SessionStatus,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<UserId>(), proptest::prelude::any::<Role>(), 0..=2)"
        )
    )]
    pub roles: HashMap<UserId, Role>,
    pub phase: Phase,
    pub scene: SceneId,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::hash_map(proptest::prelude::any::<CharacterId>(), proptest::collection::vec(proptest::prelude::any::<CardInstance>(), 0..=3), 0..=2)"
        )
    )]
    pub hands: HashMap<CharacterId, Vec<CardInstance>>,
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<CardInstance>(), 0..=3)"
        )
    )]
    pub table: Vec<CardInstance>,
    pub pending_proposal: Option<Proposal>,
    /// `ProposalId`発番用の単調カウンタ。`CardInstanceId`と異なり
    /// `pending_proposal`は裁定のたびに`None`へ戻る(除去が起きる)ため、
    /// 現在状態からの逆算では連番の一意性を保てない。そこで別途カウンタを持つ
    /// (domain-model.md「提案と裁定」参照)。
    pub proposal_seq: u64,
    /// `CardInstanceId`発番用の単調カウンタ。カードは除去されうるため、
    /// `hands`+`table`の現在の総数からの逆算では一意性を保てない
    /// (`proposal_seq`と同じ理由。domain-model.md「カードの消費・除去」参照)。
    /// 除去してもID空間は消費されたままとし、巻き戻さない。
    pub card_instance_seq: usize,
    /// 現在のシーンが`SceneEntered`で配ったカードのうち、まだ`hands`にある
    /// ものの一覧(シーン離脱時クリーンアップの対象候補。domain-model.md
    /// 「カードの消費・除去」参照)。
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<CardInstanceId>(), 0..=3)"
        )
    )]
    pub scene_local_instances: Vec<CardInstanceId>,
}
