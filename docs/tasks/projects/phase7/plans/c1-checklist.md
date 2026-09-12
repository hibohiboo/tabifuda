# P7 C1 チェックリスト: `ScenarioMeta.summary` 追加

対象: [tasks/projects/phase7/task.md](../task.md) C1。
経緯: grillingスキルでの着手前確認(2026-09-12、task.md「着手前の検討結果」参照)。
本ファイルは経緯メモ。仕様の正はdocs/design/domain-model.md。

## 設計文書

- [x] [design/domain-model.md](../../../../design/domain-model.md)
      「シナリオ構造」の`ScenarioMeta`定義に`summary: BoundedString<400>`を追加
- [x] 同文書「文字列の長さ上限(BoundedString)」の一覧に`summary`(400文字、
      識別的短文<200と説明的長文2000の中間という位置づけ)を追記

## core実装

- [x] `crates/tabifuda-core/src/scenario.rs`の`ScenarioMeta`へ
      `summary: BoundedString<400>`を追加
- [x] 後方互換(`#[serde(default)]`)の要否を検討する→不要と判断
      (SaveFileは`format_version`不一致を拒否する設計のため、フィールド追加は
      デフォルト値でなく`FORMAT_VERSION`インクリメントで吸収する)
- [x] `crates/tabifuda-cli/src/save.rs`の`FORMAT_VERSION`を2へ上げる
      (`SessionStarted`が持つ`ScenarioSnapshot`の形が変わるため)。
      domain-model.md「セッションの保存と再開」のコメント`現在1`も更新
- [x] 既存の`ScenarioMeta { .. }`リテラル(テスト・fixture含む)を洗い出し、
      `summary`を補って更新した(実施箇所):
  - `crates/tabifuda-core/src/scenario.rs`(定義本体)
  - `crates/tabifuda-core/src/engine_tests.rs`(3箇所。`summary()`ヘルパー追加)
  - `crates/tabifuda-core/src/invariant_tests.rs`
  - `crates/tabifuda-core/src/lint_tests.rs`(`summary()`ヘルパー追加)
  - `crates/tabifuda-core/src/patch_tests.rs`(`summary()`ヘルパー追加)
  - `crates/tabifuda-cli/src/chronicle.rs`
  - `crates/tabifuda-cli/src/fork.rs`
  - `crates/tabifuda-cli/tests/lint_cli.rs`(2箇所。`summary()`ヘルパー追加)
  - `crates/tabifuda-cli/tests/play_cli.rs`(`PORTABLE_TEST_SCENARIO`定数)
  - `crates/tabifuda-wasm/tests/boundary_roundtrip.rs`(`summary()`ヘルパー追加)
  - `crates/tabifuda-core/fixtures/simple_hunt_playthrough.json` /
    `.final_state.json`(ゴールデンfixture。test-strategy.md「fixtureは
    意図的に凍結する対象」の想定どおり、型追加により壊れたため人間の
    判断を経ず機械的な追記のみで更新。内容の意味は変えていない)

## ts-rs bindings

- [x] `cargo test -p tabifuda-core --features ts export_bindings`
- [x] `cargo test -p tabifuda-wasm --features ts export_bindings`
      (`TS_RS_EXPORT_DIR`環境変数の設定が必要。未設定だと出力先が異なり
      `bindings/`に反映されない。wasm-boundary.md「出力先」参照)
- [x] `crates/tabifuda-wasm/bindings/ScenarioMeta.ts`に`summary`が反映されたか確認

## シナリオデータ

- [x] `shared/scenarios/simple-hunt.json`へ`summary`を追記
      (`simple-hunt-fork.json`も同梱lint対象のため同様に追記)
- [x] テスト用ダミーシナリオを1本新規追加
      (`shared/scenarios/lost-cat.json`。1シーン・1カードの最小構成)
- [x] ダミーシナリオが`tabifuda-cli lint`を通ることを確認
      (`scenario_lint.rs`が`shared/scenarios/`配下の全`.json`を自動対象にする
      ため、`cargo test --workspace`に含まれる形で確認済み)

## 検証

- [x] `cargo test --workspace`(全件パス)
- [x] `cargo clippy --workspace -- -D warnings`(警告無し)
- [x] `cargo fmt --all -- --check`(差分無し)
- [x] `pnpm -r typecheck`(全ワークスペース通過。apps/webのwasm再ビルドも
      含めて確認)
- [x] `wasm-pack test --node`(`crates/tabifuda-wasm`。公開型`ScenarioMeta`の
      フィールド変更のため実施。全件パス)

## 終わり方

- [x] design-syncで乖離チェック(乖離ゼロ。自作コメントに経緯情報を
      書いてしまっていた1件をその場で修正)
- [x] agent-journal.mdへ2件追記(ハンドオフ前提とのズレ確認漏れ・
      `TS_RS_EXPORT_DIR`未設定時の誤生成)。wasm-boundary.mdにも恒久的な
      注意書きを追加
- [x] コミット後、C2着手前にユーザーへ区切り報告
