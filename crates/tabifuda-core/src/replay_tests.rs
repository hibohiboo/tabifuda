//! fixtureからのリプレイ・スナップショットテスト。docs/design/test-strategy.md
//! §1(c)「ゴールデン(リプレイ)テスト」に対応。回帰検知+シリアライズ後方互換+
//! (tabifuda-cliの)CLIスモークの生成元、の3役を兼ねる。
//!
//! fixtureはコンパイル時に`include_str!`で埋め込む(実行時ファイルIOを
//! coreに持ち込まないため。CLAUDE.md「coreはIO不可」)。中身はテンプレシナリオ
//! 「単純討伐」の勝利ルートの通しプレイ(提案→GM裁定を含む)を、実際に
//! decide/applyで生成したイベント列そのもの。

use crate::engine::apply;
use crate::event::Event;
use crate::session::Session;

const SIMPLE_HUNT_PLAYTHROUGH: &str = include_str!("../fixtures/simple_hunt_playthrough.json");
const SIMPLE_HUNT_FINAL_STATE: &str =
    include_str!("../fixtures/simple_hunt_playthrough.final_state.json");

/// enumに種別を追加したときもこのfixtureが読めることが、そのまま
/// シリアライズ後方互換テストになる(test-strategy.md「ゴールデン(リプレイ)
/// テスト」参照)。
fn replay_fixture() -> Session {
    let events: Vec<Event> = serde_json::from_str(SIMPLE_HUNT_PLAYTHROUGH)
        .expect("fixtureはVec<Event>としてデシリアライズできるはず");
    let mut state: Option<Session> = None;
    for event in &events {
        state = apply(state, event);
    }
    state.expect("完全なイベント列はSomeのSessionへ収束するはず")
}

#[test]
fn 単純討伐の通しプレイfixtureをリプレイすると想定の最終状態になる() {
    let session = replay_fixture();

    // 両方向で固定表現と比較する(golden_tests.rsのassert_goldenと同じ理由:
    // シリアライズ・デシリアライズどちらの破壊も検出するため)。
    let actual_json = serde_json::to_string_pretty(&session).unwrap();
    let expected_json = SIMPLE_HUNT_FINAL_STATE
        .replace("\r\n", "\n")
        .trim()
        .to_string();
    assert_eq!(
        actual_json, expected_json,
        "リプレイ最終状態のスナップショットから変化した(想定外の変更ならfixtureも同PRで更新する)"
    );

    let expected_session: Session = serde_json::from_str(SIMPLE_HUNT_FINAL_STATE).unwrap();
    assert_eq!(session, expected_session);
}
