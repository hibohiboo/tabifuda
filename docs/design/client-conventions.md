# クライアント層の表示・操作規約

対象: CLI/WASM/Web各層。decide/applyの規範(何が起きるか)は
domain-model.mdが正であり、本文書はそれを**どう見せる/どう操作させるか**の
決定を集約する(非規範。core の decide/apply には影響しない)。

置き場の判断(P3 C0決定): wasm境界そのものの型設計・API仕様は
[wasm-boundary.md](wasm-boundary.md) に分離する。本文書は複数のクライアント
実装(CLI/Web)にまたがりうる「表示・操作ロジックの決定」を集める。

## Event/Commandの網羅性(ハンドラマップパターン)

wasm-boundary.md方針3の実装。`Event`/`Command`は`#[non_exhaustive]`な
外部タグ付き判別共用体としてts-rsから生成される(`{ "CardPlayed": {...} }`
形式、内部タグ`{type: ...}`ではない)。TS側でこれを処理する場合、`switch`の
`default:`や部分的なif連鎖では新variant追加時に「黙って無視する」経路が
残ってしまう。

- `apps/web/src/core/taggedUnion.ts`の`TagOf<U>`/`PayloadOf<U,K>`/
  `HandlerMap<U,R,C>`/`dispatchTagged`を使い、**完全網羅なハンドラ
  オブジェクト**として実装する(Event専用ではなく判別共用体全般に使える
  汎用ユーティリティ)
- Rust側に新variantが追加されbindingsが再生成されると、ハンドラオブジェクト
  リテラルがキー不足で`tsc`エラーになる(CIで検出される。P3 C3「未対応の
  種別を黙って無視しない」要求を型で満たす)
- 「明示的に扱うが描画・処理しない」場合はキー自体は書いた上で`null`等の
  no-op値を返す(キーを省略するのではない)。例:
  `apps/web/src/chronicle/eventRenderers.tsx`の`CardRemoved: () => null`

## 冒険記(Web版タイムライン)のCardRemoved

domain-model.md「冒険記(chronicle): CardRemovedは明示的に扱うが、テキストと
しては描画しない」をWeb版でも採用する(crates/tabifuda-cli/src/chronicle.rs
と同じ判断)。消費・シーン離脱による自動除去は物語の流れに不要な機構的詳細
のため。

## 冒険記のカード名解決(Web版)

CLI版`chronicle.rs`はイベントを1パスで畳み込みながら`ScenarioPatched`の
`AddCardDef`を反映して名前解決するが、これはCLIが`Session`を保持せず
`Event`列だけからchronicleを描画する制約に起因するもの。Web版は
`useGameSession`が最終`Session`を保持しているため、**タイムライン全体の
名前解決に`session.scenario`(全パッチ適用済み)をそのまま使う**。
`PatchOp::AddCardDef`は重複IDを拒否した上でpushするのみで置換・削除の経路が
無い(単調追加)ため、あるイベント時点で参照される`CardId`はその時点で既に
`card_defs`に存在しており、最終状態からの解決で取りこぼしは起きない
(CLIのような逐次スナップショット再構築はWeb版には不要)。

## GM裁定UI(Web版、CLIパリティ)

`crates/tabifuda-cli/src/play.rs`と同じ`y`(採用)/`n`(却下)/`c`(カードを
配って応える)の3択をWeb版(`components/GmJudgePanel.tsx`)でも実装する。
`c`は`session/gmResponse.ts`の`nextGmCardId`(`gm-card-{n}`形式、
`scenario.card_defs`と衝突しない最小のn。CLIと同じ発番規則)で採番した
CardIdを使い、`ApplyPatch{ops:[AddCardDef{...,kind:Scenario,...},
DealCard{card,to:Party}], note}`を発行する。一意性検証はCLIと同様
`decide`内の`validate`が担い、TS側はルール分岐を持たない。

## 手札表示からの Marker 除外

`CardKind::Marker` は世界の状態・選択の成立を示す印であり
(domain-guide.md「世界はすべてカード」)、プレイヤーが選ぶ対象ではない。
そのため **クライアントの手札表示からは `CardKind::Marker` を除外する**。

- `session.hands` のデータ自体は変更しない(除外は表示層のみの決定。
  `Condition::HasCard` の判定にも影響しない)
- CLI(tabifuda-cli)は本規約に従って実装済み
- Web版UI(apps/web)も実装済み
