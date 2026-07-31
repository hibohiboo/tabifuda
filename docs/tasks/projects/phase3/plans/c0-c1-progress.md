# P3 C0/C1 実施記録(2026-08-01)

対象: docs/tasks/projects/phase3/task.md の C0(フロント層設計文書の置き場)+
C1(tabifuda-wasm)。「C0はC1と同セッションでよい」の指定どおり1サイクルとして
実施した。ブランチ `p3`。

**本文書は経緯・作業ログであり、仕様の置き場ではない**(agent-operations.md
「正を二重化しない」の規律)。決定事項の中身は各リンク先の規範文書が正。

## 実施時のTODOリスト(最終状態)

1. [x] ADR 0006を作成
2. [x] C0: フロント層設計文書の置き場を決め、既存記述を移す
3. [x] wasm境界API設計叩台作成+Opusレビュー+確定版反映
4. [x] 環境確認: wasm32ターゲット/wasm-packの有無
5. [x] core側: Lint型へSerialize/Deserialize追加+一貫性テスト
6. [x] crates/tabifuda-wasm を新設(Cargo.toml/lib.rs/WasmError/decide/apply_all/validate_patch/lint)
7. [x] wasm-bindgen-testで型往復テスト数本を書く
8. [x] CI(ADR 0003)にwasm-pack test+ts bindingsドリフト検査を追加
9. [x] TS型定義同期(ts-rs導入)を実施・検証
10. [x] bindings/をgitに追加しcargo test/clippy/fmtを全体で確認
11. [x] design-syncで設計文書との乖離チェック・修正
12. [x] 非規範文書(domain-guide/demo)への影響確認(該当なしと判断)
13. [x] 修正後のcargo test/clippy/fmt再確認
14. [x] agent-journal.mdに気づきを追記
15. [x] task.mdのfrontmatterを更新(C0/C1: done)
16. [x] コミットを作成

## やったこと

### 人間の事前決定
- フロントエンドフレームワークをReact+Viteに決定(ADR 0006)。タスク文書の
  推奨どおり

### C0: フロント層設計文書の置き場
- 「wasm境界の型・API設計」と「複数クライアント(CLI/Web)で共有される
  表示・操作ロジックの決定」を別文書に分離する方針を決定
  - `docs/design/wasm-boundary.md` を新設(C1で中身を書く)
  - `docs/design/client-conventions.md` を新設し、domain-model.mdにあった
    「CLIの手札表示からMarkerを除外する」をここへ移した

### C1: wasm境界API設計(Opusレビュー実施)
- wasm-boundary.mdにドラフトを書き、Opus 4.8(エージェント経由)で
  設計レビューを1回実施(タスク文書の必須事項)
- レビュー指摘を反映して確定:
  - `apply`を`apply_all`(複数Event一括・all-or-nothing)に変更
  - エラーは`WasmError`封筒型(`kind: rule|patch|decode`)に統一
  - ワイヤ形式(serde既定の外部タグ付け)を凍結する方針を明記
  - TS型定義同期は`ts-rs`導入を採用(撤退条件を設けて試し、成功)
- `Session`内`HashMap`のキー順非決定性の問題は、P3.5着手前に判断する
  決定ログ([wasm-boundary-decisions.md](wasm-boundary-decisions.md))として
  切り出し、C1のスコープからは外した

### C1: 実装
- core側: `LintFinding`/`LintIssue`/`Severity`にSerialize/Deserializeを追加
  (severityとissueの矛盾を防ぐ一貫性テストを1本追加)。`ts-rs`
  (`feature = "ts"`)を追加し、`BoundedString<const MAX: usize>`はTS実装を
  手書き
- `crates/tabifuda-wasm`を新設。`decide`/`apply_all`/`validate_patch`/`lint`
  をJSON文字列でwasm-bindgen越しに公開
- `wasm-bindgen-test`による型往復テスト5本を追加、`wasm-pack test --node`
  で実行確認
- `ts-rs`でRust型からTypeScript型定義を自動生成
  (`crates/tabifuda-wasm/bindings/`、41ファイル。git管理対象)
- CIに`wasm-test`ジョブを追加(境界テスト実行+TSバインディングの
  ドリフト検査。ADR 0003に方針を記録)

### 仕上げ
- design-syncで2件の乖離を検出・修正:
  `console_error_panic_hook`の未導入、`WasmError`への不要な`Deserialize`
- 非規範文書(domain-guide.md/demo.md)への影響は無し(Web UIがまだ無く
  CLIの挙動は変えていないため)
- agent-journal.mdに2件追記(actor変換の一時的な誤り、設計文書の方針を
  実装に反映し忘れた抜け)
- roadmap.md・phase3/task.mdのfrontmatterをC0/C1完了に更新

## 成果物・コミット

ブランチ `p3` に5コミット:

| コミット | 内容 |
|---|---|
| `4f42357` | ADR 0006(React+Vite採用)とP3 C0(表示ロジック規約の分離) |
| `9024fe9` | P3 C1 wasm境界API設計を確定(Opusレビュー反映) |
| `dd42d1d` | core: LintへSerialize/Deserialize追加、ts-rs型エクスポート基盤 |
| `1d60bd7` | feat: crates/tabifuda-wasmを新設 |
| `5e8a574` | docs: 進捗更新とジャーナル追記 |

主な新規ファイル:
- `docs/adr/0006-frontend-framework.md`
- `docs/design/client-conventions.md`
- `docs/design/wasm-boundary.md`
- `docs/tasks/projects/phase3/plans/wasm-boundary-decisions.md`(決定ログ)
- `crates/tabifuda-wasm/`(crate本体一式)

## 次のサイクル

C2: apps/web骨格(pnpm workspace導入、CI拡張、シナリオ読込→セッション開始
→カード進行)。着手前にPRの要否を人間に確認する。
