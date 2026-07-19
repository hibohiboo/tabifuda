//! CLIプレイループ。docs/tasks/phase2-task.md C3に対応。
//! ここは翻訳層(標準入出力↔Command/Event)であり、ルール分岐は持たない。
//! ソロプレイのため、単一ユーザーがPlayer/GM両ロールを兼ねる
//! (domain-model.md「ソロMVPでの簡略化」)。

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use tabifuda_core::{
    apply, decide, BoundedString, CardDef, CardInstance, CardKind, Character, CharacterId, Command,
    Event, RuleError, Scenario, Session, SessionStatus, UserId,
};

use crate::{chronicle, oplog};

const SOLO_CHARACTER_ID: &str = "hunter";
const SOLO_CHARACTER_NAME: &str = "旅人";

pub fn run(scenario: Scenario) {
    let actor = UserId("solo".to_string());
    let character_id = CharacterId(SOLO_CHARACTER_ID.to_string());
    let character = Character {
        id: character_id.clone(),
        name: SOLO_CHARACTER_NAME.to_string(),
        stats: HashMap::new(),
        deck: vec![],
    };

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut event_log: Vec<Event> = Vec::new();
    let mut state: Option<Session> = None;
    let (next, result) = issue(
        state,
        &actor,
        Command::StartSession {
            scenario,
            party: vec![character],
        },
        &mut event_log,
    );
    state = next;
    if let Err(err) = result {
        println!("セッションを開始できませんでした: {err}");
        return;
    }

    loop {
        let Some(session) = state.as_ref() else {
            println!("セッションが存在しません。終了します。");
            return;
        };

        match &session.status {
            SessionStatus::Ended(outcome) => {
                println!("\n=== 冒険の終わり: {outcome:?} ===");
                println!("\n{}", chronicle::render(&event_log));
                return;
            }
            SessionStatus::Paused { .. } => {
                let proposal = session
                    .pending_proposal
                    .clone()
                    .expect("Pausedならpending_proposalがある");
                println!(
                    "\n提案が届いています(GMとして裁定してください): 「{}」",
                    proposal.text.as_str()
                );
                print!("採用しますか? [y/n]: ");
                io::stdout().flush().ok();
                let Some(Ok(input)) = lines.next() else {
                    return;
                };
                let accepted = input.trim().eq_ignore_ascii_case("y");
                let (next, result) = issue(
                    state.take(),
                    &actor,
                    Command::JudgeProposal {
                        proposal: proposal.id.clone(),
                        accepted,
                    },
                    &mut event_log,
                );
                state = next;
                if let Err(err) = result {
                    println!("裁定に失敗しました: {err}");
                }
            }
            SessionStatus::Running => {
                println!("\n=== {} ===", session.scene.0);
                if let Some(scene_def) = session.scenario.0.scene_def(&session.scene) {
                    println!("{}", scene_def.narration.as_str());
                }

                let hand: Vec<(CardInstance, Option<CardDef>)> = session
                    .hands
                    .get(&character_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|instance| {
                        let def = session.scenario.0.card_def(&instance.card).cloned();
                        (instance, def)
                    })
                    .collect();

                println!("手札:");
                for (i, (_, def)) in hand.iter().enumerate() {
                    let label = def
                        .as_ref()
                        .map(|d| d.name.as_str())
                        .unwrap_or("(不明なカード)");
                    println!("  [{}] {label}", i + 1);
                }
                println!("コマンド: 番号=カードを出す / p=提案する / q=中断");
                print!("> ");
                io::stdout().flush().ok();

                let Some(Ok(input)) = lines.next() else {
                    return;
                };
                let input = input.trim().to_string();

                if input.eq_ignore_ascii_case("q") {
                    println!("プレイを中断しました。");
                    return;
                } else if input.eq_ignore_ascii_case("p") {
                    let Some(text) =
                        prompt_bounded_text(&mut lines, "提案内容を入力してください: ")
                    else {
                        return;
                    };
                    let Some(text) = text else { continue };
                    let (next, result) = issue(
                        state.take(),
                        &actor,
                        Command::Propose {
                            by: character_id.clone(),
                            text,
                        },
                        &mut event_log,
                    );
                    state = next;
                    if let Err(err) = result {
                        println!("提案に失敗しました: {err}");
                    }
                } else if let Ok(index) = input.parse::<usize>() {
                    let Some((instance, def)) =
                        index.checked_sub(1).and_then(|i| hand.get(i)).cloned()
                    else {
                        println!("その番号のカードはありません。");
                        continue;
                    };
                    let is_dialogue = matches!(def.map(|d| d.kind), Some(CardKind::Dialogue));
                    let free_text = if is_dialogue {
                        // 内側のNoneは「本文なしで出す」(スキップ/入力エラー)であり、
                        // カードの使用自体は継続する。EOFのみ終了する。
                        match prompt_bounded_text(&mut lines, "自由入力(任意。Enterでスキップ): ")
                        {
                            Some(text) => text,
                            None => return,
                        }
                    } else {
                        None
                    };
                    let (next, result) = issue(
                        state.take(),
                        &actor,
                        Command::PlayCard {
                            by: character_id.clone(),
                            card: instance.id.clone(),
                            free_text,
                        },
                        &mut event_log,
                    );
                    state = next;
                    if let Err(err) = result {
                        println!("カードを出せませんでした: {err}");
                    }
                } else {
                    println!("不明なコマンドです。");
                }
            }
        }
    }
}

/// 戻り値は2重Option: 外側`None`はEOF(呼び出し元は終了する)。内側`None`は
/// 「本文なし」(空入力、または上限超過エラーで本文を諦めた場合)を表す。
/// 内側`None`の扱いは呼び出し元に委ねる(自由入力なら本文なしで続行、
/// 提案なら提案自体を取りやめる、等)。
fn prompt_bounded_text(
    lines: &mut impl Iterator<Item = io::Result<String>>,
    prompt: &str,
) -> Option<Option<BoundedString<4096>>> {
    print!("{prompt}");
    io::stdout().flush().ok();
    let Some(Ok(input)) = lines.next() else {
        return None;
    };
    if input.trim().is_empty() {
        return Some(None);
    }
    match BoundedString::try_new(input) {
        Ok(text) => Some(Some(text)),
        Err(err) => {
            println!("入力が長すぎます: {err}");
            Some(None)
        }
    }
}

/// decide→(受理時のみ)apply の連鎖+冒険記用ログ蓄積+運用ログの記録。
/// 翻訳のみでルール分岐は持たない。
fn issue(
    state: Option<Session>,
    actor: &UserId,
    cmd: Command,
    log: &mut Vec<Event>,
) -> (Option<Session>, Result<(), RuleError>) {
    let result = decide(state.as_ref(), actor, cmd.clone());
    eprintln!("[log] {}", oplog::command_result_line(actor, &cmd, &result));
    match result {
        Ok(events) => {
            let mut next = state;
            for event in &events {
                next = apply(next, event);
            }
            log.extend(events);
            (next, Ok(()))
        }
        Err(err) => (state, Err(err)),
    }
}
