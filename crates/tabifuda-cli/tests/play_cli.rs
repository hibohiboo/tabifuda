//! `tabifuda-cli play <file>` の標準入出力結合テスト(通しプレイのスモーク)。
//! docs/tasks/phase2-task.md C4「CLIスモークテスト」に対応。ルール分岐は
//! テストしない(tabifuda-core側で済んでいる)。
//!
//! 出したカードは手札から消え、Markerは一覧に出ず、選ばなかった側の
//! 選択肢カードもシーンを離れると消えるため(domain-model.md「カードの
//! 消費・除去」参照)、番号は毎回`[1]`から振り直される。

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

/// [1]依頼を受ける(自由入力スキップ)→[1]獣の巣に到着する→提案→GM裁定(採用)
/// →[1]打ち倒す→[1]村に帰還を告げる(自由入力あり)、で勝利エンドまで到達する。
const VICTORY_INPUT: &str = "1\n\n1\np\n近道を探したい\ny\n1\n1\n最後の一言\n";

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
fn play_hides_marker_and_removes_played_and_unchosen_cards() {
    let output = run_play(VICTORY_INPUT);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Markerの「依頼受諾」は選択肢としては一度も表示されない。
    assert!(!stdout.contains("] 依頼受諾"));
    // クライマックスで選ばなかった「退く」は、シーンを離れた後の
    // 手札一覧(エピローグ以降の対話表示)には出ない。
    let epilogue = stdout
        .split("=== epilogue_win ===")
        .nth(1)
        .expect("epilogue_winへ到達しているはず");
    assert!(!epilogue.contains("] 退く"));
}

/// 提案にGMがカードを配って応えるルート(demo.md「討伐に成功するルート」)。
/// オープニングで提案→c(カード名+回答文)→y採用→配られた質問カードを出すと
/// 回答文が表示される→以降は勝利エンドまで一本道。
const ANSWER_CARD_INPUT: &str = "p\n獣の姿や被害を知りたい\nc\n獣の目撃情報を尋ねる\n銀色の毛並みの大狼だという。家畜が三頭襲われた。\ny\n2\n1\n\n1\n1\n1\n\n";

#[test]
fn play_gm_deals_answer_card_and_text_is_revealed_on_play() {
    let output = run_play(ANSWER_CARD_INPUT);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // 配られた質問カードが手札の選択肢として並ぶ。
    assert!(stdout.contains("] 獣の目撃情報を尋ねる"));
    // カードを出すと回答文(CardDef.text)が開示される。
    assert!(stdout.contains("銀色の毛並みの大狼だという。"));
    // パッチ適用後もPausedのままなので、y採用を経て勝利エンドまで到達できる。
    assert!(stdout.contains("冒険の終わり: Victory"));
    // 冒険記でもパッチ追加カードは名前解決され、内部IDに落ちない。
    assert!(stdout.contains("GMがシナリオを改修した"));
    assert!(!stdout.contains("gm-card-1"));

    // 運用ログ(stderr)にはUGC本文(回答文)を漏らさない。
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("銀色の毛並み"),
        "ops log leaked answer text: {stderr}"
    );
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
