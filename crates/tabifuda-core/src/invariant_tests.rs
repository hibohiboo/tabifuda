//! プロパティテストによる不変条件1〜5の固定。docs/design/test-strategy.md §1(b)に対応
//! (不変条件5をここに含める根拠は同文書「フェーズ別の導入順」)。

use std::collections::HashMap;

use proptest::prelude::*;

use crate::actor::Role;
use crate::card::{CardDef, CardKind, Target};
use crate::command::Command;
use crate::engine::{apply, decide};
use crate::event::Event;
use crate::ids::{CardId, CardInstanceId, CharacterId, ProposalId, ScenarioId, SceneId, UserId};
use crate::patch::{PatchOp, ScenarioPatch};
use crate::primitives::BoundedString;
use crate::scenario::{Phase, PhaseDef, Scenario, ScenarioMeta, SceneDef, SceneKind};
use crate::session::{CardInstance, ScenarioSnapshot, Session, SessionStatus};

/// コマンド列駆動のプロパティテスト用ステップ。`(UserId, Command)`をそのまま
/// タプルでArbitrary生成する代わりに名前付きにして、失敗時の出力を読みやすくする。
#[derive(Debug, Clone, proptest_derive::Arbitrary)]
struct Step {
    actor: UserId,
    cmd: Command,
}

fn steps_strategy() -> impl Strategy<Value = Vec<Step>> {
    proptest::collection::vec(any::<Step>(), 0..=8)
}

/// hands+tableの全カード実体を(所持先、CardInstanceId、CardId)の集合として捉える。
/// 不変条件4(保存則)の判定に使う。所持先が`None`なら`table`を意味する。
fn all_instances(
    session: &Session,
) -> std::collections::HashSet<(Option<CharacterId>, CardInstanceId, CardId)> {
    let mut set = std::collections::HashSet::new();
    for (character, cards) in &session.hands {
        for ci in cards {
            set.insert((Some(character.clone()), ci.id.clone(), ci.card.clone()));
        }
    }
    for ci in &session.table {
        set.insert((None, ci.id.clone(), ci.card.clone()));
    }
    set
}

/// 不変条件3(状態機械の合法性): 状態機械図(domain-model.md「セッション状態機械」)に
/// 無い遷移は起きない。`None`は「セッション未生成」を表す。
fn is_legal_status_transition(
    before: &Option<SessionStatus>,
    after: &Option<SessionStatus>,
) -> bool {
    match (before, after) {
        (None, Some(SessionStatus::Running)) => true, // StartSession
        (Some(a), Some(b)) if a == b => true,         // ステータスに影響しないイベント
        (Some(SessionStatus::Running), Some(SessionStatus::Paused { .. })) => true, // ProposalSubmitted
        (Some(SessionStatus::Paused { .. }), Some(SessionStatus::Running)) => true, // ProposalJudged
        (Some(SessionStatus::Running), Some(SessionStatus::Ended(_))) => true, // EndSession/EffectのEndSession
        _ => false,
    }
}

