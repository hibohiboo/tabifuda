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

roundtrip_test!(roundtrip_scenario_id, ScenarioId);
roundtrip_test!(roundtrip_scene_id, SceneId);
roundtrip_test!(roundtrip_card_id, CardId);
roundtrip_test!(roundtrip_card_instance_id, CardInstanceId);
roundtrip_test!(roundtrip_character_id, CharacterId);
roundtrip_test!(roundtrip_user_id, UserId);
roundtrip_test!(roundtrip_proposal_id, ProposalId);
roundtrip_test!(roundtrip_stat_id, StatId);

roundtrip_test!(roundtrip_tag, Tag);
roundtrip_test!(roundtrip_card_kind, CardKind);
roundtrip_test!(roundtrip_target, Target);
roundtrip_test!(roundtrip_effect, Effect);
roundtrip_test!(roundtrip_condition, Condition);
roundtrip_test!(roundtrip_card_def, CardDef);

roundtrip_test!(roundtrip_phase, Phase);
roundtrip_test!(roundtrip_scene_kind, SceneKind);
roundtrip_test!(roundtrip_deal, Deal);
roundtrip_test!(roundtrip_transition, Transition);
roundtrip_test!(roundtrip_scene_def, SceneDef);
roundtrip_test!(roundtrip_phase_def, PhaseDef);
roundtrip_test!(roundtrip_scenario_meta, ScenarioMeta);

roundtrip_test!(roundtrip_character, Character);

roundtrip_test!(roundtrip_outcome, Outcome);
roundtrip_test!(roundtrip_proposal, Proposal);
roundtrip_test!(roundtrip_card_instance, CardInstance);
roundtrip_test!(roundtrip_session_status, SessionStatus);

roundtrip_test!(roundtrip_role, Role);

roundtrip_test!(roundtrip_scenario, Scenario);
roundtrip_test!(roundtrip_scenario_snapshot, ScenarioSnapshot);
roundtrip_test!(roundtrip_session, Session);

roundtrip_test!(roundtrip_bounded_string, BoundedString<16>);
roundtrip_test!(roundtrip_patch_op, PatchOp);
roundtrip_test!(roundtrip_scenario_patch, ScenarioPatch);
roundtrip_test!(roundtrip_patch_error, PatchError);
roundtrip_test!(roundtrip_command, Command);
roundtrip_test!(roundtrip_event, Event);
roundtrip_test!(roundtrip_rule_error, RuleError);
