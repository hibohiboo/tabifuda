# Phase 7 ふりかえり(2026-09-12)

対象: [tasks/projects/phase7/task.md](../tasks/projects/phase7/task.md) C1〜C2
(ブランチ `phase7`)。
位置づけ: 記録([docs/README.md](../README.md)「文書区分の定義」)。
ここで挙げた対応項目は反映先の文書が正であり、本文書は経緯を残す。

## 成果

完了条件(task.md)を3点とも満たした。

| 完了条件 | 充足状況 |
|---|---|
| 複数シナリオ(テスト用ダミー含め2本以上)から選んで`StartSession`できる | 満たす。C1で`shared/scenarios/lost-cat.json`(1シーン・1カードの最小構成)を新規追加し、既存`simple-hunt.json`と合わせ2本以上とした |
| 選択画面が張り紙カードのUI(`packages/ui`のカードコンポーネント)で表示され、シナリオ名・概要が確認できる | 満たす。ただしC2着手時の確認で当初想定(P6の`Card`/`CardLarge`を直接流用)から変更し、`ScenarioMeta`(id/title/summary)専用の新規コンポーネント(`ScenarioCard`/`ScenarioCardLarge`)を作り、`.tf-card`系CSSクラスのみ流用する方式に転換した([c2-checklist.md](../tasks/projects/phase7/plans/c2-checklist.md)「着手前に確認した実装方針」)。task.md本文もこの転換を反映済み |
| 既存のPlaywrightスモークが通る(選択画面経由のフローを含む) | 満たす。`e2e/simple-hunt.spec.ts`を依頼カードタップ→「これで始める」確定の操作に書き換え、全1件パス |

### サイクル別の成果物

| サイクル | 日付 | 成果物 |
|---|---|---|
| C1 | 2026-09-12 | `ScenarioMeta`に`summary: BoundedString<400>`を追加(domain-model.md改訂+core実装+ts-rs bindings再生成)。`FORMAT_VERSION`を2へ更新。既存の`ScenarioMeta{..}`リテラル(テスト・fixture含む12箇所)を追従。`shared/scenarios/lost-cat.json`(ダミーシナリオ)を新規追加 |
| C2 | 2026-09-12 | `packages/ui`に`ScenarioCard`/`ScenarioCardLarge`/`ScenarioSelect`(張り紙アイコン・縁取り色`#8b6b4a`)を新設。`apps/web`に`loadScenarios.ts`(`import.meta.glob`+zod検証)を新設し`scenario/simpleHunt.ts`を統合廃止。`App.tsx`をシナリオ選択→`StartSession`配線に変更。edge-case-reviewer指摘を受け、モーダルを成否によらず閉じる修正・id/titleの空文字禁止・同一id重複検出・空一覧メッセージ・多重クリック防止を追加。`vitest`を新規導入し`loadScenarios.ts`の異常系を単体テストで固定。コンポーネントカタログに追加。`docs/rdra/screens.yaml`の`scenario-select`を`implemented`へ更新 |

このフェーズはC1が`crates/`・C2が`apps/web`・`packages/ui`・`tools/docs-site`という形で、サイクルごとに変更対象がRust側/TS側に明確に分かれていた。

テスト件数: `cargo test --workspace`は186件(cli 33件+core 153件)全件パスを確認した。P7ではRust側の新規テストケース追加は無い(既存テストの`ScenarioMeta`リテラルへ`summary`フィールドを補う機械的な追従のみ)。`lost-cat.json`の妥当性は既存の`scenario_lint.rs`(shared/scenarios/配下の全ファイルを自動走査する1テスト)がそのまま検証している。TS側は`packages/ui`の新規コンポーネント3点(`ScenarioCard`/`ScenarioCardLarge`/`ScenarioSelect`)には単体テストを追加していない(P6ふりかえりでtest-strategy.mdに記録した「表示コンポーネントはPlaywrightスモーク+目視確認で担保」方針の適用)。一方`loadScenarios.ts`(ロジックを持つ純粋関数)はedge-case-reviewer指摘を受けてvitestを新規導入し、5件の単体テスト(zod検証失敗・id/title空文字・同一id重複・0件時)を追加した(下記「気づきと対応」参照)。

## うまくいったこと

- **grillingスキルによる着手前確認が、フェーズ全体を通して機能した**。task.md「着手前の検討結果」でフェーズ着手前に5点(シナリオの複数化方法・型安全性・StartSession変更要否・カード情報・テスト用シナリオ)を確定させ、さらにC2着手時にも追加のグリリング(コンポーネント方針)を行っている。P6 C1で発生した「着手前インタビューを経ずに実装してしまい事後確認になった」というパターン(agent-journal.md 2026-09-11)は、P7の2サイクルでは再発しなかった。
- **P6で確立した方針が再検討なしに再利用された**。「packages/uiの表示コンポーネントは単体テスト必須としない」(test-strategy.md、P6ふりかえり由来)が、P7で新設した3つの新規コンポーネントに対して同じ扱いで適用され、この論点を再度議論した形跡が無い。
- **design-syncが着手前確認を経てもなお実装上の問題を捕捉した**。C1・C2いずれも「終わり方」のdesign-sync実行で、コードコメントに`docs/tasks/`への参照(経緯情報)が混入している問題を検出しその場で修正した。事前のインタビューだけでは防ぎきれない類の指摘を機械的チェックが継続して拾えている。
- **edge-case-reviewerがC2で実運用に影響する具体的な欠陥を複数検出**。StartSession失敗時にモーダルが閉じずErrorBannerが隠れる問題(Hand.tsxの既存パターンからの後退)、id/titleの空文字許容、同一id重複時の「先勝ち」問題を検出し、同PRで修正した。
- **P6で修正した運用上の失敗が再発していない**。`cargo fmt`の無条件実行や`pnpm --filter <pkg> lint --if-present`の誤構文(いずれもP6でジャーナル化・スキル修正済み)は、P7では発生しなかった。

