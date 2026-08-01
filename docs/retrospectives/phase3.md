# Phase 3 ふりかえり(2026-08-01)

対象: docs/tasks/projects/phase3/task.md C0〜C4
(コミット `4f42357`〜`04315b6`、ブランチ `phase3`。派生作業ブランチ
`tabifuda-wasm-readme-and-ci-fix`/`agent-ops-branch-hygiene`/
`agent-ops-checklist-branch-workflow`は`phase3`へマージ済み)。
位置づけ: 記録文書(非規範)。ここで挙げた対応項目は反映先の文書が正であり、
本文書は経緯を残す。

注記: 上記コミット範囲の前後には、docs-site(RDRAビューア)の別タスクや
依存パッケージ更新など、Phase 3と無関係な並行コミットが挟まっている
(`git log`上は同一ブランチの時系列に混在する)。本文書はPhase 3のtask.md・
plans/配下の文書に紐づく成果のみを対象とする。

## 成果

完了条件(task.md)を3点とも満たした。

| 完了条件 | 充足状況 |
|---|---|
| ブラウザで通しプレイ可能 | 満たす。C2で最小ループ、C3で自由入力・提案・GM裁定を実装し、C4のPlaywrightスモーク(`apps/web/e2e/simple-hunt.spec.ts`)で「単純討伐」を勝利まで1本、CIで自動実行される形で確認 |
| タイムラインUIで冒険記閲覧可能 | 満たす。C3の`chronicle/Timeline.tsx`+ハンドラマップパターンで、イベント列を時系列カードとして描画 |
| 生HTML挿入の静的検査がCIに入っている | 満たす。C4でESLintに`dangerouslySetInnerHTML`使用を禁止する`no-restricted-syntax`ルールを追加し、既存`web`ジョブの`lint`ステップで機械的に検査される |

### サイクル別の成果物

| サイクル | 成果物 |
|---|---|
| C0 | フロント層設計文書の置き場を分離: `docs/design/wasm-boundary.md`(wasm境界の型・API)を新設、`docs/design/client-conventions.md`(CLI/Web共通の表示・操作規約)を新設しdomain-model.mdから「Marker除外」規約を移設。`docs/adr/0006-frontend-framework.md`(React+Vite採用)を作成 |
| C1 | `crates/tabifuda-wasm`crate新設。`decide`/`apply_all`(複数Event一括all-or-nothing)/`validate_patch`/`lint`をJSON文字列でwasm-bindgen越しに公開。エラーは`WasmError`封筒型(`kind: rule\|patch\|decode`)に統一。Opus 4.8による境界API設計レビューを1回実施し、指摘を反映。`ts-rs`導入でRust型からTS型定義を自動生成(`crates/tabifuda-wasm/bindings/`、git管理)。CIに`wasm-test`ジョブ(境界テスト+TSバインディングのドリフト検査)を追加 |
| C2 | `apps/web`骨格(React+Vite)をpnpm workspaceに追加、CI`web`ジョブ新設(型チェック・lint・build)。イベント列駆動の状態管理(`useGameSession`、Sessionを独立stateとして持たず導出)。シナリオ読込→セッション開始→カードを出して進行、までの最小UI |
| C3 | 冒険記タイムラインUI。`core/taggedUnion.ts`(`TagOf`/`PayloadOf`/`HandlerMap`/`dispatchTagged`)による完全網羅なハンドラマップパターンをclient-conventions.mdに規約化。Dialogueカードの自由入力、提案UI、GM裁定UI(y/n/c、CLIパリティ)を実装 |
| C4 | UGC規律とスモーク。ESLintに`dangerouslySetInnerHTML`禁止ルール追加。Playwright導入、`simple-hunt.spec.ts`で通しプレイスモーク1本。CIの`web`ジョブ末尾にブラウザインストール+スモーク実行を追加(ADR 0003を先に更新してから実装) |

## うまくいったこと

- **Opusレビューで浮いた論点をスコープ外に切り出す運用が機能した。**
  C1のwasm境界API設計レビューで指摘された「`Session`内`HashMap`の
  キー順非決定性」を、その場でC1に取り込まず
  `plans/wasm-boundary-decisions.md`に決定ログとして切り出し、
  「P3.5着手前に判断する」と明記してC1のスコープを絞り込んだ。
  C1が肥大化せず1サイクルで完了した。
- **型で網羅性を担保する設計が実地で機能を発揮した。** C3のハンドラマップ
  パターンは、`eventRenderers`から`CardRemoved:`エントリを一時的に消し
  `tsc --noEmit`がキー不足エラーで失敗することを確認してから元に戻す、
  という実地確認を行っており、「未対応の種別を黙って無視しない」という
  P2からの申し送り事項を型システムで強制する仕組みが機能することを
  自ら検証した。
