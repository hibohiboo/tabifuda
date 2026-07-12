# Phase 3 実装タスク: WASM+Web版(ローカル)

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
開始前の儀式は phase2-task.md 冒頭と同じ(文書確認→差分報告→改訂→着手)。
**着手前にOpus 4.8でwasm境界API設計レビューを1回**(agent-operations.md)。

## 目的
バックエンドなしのローカルWebアプリで単純討伐を遊べ、カードが時系列に
並ぶ冒険記UIを見られる状態にする。

## 人間の事前決定(C1前に確認)
- フロントエンドフレームワーク(推奨: React+Vite。決定をADRに記録)

## サイクル

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
