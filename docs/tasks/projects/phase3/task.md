---
status: planned
cycles:
  C0: planned
  C1: planned
  C2: planned
  C3: planned
  C4: planned
---

# Phase 3 実装タスク: WASM+Web版(ローカル)

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
開始前の儀式は phase2/task.md 冒頭と同じ(文書確認→差分報告→改訂→着手)。
**着手前にOpus 4.8でwasm境界API設計レビューを1回**(agent-operations.md)。

## 目的
バックエンドなしのローカルWebアプリで単純討伐を遊べ、カードが時系列に
並ぶ冒険記UIを見られる状態にする。

## 人間の事前決定(C1前に確認)
- フロントエンドフレームワーク(推奨: React+Vite。決定をADRに記録)

## P2からの申し送り(docs/retrospectives/phase2.md より)

- **シナリオデータの人間レビューは実プレイで行う**: P2 C2の「データレビューを
  人間に依頼」は文面確認のみで素通しになった。以降、シナリオデータを追加・
  変更するサイクルの人間レビューは docs/demo.md の手順による実プレイを基本とする
  (Web版が動いたらWeb上でのプレイに置き換えてよい)
- **lint Warning(到達不能・詰み)の作者体験が未検証**: 検出ロジックは
  テスト済みだが、実データで警告が出た時の見え方・直しやすさは見ていない。
  シナリオを追加するサイクルで、意図的にWarningを出して確認する
- **カードの消費・除去はPhase 2で実装済み**(domain-model.md「カードの
  消費・除去」節、`Event::CardRemoved`)。C3(冒険記タイムラインUI)は
  `Event`の`match`に`CardRemoved`が含まれる前提で設計すること
  (`#[non_exhaustive]`のため、TS側のマッピングでも未対応の種別を
  黙って無視しない扱いにする)。CLI版(tabifuda-cli/src/chronicle.rs)は
  `CardRemoved`をタイムラインに描画しない判断をしたが、Web版で同じにするかは
  C3で改めて判断してよい

## サイクル

### C0: フロント層設計文書の置き場を決める(C1と同セッションでよい)
- wasm境界・UI表示ロジックの決定(例: 現状domain-model.mdに書かれている
  「CLIの手札表示からMarkerを除外する」)を置く場所を決める
  (design/配下にレイヤ別文書、例: wasm-boundary.mdを新設するか、既存文書に
  節を足すか)。決めたら該当する既存記述を移す
  (経緯: docs構造レビュー L1)

### C1: tabifuda-wasm
- wasm-bindgen で decide/apply/validate/lint をTSへ公開。
  Command/Event はJSONで受け渡し(TS型定義を生成または手書きで同期)
- 境界の型往復テスト(wasm-bindgen-test)数本のみ(ルール再テスト禁止)

### C2: apps/web 骨格
- pnpm workspace をモノレポに導入、CI拡張(型チェック・lint・build)
- シナリオ読込→セッション開始→カードを出して進行、まで(見た目は最小)
- 状態管理はイベント列を正とし、UI状態はそこから導出する

### C3: 冒険記タイムラインUI
- イベント列を時系列カードとして描画。CardPlayedはカード+free_text吹き出し、
  ScenarioPatchedは「GMが改修」カード+note、提案と裁定も1枚ずつ
- 台詞自由入力・提案・GM裁定(ソロ両ロール)のUI

### C4: UGC規律とスモーク
- cross-cutting.md §UGC の徹底: 生HTML挿入なし(dangerouslySetInnerHTML
  検出をlint/CIで禁止)、free_text長さ上限
- Playwrightスモーク1本(単純討伐を1本通す)

## 完了条件
ブラウザで通しプレイ可能 / タイムラインUIで冒険記閲覧可能 /
生HTML挿入の静的検査がCIに入っている
