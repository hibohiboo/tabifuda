# P7 C1 チェックリスト: `ScenarioMeta.summary` 追加

対象: [tasks/projects/phase7/task.md](../task.md) C1。
経緯: grillingスキルでの着手前確認(2026-09-12、task.md「着手前の検討結果」参照)。
本ファイルは経緯メモ。仕様の正はdocs/design/domain-model.md。

## 設計文書

- [ ] [design/domain-model.md](../../../../design/domain-model.md)
      「シナリオ構造」の`ScenarioMeta`定義に`summary: BoundedString<400>`を追加
- [ ] 同文書「文字列の長さ上限(BoundedString)」の一覧に`summary`(400文字、
      識別的短文<200と説明的長文2000の中間という位置づけ)を追記

## core実装

- [ ] `crates/tabifuda-core/src/scenario.rs`の`ScenarioMeta`へ
      `summary: BoundedString<400>`を追加
- [ ] 後方互換(`#[serde(default)]`)の要否を検討する。`BoundedString`に
      `Default`実装が無いため、必要なら追加するか空文字列許容の扱いを決める
- [ ] 既存の`ScenarioMeta { .. }`リテラル(テスト・fixture含む)を洗い出し、
      `summary`を補って更新する(現状の既知箇所):
  - `crates/tabifuda-core/src/scenario.rs`(定義本体・doc例があれば)
  - `crates/tabifuda-core/src/engine_tests.rs`(3箇所)
  - `crates/tabifuda-core/src/invariant_tests.rs`
  - `crates/tabifuda-core/src/lint_tests.rs`
  - `crates/tabifuda-core/src/patch_tests.rs`
  - `crates/tabifuda-cli/src/chronicle.rs`
  - `crates/tabifuda-cli/src/fork.rs`
  - `crates/tabifuda-cli/tests/lint_cli.rs`(2箇所)
  - `crates/tabifuda-wasm/tests/boundary_roundtrip.rs`
  - (上記は着手前調査時点の一覧。実装時に`rg "ScenarioMeta \{"`で再確認する)

## ts-rs bindings

- [ ] `cargo test -p tabifuda-core --features ts export_bindings`
- [ ] `cargo test -p tabifuda-wasm --features ts export_bindings`
- [ ] `crates/tabifuda-wasm/bindings/ScenarioMeta.ts`に`summary`が反映されたか確認

## シナリオデータ

- [ ] `shared/scenarios/simple-hunt.json`へ`summary`を追記
- [ ] テスト用ダミーシナリオを1本新規追加(`shared/scenarios/{id}.json`。
      1〜2シーン程度の最小構成。C2の複数選択確認用。`simple-hunt-fork.json`は
      フォーク出力のテスト成果物のため転用しない)
- [ ] ダミーシナリオが`tabifuda-cli lint`を通ることを確認

## 検証

- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace -- -D warnings`
- [ ] `cargo fmt --all`
- [ ] `pnpm -r typecheck`(bindings変更がapps/web・packages/uiの型チェックに
      影響しないか確認。client-conventions.md参照)

## 終わり方

- [ ] design-syncで乖離チェック
- [ ] agent-journal.mdへの追記要否を確認
- [ ] コミット後、C2着手前にユーザーへ区切り報告
