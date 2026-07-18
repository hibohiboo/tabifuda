//! tabifuda-core: カード制TRPG(CardWirth風)のルール・状態機械。
//!
//! このクレートの原則(CLAUDE.md 最重要ルール2・3):
//! - 純粋に保つ。IO・時刻取得・乱数生成・グローバル状態を持ち込まない。
//!   乱数が必要な場合は結果を引数/イベントとして外から与える(リプレイ決定性のため)。
//! - すべての進行はイベント。状態を直接書き換える近道を作らない。
//!   変更は必ず `decide(state, command) -> Result<Vec<Event>, RuleError>` と
//!   `apply(state, event) -> State` を通す。

/// P1で実際のドメイン型に置き換わる、CI疎通確認用のプレースホルダ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placeholder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_is_comparable() {
        assert_eq!(Placeholder, Placeholder);
    }
}
