---
paths:
  - "packages/schema/**"
  - "apps/api/src/**/*.repository.ts"
---

# マイグレーション規律と守備範囲

正: docs/tasks/projects/phase4/task.md C1(スキーマ)。ここはP4(Drizzle+Neon。
現在**凍結**、docs/roadmap.md「P4・P5の凍結」参照)着手時にのみ効く実装規約。

## マイグレーション規律

- 適用済のマイグレーション(`drizzle-kit generate` の出力)は編集しない。
  修正は新しいマイグレーションで行う
- 本番・ステージングはロールバックせず、前進のみ(forward only)で修正する。
  開発DBだけはリセットしてよい
- UNIQUE / NOT NULL などの制約を既存テーブルへ追加する前に、
  違反する行がないか読み取り専用の検査クエリで確認する

## events テーブルは追記専用(このプロジェクト固有・最重要)

正は CLAUDE.md 最重要ルール3・domain-model.md「基本原則」。
`events`(session_id + seq 一意)への UPDATE/DELETE を許すマイグレーションは
書かない。本文削除が必要な場合も cross-cutting.md「削除要求と追記のみ原則の
折り合い」の `TextRedacted` イベント追記+対象イベント本文の物理上書きで行い、
行削除・構造改変では行わない。

## DBが守るもの/アプリが守るもの

DB(制約で守る)|アプリ(コードで守る)
--|--
一意性(UNIQUE。events の session_id+seq 等) | 認可(coreのdecideに委ねる。cross-cutting.md「権限」)
参照整合(FK)・値域(CHECK) | 業務フロー(SessionStatusの遷移順序はcoreが決める)
同時実行で破れる不変条件(sessionのversionによる楽観ロック) | 外部サービスとの整合

判断基準: 「UIを通らない書き込み(バッチ・手動SQL・バグ)でも絶対に壊れてはいけないか?」
-> YesならDB制約、NoならAPI層(service)に書く
