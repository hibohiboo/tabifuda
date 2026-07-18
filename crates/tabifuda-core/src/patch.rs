//! シナリオパッチ(GMによる構造化改編)。docs/design/domain-model.md「シナリオパッチ
//! (構造化)」「C4: decide/applyの解決規則」節に対応。
//!
//! `validate`はCardWirth的な「再開の安全性」を守るゲート: パッチ適用後も
//! 「現在シーンが存在する」「配布済みカードの定義が解決可能」を壊さないことを
//! 検証する(test-strategy.md 不変条件5)。`apply_ops`は検証用クローンへの適用
//! (validate)と、decideで検証済みのパッチをSessionへ実適用する経路(engine::apply)
//! の両方で共有する。

use serde::{Deserialize, Serialize};

use crate::card::{CardDef, Target};
use crate::ids::{CardId, SceneId};
use crate::primitives::BoundedString;
use crate::scenario::{Phase, Scenario, SceneDef, Transition};
use crate::session::Session;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatchOp {
    AddCardDef(CardDef),
    /// 既存シーンの丸ごと差し替え。`SceneDef.id`と一致する既存シーンが対象
    /// (新規idの場合はSceneNotFound。「追加」はAddSceneが担う)。
    ReplaceScene(SceneDef),
    AddScene {
        phase: Phase,
        scene: SceneDef,
    },
    AddTransition {
        scene: SceneId,
        transition: Transition,
    },
    /// その場で配る(SceneDef.dealsのような入場時配布ではなく即時)。
    DealCard {
        card: CardId,
        to: Target,
    },
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioPatch {
    #[cfg_attr(
        test,
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<PatchOp>(), 0..=3)"
        )
    )]
    pub ops: Vec<PatchOp>,
    /// GMのコメント。ログカードとして表示(cross-cutting.md §UGC-3、C4で導入)。
    pub note: BoundedString<4096>,
}

/// validateの拒否理由。tabifuda-coreの公開APIはpanicしない(CLAUDE.md規約)。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[non_exhaustive]
pub enum PatchError {
    /// AddCardDefのidが既存card_defsと重複。
    #[error("duplicate card id")]
    DuplicateCardId,
    /// AddSceneのidが既存シーン(全phase通して)と重複。
    #[error("duplicate scene id")]
    DuplicateSceneId,
    /// ReplaceSceneの対象、AddTransitionの対象/遷移先、または現在シーンが
    /// (適用後の)シナリオに見つからない。
    #[error("scene not found")]
    SceneNotFound,
    /// AddSceneのphaseに対応するPhaseDefがシナリオに無い。
    #[error("phase not found")]
    PhaseNotFound,
    /// DealCardのcard、または(適用後に)配布済みカードのCardDefが
    /// シナリオに見つからない。
    #[error("card not found")]
    CardNotFound,
}

/// パッチ適用後もセッションの安全性(現在シーン・配布済みカードの解決可能性)が
/// 壊れないことを検証する。GmがApplyPatchを発行する前にdecide内で通す
/// (domain-model.md「シナリオパッチ」節「再開の安全性」)。
pub fn validate(session: &Session, patch: &ScenarioPatch) -> Result<(), PatchError> {
    let mut scenario = session.scenario.0.clone();
    apply_ops(&mut scenario, &patch.ops)?;

    if scenario.scene_def(&session.scene).is_none() {
        // v0.1のPatchOp(5種)には削除系操作が無いため実際には到達しない防御的
        // チェック。将来RemoveScene等が追加された時にここが効く。
        return Err(PatchError::SceneNotFound);
    }
    let dealt_cards = session
        .hands
        .values()
        .flatten()
        .chain(session.table.iter())
        .map(|instance| &instance.card);
    for card in dealt_cards {
        if scenario.card_def(card).is_none() {
            return Err(PatchError::CardNotFound);
        }
    }
    Ok(())
}

/// PatchOp列をシナリオへ機械的に適用する(検証用クローンへの適用・実セッションへの
/// 適用の両方で共有)。ops内の後続opは、直前までのopが反映された状態を参照できる
/// (例: 同一パッチ内でAddCardDefした直後にそのcardをDealCardする)。
pub(crate) fn apply_ops(scenario: &mut Scenario, ops: &[PatchOp]) -> Result<(), PatchError> {
    for op in ops {
        match op {
            PatchOp::AddCardDef(def) => {
                if scenario.card_def(&def.id).is_some() {
                    return Err(PatchError::DuplicateCardId);
                }
                scenario.card_defs.push(def.clone());
            }
            PatchOp::ReplaceScene(new_def) => {
                let existing = scenario
                    .phases
                    .iter_mut()
                    .flat_map(|phase_def| phase_def.scenes.iter_mut())
                    .find(|scene| scene.id == new_def.id)
                    .ok_or(PatchError::SceneNotFound)?;
                *existing = new_def.clone();
            }
            PatchOp::AddScene { phase, scene } => {
                if scenario.scene_def(&scene.id).is_some() {
                    return Err(PatchError::DuplicateSceneId);
                }
                let phase_def = scenario
                    .phases
                    .iter_mut()
                    .find(|phase_def| &phase_def.phase == phase)
                    .ok_or(PatchError::PhaseNotFound)?;
                phase_def.scenes.push(scene.clone());
            }
            PatchOp::AddTransition { scene, transition } => {
                if scenario.scene_def(&transition.to).is_none() {
                    return Err(PatchError::SceneNotFound);
                }
                let scene_def = scenario
                    .phases
                    .iter_mut()
                    .flat_map(|phase_def| phase_def.scenes.iter_mut())
                    .find(|s| &s.id == scene)
                    .ok_or(PatchError::SceneNotFound)?;
                scene_def.exits.push(transition.clone());
            }
            PatchOp::DealCard { card, .. } => {
                if scenario.card_def(card).is_none() {
                    return Err(PatchError::CardNotFound);
                }
            }
        }
    }
    Ok(())
}
