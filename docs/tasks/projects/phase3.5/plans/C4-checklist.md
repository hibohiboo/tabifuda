# P3.5 C4 チェックリスト: 持ち帰りのCLI配線

経緯メモ(仕様の正はdocs/design/domain-model.md)。フェーズ最終サイクル。

着手時のスコープ確認: task.mdのC4には「人間の事前決定」の指定なし。
- セッション終了時、portableカードの持ち出し→パーティファイルへの書き戻し
- demo.md更新(中断・再開・パーティ指定・持ち帰りの操作)
- 複数セッションファイルによる並行プレイが自然に成立することの確認

## 実装

- [x] `party.rs`: `write(party, path)`を追加(読み込みと対称、ラッパー無し)
- [x] `play.rs`: `run`/`play_loop`に`party_path: Option<&Path>`を配線。
      `--party`未指定・`--resume`経由は書き戻し先が無いため`None`
- [x] `play.rs`: Ended分岐で`maybe_write_back_party`を呼ぶ。
      `RewardsGranted`が1回も無ければ尋ねない(fork保存と同じ「該当時のみ
      尋ねる」方針)
- [x] `main.rs`: `run_play`が`party_path`を`play::run`へ渡すよう更新

## テスト

- [x] 結合テスト: 専用の最小シナリオ(#portableタグ付きカード1枚)で
      持ち出し→書き戻しの通しフローを固定(shared/scenarios/は汚さない)
- [x] 同テスト内で、書き戻し後のパーティファイルを次のセッションの
      `--party`としてそのまま読み込めることを確認(完了条件「パーティが
      セッションを跨いで持続」の直接検証)
- [x] 結合テスト: 同一シナリオから複数セッションを開始しても保存ファイル
      名(-save/-save-2/...)が衝突せず、それぞれ独立して再開できる
      (task.md C4「並行プレイが自然に成立することの確認」)

## 終わり方

- [x] cargo test --workspace / clippy / fmt
- [x] design-syncで乖離チェック(domain-model.md「パーティファイル」節・
      future-requirements.md §1/§3の「別サイクルで扱う」という未来形の
      記述が実装済み後も残っていたのを発見し修正)
- [x] demo.mdへ「6. カードを持ち帰る(portable)」節を追加
- [x] agent-journal.mdへの追記: フェーズ完了ふりかえり時に発覚した
      chronicle.rs/oplog.rsのRewardsGranted/CardsDiscarded未対応
      (C3申し送りがC4で未消化だった件)を記録。同PRでchronicle.rs/
      oplog.rsを修正し、design-syncスキルへワイルドカード確認の観点を追加
- [x] フェーズ完了のふりかえり作成(docs/agent-operations.md「フェーズ
      完了時のふりかえり」手順。docs/retrospectives/phase3.5.md)
- [x] task.md全体ステータスをdoneへ更新
- [x] apps/web(P3側)への影響確認: このブランチのapps/web/srcはP3 C2
      水準のスケルトンのみで、Event描画コンポーネント(Timeline等)は
      未マージのため今は影響なし。P3.5をmasterへ統合する際、P3側の
      Timeline実装がRewardsGranted/CardsDiscardedに対応しているか
      要確認(マージ作業時のチェック項目として報告)
