//! tabifuda-core: カード制TRPG(CardWirth風)のルール・状態機械。
//!
//! このクレートの原則(CLAUDE.md 最重要ルール2・3):
//! - 純粋に保つ。IO・時刻取得・乱数生成・グローバル状態を持ち込まない。
//!   乱数が必要な場合は結果を引数/イベントとして外から与える(リプレイ決定性のため)。
//! - すべての進行はイベント。状態を直接書き換える近道を作らない。
//!   変更は必ず `decide(state, command) -> Result<Vec<Event>, RuleError>` と
//!   `apply(state, event) -> State` を通す(decide/applyはC2で実装)。

pub mod actor;
pub mod card;
pub mod character;
pub mod command;
pub mod engine;
pub mod error;
pub mod event;
pub mod ids;
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
pub use event::Event;
pub use ids::{
    CardId, CardInstanceId, CharacterId, ProposalId, ScenarioId, SceneId, StatId, UserId,
};
pub use patch::{validate, PatchError, PatchOp, ScenarioPatch};
pub use primitives::{BoundedString, BoundedStringError, Outcome};
pub use scenario::{
    Deal, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind, Transition,
};
pub use session::{CardInstance, Proposal, ScenarioSnapshot, Session, SessionStatus};

#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod golden_tests;
#[cfg(test)]
mod invariant_tests;
#[cfg(test)]
mod patch_tests;
#[cfg(test)]
mod roundtrip_tests;
