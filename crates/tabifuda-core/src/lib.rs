//! tabifuda-core: カード制TRPG(CardWirth風)のルール・状態機械。
//!
//! このクレートが守る原則の正は docs/design/domain-model.md「基本原則」
//! (純粋性・decide/apply・イベント経由の進行・乱数決定性)。

pub mod actor;
pub mod card;
pub mod character;
pub mod command;
pub mod engine;
pub mod error;
pub mod event;
pub mod ids;
pub mod lint;
pub mod patch;
pub mod primitives;
pub mod scenario;
pub mod session;

pub use actor::Role;
pub use card::{CardDef, CardKind, Condition, Effect, Tag, Target};
pub use character::Character;
pub use command::Command;
pub use engine::{apply, decide};
pub use error::RuleError;
pub use event::{Event, RemovalReason};
pub use ids::{
    CardId, CardInstanceId, CharacterId, ProposalId, ScenarioId, SceneId, StatId, UserId,
};
pub use lint::{lint, LintFinding, LintIssue, Severity};
pub use patch::{validate, PatchError, PatchOp, ScenarioPatch};
pub use primitives::{BoundedString, BoundedStringError, Outcome};
pub use scenario::{
    Deal, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind, Transition,
};
pub use session::{CardInstance, Proposal, ScenarioSnapshot, Session, SessionStatus};

// テスト関数名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md
// D2「テストビュー」。cargo test出力がそのままGitHub Pagesの説明文になる)。
// 英語のCommand/Event/型名をそのまま含めるため先頭が大文字ASCIIになる場合があり、
// 意図的にnon_snake_caseを許可する。
#[cfg(test)]
#[allow(non_snake_case)]
mod engine_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod golden_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod invariant_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod lint_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod patch_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod replay_tests;
#[cfg(test)]
#[allow(non_snake_case)]
mod roundtrip_tests;
