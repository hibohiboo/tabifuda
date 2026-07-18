//! ドメインの基礎的な値型。IDにも集約にも属さない、下位に置くべき小さな値。
//! 上位モジュール(card/session等)がここに依存する形で依存をDAGに保つ。

use serde::{Deserialize, Serialize};

/// セッションの結末。勝利/敗北カードの選択で分岐する(domain-model.md参照)。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Victory,
    Defeat,
}
