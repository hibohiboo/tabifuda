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
pub mod ids;
pub mod scenario;
pub mod session;

pub use actor::{Actor, Role};
pub use card::{CardDef, CardKind, Condition, Effect, Tag, Target};
pub use character::Character;
pub use ids::{
    CardId, CardInstanceId, CharacterId, ProposalId, ScenarioId, SceneId, StatId, UserId,
};
pub use scenario::{
    Deal, Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind, Transition,
};
pub use session::{CardInstance, Outcome, Proposal, ScenarioSnapshot, Session, SessionStatus};

#[cfg(test)]
mod roundtrip_tests;
