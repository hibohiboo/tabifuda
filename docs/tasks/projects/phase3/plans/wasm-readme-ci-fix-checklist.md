# tabifuda-wasm README追加 + CIドリフト検査修正 チェックリスト(2026-08-01)

対象: P3 C1の派生作業。ブランチ `tabifuda-wasm-readme-and-ci-fix`(`phase3`から分岐)。

**本文書は経緯・作業ログであり、仕様の置き場ではない。** 決定事項の中身は
docs/design/wasm-boundary.md、crates/tabifuda-wasm/README.md が正。

## 背景

ユーザーから「crates/tabifuda-wasmにREADME.mdを作り、bindings/がts-rs生成物
であることを明記してほしい。更新は自動か、コマンドが必要か」と聞かれ、
確認の過程で `.github/workflows/ci.yml` の `wasm-test` ジョブに
`TS_RS_EXPORT_DIR` 環境変数が設定されておらず、`cargo test -p tabifuda-core
--features ts export_bindings` が `crates/tabifuda-wasm/bindings/` ではなく
`crates/tabifuda-core/bindings/` に出力してしまうバグを発見した(ローカルで
再現確認済み)。CIのドリフト検査(`git diff --exit-code -- crates/
tabifuda-wasm/bindings`)は対象ディレクトリを固定しているため、tabifuda-core
由来の38ファイルが実質検証されないまま緑になっていた。

## チェックリスト

- [x] ci.ymlの `wasm-test` ジョブに `TS_RS_EXPORT_DIR` を設定して修正
- [x] 修正後のコマンドをローカルで再現し、`crates/tabifuda-core/bindings/`
      が作られず `crates/tabifuda-wasm/bindings/` のみが更新されることを確認
- [x] `crates/tabifuda-wasm/README.md` を作成
      (bindings/がts-rs生成物であること、更新コマンドを明記)
- [x] docs/design/wasm-boundary.md との整合を確認(wasm-boundary.md自体は
      TS_RS_EXPORT_DIR明記済みで問題なし。ADR 0003のコマンド例に同じ抜けを
      発見し、あわせて修正)
- [ ] コミットし `phase3` へマージ

## 次への反映

CIのドリフト検査を実装した際(前回のP3 C1サイクル)、`TS_RS_EXPORT_DIR`を
指定しないと生成先がクレートごとのデフォルト(`<crate>/bindings/`)に
散らばることをローカル検証時には気づいていたが、CIコマンドへの反映を
忘れていた。agent-journal.mdに記録する。