## 課題(ジャーナル記録済み)

[docs/agent-journal.md](../agent-journal.md)にP7関連エントリが4件ある(いずれも2026-09-12)。

1. **C1着手直前**: ハンドオフメモが前提としていた`grilling-skill`/`phase7-kickoff`ブランチが実在せず、該当コミットが`master`へ直接pushされていた既成事実(CLAUDE.md「やらないこと: masterへの直接コミット」に反する)に気づいた。誰の操作かは特定できず、ユーザーに報告した上で「何もせず先に進む」の判断を得て続行した。
2. **C1**: `ts-rs`の`export_bindings`テストを`TS_RS_EXPORT_DIR`未設定で実行し、テスト自体は成功したものの正規の出力先ではなく各クレート内のデフォルト出力先へ誤って生成物が書き出された。`git status`で発覚し、環境変数を設定し直して再実行した。wasm-boundary.mdに恒久的な注記を追加。
3. **C2**: C1で追加した`lost-cat.json`に加え、既存の`shared/scenarios/simple-hunt-fork.json`(CLIフォーク出力のデモ用サンプル)も`import.meta.glob`の動的検出に引っかかり、依頼選択一覧に「単純討伐」が2件重複表示される状態でPlaywrightスモークが失敗した。ユーザーに確認の上「表示の重複は許容する」の判断を得て、テストのロケータを`.first()`で一意化した。
4. **C2**: Vite previewサーバーが同一セッション内の以前のビルド・テスト実行からバックグラウンドで起動したまま残っており、古いビルド成果物を配信し続けたことでPlaywrightスモークが「要素が見つからない」でタイムアウトした。`netstat`で古いプロセスを特定・終了させて解消した。

4件中3件(#1・#2・#4)が「文書の記述・コマンドの成功(exit 0)・バックグラウンドプロセスの状態を鵜呑みにせず、実際の環境状態を確認する」という同根の傾向を持つ。

## 気づきと対応(反映先)

| 気づき | 対応 | 反映先 |
|---|---|---|
| task.md C2本文が「張り紙カードUI(P6の`Card`/`CardLarge`を流用)」と書かれたままC2着手時のグリリングで方針転換した実態と食い違っていた | task.mdの当該記述・完了条件を実態(`ScenarioMeta`専用の新規コンポーネント)に合わせて修正した(同PRで反映) | [task.md](../tasks/projects/phase7/task.md) |
| future-requirements.md §11(依頼選択UI)が「先送り分は起票済み・サイクル未設計」という古い記述のままP7完了に至っていた。同§で言及していた「難易度目安の表示は時期尚早のためfuture-requirements.mdへ送る」も未追記だった | §11を「実装済み」の記録に書き換え、残る要望として難易度目安の表示を追記した(同PRで反映) | [future-requirements.md](../requirements/future-requirements.md)§11 |
| `shared/scenarios/README.md`が、このディレクトリが`apps/web`の依頼選択画面から動的検出される対象になったこと・`simple-hunt-fork.json`の重複表示を許容する判断を記録していなかった。次にこのディレクトリへファイルを追加する人が同じ調査を再度行うおそれがあった | 動的検出の対象であること・id重複時の挙動・フォーク出力サンプルが混ざりうることを追記した(同PRで反映) | [shared/scenarios/README.md](../../shared/scenarios/README.md) |
| P7の課題4件中3件が「環境状態(ブランチ実在性・bindings出力先・稼働中プロセス)を確認せず文書やコマンドの成否だけを信じた」という同根の傾向を持つ | 対応見送り。各事象は個別に該当箇所(wasm-boundary.md等)へ反映済みで、共通原則として抽象化するほどの実害増大は確認できていない。同種の事象が今後も繰り返されるようであれば、phase-cycleスキル等への一般原則追記を再検討する | 対応なし(観察事項として記録に留める) |
| `loadScenarios.ts`(zod検証・id重複除去等のロジックを持つ純粋関数)に単体テストが無かった(edge-case-reviewer指摘) | ユーザー確認の上でvitestを導入し、`buildScenarioList`関数として切り出して単体テストを追加(zod検証失敗・id/title空文字・同一id重複・0件時の5ケース)。`.claude/rules/testing.md`・test-strategy.mdへ反映 | [test-strategy.md](../design/test-strategy.md)「5. E2E/スモーク」、`.claude/rules/testing.md` |
| 「これで始める」ボタンに多重クリック防御が無かった(edge-case-reviewerで指摘。連打でStartSessionが二重dispatchされうる) | ユーザー確認の上で対応。`ScenarioCardLarge`にconfirming状態を持たせ、確定操作後はボタンをdisabled化した | [ScenarioCardLarge.tsx](../../packages/ui/src/components/ScenarioCardLarge.tsx) |
