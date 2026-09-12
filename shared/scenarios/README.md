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

## apps/web(依頼選択画面)からの動的検出(P7 C2、2026-09-12)

このディレクトリ配下の`*.json`は、`apps/web`の依頼選択画面
(`scenario/loadScenarios.ts`)が`import.meta.glob`で**非再帰**に動的検出する
対象でもある(client-conventions.md「シナリオの動的検出とランタイム検証」)。
そのため、ここに置いたファイルはプレイヤーが選べる「依頼」として一覧に
現れる。

- `meta.id`が重複するファイルを置かない(重複時は2件目以降が警告付きで
  除外され、選んでも1件目の中身が開始される)
- CLIのフォーク出力デモ用サンプル(`simple-hunt-fork.json`)のように、
  本来「選ぶための依頼」ではないファイルを置くと一覧に混ざる。現状は
  同名タイトルでの重複表示を許容する判断としている(2026-09-12、
  agent-journal.md参照。フォーク直後に他者の一覧へ即座に現れる運用は
  想定していないため実害は小さいとの判断)
