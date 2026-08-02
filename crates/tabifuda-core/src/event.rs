//! decideの出力・冒険記の構成要素。docs/design/domain-model.md「コマンドとイベント」節に
//! 対応。C2でSessionStarted/SceneEntered/CardDealt/CardPlayed/EffectApplied/
//! PhaseAdvanced/SessionEnded、C3でProposalSubmitted/ProposalJudged、
//! C4でScenarioPatchedを実装。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actor::Role;
use crate::card::{CardDef, Effect};
use crate::character::Character;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, SceneId, UserId};
use crate::patch::ScenarioPatch;
use crate::primitives::{BoundedString, Outcome};
use crate::scenario::Phase;
use crate::session::ScenarioSnapshot;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
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
                strategy = "proptest::collection::btree_map(proptest::prelude::any::<UserId>(), proptest::prelude::any::<Role>(), 0..=2)"
            )
        )]
        roles: BTreeMap<UserId, Role>,
        initial_phase: Phase,
        initial_scene: SceneId,
    },
    SceneEntered {
        scene: SceneId,
        narration: String,
        /// このシーンの`scene_def.deals`から実際に配ったカードの一覧
        /// (カード効果由来の`DealCard`は含まない)。domain-model.md
        /// 「カードの消費・除去」参照。`#[serde(default)]`は、この
        /// フィールド追加前に記録されたfixtureを空Vecとして読める
        /// ようにするため(後方互換)。
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<CardInstanceId>(), 0..=3)"
            )
        )]
        #[serde(default)]
        local_instances: Vec<CardInstanceId>,
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
    /// カードが手札から除去された(domain-model.md「カードの消費・除去」参照)。
    CardRemoved {
        from: CharacterId,
        card: CardId,
        instance: CardInstanceId,
        reason: RemovalReason,
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
    /// セッション終了処理(finalize)で、`to`が持ち出し可(`#portable`タグ)な
    /// カードを持ち帰った(domain-model.md「セッション終了処理(finalize)」。
    /// 2026-07-20決定の§3)。`cards`はCardDefの凍結コピー(CardIdはそのシナリオ
    /// 内でしか解決できないため)。空になることは無い(空ならイベント自体を
    /// 発行しない)。
    RewardsGranted {
        to: CharacterId,
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<CardDef>(), 1..=2)"
            )
        )]
        cards: Vec<CardDef>,
    },
    /// finalizeで、`from`の手札のうち持ち出し不可だったカードが破棄された
    /// (domain-model.md「セッション終了処理(finalize)」)。監査記録のみで
    /// 状態(hands)は変更しない(セッションはこの時点で既にEnded)。
    CardsDiscarded {
        from: CharacterId,
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<CardId>(), 1..=2)"
            )
        )]
        cards: Vec<CardId>,
    },
}

/// `Event::CardRemoved`の理由。domain-model.md「カードの消費・除去」参照。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RemovalReason {
    /// 使用による消費(`CardKind::is_consumed_on_play`が`true`のカードを出した)。
    Consumed,
    /// シーンを離れたことによる自動消去(未使用の選択肢カード)。
    SceneLeft,
}
