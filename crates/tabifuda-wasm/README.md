# tabifuda-wasm

`tabifuda-core` の `decide`/`apply_all`/`validate_patch`/`lint` を
wasm-bindgen経由でTypeScriptへ公開する境界クレート。設計の正は
[docs/design/wasm-boundary.md](../../docs/design/wasm-boundary.md)。

## bindings/ ディレクトリ

`bindings/` 配下の `.ts` ファイルはすべて [ts-rs](https://github.com/Aleph-Alpha/ts-rs)
によるRust型からの自動生成物。**手で編集しない**(コミットには含めるが、
生成コマンドを再実行すれば同じ内容になる)。

対象は境界の入出力に直接現れる最上位型(`Command`/`Event`/`Session`/
`Scenario`/`RuleError`/`PatchError`/`ScenarioPatch`/`LintFinding`は
`tabifuda-core`側、`WasmError`は本クレート側)と、それらが依存する下位の型
(`CardDef`/`Effect`等)。

## bindings/ の更新は自動化されていない

コード変更後の再生成は**手動でコマンドを実行する必要がある**(git hookや
ビルドスクリプトによる自動更新は無い)。CI(`.github/workflows/ci.yml`の
`wasm-test`ジョブ)は「再生成して差分が無いこと」を検証するだけで、
生成自体・コミットは行わない。型を追加・変更したら、リポジトリルートで
次を実行してから `git add crates/tabifuda-wasm/bindings/` でコミットする:

```sh
TS_RS_EXPORT_DIR="$(pwd)/crates/tabifuda-wasm/bindings" \
  cargo test -p tabifuda-core --features ts export_bindings
TS_RS_EXPORT_DIR="$(pwd)/crates/tabifuda-wasm/bindings" \
  cargo test -p tabifuda-wasm --features ts export_bindings
```

2回に分かれているのは、`tabifuda-wasm`の`WasmError`が依存する
`RuleError`/`PatchError`を`tabifuda-core`側が先に生成する必要があるため。
両方実行すれば`bindings/`配下の全ファイルが揃う。

`TS_RS_EXPORT_DIR`を省略すると、ts-rsは既定の出力先
(`<crateのCargo.tomlがあるディレクトリ>/bindings/`)に書き出してしまい、
`tabifuda-core`側の型が`crates/tabifuda-core/bindings/`という別の場所に
生成される(2026-08-01に実際に発生した不具合。CIのドリフト検査対象は
`crates/tabifuda-wasm/bindings/`に固定しているため、環境変数を忘れると
`tabifuda-core`側の変更が検証されないまま緑になる)。**必ず環境変数を
指定すること。**

## テスト

```sh
wasm-pack test --node crates/tabifuda-wasm
```

境界の型往復テスト(`tests/boundary_roundtrip.rs`)を実行する。ルール自体の
正しさは`tabifuda-core`側でテスト済みのため、ここではJSONが正しく境界を
越えるかのみを見る(docs/design/test-strategy.md参照)。
