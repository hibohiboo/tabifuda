# Phase 3 C3「冒険記タイムラインUI」実装計画

## Context

C2でapps/webの骨格とイベント列駆動の状態管理(`useGameSession`)、最小ゲーム
ループ(シナリオ読込→セッション開始→カード進行)が動く状態になった。C3は
task.mdの「冒険記タイムラインUI」「台詞自由入力・提案・GM裁定(ソロ両ロール)
のUI」を実装し、task.md完了条件の「タイムラインUIで冒険記閲覧可能」を満たす。

wasm-boundary.md方針3が、TS側のEvent処理は`switch`ではなく**完全網羅の
ハンドラマップ**で行うことを明記しており(`#[non_exhaustive]`に新variantが
増えたらTSコンパイルが失敗する形にする)、C3でこれを実装しclient-
conventions.mdに反映することが明示的に求められている。これが今回の中核的な
技術要件。

GM裁定UIはCLIパリティ(y=採用/n=却下/c=カードを配って応える)で実装する
(ユーザー承認済み)。CLIの`play.rs`/`chronicle.rs`を参照実装とする。

## 参照実装から読み取った前提

- `crates/tabifuda-cli/src/chronicle.rs`: `CardRemoved`は明示的にmatchするが
  本文は空(描画しない)。`ScenarioPatched`は`note`のみ表示。`SceneEntered`は
  生の`SceneId`を見出しに使う(`SceneDef`に表示名フィールドは無い)。
- `crates/tabifuda-cli/src/play.rs`: Paused画面は`y`/`n`/`c`。`c`はGMに
  カード名・回答文を入力させ、`gm-card-{n}`形式で連番発番したCardIdを使い
  `ApplyPatch{ops:[AddCardDef{...,kind:Scenario,...}, DealCard{card,to:Party}],
  note: 定型文}`を1回発行、Pausedのまま継続(複数回`c`可)。
- `PatchOp::AddCardDef`はcore側(`patch.rs`)で重複ID拒否のみ・置換や削除の
  経路が無い(単調追加)。そのため**タイムラインのカード名解決は最終的な
  `session.scenario`(全パッチ適用済み)をそのまま使えば足りる**
  (CLIのような1パスでのscenario再構築は不要。Web版は既に`Session`全体を
  `useGameSession`で保持しているため)。

## 設計決定

### ハンドラマップ(wasm-boundary.md方針3の実装)

`apps/web/src/core/taggedUnion.ts`に、ts-rs生成の外部タグ付き判別共用体
(`{ "CardPlayed": {...} }`形式)全般に使える汎用ユーティリティを置く
(Event専用ではなく将来Command等にも転用可能にする):

```ts
export type TagOf<U extends object> = U extends unknown ? keyof U : never;
export type PayloadOf<U extends object, K extends TagOf<U>> =
  Extract<U, Record<K, unknown>>[K];
export type HandlerMap<U extends object, R, C = void> = {
  [K in TagOf<U>]: (payload: PayloadOf<U, K>, ctx: C) => R;
};
export function tagOf<U extends object>(value: U): TagOf<U>;
export function dispatchTagged<U extends object, R, C = void>(
  value: U, handlers: HandlerMap<U, R, C>, ctx: C,
): R;
```

Rust側の`Event`に新variantが追加されbindingsが再生成されると、
`HandlerMap<Event, ...>`を満たすオブジェクトリテラルはキー不足で`tsc`が
落ちる。「明示的に扱うが描画しない」場合はキー自体は書いた上で`null`を
返す(キー省略ではない)。このパターンをclient-conventions.mdに追記する。

### タイムライン表示仕様

| Eventタグ | 描画 | 内容 |
|---|---|---|
| `SessionStarted` | ○ | 「冒険『{title}』が始まった」+参加者名 |
| `SceneEntered` | ○ | 見出し(生の`SceneId`)+`narration` |
| `CardDealt` | ○(控えめ) | 「{to}に『{cardName}』が配られた」 |
| `CardPlayed` | ○ | 「{by}は『{cardName}』を出した」+`free_text`があれば吹き出し |
| `CardRemoved` | ✗(`null`) | 描画しない(domain-model.md「冒険記」・CLIと同じ判断) |
| `EffectApplied` | ○(簡素) | 「(未解決の効果が記録された)」 |
| `ProposalSubmitted` | ○ | 「{by}が提案した:『{text}』」 |
| `ScenarioPatched` | ○ | 「GMがシナリオを改修した:『{note}』」(`ops`詳細は出さない、CLIと同じ) |
| `ProposalJudged` | ○ | 「GMは提案を採用/却下した」 |
| `PhaseAdvanced` | ○ | 「── フェーズが{phase}へ ──」 |
| `SessionEnded` | ○ | 「=== 冒険の終わり: 勝利/敗北 ===」 |

カード名解決は`session.scenario`(最終状態)から`findCardDef`で引く。
ui-visual-design.mdの「フェーズ単位の山→展開」案は同文書が「初期案、C3で
実データを見て調整してよい」としているとおり、今回はフラットな時系列
リストに留める(実データ量が少ないため)。

### 自由入力(Dialogueカードのみ)

`Hand`にローカルstate(「どのインスタンスが自由入力待ちか」)を持たせる。
`CardKind::Dialogue`のカードをクリックすると即発行せず、そのカードの位置に
`FreeTextInput`(上限4096文字、空欄可)を展開。送信で
`PlayCard{by,card,free_text}`を発行。非Dialogueは従来どおり即時発行
(`free_text: null`)。

