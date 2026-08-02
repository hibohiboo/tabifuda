# Phase 3.5 ふりかえり(2026-08-02)

対象: [tasks/projects/phase3.5/task.md](../tasks/projects/phase3.5/task.md) C1〜C4
(ブランチ `phase3.5`)。
位置づけ: 記録文書(非規範)。ここで挙げた対応項目は反映先の文書が正であり、
本文書は経緯を残す。

## 成果

完了条件(task.md)を4点とも満たした。

| 完了条件 | 充足状況 |
|---|---|
| 中断→再開で通しプレイが最後まで可能(Paused中の中断を含む) | 満たす。C1で結合テスト「保存→再開→勝利エンドまで」「Paused中に中断→再開すると裁定待ちに戻り勝利エンドまで到達」を固定 |
| パーティがセッションを跨いで持続し、portableカードの持ち帰りがテストで固定 | 満たす。C4で持ち出し→書き戻し→次セッションの`--party`読み込みまでの結合テストを固定 |
| 非portableカードが持ち出されない拒否系テストがある | 満たす。C3の`engine_tests`に非portableカードの破棄・Markerの持ち出し不可(タグを付けても常に不可)の拒否系2件 |
| lint拡張(シナリオ非依存検査)が入っている | 満たす。C3で`PortableCardIsScenarioDependent`(GotoScene/AdvancePhase/DealCard/HasCard検出)を追加 |

### サイクル別の成果物

| サイクル | 成果物 |
|---|---|
| C1 | `Session`/`Character`のHashMap→BTreeMap化、`SaveFile{format_version, events}`設計と`save.rs`実装、`play --resume`、`q`での中断保存(Paused中含む) |
| C2 | `party.rs`(`Vec<Character>`読込・空配列拒否・CharacterId重複拒否)、`play --party`、無指定時は既定ソロ「旅人」を維持 |
| C3 | `#portable`タグ(予約タグ方式)、`Character.owned_cards`、`Event::RewardsGranted`/`CardsDiscarded`、finalize(decide/apply)、lint拡張 |
| C4 | `party.rs::write`、セッション終了時のパーティファイル書き戻し配線、demo.md「6. カードを持ち帰る」追加、並行セッション(`-save`/`-save-2`)の衝突回避確認、chronicle.rs/oplog.rsの新Event対応(下記) |

テストは最終時点で tabifuda-core 153件・tabifuda-cli 33件。

## うまくいったこと

- **前フェーズのふりかえりの申し送りが実際に機能した**。phase3ふりかえりが
  指摘した`Session`内HashMapの非決定性を、C1着手前の「人間の事前決定」
  ゲートとして先に解消した。
- **リスクの高い決定を実装前に人間承認するゲートが機能**(C1のformat_version
  方針、C3のEvent追加承認・タグ方式選定)。C2/C4は事前決定不要と明示的に
  スコープ確認しており、ゲートの要否判断自体が文書化されていた。
- **先回り実装の回避が機能**。C3でfuture-requirements.md旧§3の2経路
  (持ち出しportable / 作者定義報酬`GrantPoints`)のうちtask.md明記の前者
  のみに絞り、`GrantPoints`は「キャラメイク機構が無く置き場が無い」として
  実装せず据え置いた判断が記録されている。
- **design-syncが複数箇所への反映漏れを継続して捕捉**(C1: wasm-boundary.md
  「既知の課題」節、C4: domain-model.md「パーティファイル」節と
  future-requirements.mdの未来形記述の残存)。

## 課題(ジャーナル記録済み)

`docs/agent-journal.md`にP3.5関連エントリ3件。

1. **2026-08-01(C1)**: HashMap→BTreeMap化の決定を決定ログ・domain-model.md
   へは反映したが、背景説明を書いた規範文書wasm-boundary.md「既知の課題」
   節の更新を失念しかけた。design-syncで同日中に発覚・修正。
2. **2026-08-02(C3→C4)**: `Event::RewardsGranted`/`CardsDiscarded`追加時、
   CardRemoved追加時の既存方針(新Event種別はchronicle.rs/oplog.rsの
   matchにワイルドカードへ流さず明示アームを足す)を適用し忘れた。
   C3チェックリストに「C4への申し送り」と書いたが、C4の着手時チェックで
   拾われず、両ファイルとも`_ =>`("未知の出来事"/`Unknown`)に落ちたまま
   demo.mdまで更新してしまっていた。フェーズ完了ふりかえりのドラフト
   作成(retrospectiveエージェント)で発覚し、本サイクル内で
   chronicle.rs(明示描画)・oplog.rs(種別名記録)を修正しテストで固定した
   (コミット`bf2871a`)。
