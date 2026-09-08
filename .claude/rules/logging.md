---
paths:
  - "src/**"
---

# 構造化・要約・秘密値禁止
## ルール

1. 業務コードで console.log / console.error を直接使わない。ログはsrc/lib/logging/の共通関数を経由する（直書きするとrequestIdの付与とマスク処理を迂回するため）
2. ラップするのは外部ID・副作用・失敗しうる処理だけ（DB・外部API・通知送信）。純粋関数はラップしない（ノイズになるだけ）
3. エラーは記録した後必ず再スローする。握りつぶして正常系を返さない。
4. 引数は要約だけを記録する（IDなど）。オブジェクトを丸ごと渡さない
5. infoを出すのは外部APIの成功・１秒を超える処理・重要イベントだけ。それ以外の成功は黙る（ログのノイズを増やさない）

## 標準キー

キー|内容
--|--
name| domain.function 形式 (例: invoice.createDraft)
userId|関連する主体のID
requestId|リクエスト単位の相関ID
err|  {name, message ,stack } に整形
ms| 所要時間

## 秘密値をログに流さない
認証トークン・パスワード再設定リンク・APIキー・カード/口座情報・生の個人情報はいかなるログにも書かない

// ✖ リンクは実質パスワード。ログに残してはいけない
console.log('rest link generated', email, link)

// ✅ 共通関数経由で要約のみ。リンク自体は記録しない
log.info('auth.sendResetLink', {emailDomain: email.split('@')[1]});