### 提案UI

`Running`中に`SceneView`の下へ常設する`ProposalForm`(`FreeTextInput`の
薄いラッパー、上限4096文字)。送信で`Propose{by: SOLO_CHARACTER_ID, text}`
を発行。

### GM裁定UI(y/n/c、CLIパリティ)

`session.status === {"Paused":{proposal}}`のとき`GmJudgePanel`を表示。
`session.pending_proposal`(状態機械の不変条件によりnon-null)を渡す。

- 「採用する」/「却下する」: `JudgeProposal{proposal: proposal.id, accepted}`
- 「カードを配って応える」: カード名・回答文の入力フォームを展開。送信で
  `session/gmResponse.ts`の`buildAnswerPatch(scenario, name, text)`が
  `nextGmCardId`(`gm-card-{n}`、`scenario.card_defs`と衝突しない最小のn。
  CLIと同じ発番規則)で採番したCardIdを使い、
  `ApplyPatch{patch:{ops:[AddCardDef{id,name,kind:"Scenario",text,tags:[],
  effects:[],requires:[]}, DealCard{card:id,to:"Party"}], note:"提案に
  応えてカードを配布"}}`を発行。Pausedのまま継続(複数回`c`可能、CLIと同じ)。
  一意性検証は`decide`内の`validate`が最終的に担う(CLI同様TS側はルール
  分岐を持たない)

## 追加/変更ファイル

```
apps/web/src/
  core/
    taggedUnion.ts        # 新規
    bindings.ts           # 追加re-export: Proposal, PatchOp, ScenarioPatch, Target
  chronicle/
    eventRenderers.tsx    # 新規: HandlerMap<Event, ReactNode|null, {scenario}>
    Timeline.tsx           # 新規: <Timeline events scenario />
  session/
    useGameSession.ts      # 変更: events も返す(Timelineの入力)
    gmResponse.ts           # 新規: nextGmCardId, buildAnswerPatch
  components/
    FreeTextInput.tsx      # 新規: 上限付きテキスト入力(共通部品)
    Hand.tsx                # 変更: Dialogueのみ自由入力インライン展開
    ProposalForm.tsx        # 新規
    GmJudgePanel.tsx        # 新規: y/n/c
    SceneView.tsx            # 変更: ProposalFormを配線
  App.tsx                   # 変更: Timeline/GmJudgePanel配線、Paused分岐を本実装に
  App.css                   # 変更: タイムライン/吹き出し用の最小スタイル
docs/design/client-conventions.md  # 追記: ハンドラマップパターン、CardRemoved
                                     # 非描画、カード名解決方針、GM裁定UI(y/n/c)
```

## チェックリスト分割案(コミット単位)

1. `core/taggedUnion.ts`追加 + `client-conventions.md`にハンドラマップ
   パターンを追記
2. `core/bindings.ts`拡張、`useGameSession.ts`が`events`を返すよう変更
3. `chronicle/eventRenderers.tsx` + `chronicle/Timeline.tsx`(表の仕様どおり。
   CardRemoved非描画・カード名解決方針をclient-conventions.mdに追記)
4. `components/FreeTextInput.tsx` + `Hand.tsx`のDialogue自由入力
5. `components/ProposalForm.tsx` + `SceneView.tsx`/`App.tsx`配線
6. `session/gmResponse.ts` + `components/GmJudgePanel.tsx`(y/n/c) +
   `App.tsx`のPaused分岐を本実装に置換。client-conventions.mdにGM裁定UI
   (CLIパリティ)を追記
7. `App.css`にタイムライン/吹き出し用スタイル追記
8. 仕上げ: design-syncでの乖離チェック、task.mdのC3をdoneに更新、
   agent-journal.md追記

## 検証手順

1. `pnpm --filter @tabifuda/web typecheck`が通ること
2. **ハンドラマップの静的検査を実地確認**: `eventRenderers`から
   `CardRemoved:`エントリを一時的に消し、`tsc --noEmit`がキー不足エラーで
   失敗することを確認してから元に戻す
3. `pnpm --filter @tabifuda/web lint`が通ること
4. `pnpm --filter @tabifuda/web dev`でブラウザ確認(Playwrightで手動確認):
   - はじめる→タイムラインに`SessionStarted`/`SceneEntered`が出る
   - Dialogueカードで自由入力→送信すると吹き出しが出る/スキップ時は出ない
   - 提案送信→`Paused`に遷移しタイムラインに`ProposalSubmitted`+
     `GmJudgePanel`が出る
   - 「カードを配って応える」→新カードが手札に配られ、出すと回答文が読める
     (`CardDef.text`)。再度Pausedのまま→最後に採用/却下で`Running`に戻る
   - `simple-hunt.json`を最後まで進め`SessionEnded`+勝敗表示に到達
   - devtoolsコンソールにエラーが無いこと
5. `pnpm --filter @tabifuda/web build`→`preview`で再確認
6. `cargo test/clippy/fmt`(Rust側は無変更のはずだが確認)

## Critical Files
- `crates/tabifuda-cli/src/chronicle.rs`, `crates/tabifuda-cli/src/play.rs`
- `crates/tabifuda-wasm/bindings/Event.ts`
- `apps/web/src/session/useGameSession.ts`, `apps/web/src/App.tsx`
- `docs/design/client-conventions.md`, `docs/design/wasm-boundary.md`
