---
paths:
  - "apps/api/src/**"
---

# apps/api アーキテクチャ規約

## 構造(どこに何を置くか)

```
src/
  - features/<機能名>/
    - <機能名>.controller.ts # リクエストの受け口
    - <機能名>.service.ts    # ビジネスロジック
    - <機能名>.repository.ts # データベースの読み書き
    - <機能名>.types.ts      # 型定義
  - lib/ # 機能横断の共通処理のみ
```

迷ったらこの順で問う:

1. 特定機能の業務ロジック -> `features/<機能名>`
2. どの機能にも属さない基盤 -> `lib/`
3. 複数機能で共有したい処理 -> 安易に共通化せず、まず各機能に置く

## 1ファイル1責務

正はCLAUDE.md最重要ルール5(docs/adr/0007-ssot-single-responsibility.md)。

## 機能境界

- 他機能のテーブルへ直接クエリしない。所有機能のservice関数を経由する
  (例: 入金処理がユーザー情報を更新するときはuserの関数を呼ぶ)
- controllerに業務ロジック・クエリを直書きしない。serviceの関数を呼ぶだけにする
- barrel export(index.tsへの集約・再エクスポート)禁止。実ファイルへ直接importする

## 副作用の分離(Functional Core / Imperative Shell)

- 業務判断・計算・変換は純粋関数に集める(Functional Core)
- DBの読み書き・外部API・時刻・乱数・環境変数参照は外部の薄い層へ寄せ、
  Coreが返した結果を実行するだけに近づける(Imperative Shell)
- 狙いは、複雑なロジックを外部依存なしでテストできる形に保つこと