proptest! {
    /// 不変条件2(整合性)+1(決定性): decideが返すイベント列は必ずエラーなくapplyできる。
    /// かつ、その結果得られたイベント列(冒険記)を最初からfoldし直せば同じ状態に戻る
    /// (リプレイ可能性)。
    #[test]
    fn invariant_decide_output_is_applyable_and_replay_is_deterministic(steps in steps_strategy()) {
        let mut state: Option<Session> = None;
        let mut log: Vec<Event> = Vec::new();

        for step in steps {
            if let Ok(events) = decide(state.as_ref(), &step.actor, step.cmd) {
                for event in &events {
                    let next = apply(state.clone(), event);
                    prop_assert!(
                        next.is_some(),
                        "decideが返したイベントがapplyで拒否された: {:?}",
                        event
                    );
                    state = next;
                    log.push(event.clone());
                }
            }
        }

        let mut replay: Option<Session> = None;
        for event in &log {
            replay = apply(replay, event);
        }
        prop_assert_eq!(replay, state);
    }

    /// 不変条件3(状態機械の合法性)。
    #[test]
    fn invariant_state_machine_never_takes_illegal_transitions(steps in steps_strategy()) {
        let mut state: Option<Session> = None;

        for step in steps {
            if let Ok(events) = decide(state.as_ref(), &step.actor, step.cmd) {
                for event in &events {
                    let before = state.as_ref().map(|s| s.status.clone());
                    let next = apply(state.clone(), event);
                    prop_assert!(next.is_some());
                    let after = next.as_ref().map(|s| s.status.clone());
                    prop_assert!(
                        is_legal_status_transition(&before, &after),
                        "不正な状態遷移: {:?} -> {:?} (event={:?})",
                        before,
                        after,
                        event
                    );
                    state = next;
                }
            }
        }
    }

    /// 不変条件4(保存則): カードは`CardDealt`イベント無しに手札/tableに現れず、
    /// `CardRemoved`イベント無しに消えない。
    #[test]
    fn invariant_cards_only_change_via_card_dealt(steps in steps_strategy()) {
        let mut state: Option<Session> = None;

        for step in steps {
            if let Ok(events) = decide(state.as_ref(), &step.actor, step.cmd) {
                for event in &events {
                    let before = state.as_ref().map(all_instances).unwrap_or_default();
                    let next = apply(state.clone(), event);
                    prop_assert!(next.is_some());
                    let after = all_instances(next.as_ref().unwrap());

                    match event {
                        Event::CardDealt { to, card, instance } => {
                            let mut expected = before.clone();
                            let inserted =
                                expected.insert((Some(to.clone()), instance.clone(), card.clone()));
                            prop_assert!(
                                inserted,
                                "同じCardInstanceIdが重複して配布された: {:?}",
                                instance
                            );
                            prop_assert_eq!(after, expected);
                        }
                        Event::CardRemoved { from, card, instance, .. } => {
                            let mut expected = before.clone();
                            let removed =
                                expected.remove(&(Some(from.clone()), instance.clone(), card.clone()));
                            prop_assert!(
                                removed,
                                "存在しないCardInstanceIdが除去された: {:?}",
                                instance
                            );
                            prop_assert_eq!(after, expected);
                        }
                        _ => {
                            prop_assert_eq!(after, before, "CardDealt/CardRemoved以外でカード集合が変化した: {:?}", event);
                        }
                    }
                    state = next;
                }
            }
        }
    }
}

// ---- 不変条件5(パッチ安全性)専用の生成器 ----
//
// `Session`/`ScenarioPatch`をそのまま`any::<T>()`で生成すると、シーンid・カードidが
// 互いに独立な乱数文字列になり、validateがほぼ常に「現在シーンが存在しない」で
// 拒否してしまい受理側をほとんど検証できない。id空間を小さな固定プールに絞った
// 専用生成器で、パッチとシナリオが同じidを参照し合う確率を上げる。

fn small_id_pool() -> impl Strategy<Value = String> {
    proptest::sample::select(vec!["a".to_string(), "b".to_string(), "c".to_string()])
}

fn small_card_def(id: String) -> CardDef {
    CardDef {
        id: CardId(format!("c-{id}")),
        name: BoundedString::try_new(id).unwrap(),
        kind: CardKind::Item,
        text: BoundedString::try_new("").unwrap(),
        tags: vec![],
        effects: vec![],
        requires: vec![],
    }
}

fn small_scene_def(id: String) -> SceneDef {
    SceneDef {
        id: SceneId(format!("s-{id}")),
        kind: SceneKind::Conversation,
        narration: BoundedString::try_new("").unwrap(),
        deals: vec![],
        exits: vec![],
    }
}

fn small_scenario_strategy() -> impl Strategy<Value = Scenario> {
    (
        proptest::collection::hash_set(small_id_pool(), 0..=3),
        proptest::collection::hash_set(small_id_pool(), 1..=3),
    )
        .prop_map(|(card_ids, scene_ids)| {
            let card_defs = card_ids.into_iter().map(small_card_def).collect();
            let scenes = scene_ids
                .into_iter()
                .map(small_scene_def)
                .collect::<Vec<_>>();
            Scenario {
                meta: ScenarioMeta {
                    id: ScenarioId("scenario".to_string()),
                    title: BoundedString::try_new("").unwrap(),
                    author: BoundedString::try_new("").unwrap(),
                    forked_from: None,
                },
                card_defs,
                phases: vec![PhaseDef {
                    phase: Phase::Opening,
                    scenes,
                }],
            }
        })
}

