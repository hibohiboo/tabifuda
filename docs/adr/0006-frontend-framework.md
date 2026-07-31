# ADR 0006: フロントエンドフレームワークに React + Vite を採用

状態: 採用 / 日付: 2026-08-01

## 文脈

Phase 3(WASM+Web版)で apps/web を新設するにあたり、フロントエンド
フレームワークを決める必要がある(docs/tasks/projects/phase3/task.md
「人間の事前決定」)。候補は React+Vite / Svelte+Vite / Vanilla TS+Vite。

## 決定

React + Vite を採用する。

理由:

1. **エコシステムの厚さ。** wasm-bindgen 生成物の型定義との相性、
   状態管理ライブラリ、テストツール(Testing Library等)、Playwright連携の
   実績が最も豊富で、P3.5/P4以降も見据えた際の情報量が多い
2. **cross-cutting.md との整合。** 「自由入力(UGC)の取り扱い」で
   `dangerouslySetInnerHTML` 禁止を明記しており、Reactのデフォルト
   エスケープ挙動を前提にした設計になっている
3. **エージェント実装との親和性。** タスク文書のモデルラダー
   (agent-operations.md)は「Sonnet 5が既定」であり、Reactは学習データが
   豊富でエージェントによる実装・レビューの安定度が高い

Svelte・Vanilla TSを採用しない理由: いずれもエコシステムの厚さで
Reactに劣り、消去法で外れる。将来UIが複雑化した場合の再評価は妨げない。

## 帰結

- apps/web は Vite + React + TypeScript で構築する
- pnpm workspace への追加時、cross-cutting.md の UGC規律(生HTML禁止の
  lint/CI組込)を React 前提で実装する
- 状態管理はイベント列を正とし、UI状態はそこから導出する
  (phase3/task.md C2の方針どおり。Reduxのような別正は持ち込まない)