- **ジャーナルで見つけた運用の穴を即日スキルへ反映し、以降の作業で
  実際に運用が回った。** ブランチ命名重複の発見当日にphase-cycleスキルへ
  手順を追記し、その後の派生作業3ブランチは手順どおり専用ブランチで
  作業→`phase3`へマージ、という形で進んだ。
- **design-syncの実装直後チェックが、リリース前に複数の乖離を捕捉した。**
  C1の`console_error_panic_hook`未導入、C3の`FREE_TEXT_MAX`誤用を、
  いずれも実装中ではなくサイクル終わりのdesign-syncチェックで検出・修正できた。
- **決定ログとADRの追記を「実装前に」行う規律が、CI変更で機能した。**
  C4のPlaywright追加でも、先にADR 0003へジョブ構成変更を追記してから
  `ci.yml`を実装する順序が守られた。

## 課題(ジャーナル記録済み)

`docs/agent-journal.md`2026-08-01付近にPhase 3関連のエントリが8件記録
されている。いずれもその場または同サイクル内で対応済み。

1. wasm-boundary型とプリミティブ引数の混同(C1)。ビルド前のセルフレビューで発覚・修正。
2. 設計文書の「方針」節が実装チェックリストとして機能していなかった(C1)。design-syncで発覚・修正。
3. ブランチ命名の並立とマージ済みブランチの放置(C1着手時)。人間指摘で発覚、当日中に整理・スキル反映。
4. CIのドリフト検査を無効化する`TS_RS_EXPORT_DIR`未設定(C1)。README作成依頼をきっかけに自己発見・修正。
5. typescript-eslintのTS 7系非互換(C2)。lint実行で発覚、バージョン固定で回避。
6. Bashツールのcwd永続による`rm`失敗の隠蔽(C2)。`git status`で発覚・再実行。
7. 報告時の判断帰属の誤った一般化(C2完了報告)。ユーザー指摘で発覚し、事後承認を取得。
8. `CardDef.text`の文字数上限にPlayCard用定数を誤流用(C3)。design-syncで発覚・修正。

C4は`c4-checklist.md`に「今回は特筆すべき誤解なし」と明記されており、
C4単独の新規ジャーナルエントリは無い。

### P2からの申し送り3点の扱い

| 申し送り事項 | P3での扱い |
|---|---|
| シナリオデータの人間レビューは実プレイで行う | **行使機会なし**。P3では`shared/scenarios/`のシナリオデータ自体の追加・変更が発生しなかった |
| lint Warning(到達不能・詰み)の作者体験が未検証 | **行使機会なし**。同上の理由で検証されていない |
| CardRemoved方針のCLI/Web一致確認 | **対応済み**。C3で`client-conventions.md`に明記し、`CardRemoved: () => null`として実装した |

前2点は「シナリオデータを追加・変更するサイクル」という前提条件自体が
P3中に発生しなかったため未消化のまま残っている。

## 気づきと対応(反映先)

| 気づき | 対応 | 反映先 |
|---|---|---|
| `Session`内`HashMap`のキー順非決定性が未着手のままwasm-boundary-decisions.mdに残っている | P3.5 C1着手時に決定する旨をtask.mdから直接たどれるようにする | 済: docs/tasks/projects/phase3.5/task.md C1に決定ログへのポインタを追記 |
| P2からの申し送り2点(シナリオデータの人間レビュー実プレイ化、lint Warning作者体験検証)が2フェーズ連続で未消化 | 次にシナリオデータを追加・変更するサイクルの着手時チェック項目として申し送りを継続する | 済: docs/tasks/projects/phase3.5/task.mdに申し送り事項として追記 |
| design-syncの「実装直後チェック」が、文書の「方針」節の反映漏れを事後的に複数回捕捉した | 実装完了時に確定済み文書の各項目を1つずつ実装箇所と突き合わせる、という進め方をスキル手順に明記する | 済: .claude/skills/design-sync/SKILL.mdに手順を追記 |
| roadmap.mdのP3行が「進行中」のまま更新されていなかった | フェーズ完了を反映する | 済: docs/roadmap.md更新 |
| CI設定変更(`TS_RS_EXPORT_DIR`環境変数抜け)は、ローカルで都度手打ちした値がCI定義への転記時に抜けたことが原因(2026-07-31のCARGO_TERM_COLOR環境差の件と傾向が近い、2回目相当) | CI設定変更を伴うサイクルの終わり方に、ローカル確認コマンドをそのまま転記する注意を明記する | 済: phase-cycleスキル「終わり方」に一言追記 |