/// 現在シーンは必ずシナリオ内の実在シーンから選ぶ(validateが即座に「現在シーン
/// 消失」で拒否してしまわないため)。手札のカードは既存card_def参照/未定義参照の
/// 両方を生成し、受理・拒否どちらの経路も自然に踏む。
fn small_session_strategy() -> impl Strategy<Value = Session> {
    small_scenario_strategy().prop_flat_map(|scenario| {
        let scene_ids: Vec<SceneId> = scenario
            .phases
            .iter()
            .flat_map(|p| p.scenes.iter().map(|s| s.id.clone()))
            .collect();
        let card_ids: Vec<CardId> = scenario.card_defs.iter().map(|c| c.id.clone()).collect();

        let current_scene = proptest::sample::select(scene_ids);
        let dealt_card = if card_ids.is_empty() {
            small_id_pool()
                .prop_map(|id| CardId(format!("undefined-{id}")))
                .boxed()
        } else {
            prop_oneof![
                proptest::sample::select(card_ids).boxed(),
                small_id_pool()
                    .prop_map(|id| CardId(format!("undefined-{id}")))
                    .boxed(),
            ]
            .boxed()
        };

        let scenario_for_session = scenario.clone();
        (current_scene, proptest::collection::vec(dealt_card, 0..=3)).prop_map(
            move |(scene, dealt_cards)| {
                let mut hands = HashMap::new();
                hands.insert(
                    CharacterId("ch1".to_string()),
                    dealt_cards
                        .into_iter()
                        .enumerate()
                        .map(|(i, card)| CardInstance {
                            id: CardInstanceId(format!("ci-{i}")),
                            card,
                        })
                        .collect(),
                );
                let mut roles = HashMap::new();
                roles.insert(UserId("gm".to_string()), Role::Gm);
                Session {
                    scenario: ScenarioSnapshot(scenario_for_session.clone()),
                    party: vec![],
                    status: SessionStatus::Paused {
                        proposal: ProposalId("p".to_string()),
                    },
                    roles,
                    phase: Phase::Opening,
                    scene,
                    hands,
                    table: vec![],
                    pending_proposal: None,
                    proposal_seq: 0,
                    card_instance_seq: 0,
                    scene_local_instances: vec![],
                }
            },
        )
    })
}

fn small_patch_strategy() -> impl Strategy<Value = ScenarioPatch> {
    let op = prop_oneof![
        small_id_pool().prop_map(|id| PatchOp::DealCard {
            card: CardId(format!("c-{id}")),
            to: Target::Party,
        }),
        small_id_pool().prop_map(|id| PatchOp::AddCardDef(small_card_def(id))),
        small_id_pool().prop_map(|id| PatchOp::ReplaceScene(small_scene_def(id))),
    ];
    proptest::collection::vec(op, 0..=3).prop_map(|ops| ScenarioPatch {
        ops,
        note: BoundedString::try_new("プロパティテスト用パッチ").unwrap(),
    })
}

proptest! {
    /// 不変条件5(パッチ安全性): validateを通ったパッチ(=ApplyPatchが受理された)は、
    /// 適用後も「現在シーンが存在」「配布済みカードの定義が解決可能」を壊さない。
    #[test]
    fn invariant_validated_patch_preserves_session_safety(
        session in small_session_strategy(),
        patch in small_patch_strategy(),
    ) {
        let gm = UserId("gm".to_string());
        if let Ok(events) = decide(Some(&session), &gm, Command::ApplyPatch { patch }) {
            let mut state = Some(session);
            for event in &events {
                state = apply(state, event);
            }
            let session = state.expect("decideの出力は必ずapply可能(不変条件2)");

            prop_assert!(
                session.scenario.0.scene_def(&session.scene).is_some(),
                "パッチ適用後に現在シーンが解決できなくなった"
            );
            for cards in session.hands.values() {
                for ci in cards {
                    prop_assert!(
                        session.scenario.0.card_def(&ci.card).is_some(),
                        "パッチ適用後に配布済みカードの定義が解決できなくなった: {:?}",
                        ci.card
                    );
                }
            }
        }
    }
}
