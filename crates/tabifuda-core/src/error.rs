//! decideの拒否理由。docs/design/domain-model.md「C2: decide/applyの解決規則」に対応。
//! tabifuda-coreの公開APIはpanicしない(CLAUDE.md規約)。decideの失敗は必ずここへ。

use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[non_exhaustive]
pub enum RuleError {
    /// 実行者にコマンドを行う権限が無い(役割未登録・担当キャラ外・GM専用等)。
    #[error("forbidden")]
    Forbidden,
    /// state == None でStartSession以外のコマンドが渡された。
    #[error("no active session")]
    NoActiveSession,
    /// state == Some でStartSessionが渡された。
    #[error("session already started")]
    SessionAlreadyStarted,
    /// status == Ended の間に発行された(StartSession以外の)コマンド。
    #[error("session already ended")]
    SessionEnded,
    /// status == Paused の間に発行されたPlayCard/Propose/EndSession。
    #[error("session is paused")]
    SessionPaused,
    /// ApplyPatchはstatus == Pausedの間のみ許可される(status != Pausedで拒否)。
    #[error("session is not paused")]
    SessionNotPaused,
    /// StartSession時、シナリオに有効な先頭シーンが無い(phasesが空、または
    /// 先頭phaseにscenesが無い)。
    #[error("scenario has no scenes")]
    ScenarioHasNoScenes,
    /// PlayCardのCardInstanceIdが実行者キャラの手札に無い、または対応する
    /// CardDefがシナリオに見つからない。
    #[error("card not found")]
    CardNotFound,
    /// CardDef.requiresの条件が満たされていない。
    #[error("condition not met")]
    ConditionNotMet,
    /// GotoScene効果の遷移先シーンがシナリオに存在しない。
    #[error("scene not found")]
    SceneNotFound,
    /// AdvancePhase効果だが現在のPhaseが既に最後(Climax)で次が無い。
    #[error("no next phase")]
    NoNextPhase,
    /// JudgeProposalが指定したProposalIdが、現在のPaused状態が保持する
    /// pending_proposalと一致しない(status==Runningで裁定対象が無い場合を含む)。
    #[error("proposal not found")]
    ProposalNotFound,
    /// ApplyPatchのパッチが`patch::validate`を通らなかった。
    #[error("invalid patch: {0}")]
    InvalidPatch(#[from] crate::patch::PatchError),
}
