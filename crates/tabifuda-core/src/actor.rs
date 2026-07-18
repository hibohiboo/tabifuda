//! 役割と権限。docs/design/domain-model.md「アクターと権限」節に対応。
//!
//! role の信頼モデル(同文書、2026-07-18決定): `Session.roles` が役割の唯一の正。
//! decide は認証済みの `UserId` だけを受け取り、役割は roles から自分で解決する。
//! 呼び出し側が役割を自己申告する型(旧 `Actor { user, role }`)は持たない
//! (「GMを名乗るだけの権限昇格」を型レベルで不可能にするため)。

use serde::{Deserialize, Serialize};

use crate::ids::CharacterId;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// 進行役。Player の権限を包含する(PlayCard/Propose も可)。
    Gm,
    Player {
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::collection::vec(proptest::prelude::any::<CharacterId>(), 0..=3)"
            )
        )]
        characters: Vec<CharacterId>,
    },
}
