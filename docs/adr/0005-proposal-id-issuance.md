# ADR 0005: ProposalId は Session 内の連番カウンタで発番する

状態: 採用 / 日付: 2026-07-19

## 文脈

P1 C3 で `Session.proposal_seq: u64` による連番発番を決定した
(domain-model.md「C3: decide/applyの解決規則」)。その際の記録は
「UUID(v4等)はcoreの純粋性(乱数禁止)に抵触するため不採用」としていたが、
phase2 完了後の再検討で、この根拠が不正確であることが判明した:
**外部で発行したUUIDをCommandに載せて渡すだけなら、core内で乱数を引かず
純粋性には抵触しない**。IDはイベントに焼き込まれるためリプレイ決定性も
保たれる。そこで発番方式を改めて比較した(議論の全経緯は
docs/tasks/plans/proposal-id-issuance-decisions.md)。

検討した選択肢:

- (A) 現行維持: `Session.proposal_seq` を単調カウンタとして持ち、
  decide が `proposal-{seq}` を発行、apply(`ProposalSubmitted`)が +1
- (B) 外部発行: `Command::Propose` にID(UUID v4/v7等)を添付し、
  CLI/API層が発行する。カウンタは削除
- (C) core が発番関数(`next_proposal_id(&Session)` 等)を公開し、
  呼び出し側が取得して Command に添付。カウンタは維持

## 決定

**(A) 現行の連番方式を維持する。** あわせて不採用理由を訂正する:
UUID(外部発行)を採らない理由は純粋性ではなく、次の2点である。

1. **一意性の責務を core に閉じるため。** `pending_proposal` は裁定後に
   `None` へ戻り、過去に発行した ProposalId は現在状態に残らない。よって
   外部発行(B)では core が重複IDを検出できない(検出には発行済みID全集合の
   保持が必要で、u64カウンタ1個より重い)。UUID v4/v7 の偶発的衝突確率は
   無視してよいが、問題は確率ではなく**検証可能性**にある: core は受け取った
   IDが「新規に正しく生成された」か「過去IDの再利用・固定文字列」かを区別
   できず、一意性が「core が維持する不変条件」から「全呼び出し側の行儀に
   ついての仮定」へ変わる。想定すべき失敗モードはリトライ実装のバグによる
   再送や、P4(API公開)での悪意ある入力である。これは Actor.role の
   自己申告を廃止した H2 決定(p1-c1-review-decisions.md #4)と同じ
   「検証より構造で守る」方針の適用である
2. **CardInstanceId と発番戦略を統一するため。** `DealCard` の
   `Target::Party` 解決や `GotoScene` 先シーンの deals 配布では、必要な
   ID の個数と対応順が decide の解決規則の関数になる(実例: 「単純討伐」の
   `reply` カード1枚の PlayCard で、effect 由来+シーン入場由来の
   CardInstanceId が2個発行される)。外部発行はこの解決ロジックの複製を
   全フロントエンド(CLI/Web/API)に強い、バージョン齟齬がそのまま
   ID誤対応になる。ProposalId だけ外部発行にすると発番戦略が二本立てになる

(C)を採らない理由: カウンタは結局 Session に残り、core が添付IDを照合
検証するなら添付の意味がなく、検証しないなら一意性が壊れる。(A)の劣化版。

なお、クライアント発行IDの本来の利点(リトライの冪等化)は、core が重複を
検出できない以上 core 層では実現できない。冪等化は P4 の API 層の冪等キー
として扱う(本ADRの範囲外)。

## 帰結

- domain-model.md C3 の「UUIDは純粋性に抵触するため不採用」という記述を
  本ADR参照へ差し替える(不正確な根拠を正へ残さない)
- 実装変更なし(現行実装が本決定と一致するため)
- P4 で API 層を設計する際、リトライ冪等化は API 層の冪等キーで行い
  core には持ち込まない、という本ADRの前提を引き継ぐ
- 将来、ID発番を要する新エンティティを追加する場合も同方式
  (Session 内の単調カウンタ+decide 発番+apply インクリメント)に揃える
