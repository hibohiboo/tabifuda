# Phase 6 ふりかえり(2026-09-12)

対象: [tasks/projects/phase6/task.md](../tasks/projects/phase6/task.md) C1〜C3
(ブランチ `phase6`)。
位置づけ: 記録([docs/README.md](../README.md)「文書区分の定義」)。
ここで挙げた対応項目は反映先の文書が正であり、本文書は経緯を残す。

## 成果

完了条件(task.md)を4点とも満たした。

| 完了条件 | 充足状況 |
|---|---|
| 手札・冒険記・GM裁定パネルのカードが、白銀比縦長・種類別既定アイコンを持つカードコンポーネントで表示される(小/大2サイズとも実装済み) | 満たす。C1で小サイズ`Card`、C2で大サイズ`CardLarge`+`Hand.tsx`置換、C3で`eventRenderers.tsx`(冒険記)・`GmJudgePanel.tsx`へ適用 |
| コンポーネントカタログでカードの全種類(Marker含む)を確認できる | 満たす。C1で6種類の見本を追加、C2で「CardKind一覧」凡例セクションを追加 |
| 既存のPlaywrightスモークが通る | 満たす。C2で`simple-hunt.spec.ts`を新フロー(タップ→モーダル展開→「出す」)に書き換え、以降のサイクルでも継続して通過を確認 |
| Q4(身近な人に見せるか)の再判断結果が記録されている | 満たす。C3で再判断し「まだ見せない(現状維持)」と記録(2026-09-12、[c3-checklist.md](../tasks/projects/phase6/plans/c3-checklist.md)・[post-p3.5-replanning-decisions.md](../tasks/plans/post-p3.5-replanning-decisions.md)) |

### サイクル別の成果物

| サイクル | 日付 | 成果物 |
|---|---|---|
| C1 | 2026-08-12 | `packages/ui`に`Card`(小サイズ、白銀比1:√2、CardWirthの74x94相当)を新設。`CardKind`6種の既定アイコンを自作SVGで実装(`cardIcons.tsx`)。コンポーネントカタログに6種見本を追加 |
| C2 | 2026-09-11〜12 | `CardLarge`(大サイズ)・`Modal`(タップ→モーダル展開UI)を新設。`CARD_KIND_COLORS`(種別ごとの縁取り色)追加。`Hand.tsx`を新カードUIに置換。配色・フォント・アニメーション手段・操作フローをui-visual-design.mdへ確定。コンポーネントカタログに「CardKind一覧」凡例を追加 |
| C3 | 2026-09-12 | 冒険記(`eventRenderers.tsx`)の`CardDealt`/`CardPlayed`/`RewardsGranted`/`CardsDiscarded`に小`Card`を添付、`GmJudgePanel.tsx`に入力中カードのプレビュー表示を追加。Q4再訪 |

このフェーズは`crates/`への変更が無い(TS側(`packages/ui`・`apps/web`・`tools/docs-site`)のみ)。`packages/ui`の新規コンポーネント(`Card`/`CardLarge`/`Modal`)に対する単体テストは無く、検証は`apps/web/e2e/simple-hunt.spec.ts`(Playwrightスモーク)とブラウザでの目視確認に依っている(評価は課題節参照)。

## うまくいったこと

- **前サイクルの反省が次サイクルで実際に活かされた**。C1は「着手前に人間へ質問する」手順を踏まずに実装してしまったが([c1-checklist.md](../tasks/projects/phase6/plans/c1-checklist.md)「事後確認」)、C2は[c2-checklist.md](../tasks/projects/phase6/plans/c2-checklist.md)冒頭で明示的にインタビューを行ってから着手しており、C1の反省が同じフェーズ内で機能した実例。
- **3段階レビュー(eng-practicesセルフレビュー→`/code-review`→`edge-case-reviewer`)が各サイクルで実際に重大な指摘を検出**。C2では`edge-case-reviewer`がP1 3件(IME変換中Escapeでの誤クローズ、フォーカストラップ未実装によるモーダル外操作でのデータロス、CardDef未解決フォールバックの未確認使用)を検出、C3でも`edge-case-reviewer`がP1 2件(モーダルのフォーカス復帰が「出す」の主要フローで空振り、複数枚カードイベントでの画面幅はみ出し)を検出し、いずれも同PRで修正・実機確認済み。
- **design-syncが複数サイクルで文書と実装の乖離を継続して捕捉**(C1: 74x94の縦横比丸め誤り、C2: 白銀比の適用範囲・アニメーション手段の用語、C3: `eventRenderers.tsx`の記載パスの古さ・`GmJudgePanel`プレビューの記載漏れ)。
- **設計文書を先に直してから実装を直す順序を実践できた**。C2で発覚した`CardKind::Proposal`のサンプルデータが実在しない運用を前提にしていた問題(ユーザー指摘)では、ui-visual-design.md・domain-model.mdを先に更新してから`CardLarge`の自由入力欄をProposalへ拡張しており、CLAUDE.md最重要ルール1(設計文書が正)を実地で守れた。

## 課題(ジャーナル記録済み)

[docs/agent-journal.md](../agent-journal.md)にP6関連エントリが複数件ある(2026-08-12〜09-12)。

