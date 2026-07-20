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
