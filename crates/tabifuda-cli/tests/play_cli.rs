//! `tabifuda-cli play <file>` の標準入出力結合テスト(通しプレイのスモーク)。
//! docs/tasks/phase2-task.md C4「CLIスモークテスト」に対応。ルール分岐は
//! テストしない(tabifuda-core側で済んでいる)。入力シーケンスは
//! fixtures/simple_hunt_playthrough.json を生成した通しプレイ
//! (提案→GM裁定→勝利)と同じ操作列。

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tabifuda-cli"))
}

fn scenario_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../shared/scenarios/simple-hunt.json")
}

fn run_play(input: &str) -> std::process::Output {
    let mut child = bin()
        .arg("play")
        .arg(scenario_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

/// [1]依頼を受ける(自由入力スキップ)→[3]獣の巣に到着する→提案→GM裁定(採用)
/// →[4]打ち倒す→[6]村に帰還を告げる(自由入力あり)、で勝利エンドまで到達する。
const VICTORY_INPUT: &str = "1\n\n3\np\n近道を探したい\ny\n4\n6\n最後の一言\n";

#[test]
fn play_reaches_victory_and_prints_chronicle() {
    let output = run_play(VICTORY_INPUT);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("冒険の終わり: Victory"));
    // 冒険記(ドメインログ)には自由入力本文が現れてよい。
    assert!(stdout.contains("単純討伐"));
    assert!(stdout.contains("最後の一言"));
}

#[test]
fn play_ops_log_omits_free_text_even_through_real_process() {
    let secret = "近道を探したい";
    let output = run_play(VICTORY_INPUT);
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains(secret),
        "ops log leaked proposal text: {stderr}"
    );
    assert!(stderr.contains("[log]"));
}