1. **C1**: 「74x94相当が収まるサイズ」という文書の具体的寸法を読みつつ、実装ではアイコン領域を74x74の正方形にしてしまった。design-syncで発覚し同PR内で`aspect-ratio: 74/94`に修正。
2. **C1**: TS側のみの変更サイクルにもかかわらず`cargo fmt --check`を無条件実行し、docs-siteに無いlintスクリプトも実行してエラーになった。phase-cycleスキルの「終わり方」を変更範囲で分岐する形に修正するきっかけとなった。
3. **C1の事後確認**: C1が着手前インタビューの対話プロセスを経ずに実装されていたことに、C2着手前の会話で気づいた。事後的に4点を質問し記録、phase-cycleスキルに「着手前に人間へ質問する」手順を追加。
4. **C2**: `pnpm --filter <pkg> lint --if-present`という誤った構文(CLAUDE.md等に記載されていた)を実行してexit 1になった。正しくは`pnpm --filter <pkg> run --if-present lint`。CLAUDE.md・phase-cycleスキル・client-conventionsスキルの3箇所を修正。
5. **C2**: a11yの論点を「表示(読み上げ)」「操作(フォーカス移動)」に分けずに一括で質問したため、モーダルのフォーカストラップ起因のデータロスという操作フロー由来の問題を着手前に想定できなかった(edge-case-reviewerが事後検出)。
6. **C2**: タイトル自動縮小の判定に`scrollWidth`/`clientWidth`比較を使ったが、`align-self`の設定次第で「収まっているか」を正しく判定できない/`overflow: hidden`要素では余裕量を取得できないというブラウザ仕様に2度ハマった。最終的にCanvas `measureText`によるテキスト幅の直接計算に切り替えて解消。
7. **C2**: コンポーネントカタログの`CardKind::Proposal`サンプルに、実装上どこにも存在しない具体的な提案内容のカード名を割り当てていた(実際の提案操作は`Command::Propose`という別経路)。ユーザー指摘で発覚し、設計文書を先に更新してから実装・カタログサンプルを修正した。
8. **C2→C3境界**: C2完了直後、「Timelineに選んだ小カードを表示するようにして」という依頼が実質的にC3(task.mdの次サイクル)のスコープ本体だったが、会話の慣性で「C2の追加フィードバック対応」として処理しかけた。ユーザー確認の上でC3として正式着手し直した。
9. **C3**: モーダルのフォーカス復帰処理を、要素がDOMに残る「キャンセル」経路でのみPlaywright検証し、要素が消滅する「出す」経路では未検証だった。後続のedge-case-reviewerで空振りが発覚し、フォールバック先を追加して修正。

## 気づきと対応(反映先)

| 気づき | 対応 | 反映先 |
|---|---|---|
| 列挙型(`CardKind`等)のカタログ向けサンプルデータを機械的に埋める際、各バリアントの実装上の生成・消費経路を確認しないと、実在しない設計をカタログという「規範に近い見本」に紛れ込ませうる(課題7の実例) | 「列挙型の網羅サンプルを作る際は、各バリアントが実装(decide/apply/CLI/UI)のどこで生成・消費されるかを1つずつ確認する」観点をclient-conventions.mdに追記した(同PRで反映) | [client-conventions.md](../design/client-conventions.md)「UIコンポーネントの置き場」 |
| `packages/ui`のカードコンポーネント(Card/CardLarge/Modal)に単体テストが1件も無いままP6完了。C1着手前インタビューとC2のedge-case-reviewerの両方で独立に「現状のまま許容」と判断されたが、その方針が規範文書に記録されておらず、次にpackages/uiへコンポーネントを追加するサイクルで同じ論点を再検討し直す可能性がある | 「packages/uiの表示コンポーネントはPlaywrightスモーク+目視確認で担保し、ロジックが薄いものは単体テスト必須としない」旨をtest-strategy.mdに追記した(同PRで反映) | [test-strategy.md](../design/test-strategy.md) |
| C2完了(frontmatterがdoneに更新)後も、ユーザーの視覚フィードバックにより多数の追加修正コミット(タイトル位置・自動縮小方式・横アイコンの追加/復活/撤回・白銀比の再確定等)が続いた。ビジュアル調整が主目的のサイクルでは、実装完了時点のレビューだけでは方針が確定しきらず、実機で見た人間の主観フィードバックによる複数回の巻き戻しが起きやすい | 対応見送り。完了条件の未達や手戻りコスト増大という実害は確認できておらず、doneマーキング自体が誤りだったとは言えない(通常のデザインレビューの反復と判断)。強い根拠が無いため反映しない | 対応なし(観察事項として記録に留める) |
| docs-site D8(マニュアルタブ)のような、P6と直接関係しない要望が、phase6ブランチから枝分かれした別ブランチ(`docs-site-d8-manual-tab`)で対応されphase6へマージされた。運用ルール(タスク発行前提・専用ブランチ)には違反していない | 対応不要(運用違反ではない)。同種のケースが繰り返し起きるようであれば、フェーズブランチから分岐した無関係タスクの扱いを明文化する要否を再検討する | 対応なし(観察事項として記録に留める。再発頻度を見て判断) |