3. **2026-08-02(フェーズ完了報告後)**: ユーザーから「apps/web側のEvent
   描画がRewardsGranted/CardsDiscardedに対応しているか確認したい」と
   指摘され、実際に`pnpm -r typecheck`を実行するまで気づかなかった、
   より根の深い問題が発覚。P3.5全サイクルを通じて
   `crates/tabifuda-wasm/bindings/`(gitコミット対象のts-rs生成TS
   バインディング)を一度も再生成していなかった。task.mdの「P3.5は依存が
   crates/のみでP3・Webとは独立」という記述自体は正しく、それを信頼して
   Web側を確認しなかったこと自体はユーザーも当時気づいていなかった通常の
   判断であり、エージェント固有の誤りではない。本質は「宣言された依存
   範囲の**外**で不具合が見つかった場合にどう扱うか」の運用ルールが
   これまで無かったこと。実際に`pnpm -r typecheck`を走らせると、
   `packages/ui`の`eventRenderers.tsx`(HandlerMap網羅性チェック)が
   コンパイルエラーになり、さらに`tools/docs-site`(未分類テストスイート
   `party::tests`/`save::tests`、Character型サンプルデータ不足)・
   `apps/web`(soloParty.tsのCharacter組み立て不足)でも同型の追従漏れが
   芋づる式に見つかった。**Event/Character等の共有型はRustとTS双方から
   参照されるため、「crates/のみ」という依存宣言は「Web側に実害が及ばない」
   ことまでは保証しない**という点が根本の学び。今回分は全て修正し
   `pnpm -r typecheck`/`lint`/`build`が通ることを確認済み。

## 気づきと対応(反映先)

| 気づき | 対応 | 反映先 |
|---|---|---|
| 新Event追加時、CLI/Web層の`_ =>`ワイルドカードに黙って落ちる危険がdesign-syncの照合観点に明記されていなかった | design-syncスキルの照合観点に「`_ =>`ワイルドカードを持つmatch文の確認」を追加 | 反映済み: `.claude/skills/design-sync/SKILL.md` |
| crates/配下のpub型(#[non_exhaustive] enum・ts-rs対象struct)を変更したサイクルで、tabifuda-wasmのTSバインディング再生成・`pnpm -r typecheck`が終わり方チェックに入っていなかった | phase-cycleスキル「終わり方」に、非exhaustive enum等を変更したら`crates/tabifuda-wasm/bindings/`を再生成し`pnpm -r typecheck`まで通す手順を追加 | 反映済み: `.claude/skills/phase-cycle/SKILL.md`「終わり方」手順2 |
| 「このフェーズは〇〇にのみ依存」という宣言自体は正しく、それを信頼して依存先を確認しないのは通常の判断(ユーザー指摘: 人間側も気づいていなかった)。問題は、宣言の**外側**で不具合が見つかった場合に「その場で現フェーズに畳み込む」か「別チケットに切り出す」かの運用ルールが無かったこと | 宣言範囲の内側で完結する不具合は従来どおり同PRで直し、範囲の**外**に及ぶものはdocs/tasks/plans/へ別途書き出して人間判断を仰ぐ、という区別を明文化 | 反映済み: `docs/agent-operations.md`「宣言された依存範囲の外で見つかった不具合の扱い」節を新設 |
| チェックリスト内の「次サイクルへの申し送り」メモが機械的に拾われず1サイクル分すり抜けた | 上記のスキル・運用ルール改善で当面代替する(申し送り自体の転記を仕組み化するのは費用対効果が低いと判断し見送り) | 対応なし(意図的に見送り。再発したら再検討) |
| `rewards: Vec<Reward>`(シナリオ作者が明示定義する報酬)・`GrantPoints`はキャラメイク機構が無く未実装のまま | future-requirements.mdに残置済み。次にキャラメイク機構を設計するフェーズの着手時チェック項目に含めるとよい | future-requirements.md §3(既に反映済み。次フェーズ着手時に参照) |
| 同一パーティの並行参加時、報酬の書き戻しが競合しうる問題は未決のまま | 引き続き将来要望として保持 | future-requirements.md §1派生論点(既存のまま) |

## 次フェーズへの申し送り

- P4(バックエンド)着手時、finalizeの書き戻し先が「パーティファイル」から
  「DBのマスターデータ」へ変わる想定(future-requirements.md §2/§3参照)。
  CLI層の`party.rs::write`相当のロジックをAPI層でどう置き換えるかは
  P4着手時の設計判断に持ち越す
- crates/配下のpub型を変更するフェーズ・サイクルでは、たとえそのフェーズが
  「Web非対応」を謳っていても、終わり方チェックで`pnpm -r typecheck`まで
  通すことを徹底する(上記の教訓。phase-cycleスキルに反映済み)
