//! 運用ログの薄い層。docs/design/cross-cutting.md「2種類のログを区別する」に対応。
//! 種別+IDのみを記録し、自由入力本文(free_text/提案text/パッチnote等)を
//! 書かない。ここは翻訳(Command/Eventから安全な要約文字列を作るだけ)であり、
//! ルール分岐は持たない。

use tabifuda_core::{Command, Event, RuleError, UserId};

pub fn command_result_line(
    actor: &UserId,
    cmd: &Command,
    result: &Result<Vec<Event>, RuleError>,
) -> String {
    let cmd_desc = command_summary(cmd);
    match result {
        Ok(events) => {
            let kinds: Vec<&str> = events.iter().map(|e| event_kind(e)).collect();
            format!(
                "actor={} cmd={cmd_desc} result=ok events=[{}]",
                actor.0,
                kinds.join(",")
            )
        }
        Err(err) => format!("actor={} cmd={cmd_desc} result=err error={err}", actor.0),
    }
}

/// Command種別+ID(自由入力本文を含むフィールドは意図的に落とす)。
fn command_summary(cmd: &Command) -> String {
    match cmd {
        Command::StartSession { .. } => "StartSession".to_string(),
        Command::PlayCard { by, card, .. } => format!("PlayCard(by={},card={})", by.0, card.0),
        Command::Propose { by, .. } => format!("Propose(by={})", by.0),
        Command::ApplyPatch { .. } => "ApplyPatch".to_string(),
        Command::JudgeProposal { proposal, accepted } => {
            format!("JudgeProposal(proposal={},accepted={accepted})", proposal.0)
        }
        Command::GmAdvance { to } => format!("GmAdvance(to={})", to.0),
        Command::EndSession { outcome } => format!("EndSession(outcome={outcome:?})"),
        _ => "Unknown".to_string(),
    }
}

/// Event種別のみ(本文フィールドは一切含めない)。
fn event_kind(event: &Event) -> &'static str {
    match event {
        Event::SessionStarted { .. } => "SessionStarted",
        Event::SceneEntered { .. } => "SceneEntered",
        Event::CardDealt { .. } => "CardDealt",
        Event::CardPlayed { .. } => "CardPlayed",
        Event::CardRemoved { .. } => "CardRemoved",
        Event::EffectApplied { .. } => "EffectApplied",
        Event::PhaseAdvanced { .. } => "PhaseAdvanced",
        Event::ProposalSubmitted { .. } => "ProposalSubmitted",
        Event::ScenarioPatched { .. } => "ScenarioPatched",
        Event::ProposalJudged { .. } => "ProposalJudged",
        Event::SessionEnded { .. } => "SessionEnded",
        _ => "Unknown",
    }
}

// テスト名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md D2)
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use tabifuda_core::{BoundedString, CardId, CardInstanceId, CharacterId, ProposalId};

    #[test]
    fn 運用ログはPlayCardの自由入力本文を書かない() {
        let secret = "この自由入力の本文はログに書かれてはいけない";
        let cmd = Command::PlayCard {
            by: CharacterId("ch1".to_string()),
            card: CardInstanceId("ci1".to_string()),
            free_text: Some(BoundedString::try_new(secret).unwrap()),
        };
        let events = vec![Event::CardPlayed {
            by: CharacterId("ch1".to_string()),
            card: CardId("c1".to_string()),
            free_text: Some(BoundedString::try_new(secret).unwrap()),
        }];
        let line = command_result_line(&UserId("solo".to_string()), &cmd, &Ok(events));
        assert!(!line.contains(secret), "log line leaked free_text: {line}");
    }

    #[test]
    fn 運用ログはProposeの提案本文を書かない() {
        let secret = "この提案文もログに書かれてはいけない";
        let cmd = Command::Propose {
            by: CharacterId("ch1".to_string()),
            text: BoundedString::try_new(secret).unwrap(),
        };
        let events = vec![Event::ProposalSubmitted {
            id: ProposalId("proposal-0".to_string()),
            by: CharacterId("ch1".to_string()),
            text: BoundedString::try_new(secret).unwrap(),
        }];
        let line = command_result_line(&UserId("solo".to_string()), &cmd, &Ok(events));
        assert!(
            !line.contains(secret),
            "log line leaked proposal text: {line}"
        );
    }

    #[test]
    fn 運用ログはエラー時もpanicせず種別を記録する() {
        let cmd = Command::EndSession {
            outcome: tabifuda_core::Outcome::Victory,
        };
        let line = command_result_line(
            &UserId("solo".to_string()),
            &cmd,
            &Err(RuleError::SessionEnded),
        );
        assert!(line.contains("result=err"));
    }
}
