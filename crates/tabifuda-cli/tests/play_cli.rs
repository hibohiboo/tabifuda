//! `tabifuda-cli play <file>` の標準入出力結合テスト(通しプレイのスモーク)。
//! ルール分岐はテストしない(tabifuda-core側で済んでいる)。
//!
//! 出したカードは手札から消え、Markerは一覧に出ず、選ばなかった側の
//! 選択肢カードもシーンを離れると消えるため(domain-model.md「カードの
//! 消費・除去」参照)、番号は毎回`[1]`から振り直される。
//!
//! テスト名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md D2)。
#![allow(non_snake_case)]

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
    run_play_at(&scenario_path(), input)
}

fn run_play_at(path: &Path, input: &str) -> std::process::Output {
    let mut child = bin()
        .arg("play")
        .arg(path)
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

fn run_play_with_party_at(scenario: &Path, party: &Path, input: &str) -> std::process::Output {
    let mut child = bin()
        .arg("play")
        .arg(scenario)
        .arg("--party")
        .arg(party)
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

fn run_resume_at(path: &Path, input: &str) -> std::process::Output {
    let mut child = bin()
        .arg("play")
        .arg("--resume")
        .arg(path)
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
fn 通しプレイは勝利エンドに到達し冒険記を表示する() {
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
fn 通しプレイはMarkerを隠し使用済みと選ばなかったカードを一覧から消す() {
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
/// 回答文が表示される→以降は勝利エンドまで一本道。改編があるため終了時に
/// フォーク保存を聞かれる(末尾の応答は呼び出し側が足す)。
const ANSWER_CARD_BASE: &str = "p\n獣の姿や被害を知りたい\nc\n獣の目撃情報を尋ねる\n銀色の毛並みの大狼だという。家畜が三頭襲われた。\ny\n2\n1\n\n1\n1\n1\n\n";

#[test]
fn 通しプレイはGM配布カードの回答文を使用時に開示する() {
    let output = run_play(&format!("{ANSWER_CARD_BASE}n\n"));
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

/// フォーク出力(domain-model.md「フォーク出力」): 改編ありセッションを
/// y で保存すると、元ファイルの隣に `-fork.json` ができる。
/// - meta.id はファイル語幹、forked_from は元id(由来追跡)
/// - DealCard パッチ分は配布時のシーンの deals に組み込まれる
///   (次のセッションでも同じ場面で配られる)
/// - 出力物はそのまま lint を通る
#[test]
fn 通しプレイは改編ありセッション終了時にdeals統合済みのフォークを由来付きで保存する() {
    // shared/ を汚さないよう一時ディレクトリへコピーして実行する。
    let dir = std::env::temp_dir().join(format!("tabifuda-fork-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let playing_copy = dir.join("simple-hunt.json");
    std::fs::copy(scenario_path(), &playing_copy).unwrap();

    let output = run_play_at(&playing_copy, &format!("{ANSWER_CARD_BASE}y\n"));
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let fork_path = dir.join("simple-hunt-fork.json");
    assert!(fork_path.exists(), "fork file was not written");
    let fork: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&fork_path).unwrap()).unwrap();
    assert_eq!(fork["meta"]["id"], "simple-hunt-fork");
    assert_eq!(fork["meta"]["forked_from"], "simple-hunt");

    // 配布時に居たオープニングシーン(op_request)の入場時配布に組み込まれる。
    let deals = fork["phases"][0]["scenes"][0]["deals"].as_array().unwrap();
    assert!(
        deals.iter().any(|d| d["card"] == "gm-card-1"),
        "dealt card was not merged into scene deals: {deals:?}"
    );

    // 出力物は独立したシナリオとして lint を通る。
    let lint = bin().arg("lint").arg(&fork_path).output().unwrap();
    assert!(
        lint.status.success(),
        "fork does not pass lint: {}",
        String::from_utf8_lossy(&lint.stdout)
    );

    std::fs::remove_dir_all(&dir).ok();
}

/// 中断・再開(domain-model.md「セッションの保存と再開(CLIの決定)」)。
/// Running中に`q`で中断・保存し、`play --resume`で続きから
/// 勝利エンドまで到達できる(イベント列だけで自己完結する設計の確認)。
#[test]
fn 中断して保存したセッションはresumeで続きから勝利エンドまで到達する() {
    let dir =
        std::env::temp_dir().join(format!("tabifuda-save-test-running-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let playing_copy = dir.join("simple-hunt.json");
    std::fs::copy(scenario_path(), &playing_copy).unwrap();

    // [1]依頼を受ける(自由入力スキップ)→[1]獣の巣に到着する、で中断・保存する。
    let output = run_play_at(&playing_copy, "1\n\n1\nq\ny\n");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("保存しました"), "stdout:\n{stdout}");

    let save_path = dir.join("simple-hunt-save.json");
    assert!(save_path.exists(), "save file was not written");

    // 提案→GM裁定(採用)→打ち倒す→帰還を告げる、で勝利エンドまで到達する
    // (VICTORY_INPUTの続き)。
    let output = run_resume_at(&save_path, "p\n近道を探したい\ny\n1\n1\n最後の一言\n");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("セッションを再開しました"));
    assert!(stdout.contains("冒険の終わり: Victory"));

    std::fs::remove_dir_all(&dir).ok();
}

/// Paused中(提案の裁定待ち)に`q`で中断・保存しても、再開すると裁定待ちの
/// ままへ戻り、そこから通しプレイを継続できる(domain-model.md「セッション
/// の保存と再開」: 状態機械はイベント列から復元されるため、中断した状態を
/// 過不足なく再現する)。
#[test]
fn Paused中に中断して保存したセッションはresumeで裁定待ちに戻る() {
    let dir =
        std::env::temp_dir().join(format!("tabifuda-save-test-paused-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let playing_copy = dir.join("simple-hunt.json");
    std::fs::copy(scenario_path(), &playing_copy).unwrap();

    // 提案を出してPausedにしてから中断・保存する。
    let output = run_play_at(&playing_copy, "1\n\n1\np\n近道を探したい\nq\ny\n");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("保存しました"));

    let save_path = dir.join("simple-hunt-save.json");
    assert!(save_path.exists(), "save file was not written");

    // 再開直後の画面が裁定待ち(y/n/c/q)であること、かつy採用から
    // 勝利エンドまで到達できることをあわせて確認する。
    let output = run_resume_at(&save_path, "y\n1\n1\n最後の一言\n");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("提案が届いています"));
    assert!(stdout.contains("冒険の終わり: Victory"));

    std::fs::remove_dir_all(&dir).ok();
}

/// セーブファイルのformat_versionが現在の実装と一致しない場合は拒否する
/// (domain-model.md「セッションの保存と再開」: 警告付き読込はしない)。
#[test]
fn resumeはformat_version不一致の保存ファイルを拒否する() {
    let dir =
        std::env::temp_dir().join(format!("tabifuda-save-test-version-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let bad_save = dir.join("bad-save.json");
    std::fs::write(&bad_save, r#"{"format_version":999,"events":[]}"#).unwrap();

    let output = run_resume_at(&bad_save, "");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("保存ファイルを読み込めませんでした"),
        "stdout:\n{stdout}"
    );

    std::fs::remove_dir_all(&dir).ok();
}

/// パーティファイル(domain-model.md「パーティファイル(CLIの決定)」):
/// `--party`で読み込んだキャラで通しプレイでき、冒険記にその名前が載る
/// (CLIはパーティ先頭を操作対象キャラとする)。
#[test]
fn partyで読み込んだキャラで通しプレイでき冒険記に名前が載る() {
    let dir =
        std::env::temp_dir().join(format!("tabifuda-party-test-valid-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let party_path = dir.join("party.json");
    std::fs::write(
        &party_path,
        r#"[{"id":"traveler2","name":"旅人2","stats":{},"deck":[]}]"#,
    )
    .unwrap();

    let output = run_play_with_party_at(&scenario_path(), &party_path, VICTORY_INPUT);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("冒険の終わり: Victory"));
    assert!(stdout.contains("参加者: 旅人2"));

    std::fs::remove_dir_all(&dir).ok();
}

/// 空配列のパーティファイルは拒否される(domain-model.md「パーティファイル
/// (CLIの決定)」: 空配列を拒否)。
#[test]
fn 空配列のパーティファイルは拒否される() {
    let dir =
        std::env::temp_dir().join(format!("tabifuda-party-test-empty-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let party_path = dir.join("party.json");
    std::fs::write(&party_path, "[]").unwrap();

    let output = run_play_with_party_at(&scenario_path(), &party_path, "");
    let stderr = String::from_utf8_lossy(&output.stderr);
    std::fs::remove_dir_all(&dir).ok();

    assert!(!output.status.success());
    assert!(stderr.contains("パーティが空です"), "stderr:\n{stderr}");
}

/// `CharacterId`が重複するパーティファイルは拒否される(domain-model.md
/// 「パーティファイル(CLIの決定)」: 重複を拒否)。
#[test]
fn CharacterId重複のパーティファイルは拒否される() {
    let dir = std::env::temp_dir().join(format!("tabifuda-party-test-dup-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let party_path = dir.join("party.json");
    std::fs::write(
        &party_path,
        r#"[{"id":"a","name":"A","stats":{},"deck":[]},{"id":"a","name":"A2","stats":{},"deck":[]}]"#,
    )
    .unwrap();

    let output = run_play_with_party_at(&scenario_path(), &party_path, "");
    let stderr = String::from_utf8_lossy(&output.stderr);
    std::fs::remove_dir_all(&dir).ok();

    assert!(!output.status.success());
    assert!(
        stderr.contains("CharacterIdが重複しています"),
        "stderr:\n{stderr}"
    );
}

/// 持ち出し可能カードの最小シナリオ。`loot`(`#portable`、Item)を配り、
/// `finish`(Dialogue)を出すとVictoryで終わる。simple-huntにはportableな
/// カードが無いため、finalize(RewardsGranted)を実際に発火させるには
/// 専用の最小シナリオが要る(shared/scenarios/は汚さない)。
const PORTABLE_TEST_SCENARIO: &str = r##"{
  "meta": {"id": "portable-test", "title": "t", "author": "t", "forked_from": null},
  "card_defs": [
    {"id": "loot", "name": "宝物", "kind": "Item", "text": "", "tags": ["#portable"], "effects": [], "requires": []},
    {"id": "finish", "name": "終える", "kind": "Dialogue", "text": "", "tags": [], "effects": [{"EndSession": "Victory"}], "requires": []}
  ],
  "phases": [
    {"phase": "Opening", "scenes": [
      {"id": "s1", "kind": "Conversation", "narration": "n", "deals": [
        {"card": "loot", "to": "Party"},
        {"card": "finish", "to": "Party"}
      ], "exits": []}
    ]}
  ]
}"##;

/// 持ち出し(domain-model.md「セッション終了処理(finalize)」・「パーティ
/// ファイル(CLIの決定)」): `#portable`なカードを手札に残したままVictoryで
/// 終えると、RewardsGrantedが発行され、`y`でパーティファイルへ書き戻される。
/// 書き戻し後のファイルにはCardDef凍結コピー(`owned_cards`)が載る。
#[test]
fn 持ち出したカードはパーティファイルへ書き戻される() {
    let dir = std::env::temp_dir().join(format!(
        "tabifuda-party-test-writeback-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let scenario_path = dir.join("portable-test.json");
    std::fs::write(&scenario_path, PORTABLE_TEST_SCENARIO).unwrap();
    let party_path = dir.join("party.json");
    std::fs::write(
        &party_path,
        r#"[{"id":"hunter","name":"旅人","stats":{},"deck":[]}]"#,
    )
    .unwrap();

    // [1]宝物を拾う(消費されず残る)→[2]終える(自由入力スキップ)→Victory→書き戻しy。
    let output = run_play_with_party_at(&scenario_path, &party_path, "1\n2\n\ny\n");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("冒険の終わり: Victory"));
    assert!(
        stdout.contains("パーティを書き戻しました"),
        "stdout:\n{stdout}"
    );

    let written: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&party_path).unwrap()).unwrap();
    let owned = written[0]["owned_cards"].as_array().unwrap();
    assert_eq!(owned.len(), 1);
    assert_eq!(owned[0]["id"], "loot");
    assert_eq!(owned[0]["tags"], serde_json::json!(["#portable"]));

    std::fs::remove_dir_all(&dir).ok();
}

/// 並行プレイの確認(task.md C4「複数セッションファイルによる並行プレイが
/// 自然に成立することの確認」)。同じシナリオから2つのセッションを開始して
/// 中断・保存しても、保存ファイル名が連番で衝突せず、それぞれ独立して
/// resumeできる(セッションはSessionStartedにシナリオ・パーティの凍結
/// コピーを持つため自己完結。domain-model.md「セッションの保存と再開」)。
#[test]
fn 同じシナリオから開始した複数セッションは保存ファイルが衝突せず独立して再開できる() {
    let dir = std::env::temp_dir().join(format!("tabifuda-parallel-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let playing_copy = dir.join("simple-hunt.json");
    std::fs::copy(scenario_path(), &playing_copy).unwrap();

    // 1つ目のセッション: [1]依頼を受ける→[1]獣の巣に到着する、で中断・保存。
    let first = run_play_at(&playing_copy, "1\n\n1\nq\ny\n");
    assert!(first.status.success());
    let first_save = dir.join("simple-hunt-save.json");
    assert!(first_save.exists(), "first save file was not written");

    // 2つ目のセッション(1つ目をまだ削除せず、同じシナリオから開始)。
    let second = run_play_at(&playing_copy, "1\n\n1\nq\ny\n");
    assert!(second.status.success());
    let second_save = dir.join("simple-hunt-save-2.json");
    assert!(
        second_save.exists(),
        "second save file was not written (naming collided with the first)"
    );

    // それぞれ独立して続きから勝利エンドまで到達できる(互いに干渉しない)。
    let resume_first = run_resume_at(&first_save, "p\n近道を探したい\ny\n1\n1\n最初の冒険\n");
    assert!(resume_first.status.success());
    assert!(String::from_utf8_lossy(&resume_first.stdout).contains("冒険の終わり: Victory"));

    let resume_second = run_resume_at(&second_save, "p\n近道を探したい\ny\n1\n1\n二つ目の冒険\n");
    assert!(resume_second.status.success());
    assert!(String::from_utf8_lossy(&resume_second.stdout).contains("冒険の終わり: Victory"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn 通しプレイの運用ログは実プロセスを通しても自由入力本文を漏らさない() {
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
