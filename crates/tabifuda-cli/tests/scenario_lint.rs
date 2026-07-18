//! shared/scenarios/ 配下の同梱テンプレシナリオ全件に対するlint実行。
//! docs/design/test-strategy.md §2「シナリオデータ(lintをテストとして実行)」に対応。

use std::path::Path;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tabifuda-cli"))
}

#[test]
fn all_bundled_scenarios_pass_lint() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../shared/scenarios");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        checked += 1;
        let output = bin().arg("lint").arg(&path).output().unwrap();
        assert!(
            output.status.success(),
            "{} failed lint:\n{}",
            path.display(),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    assert!(
        checked > 0,
        "no scenario files found under {}",
        dir.display()
    );
}
