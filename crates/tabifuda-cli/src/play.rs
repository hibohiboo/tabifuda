//! CLIプレイループ。docs/tasks/projects/phase2/task.md C3に対応。
//! ここは翻訳層(標準入出力↔Command/Event)であり、ルール分岐は持たない。
//! ソロプレイのため、単一ユーザーがPlayer/GM両ロールを兼ねる
//! (domain-model.md「ソロMVPでの簡略化」)。

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use tabifuda_core::{
    apply, decide, BoundedString, CardDef, CardId, CardInstance, CardKind, Character, CharacterId,
    Command, Event, PatchOp, RuleError, Scenario, ScenarioId, ScenarioPatch, Session,
    SessionStatus, Target, UserId,
};

use crate::{chronicle, fork, oplog};

const SOLO_CHARACTER_ID: &str = "hunter";
const SOLO_CHARACTER_NAME: &str = "旅人";

pub fn run(scenario: Scenario, scenario_path: &Path) {
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
                maybe_save_fork(session, &event_log, scenario_path, &mut lines);
                return;
            }
            SessionStatus::Paused { .. } => {
                let proposal = session
                    .pending_proposal
                    .clone()
                    .expect("Pausedならpending_proposalがある");
                println!(
                    "\n提案が届いています(GMとして応答してください): 「{}」",
                    proposal.text.as_str()
                );
                print!("y=採用して再開 / n=却下して再開 / c=カードを配って応える [y/n/c]: ");
                io::stdout().flush().ok();
                let Some(Ok(input)) = lines.next() else {
                    return;
                };
                let input = input.trim().to_string();
                if input.eq_ignore_ascii_case("c") {
                    // domain-model.md「提案への応答UI(CLIの決定)」: カード名と
                    // 回答文からCardDefを組み立て、AddCardDef+DealCardを1パッチで
                    // 発行する。適用後もPausedのまま(y/nで締めるまで繰り返せる)。
                    let Some(name) = prompt_bounded_text::<200>(&mut lines, "カード名: ")
                    else {
                        return;
                    };
                    let Some(name) = name else { continue };
                    let Some(text) =
                        prompt_bounded_text::<2000>(&mut lines, "回答文(カードを出すと表示): ")
                    else {
                        return;
                    };
                    let Some(text) = text else { continue };
                    let card_id = next_gm_card_id(&session.scenario.0);
                    let def = CardDef {
                        id: card_id.clone(),
                        name,
                        kind: CardKind::Scenario,
                        text,
                        tags: vec![],
                        effects: vec![],
                        requires: vec![],
                    };
                    let patch = ScenarioPatch {
                        ops: vec![
                            PatchOp::AddCardDef(def),
                            PatchOp::DealCard {
                                card: card_id,
                                to: Target::Party,
                            },
                        ],
                        note: BoundedString::try_new("提案に応えてカードを配布")
                            .expect("定型文は上限内"),
                    };
                    let (next, result) = issue(
                        state.take(),
                        &actor,
                        Command::ApplyPatch { patch },
                        &mut event_log,
                    );
                    state = next;
                    match result {
                        Ok(()) => println!("カードを配りました(裁定待ちのまま)。"),
                        Err(err) => println!("カードを配れませんでした: {err}"),
                    }
                } else if input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("n") {
                    let accepted = input.eq_ignore_ascii_case("y");
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
                } else {
                    println!("不明なコマンドです。");
                }
            }
            SessionStatus::Running => {
                println!("\n=== {} ===", session.scene.0);
                if let Some(scene_def) = session.scenario.0.scene_def(&session.scene) {
                    println!("{}", scene_def.narration.as_str());
                }

                // Markerは「選んだ記録」としてsession.handsには残すが、選ぶ対象
                // ではない世界の状態を示す印なので一覧には出さない
                // (domain-model.md「カードの消費・除去」参照)。
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
                    .filter(|(_, def)| {
                        !matches!(def.as_ref().map(|d| &d.kind), Some(CardKind::Marker))
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
                    let is_dialogue =
                        matches!(def.as_ref().map(|d| &d.kind), Some(CardKind::Dialogue));
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
                    match result {
                        // カード本文の開示(domain-model.md「カード使用時のtext表示
                        // (CLIの決定)」。GMが配った質問カードの回答もここで読める)。
                        Ok(()) => {
                            if let Some(text) = def.as_ref().map(|d| d.text.as_str()) {
                                if !text.is_empty() {
                                    println!("{text}");
                                }
                            }
                        }
                        Err(err) => println!("カードを出せませんでした: {err}"),
                    }
                } else {
                    println!("不明なコマンドです。");
                }
            }
        }
    }
}

