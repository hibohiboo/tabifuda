//! フォーク出力: セッション終了後、パッチ適用済みシナリオを別シナリオとして
//! 書き出すための構築(domain-model.md「フォーク出力」)。
//! ここは翻訳層の純粋関数であり、IO(確認プロンプト・パス発番・書き込み)は
//! play.rsが担う。

use tabifuda_core::{Deal, Event, PatchOp, Scenario, ScenarioId, SceneId};

/// パッチ適用済みの最終シナリオ(`Session.scenario`)とイベントログから
/// フォークシナリオを構築する。
/// - `meta.id`: 呼び出し側が出力ファイル名の語幹から決めた`fork_id`
/// - `meta.forked_from`: 元シナリオのid
/// - `PatchOp::DealCard`分を「配布時に居たシーン」の`deals`へ追加する
///   (`DealCard`は「その場で配る」で入場時配布に残らないため、この変換を
///   しないとフォークを次に遊んでも配られない。同一シーン×同一カード×
///   同一宛先の重複は1件にまとめる)
pub fn build_fork(final_scenario: &Scenario, events: &[Event], fork_id: ScenarioId) -> Scenario {
    let mut fork = final_scenario.clone();
    fork.meta.forked_from = Some(final_scenario.meta.id.clone());
    fork.meta.id = fork_id;

    // SceneEnteredで現在シーンを追跡する(ScenarioPatchedはPaused中に起き、
    // Pausedはシーンを変えないため、直近の入場シーン=配布時のシーン)。
    let mut current_scene: Option<SceneId> = None;
    for event in events {
        match event {
            Event::SceneEntered { scene, .. } => current_scene = Some(scene.clone()),
            Event::ScenarioPatched { patch } => {
                let Some(scene_id) = current_scene.as_ref() else {
                    continue;
                };
                for op in &patch.ops {
                    if let PatchOp::DealCard { card, to } = op {
                        let deal = Deal {
                            card: card.clone(),
                            to: to.clone(),
                        };
                        add_deal(&mut fork, scene_id, deal);
                    }
                }
            }
            _ => {}
        }
    }
    fork
}

fn add_deal(fork: &mut Scenario, scene_id: &SceneId, deal: Deal) {
    for phase in &mut fork.phases {
        for scene in &mut phase.scenes {
            if &scene.id == scene_id {
                if !scene.deals.contains(&deal) {
                    scene.deals.push(deal);
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tabifuda_core::{
        BoundedString, CardDef, CardKind, Phase, PhaseDef, ScenarioMeta, ScenarioPatch, SceneDef,
        SceneKind, Target,
    };

    fn short<const N: usize>(s: &str) -> BoundedString<N> {
        BoundedString::try_new(s).unwrap()
    }

    fn fixture_scenario() -> Scenario {
        Scenario {
            meta: ScenarioMeta {
                id: ScenarioId("orig".to_string()),
                title: short("題"),
                author: short("a"),
                forked_from: None,
            },
            card_defs: vec![CardDef {
                id: tabifuda_core::CardId("gm-card-1".to_string()),
                name: short("尋ねる"),
                kind: CardKind::Scenario,
                text: short("回答"),
                tags: vec![],
                effects: vec![],
                requires: vec![],
            }],
            phases: vec![PhaseDef {
                phase: Phase::Opening,
                scenes: vec![SceneDef {
                    id: SceneId("op".to_string()),
                    kind: SceneKind::Conversation,
                    narration: short("開幕"),
                    deals: vec![],
                    exits: vec![],
                }],
            }],
        }
    }

    fn deal_patch_events() -> Vec<Event> {
        vec![
            Event::SceneEntered {
                scene: SceneId("op".to_string()),
                narration: "開幕".to_string(),
                local_instances: vec![],
            },
            Event::ScenarioPatched {
                patch: ScenarioPatch {
                    ops: vec![PatchOp::DealCard {
                        card: tabifuda_core::CardId("gm-card-1".to_string()),
                        to: Target::Party,
                    }],
                    note: short("配布"),
                },
            },
        ]
    }

    #[test]
    fn build_fork_sets_id_and_forked_from() {
        let fork = build_fork(
            &fixture_scenario(),
            &[],
            ScenarioId("orig-fork".to_string()),
        );
        assert_eq!(fork.meta.id, ScenarioId("orig-fork".to_string()));
        assert_eq!(fork.meta.forked_from, Some(ScenarioId("orig".to_string())));
    }

    /// DealCardパッチが配布時のシーンのdealsへ組み込まれる
    /// (domain-model.md「フォーク出力」の核。これが無いとフォークを
    /// 次に遊んでも配られない)。
    #[test]
    fn build_fork_merges_deal_card_into_scene_deals() {
        let fork = build_fork(
            &fixture_scenario(),
            &deal_patch_events(),
            ScenarioId("orig-fork".to_string()),
        );
        let scene = fork.scene_def(&SceneId("op".to_string())).unwrap();
        assert_eq!(
            scene.deals,
            vec![Deal {
                card: tabifuda_core::CardId("gm-card-1".to_string()),
                to: Target::Party,
            }]
        );
    }

    /// 同一シーン×同一カード×同一宛先は1件にまとめる(同じ提案対応中に
    /// 誤って二度配っても、フォークで二重配布にしない)。
    #[test]
    fn build_fork_deduplicates_identical_deals() {
        let mut events = deal_patch_events();
        events.extend(deal_patch_events().into_iter().skip(1));
        let fork = build_fork(
            &fixture_scenario(),
            &events,
            ScenarioId("orig-fork".to_string()),
        );
        let scene = fork.scene_def(&SceneId("op".to_string())).unwrap();
        assert_eq!(scene.deals.len(), 1);
    }
}
