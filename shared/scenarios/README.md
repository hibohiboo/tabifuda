# shared/scenarios/

シナリオデータ(JSON)の配置場所。規約は docs/design/domain-model.md
「シナリオファイルの配置」節を参照。

- ファイル名: `{ScenarioId}.json`
- 中身: `tabifuda_core::Scenario` を serde_json でシリアライズした
  JSONオブジェクト1個
- 特定の crate や pnpm パッケージに従属しない(core/cli/将来のweb/apiが
  同じパスを参照する)
- 読み込みはIO層(tabifuda-cli等)が行う。tabifuda-core自体はファイルを
  読まない

検証: `tabifuda-cli lint <file>` または `cargo test --workspace`
(同梱シナリオ全件に対するlintテスト)。
