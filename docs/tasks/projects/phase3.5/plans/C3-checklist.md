# P3.5 C3 チェックリスト: 報酬と持ち帰り(コア)

経緯メモ(仕様の正はdocs/design/domain-model.md)。

着手前の人間決定(2026-08-02):
- Event追加(`RewardsGranted`/`CardsDiscarded`) → **承認**
- portableの表現 → **予約済み`tags`の`#portable`方式**(`CardDef.portable: bool`は追加しない)

## スコープの絞り込み(着手時の判断)

future-requirements.md旧§3は「手札からの持ち出し(portable)」と
「シナリオ作者が明示定義する報酬(`rewards: Vec<Reward>`、`GrantPoints`含む)」
の2経路を併記していたが、task.mdのC3節本文が明示するのは前者のみ
(「持ち出しは既定不可・明示のみ可、Markerは常に不可、持ち帰りはCardDefの
凍結コピー」「lint拡張」)。`GrantPoints`はポイントを消費するキャラメイク
機構が現行モデルに存在せず置き場が無いため、後者は引き続き
future-requirements.mdに残し、C3では実装しない(先回り実装の回避)。

## 実装

- [x] `CardDef::PORTABLE_TAG`定数・`is_portable()`(Markerは常に不可)
- [x] `Character.owned_cards: Vec<CardDef>`追加(`#[serde(default)]`で
      既存ファイル・fixtureとの後方互換を維持)
- [x] `Event::RewardsGranted`/`CardsDiscarded`追加
- [x] `decide_end_session`・`Effect::EndSession`分岐の両方からfinalizeを
      発行(`hands_after`でEffect解決中の手札変化を畳み込んでから判定)
- [x] `apply`: `RewardsGranted`→`Character.owned_cards`へ追記、
      `CardsDiscarded`→監査記録のみ(hands不変。不変条件4のスコープを
      変えないため)
- [x] lint拡張: `PortableCardIsScenarioDependent`(GotoScene/AdvancePhase/
      DealCard効果、HasCard条件を検出)

## テスト

- [x] engine_tests: portableカードの持ち帰り(正常系)
- [x] engine_tests: 非portableカードの破棄(拒否系)
- [x] engine_tests: Markerは`#portable`タグがあっても持ち出せない(拒否系)
- [x] engine_tests: 既存のEndSessionテストをfinalize込みの期待値へ更新
- [x] lint_tests: シナリオ依存効果ごと(GotoScene/AdvancePhase/DealCard/
      HasCard)の検出+受理系2件
- [x] golden_tests: RewardsGranted/CardsDiscardedのワイヤ形式固定
- [x] replay fixtureの実測更新(owned_cards追加分のみ)

## 終わり方

- [x] cargo test --workspace / clippy / fmt / `--features ts`ビルド
- [ ] design-syncで乖離チェック
- [ ] demo.mdへの影響確認(portable/finalizeはCLI未配線のためC4まで
      表面化しない想定。確認のみ)
- [ ] agent-journal.mdへの追記(誤解があれば)
