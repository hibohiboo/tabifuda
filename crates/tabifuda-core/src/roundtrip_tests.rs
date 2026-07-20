//! 全公開型のserialize/deserialize往復テスト(プロパティで一括)。
//! docs/design/test-strategy.md §1(d) に対応。

use proptest::prelude::*;

use crate::*;

macro_rules! roundtrip_test {
    ($test_name:ident, $ty:ty) => {
        proptest! {
            #[test]
            fn $test_name(value in any::<$ty>()) {
                let json = serde_json::to_string(&value).unwrap();
                let restored: $ty = serde_json::from_str(&json).unwrap();
                prop_assert_eq!(value, restored);
            }
        }
    };
}

roundtrip_test!(ScenarioIdの往復変換で値が保持される, ScenarioId);
roundtrip_test!(SceneIdの往復変換で値が保持される, SceneId);
roundtrip_test!(CardIdの往復変換で値が保持される, CardId);
roundtrip_test!(CardInstanceIdの往復変換で値が保持される, CardInstanceId);
roundtrip_test!(CharacterIdの往復変換で値が保持される, CharacterId);
roundtrip_test!(UserIdの往復変換で値が保持される, UserId);
roundtrip_test!(ProposalIdの往復変換で値が保持される, ProposalId);
roundtrip_test!(StatIdの往復変換で値が保持される, StatId);

roundtrip_test!(Tagの往復変換で値が保持される, Tag);
roundtrip_test!(CardKindの往復変換で値が保持される, CardKind);
roundtrip_test!(Targetの往復変換で値が保持される, Target);
roundtrip_test!(Effectの往復変換で値が保持される, Effect);
roundtrip_test!(Conditionの往復変換で値が保持される, Condition);
roundtrip_test!(CardDefの往復変換で値が保持される, CardDef);

roundtrip_test!(Phaseの往復変換で値が保持される, Phase);
roundtrip_test!(SceneKindの往復変換で値が保持される, SceneKind);
roundtrip_test!(Dealの往復変換で値が保持される, Deal);
roundtrip_test!(Transitionの往復変換で値が保持される, Transition);
roundtrip_test!(SceneDefの往復変換で値が保持される, SceneDef);
roundtrip_test!(PhaseDefの往復変換で値が保持される, PhaseDef);
roundtrip_test!(ScenarioMetaの往復変換で値が保持される, ScenarioMeta);

roundtrip_test!(Characterの往復変換で値が保持される, Character);

roundtrip_test!(Outcomeの往復変換で値が保持される, Outcome);
roundtrip_test!(Proposalの往復変換で値が保持される, Proposal);
roundtrip_test!(CardInstanceの往復変換で値が保持される, CardInstance);
roundtrip_test!(SessionStatusの往復変換で値が保持される, SessionStatus);

roundtrip_test!(Roleの往復変換で値が保持される, Role);

roundtrip_test!(Scenarioの往復変換で値が保持される, Scenario);
roundtrip_test!(ScenarioSnapshotの往復変換で値が保持される, ScenarioSnapshot);
roundtrip_test!(Sessionの往復変換で値が保持される, Session);

roundtrip_test!(BoundedStringの往復変換で値が保持される, BoundedString<16>);
roundtrip_test!(PatchOpの往復変換で値が保持される, PatchOp);
roundtrip_test!(ScenarioPatchの往復変換で値が保持される, ScenarioPatch);
roundtrip_test!(PatchErrorの往復変換で値が保持される, PatchError);
roundtrip_test!(Commandの往復変換で値が保持される, Command);
roundtrip_test!(Eventの往復変換で値が保持される, Event);
roundtrip_test!(RuleErrorの往復変換で値が保持される, RuleError);
