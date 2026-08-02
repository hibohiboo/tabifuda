use std::process::ExitCode;

mod chronicle;
mod fork;
mod oplog;
mod party;
mod play;
mod save;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, path] if cmd == "lint" => run_lint(path),
        [cmd, path] if cmd == "play" => run_play(path, None),
        [cmd, path, flag, party_path] if cmd == "play" && flag == "--party" => {
            run_play(path, Some(party_path))
        }
        [cmd, flag, path] if cmd == "play" && flag == "--resume" => run_play_resume(path),
        _ => {
            eprintln!("usage: tabifuda-cli lint <file>");
            eprintln!("       tabifuda-cli play <file> [--party <party-file>]");
            eprintln!("       tabifuda-cli play --resume <session-file>");
            ExitCode::FAILURE
        }
    }
}

fn load_scenario(path: &str) -> Result<tabifuda_core::Scenario, ExitCode> {
    let text = std::fs::read_to_string(path).map_err(|err| {
        eprintln!("failed to read {path}: {err}");
        ExitCode::FAILURE
    })?;
    serde_json::from_str(&text).map_err(|err| {
        eprintln!("failed to parse {path}: {err}");
        ExitCode::FAILURE
    })
}

fn run_play(path: &str, party_path: Option<&str>) -> ExitCode {
    let scenario = match load_scenario(path) {
        Ok(scenario) => scenario,
        Err(code) => return code,
    };
    let party = match party_path {
        Some(party_path) => match party::load(std::path::Path::new(party_path)) {
            Ok(party) => Some(party),
            Err(err) => {
                eprintln!("failed to load {party_path}: {err}");
                return ExitCode::FAILURE;
            }
        },
        None => None,
    };
    play::run(
        scenario,
        std::path::Path::new(path),
        party,
        party_path.map(std::path::Path::new),
    );
    ExitCode::SUCCESS
}

fn run_play_resume(path: &str) -> ExitCode {
    play::resume(std::path::Path::new(path));
    ExitCode::SUCCESS
}

fn run_lint(path: &str) -> ExitCode {
    let scenario = match load_scenario(path) {
        Ok(scenario) => scenario,
        Err(code) => return code,
    };

    let findings = tabifuda_core::lint(&scenario);
    if findings.is_empty() {
        println!("ok: no issues found");
        return ExitCode::SUCCESS;
    }

    let mut has_error = false;
    for finding in &findings {
        println!("{finding}");
        if finding.severity == tabifuda_core::Severity::Error {
            has_error = true;
        }
    }

    if has_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
