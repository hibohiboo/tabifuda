use std::process::ExitCode;

mod oplog;
mod play;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, path] if cmd == "lint" => run_lint(path),
        [cmd, path] if cmd == "play" => run_play(path),
        _ => {
            eprintln!("usage: tabifuda-cli lint <file>");
            eprintln!("       tabifuda-cli play <file>");
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

fn run_play(path: &str) -> ExitCode {
    let scenario = match load_scenario(path) {
        Ok(scenario) => scenario,
        Err(code) => return code,
    };
    play::run(scenario);
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
