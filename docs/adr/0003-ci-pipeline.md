# ADR 0003: CIパイプライン構成

状態: 採用 / 日付: 2026-07-12

## 文脈

cross-cutting.md(シークレットスキャン・cargo auditの方針)と
test-strategy.md(fmt/clippy/testを常時ゲートにする方針)にCIの**方針**は
既に記述されていたが、それを実現する具体的なジョブ構成・使用アクション・
バージョン固定方針を記録した文書がなかった。Phase 0で `.github/workflows/ci.yml`
を実装した際、根拠となる設計文書が存在しないままの実装になっていた
(CLAUDE.md最重要ルール1違反)。本ADRでこの欠落を埋める。

## 決定

`.github/workflows/ci.yml` に `push` / `pull_request` トリガーで3ジョブを置く。

| ジョブ | 内容 | 失敗時の扱い |
|---|---|---|
| lint-test | `cargo fmt --all -- --check` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` | CI失敗(必須) |
| gitleaks | `gitleaks/gitleaks-action` によるシークレットスキャン | CI失敗(必須) |
| cargo-audit | `rustsec/audit-check` による依存脆弱性チェック | `continue-on-error: true`(P0は警告のみ。cross-cutting.md「依存関係」節よりP4から必須化) |
| docs-site | pnpm install → `pnpm -r typecheck` → `pnpm -r build`(docs-siteの型検査・ビルド。RDRAデータ・task.md frontmatter・docs内リンク切れの各検証もbuild時に内包される。詳細は下記追記) | CI失敗(必須) |

使用アクションとバージョン固定方針:

- `actions/checkout`、`dtolnay/rust-toolchain`、`Swatinem/rust-cache`、
  `gitleaks/gitleaks-action`、`rustsec/audit-check` を使う
- バージョンは**メジャータグ固定**(例: `@v7`)とし、マイナー・パッチ更新は
  自動追従させる。マイナー・パッチ更新の追従は本ADR更新不要
- `dtolnay/rust-toolchain` はタグ運用ではなく `@stable` を使う
  (rust-toolchain.toml側でチャンネル・バージョンを固定しているため)
- メジャーバージョンの更新は本文の表を書き換えた上で実施する
  (agent-operations.mdのモデル配分表ではP0の「CI設定の微修正」はHaiku担当)

## 帰結

- ci.ymlのジョブ構成(ジョブの追加・削除、トリガー変更、必須/警告の区分変更)を
  変える場合は、先に本ADRを更新してから実装する(CLAUDE.md最重要ルール1)
- 各アクションのメジャーバージョン更新(パッチ・マイナーではなく)は
  フェーズ移行時など節目で確認し、本ADRの表を同期する
- シナリオlint等、P2以降に追加されるCIステップは本ADRの表に追記する
  (test-strategy.md「CIゲート」節の「シナリオlint」を実装する時点)

## 追記(2026-07-20): GitHub Pages デプロイ(pages.yml)

docs-site(旧rdra-viewer。[../tasks/tools/docs-site/task.md](../tasks/tools/docs-site/task.md))
の公開のため、`.github/workflows/pages.yml` を追加した。ci.yml とは独立の
ワークフローとする(Rust CI のゲートと混ぜない)。

| ジョブ | 内容 | トリガー |
|---|---|---|
| build | pnpm install → `pnpm -r typecheck` → `pnpm -r build` → `tools/docs-site/dist` を Pages アーティファクト化 | master への push / 手動(workflow_dispatch) |
| deploy | `actions/deploy-pages` で github-pages 環境へデプロイ | build 成功後 |

- 追加アクション: `pnpm/action-setup`、`actions/setup-node`、
  `actions/configure-pages`(`enablement: true` で Pages 未有効時に自動有効化)、
  `actions/upload-pages-artifact`、`actions/deploy-pages`
  (バージョン固定方針は本文と同じメジャータグ固定)
- pnpm のバージョンはルート package.json の `packageManager` を正とする
- PR 時の typecheck / build ゲート追加はビューアタスク C3 で ci.yml 側に入れる
  (その際は本文の表へ追記する)

## 追記(2026-07-20): テストビュー生成のための Rust toolchain(D2)

docs-site のテストビュー(`tools/docs-site/scripts/gen-test-report.mjs`)は
`pnpm -r typecheck` / `pnpm -r build` の中で `cargo test --workspace` を実行し、
その結果を GitHub Pages に表示する。そのため pages.yml の build ジョブに
`dtolnay/rust-toolchain@stable` と `Swatinem/rust-cache@v2` を追加した
(ci.yml と同じアクション。バージョン固定方針も同じ)。ci.yml と pages.yml は
独立ワークフローのままだが、両方が Rust ツールチェーンを必要とする点は共通。

## 追記(2026-07-22): docs/ 内リンク切れチェックをデプロイ前ゲートにする

`tools/docs-site/scripts/check-doc-links.mjs` を追加し、docs/ 配下の
markdown間相対リンクが指すファイルの存在をビルド時に検証する(アンカーの
存在までは検証しない。RDRA YAML の `source:` フィールドのリンク先検証は
別途 docs-site タスク C3 のスコープ)。

- 既存の progressFrontmatterCheck / testReportPlugin と同じ形の vite
  プラグイン(`buildStart`)として実装し、**pages.yml 側は変更しない**
  (既存の `pnpm -r build` ステップがそのままゲートになる)
- リンク切れがあれば `vite build` がエラーで落ち、Pages への公開物に
  リンク切れが混入しない。ローカルの `pnpm build`/`pnpm dev` でも同様に検出する
- 単体実行したい場合は `pnpm --dir tools/docs-site check:doc-links`

## 追記(2026-07-25): PR時のdocs-site typecheck/buildゲート(C3)

`pnpm -r typecheck` / `pnpm -r build` が実行されるのはこれまで pages.yml
(master への push 時のみ)だけで、PR では docs-site 側の検証が一切走って
いなかった(vite.config.ts の buildStart プラグイン群 —
progressFrontmatterCheck / testReportPlugin / docLinkCheckPlugin /
rdraDataCheckPlugin — は全て `vite build`/`vite dev` 経由でしか動かない)。
ci.yml に `docs-site` ジョブを追加し、push/pull_request の両方でこれらの
検証を必須ゲート化した。

- ジョブ内容: `pnpm/action-setup` → `actions/setup-node`(pages.ymlと同じ
  Node 22)→ `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`
  (`gen-test-report.mjs` が `cargo test --workspace` を実行するため。
  pages.yml と同じ理由) → `pnpm install --frozen-lockfile` →
  `pnpm -r typecheck` → `pnpm -r build`
- pages.yml とは別ワークフローのまま(本ADR冒頭の方針どおり、Rust CI の
  ゲートと混ぜない)。内容が pages.yml の build ジョブとほぼ重複するが、
  「PRでも同じ検証を通す」ことが目的でありワークフロー自体は分離を維持する
- これにより、C3で追加した RDRA YAML スキーマ検証・`source` のリンク先/
  アンカー存在チェック(`check-rdra-data.mjs`)と、既存の frontmatter
  検証・docs内リンク切れ検証が PR 時にも効くようになった

## 追記(2026-07-27): アクション・Nodeバージョンの更新

CI/pages.ymlで使用するアクションとNodeのバージョンが古くなっていたため、
メジャーバージョンを追従させた(本ADR「メジャーバージョンの更新は
本文の表を書き換えた上で実施する」に基づく確認・更新)。

| アクション/設定 | 旧 | 新 |
|---|---|---|
| `actions/setup-node` | v4 | v7 |
| `actions/configure-pages` | v5 | v6 |
| `actions/deploy-pages` | v4 | v5 |
| `actions/upload-pages-artifact` | v3 | v5 |
| `pnpm/action-setup` | v4 | v6 |
| Node.js (ci.yml docs-site job / pages.yml build job) | 22 | 24(2026-07時点のActive LTS。22はMaintenance LTS) |

各メジャーバージョンのリリースノートを確認し、破壊的変更は
Node.jsランタイムの引き上げ(node20→node24)やnpm限定の自動キャッシュ検出
追加など、本リポジトリの使い方(`cache: pnpm`を明示指定、`packageManager`
はpnpm)には影響しないことを確認済み。`actions/checkout@v7`・
`Swatinem/rust-cache@v2`・`gitleaks/gitleaks-action@v3`・
`rustsec/audit-check@v2`・`dtolnay/rust-toolchain@stable` は
既に最新メジャーのため変更なし。

## 追記(2026-08-01): wasm-testジョブ(P3 C1)

`crates/tabifuda-wasm`(docs/design/wasm-boundary.md)を追加し、
`#[wasm_bindgen_test]`による境界の型往復テストを実装した。
このテストは`target_arch = "wasm32"`限定でコンパイルされるため
`cargo test --workspace`(lint-testジョブ、ホストターゲット)の対象にならず、
専用ジョブが必要(wasm-boundary.md「crateの物理配置」の指摘どおり、これを
怠るとテストが1本も実行されずCIが緑になる)。

