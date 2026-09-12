---
status: done
cycles:
  C1: done
  C2: done
---

# Phase 7 実装タスク: 依頼(シナリオ)選択画面

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
**開始前の儀式(全フェーズ共通)**: CLAUDE.md と docs/design/ の関連文書を読む。

## 経緯

[tasks/projects/phase6/task.md](../phase6/task.md)(カードUI改善)の
起票時、依頼選択の張り紙カードUI([requirements/future-requirements.md](../../../requirements/future-requirements.md)
§11)を検討したところ、**シナリオ選択機能自体が未実装**
(apps/webは`simple-hunt`固定で起動する)ことが分かった。カードの
ビジュアル仕様(白銀比・既定アイコン)はP6が担うが、「複数シナリオから
選ぶ」という機能自体は別物のためP6のスコープ外とし、本フェーズとして
切り出した。

## 目的

依頼(遊べるシナリオ)を複数の中から選んで開始できるようにし、選択画面を
張り紙カードのUIで実装する。

## スコープ

**ソロプレイの範囲に限定する**(P4=非同期マルチプレイのバックエンドに
依存しない)。ローカルに複数シナリオ(`shared/scenarios/`等)を持たせ、
その中から選んで `StartSession` する形を想定する。

GMのセッション募集(GMが複数シナリオをストック・カスタマイズし、募集して
プレイヤーを集める非同期UX。[future-requirements.md](../../../requirements/future-requirements.md)
§1「セッション募集(非同期マルチプレイ、未モデル化)」)は**本フェーズに
含めない**。P4凍結解除後、§1側で改めて扱う。

## 前提となる設計決定

- カードの見た目(白銀比・種類別既定アイコン・2サイズ)は
  [design/ui-visual-design.md](../../../design/ui-visual-design.md)
  「カードのビジュアル方針」が正。**P6完了(カードコンポーネントの実装)を
  前提とする**(二重投資を避けるため、P6のコンポーネントを流用する)
- 画面の想定は [docs/rdra/screens.yaml](../../../rdra/screens.yaml) の
  「依頼選択」(`status: future`)

## 着手前の検討結果(2026-09-12、grillingスキルで確定)

- **シナリオを複数持たせる方法**: 1シナリオ1ファイル(`shared/scenarios/{id}.json`
  の既存配置を維持)。一覧への集約は`import.meta.glob`等でのビルド時動的検出
  とし、ファイルを置くだけで一覧に反映される形にする(手動indexは持たない)
- **型安全性の補完**: 動的検出で失われる型の弱さは、依頼選択画面が表示する
  最小限の情報(`meta.id`/`meta.title`/`meta.summary`)のみをzodスキーマで
  検証することで補う。`Scenario`型全体はts-rs生成が正(SSoT)であり、
  `card_defs`/`phases`等はzod化せず既存型へ`as`キャストで委ねる(壊れていれば
  プレイ開始時にcoreの`decide`が拒否する既存の安全網に委ねる)。zodは新規依存
  追加(現状未導入。導入時にclient-conventions.mdへ実装パターンとして追記する)
- **`StartSession`コマンドの変更要否**: 変更しない。選択画面はクライアント側で
  保持する複数`Scenario`オブジェクトから1つを選び、そのまま`dispatch`する
  だけで配線できる(coreの型・Event/Commandの追加なし)
- **依頼(張り紙)カードに載せる情報**: `title` + `summary`(概要、1〜2行)。
  `ScenarioMeta`に`summary: BoundedString<400>`を新規追加する(domain-model.md
  改訂+ts-rs bindings再生成を伴う)。難易度目安は見送り
  (判定システム未実装のため裏付けとなる指標が無く時期尚早。
  future-requirements.mdへ送る)
- **テスト用シナリオ**: 複数シナリオからの選択を確認するため、簡単なダミー
  シナリオを1本新規に追加する(既存の`simple-hunt-fork.json`はフォーク出力の
  テスト成果物であり転用しない)

## サイクル

### C1: `ScenarioMeta.summary`追加

- domain-model.mdの`ScenarioMeta`定義に`summary: BoundedString<400>`を追加
  (「文字列の長さ上限」節の一覧にも追記)
- `crates/tabifuda-core`の`ScenarioMeta`へ`summary`フィールドを追加
- `ts-rs` bindings再生成(`crates/tabifuda-wasm/bindings/`)
- `shared/scenarios/simple-hunt.json`へ`summary`を追記
- テスト用ダミーシナリオ(`shared/scenarios/{id}.json`、1〜2シーン程度の
  最小構成)を1本新規追加し、`summary`込みで作成する

### C2: 依頼選択画面の実装

- `apps/web`に依頼選択画面を新設(`shared/scenarios/`を`import.meta.glob`で
  動的検出し、選択画面用の最小情報をzodスキーマで検証)
- 張り紙カードUI(グリッド表示。rdra/screens.yaml「依頼選択」のワイヤー
  フレームに従う)。**着手時の確認で当初想定から変更**: `ScenarioMeta`
  (id/title/summary)は`CardDef`と型が異なるため、P6の`Card`/`CardLarge`を
  直接流用せず、`ScenarioMeta`専用の新規コンポーネント(`ScenarioCard`/
  `ScenarioCardLarge`)を`packages/ui`に作り、`.tf-card`系CSS(白銀比・
  2サイズ)のみ流用した(2026-09-12、c2-checklist.md「着手前に確認した
  実装方針」)
- 選択確定で該当`Scenario`を`StartSession`へそのまま渡す配線
- apps/webのPlaywrightスモークを更新(複数シナリオからの選択→開始を確認)
- `docs/rdra/screens.yaml`の`scenario-select`エントリの`status`を
  `implemented`へ更新

## 完了条件

- 複数シナリオ(テスト用ダミー含め2本以上)から選んで `StartSession` できる
- 選択画面が張り紙カードのUI(`packages/ui`の`ScenarioSelect`系
  コンポーネント)で表示され、シナリオ名・概要が確認できる
- 既存のPlaywrightスモークが通る(選択画面経由のフローを含む)

## やらないこと

- GMのセッション募集(非同期マルチプレイ前提。future-requirements.md §1)
- P4(バックエンド)の再開・凍結解除の判断
- 難易度目安の表示(判定システム未実装のため時期尚早。future-requirements.mdへ)
- ロール制(future-requirements.md §12。判定・戦闘同様、枠組みの検討のみで
  実装は別途)