/// セッション終了後のフォーク保存(domain-model.md「フォーク出力」)。
/// `ScenarioPatched`が1回以上あるときだけ保存を尋ね、パッチ適用済みの
/// `session.scenario`を基に`fork::build_fork`で構築して書き出す。
/// EOF・y以外の入力・書き込み失敗はいずれも保存せず終了する(冒険記の
/// 表示は済んでいるため、ここでの失敗はセッション自体を壊さない)。
fn maybe_save_fork(
    session: &Session,
    events: &[Event],
    scenario_path: &Path,
    lines: &mut impl Iterator<Item = io::Result<String>>,
) {
    if !events
        .iter()
        .any(|e| matches!(e, Event::ScenarioPatched { .. }))
    {
        return;
    }
    print!("\nシナリオはセッション中に改編されました。フォークとして保存しますか? [y/n]: ");
    io::stdout().flush().ok();
    let Some(Ok(input)) = lines.next() else {
        return;
    };
    if !input.trim().eq_ignore_ascii_case("y") {
        return;
    }

    let path = fork_output_path(scenario_path);
    // meta.idは出力ファイル名の語幹と揃える(domain-model.md「フォーク出力」)。
    let fork_id = ScenarioId(
        path.file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("{}-fork", session.scenario.0.meta.id.0)),
    );
    let forked = fork::build_fork(&session.scenario.0, events, fork_id);
    let json = match serde_json::to_string_pretty(&forked) {
        Ok(json) => json,
        Err(err) => {
            println!("フォークを構築できませんでした: {err}");
            return;
        }
    };
    match std::fs::write(&path, json + "\n") {
        Ok(()) => println!("フォークを保存しました: {}", path.display()),
        Err(err) => println!("フォークを保存できませんでした: {err}"),
    }
}

/// 元ファイルの隣に`{語幹}-fork.json`(既存ファイルと衝突したら
/// `{語幹}-fork-2.json`から連番)で出力パスを決める。
fn fork_output_path(scenario_path: &Path) -> PathBuf {
    let stem = scenario_path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "scenario".to_string());
    let dir = scenario_path.parent().unwrap_or_else(|| Path::new("."));
    let first = dir.join(format!("{stem}-fork.json"));
    if !first.exists() {
        return first;
    }
    (2..)
        .map(|n| dir.join(format!("{stem}-fork-{n}.json")))
        .find(|p| !p.exists())
        .expect("連番はいつか空きに当たる")
}

/// GMが配るカードのCardId発番(`gm-card-{n}`)。既存card_defsと重複しない
/// 最小の連番を探す。一意性の検証責務はコアのvalidate(DuplicateCardId)にあり、
/// ここは入力の組み立てのみ(domain-model.md「提案への応答UI(CLIの決定)」)。
fn next_gm_card_id(scenario: &Scenario) -> CardId {
    (1..)
        .map(|n| CardId(format!("gm-card-{n}")))
        .find(|id| scenario.card_def(id).is_none())
        .expect("連番はいつか空きに当たる")
}

/// 戻り値は2重Option: 外側`None`はEOF(呼び出し元は終了する)。内側`None`は
/// 「本文なし」(空入力、または上限超過エラーで本文を諦めた場合)を表す。
/// 内側`None`の扱いは呼び出し元に委ねる(自由入力なら本文なしで続行、
/// 提案なら提案自体を取りやめる、等)。
fn prompt_bounded_text<const N: usize>(
    lines: &mut impl Iterator<Item = io::Result<String>>,
    prompt: &str,
) -> Option<Option<BoundedString<N>>> {
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