`ci.yml`に`wasm-test`ジョブを追加する:

| ジョブ | 内容 | 失敗時の扱い |
|---|---|---|
| wasm-test | `dtolnay/rust-toolchain`(`targets: wasm32-unknown-unknown`)→ `jetli/wasm-pack-action`でwasm-pack導入 → `wasm-pack test --node crates/tabifuda-wasm` | CI失敗(必須) |

- 追加アクション: `jetli/wasm-pack-action`(メジャータグ`@v0.4.0`固定。
  本ADR冒頭の「メジャー・パッチ更新は自動追従」の対象外とし、
  wasm-pack自体の破壊的変更を待ってからバージョンを上げる)
- `--node`でNode.js実行とする(ブラウザヘッドレス実行は導入しない。
  Node.jsはdocs-siteジョブで既に導入実績があり、追加のブラウザ
  インストールコストを避けるため)
- `wasm-bindgen`クレートと`wasm-bindgen-cli`(wasm-packが内部で使う)の
  バージョン不一致はビルド失敗の典型要因(wasm-boundary.md参照)。
  不一致が起きたら`crates/tabifuda-wasm/Cargo.toml`の`wasm-bindgen`
  バージョンを`wasm-pack`が解決したバージョンに合わせる

同ジョブに、TS型定義(`crates/tabifuda-wasm/bindings/`。`ts-rs`による自動生成、
wasm-boundary.md「決定した論点1」)のドリフト検査ステップも追加する:
`TS_RS_EXPORT_DIR`を`crates/tabifuda-wasm/bindings`の絶対パスに設定した上で
`cargo test -p tabifuda-core --features ts export_bindings` →
`cargo test -p tabifuda-wasm --features ts export_bindings` で
`bindings/`を再生成し、`git diff --exit-code -- crates/tabifuda-wasm/bindings`
で差分が無いことを確認する(コミットされたbindings/が最新のRust型と
同期していることを保証する)。`bindings/`はgit管理対象(生成物だが、
apps/web(P3 C2〜)がこれをimportする配布物のため)。

**`TS_RS_EXPORT_DIR`は省略しない。** 省略するとts-rsの既定出力先
(`<crateのCargo.tomlがあるディレクトリ>/bindings/`)に書き出され、
`tabifuda-core`側の型が`crates/tabifuda-core/bindings/`という別の場所に
生成されてしまう。ドリフト検査の対象は`crates/tabifuda-wasm/bindings/`に
固定しているため、これを忘れると`tabifuda-core`側の変更が実質検証されない
まま緑になる(2026-08-01、ci.yml初版で発生・修正)。
