---
paths:
  - "apps/api/src/**"
---

# apps/api アーキテクチャ規約

正: docs/design/domain-model.md・cross-cutting.md。ここはP4(Hono+Drizzle+Neon。
現在**凍結**、docs/roadmap.md「P4・P5の凍結」参照)着手時にのみ効く実装規約。

## 構造(どこに何を置くか)

```
src/
  features/<機能名>/       # 例: sessions, scenarios, characters, parties
    <機能名>.controller.ts # Honoルートの受け口。認証(誰か)の確認のみ
    <機能名>.service.ts    # ビジネスロジック(薄く。ルール判定はcoreへ委譲)
    <機能名>.repository.ts # Drizzleでのデータベース読み書き
    <機能名>.types.ts      # 型定義
  lib/                    # 機能横断の共通処理のみ
```

迷ったらこの順で問う:

1. 特定機能の業務ロジック -> `features/<機能名>`
2. どの機能にも属さない基盤 -> `lib/`
3. 複数機能で共有したい処理 -> 安易に共通化せず、まず各機能に置く

## 1ファイル1責務
- ファイルの担当を一文で言えること
- ファイルが肥大したら、責務の境界で割る

正はCLAUDE.md最重要ルール5(docs/adr/0007-ssot-single-responsibility.md)。

## 機能境界

- 他機能のテーブルへ直接クエリしない。所有機能のservice関数を経由する
  (例: セッション進行がキャラクター情報を読むときはcharactersのservice関数を呼ぶ)
- controllerに業務ロジック・クエリを直書きしない。serviceの関数を呼ぶだけにする
- barrel export(index.tsへの集約・再エクスポート)禁止。実ファイルへ直接importする

## 認可はcoreに委ねる(このプロジェクト固有・最重要)

正は cross-cutting.md「権限」。**controller/serviceは認証(誰か)だけを担い、
認可(何ができるか)を自前で判定しない**。コマンド受付は
「認証→Actor構築→(tabifuda-wasm経由で)decide→events追記」の型を守る
(docs/tasks/projects/phase4/task.md C2)。decideが返すRuleErrorをHTTPエラーへ
変換するだけにし、「PausedならXXXを拒否」のような分岐をservice層に複製しない
(docs/design/test-strategy.md「apps/api(P4〜)」の契約テスト方針とも整合させる)。

## 副作用の分離(Functional Core / Imperative Shell)

- 業務判断・計算・変換はtabifuda-core(decide/apply)に集める
- DBの読み書き・外部API・時刻・乱数・環境変数参照はapps/api側の薄い層へ寄せ、
  coreが返した結果を永続化するだけに近づける
- 狙いは、複雑なロジックを外部依存なしでテストできる形に保つこと
  (ADR 0001「Functional Core, Imperative Shell」の具体化)
