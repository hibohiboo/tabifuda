# ADR 0002: TS側パッケージマネージャに pnpm を採用

状態: 採用 / 日付: 2026-07-12

## 文脈

apps/web(P3〜)・apps/api(P4〜)・packages/schema(P4〜)を含むTSモノレポの
パッケージマネージャを決める必要がある。以前の別プロジェクトでは
bun + turbo で管理していた経験があり、bun・npm・pnpm を比較検討した。

## 決定

pnpm を採用する。turbo(タスクオーケストレーション)は pnpm workspaces の
上に併用可能なため、必要になった時点で追加してよい。

理由:

1. **本番ランタイムとの一致。** apps/api は Hono on AWS Lambda で、Lambdaの
   標準ランタイムは Node.js。bun をパッケージマネージャとして使っても
   本番は Node で動くため、bunランタイム固有の速度上の利点は本番に届かず、
   「開発はbun、本番はNode」の差異リスクだけが残る
2. **幻の依存の早期検出。** pnpmはシンボリックリンク構造により、
   package.jsonに宣言していない依存のimportを即エラーにする。
   本プロジェクトはエージェントが実装主体(docs/agent-operations.md)であり、
   npm/bunのフラットなnode_modulesで「たまたま動く」import を
   見逃すリスクを構造的に減らせる
3. **Windows + wasm-bindgen ツールチェーンとの親和性。** 開発環境はWindows。
   pnpm + Node は wasm-pack/wasm-bindgen 周辺で最も実績のある組み合わせで、
   P3のWASM境界(test-strategy.mdでリスク箇所として明記)に
   ツールチェーン起因の変数を増やさない
4. **周辺エコシステムの一級対応。** Drizzle・Lambdaデプロイ系・
   GitHub Actionsキャッシュ等はpnpm workspacesを標準的に扱う。
   `packageManager`フィールドによるバージョン固定は、リプレイ決定性を
   重視する本プロジェクトの思想と整合する

npmを採用しない理由: pnpmに対する優位が「追加インストール不要」のみで、
速度・ディスク効率・依存の厳格さのいずれでも劣るため消去法で外れる。

## 帰結

- CLAUDE.md の「(pnpm系はP3以降に追記)」は本ADRを指す形で解消する
- apps/web・apps/api・packages/schema 追加時、`pnpm-workspace.yaml` と
  ルート`package.json`の`packageManager`フィールドを整備する
- タスクオーケストレーション(turbo等)が必要になった場合はpnpm workspacesの
  上に追加で検討する(本ADRの範囲外)
- 将来Lambdaランタイムをbun/LLRT等に置き換える判断がなされた場合、
  本ADRを再評価する
