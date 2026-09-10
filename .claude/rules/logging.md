---
paths:
  - "apps/api/src/**"
---

# 運用ログの規律(apps/api、P4〜)

正は docs/design/cross-cutting.md「2種類のログを区別する」。ここはP4
(Hono+Drizzle+Neon。現在**凍結**、docs/roadmap.md「P4・P5の凍結」参照)
着手時にのみ効く実装規約であり、cross-cutting.mdの方針をapps/apiの実装
レベルに落とし込む。

## ドメインログと運用ログを混同しない

- **ドメインログ(冒険記)= Event列そのもの**。イベントストア(Neon)に
  そのまま追記する。ここはtabifuda-coreのdecide/applyが担い、apps/api側の
  ログ機構では扱わない
- **運用ログ = 障害調査・監視用の構造化ログ**。本ファイルの対象はこちらのみ

## 運用ログのルール

1. 業務コードで console.log / console.error を直接使わない。ログは
   src/lib/logging/ の共通関数を経由する(直書きするとrequestIdの付与を
   迂回するため)
2. ラップするのは外部I/O・副作用・失敗しうる処理だけ(DB・decide呼び出し・
   通知送信)。純粋関数はラップしない
3. エラーは記録した後必ず再スローする。握りつぶして正常系を返さない
4. **自由入力テキスト・カード本文をログに書かない**(cross-cutting.md
   「禁止事項」。台詞カードのfree_text、提案text等の個人情報混入経路を断つ)。
   引数は種別+IDの要約だけを記録する
5. infoを出すのは外部API・decide呼び出しの失敗、1秒を超える処理、
   重要イベントだけ。それ以外の成功は黙る(ログのノイズを増やさない)

## 標準キー

キー|内容
--|--
name | domain.function 形式(例: session.playCard)
sessionId / userId | 関連する主体のID
requestId | リクエスト単位の相関ID
err | {name, message, stack} に整形
ms | 所要時間

## 秘密値をログに流さない

認証トークン・APIキー・生の個人情報はいかなるログにも書かない。
