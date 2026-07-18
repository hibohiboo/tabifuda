---
name: core-invariants
description: crates のコア(decide/apply)のテストを書く・レビューするときの観点チェックリスト。「コアのテストを書く」「不変条件」「proptest」「拒否系の網羅」等、コアのテスト作成・レビュー時に使う。正は docs/design/test-strategy.md(このスキルは観点の索引と手順のみ)。
---

# Core Invariants

コアのテストを書く・レビューするときのチェックリスト。
**詳細と変更は docs/design/test-strategy.md が正。** 文書を変えたら
このスキルも同PRで更新する。

## 手順

1. docs/design/test-strategy.md の「crates/core」節を読む(このスキルだけで書かない)
2. 下のチェックリストで対象コマンド・不変条件の抜けを確認する
3. 重複防止ルール(同文書)に照らし、置き場所が正しいか確認する

## チェックリスト

### テーブル駆動 decide テスト(CLAUDE.md規約)

- [ ] 各Commandに「受理 → 期待イベント列」と「拒否 → 期待RuleError」が**対で**あるか
- [ ] 拒否系の網羅対象(test-strategy.md §1a)を落としていないか。
      代表: Paused中のPlayCard/Propose、条件未達カード、存在しない参照先、Ended後の全Command
- [ ] 新しいCommand/Eventを足したとき、対になるテストを同時に足したか

### プロパティテスト(proptest)— 不変条件1〜5の索引

| # | 名前 | 一言 |
|---|---|---|
| 1 | 決定性 | 同じイベント列のapplyは常に同じ状態(リプレイ可能性) |
| 2 | 整合性 | decideが返したイベント列は必ずエラーなくapplyできる |
| 3 | 状態機械の合法性 | Running/Paused/Ended の遷移図に無い遷移は起きない |
| 4 | 保存則 | カードは配布イベント無しに手札に現れない/消えない |
| 5 | パッチ安全性 | validateを通ったパッチは適用後もセッションを壊さない |

導入順はフェーズ対応(P1: 1〜4、P2で5)。定義の全文は test-strategy.md を参照。

- [ ] ランダム生成の対象はコマンド列とパッチ。シナリオは小さな固定物+生成物の両方
- [ ] ネストしたコレクション(Vec/HashMap)を持つ型への `Arbitrary` deriveは、
      各コレクションフィールドにサイズ上限strategy(`0..=3` 程度)を最初から付けたか
      (無制限は実行時間・メモリが爆発する。詳細は test-strategy.md §1b)
- [ ] 境界値はテーブル駆動、一般則はプロパティ、と役割分担できているか
      (同じ条件を両方で細かく書かない)

### シリアライズ

- [ ] 全公開型に roundtrip(serialize→deserialize→同値)テストがあるか(プロパティで一括)
- [ ] enumはタグ付き表現+`#[non_exhaustive]` か(CLAUDE.md規約)

### コアの純粋性

- [ ] テストに時刻・乱数・IOを持ち込んでいないか(乱数は結果を外から与える)
- [ ] 公開APIにpanicが無いか(エラーは RuleError / PatchError)
