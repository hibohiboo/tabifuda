//! ドメインの基礎的な値型。IDにも集約にも属さない、下位に置くべき小さな値。
//! 上位モジュール(card/session等)がここに依存する形で依存をDAGに保つ。

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// セッションの結末。勝利/敗北カードの選択で分岐する(domain-model.md参照)。
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Victory,
    Defeat,
}

/// 長さ上限付き文字列(cross-cutting.md §自由入力(UGC)、domain-model.md
/// 「文字列の長さ上限(BoundedString)」参照)。DoSと保存コストの上限を型レベルで持たせる。
/// `try_new` はResultを返しpanicしない。custom Deserializeにより、
/// JSON経由(シナリオ読込・イベント再生・API入力)でも境界を強制する。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundedString<const MAX: usize>(String);

/// 文字数(`chars().count()`)がMAXを超えた場合のエラー。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("string length {actual} exceeds max {max}")]
pub struct BoundedStringError {
    pub max: usize,
    pub actual: usize,
}

impl<const MAX: usize> BoundedString<MAX> {
    pub fn try_new(value: impl Into<String>) -> Result<Self, BoundedStringError> {
        let value = value.into();
        let actual = value.chars().count();
        if actual > MAX {
            Err(BoundedStringError { max: MAX, actual })
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<const MAX: usize> Serialize for BoundedString<MAX> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, const MAX: usize> Deserialize<'de> for BoundedString<MAX> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl<const MAX: usize> proptest::arbitrary::Arbitrary for BoundedString<MAX> {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: ()) -> Self::Strategy {
        use proptest::prelude::*;
        // 上限ぎりぎりの境界値も生成対象にしつつ、テスト実行時間を抑えるため
        // 生成する文字数はMAXと32のいずれか小さい方までに収める。
        let cap = MAX.min(32);
        proptest::collection::vec(proptest::char::any(), 0..=cap)
            .prop_map(|chars| {
                BoundedString::try_new(chars.into_iter().collect::<String>()).unwrap()
            })
            .boxed()
    }
}
