//! 冒険記(イベント列→テキスト)の描画。docs/tasks/projects/phase2/task.md C4に対応。
//! domain-model.md「ログUI(Web版)はEvent列をそのまま時系列カードとして
//! 描画する」の思想をCLI向けのプレーンテキストとして翻訳したもの。
//! ここは翻訳のみ(ルール分岐を持たない)。運用ログ(oplog)とは異なり、
//! 冒険記は自由入力本文を含めてよい(cross-cutting.md「2種類のログを
//! 区別する」: ドメインログ=冒険記は製品価値そのもの)。

use tabifuda_core::{CardId, Event, PatchOp, Scenario};

pub fn render(events: &[Event]) -> String {
    let mut scenario: Option<Scenario> = None;
    let mut lines: Vec<String> = Vec::new();

    for event in events {
        match event {
            Event::SessionStarted {
                scenario: snapshot,
                party,
                ..
            } => {
                lines.push(format!(
                    "◆ 冒険「{}」が始まった。",
                    snapshot.0.meta.title.as_str()
                ));
                for character in party {
                    lines.push(format!("  参加者: {}", character.name));
                }
                scenario = Some(snapshot.0.clone());
            }
            Event::SceneEntered {
                scene, narration, ..
            } => {
                lines.push(String::new());
                lines.push(format!("--- {} ---", scene.0));
                lines.push(narration.clone());
            }
            Event::CardDealt { to, card, .. } => {
                lines.push(format!(
                    "（{}に「{}」が配られた）",
                    to.0,
                    card_name(scenario.as_ref(), card)
                ));
            }
            Event::CardPlayed {
                by,
                card,
                free_text,
            } => {
                let name = card_name(scenario.as_ref(), card);
                match free_text {
                    Some(text) => lines.push(format!(
                        "{}は「{name}」を出した。「{}」",
                        by.0,
                        text.as_str()
                    )),
                    None => lines.push(format!("{}は「{name}」を出した。", by.0)),
                }
            }
            // 消費・シーン離脱による自動除去は物語に不要な機構的詳細のため描画しない
            // (domain-model.md「カードの消費・除去」参照)。
            Event::CardRemoved { .. } => {}
            Event::EffectApplied { .. } => {
                lines.push("（未解決の効果が記録された)".to_string());
            }
            Event::PhaseAdvanced { phase } => {
                lines.push(format!("――フェーズが{phase:?}へ進んだ。"));
            }
            Event::ProposalSubmitted { by, text, .. } => {
                lines.push(format!("{}が提案した: 「{}」", by.0, text.as_str()));
            }
            Event::ScenarioPatched { patch } => {
                // パッチで足されたカードの名前がID表示に落ちないよう、AddCardDef分を
                // 名前解決に反映する(表示用の翻訳。patch::apply_opsの再実装ではない。
                // domain-model.md「カード使用時のtext表示(CLIの決定)」参照)。
                if let Some(s) = scenario.as_mut() {
                    for op in &patch.ops {
                        if let PatchOp::AddCardDef(def) = op {
                            s.card_defs.push(def.clone());
                        }
                    }
                }
                lines.push(format!(
                    "GMがシナリオを改修した: 「{}」",
                    patch.note.as_str()
                ));
            }
            Event::ProposalJudged { accepted, .. } => {
                lines.push(format!(
                    "GMは提案を{}。",
                    if *accepted {
                        "採用した"
                    } else {
                        "却下した"
                    }
                ));
            }
            Event::SessionEnded { outcome } => {
                lines.push(String::new());
                lines.push(format!("=== 冒険の終わり: {outcome:?} ==="));
            }
            _ => lines.push("（未知の出来事が記録された)".to_string()),
        }
    }

    lines.join("\n")
}

fn card_name(scenario: Option<&Scenario>, card: &CardId) -> String {
    scenario
        .and_then(|s| s.card_def(card))
        .map(|def| def.name.as_str().to_string())
        .unwrap_or_else(|| card.0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tabifuda_core::{BoundedString, CharacterId, Outcome};

    #[test]
    fn render_includes_free_text_body_unlike_oplog() {
        let secret = "討伐を終え、村へ戻ります。";
        let events = vec![Event::CardPlayed {
            by: CharacterId("hunter".to_string()),
            card: CardId("end_victory".to_string()),
            free_text: Some(BoundedString::try_new(secret).unwrap()),
        }];
        let text = render(&events);
        assert!(text.contains(secret));
    }

    #[test]
    fn render_falls_back_to_card_id_when_scenario_unknown() {
        let events = vec![Event::CardDealt {
            to: CharacterId("hunter".to_string()),
            card: CardId("mystery".to_string()),
            instance: tabifuda_core::CardInstanceId("mystery-0".to_string()),
        }];
        let text = render(&events);
        assert!(text.contains("mystery"));
    }

    /// ScenarioPatchedのAddCardDef分が名前解決に反映され、パッチで足された
    /// カードがID表示に落ちないこと(domain-model.md「カード使用時のtext表示
    /// (CLIの決定)」)。
    #[test]
    fn render_resolves_card_added_by_patch() {
        use std::collections::HashMap;
        use tabifuda_core::{
            CardDef, CardInstanceId, CardKind, Phase, Scenario, ScenarioId, ScenarioMeta,
            ScenarioPatch, ScenarioSnapshot, SceneId,
        };

        let scenario = Scenario {
            meta: ScenarioMeta {
                id: ScenarioId("s".to_string()),
                title: BoundedString::try_new("題").unwrap(),
                author: BoundedString::try_new("a").unwrap(),
                forked_from: None,
            },
            card_defs: vec![],
            phases: vec![],
        };
        let def = CardDef {
            id: CardId("gm-card-1".to_string()),
            name: BoundedString::try_new("獣の目撃情報を尋ねる").unwrap(),
            kind: CardKind::Scenario,
            text: BoundedString::try_new("銀色の大狼だという。").unwrap(),
            tags: vec![],
            effects: vec![],
            requires: vec![],
        };
        let events = vec![
            Event::SessionStarted {
                scenario: ScenarioSnapshot(scenario),
                party: vec![],
                roles: HashMap::new(),
                initial_phase: Phase::Opening,
                initial_scene: SceneId("op".to_string()),
            },
            Event::ScenarioPatched {
                patch: ScenarioPatch {
                    ops: vec![PatchOp::AddCardDef(def)],
                    note: BoundedString::try_new("提案に応えてカードを配布").unwrap(),
                },
            },
            Event::CardDealt {
                to: CharacterId("hunter".to_string()),
                card: CardId("gm-card-1".to_string()),
                instance: CardInstanceId("gm-card-1-0".to_string()),
            },
        ];
        let text = render(&events);
        assert!(text.contains("獣の目撃情報を尋ねる"));
        assert!(!text.contains("gm-card-1"));
    }

    #[test]
    fn render_shows_outcome_on_session_ended() {
        let events = vec![Event::SessionEnded {
            outcome: Outcome::Victory,
        }];
        let text = render(&events);
        assert!(text.contains("Victory"));
    }
}
