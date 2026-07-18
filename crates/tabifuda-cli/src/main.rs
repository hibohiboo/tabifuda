use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, path] if cmd == "lint" => run_lint(path),
        _ => {
            eprintln!("usage: tabifuda-cli lint <file>");
            ExitCode::FAILURE
        }
    }
}

fn run_lint(path: &str) -> ExitCode {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            return ExitCode::FAILURE;
        }
    };
    let scenario: tabifuda_core::Scenario = match serde_json::from_str(&text) {
        Ok(scenario) => scenario,
        Err(err) => {
            eprintln!("failed to parse {path}: {err}");
            return ExitCode::FAILURE;
        }
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
