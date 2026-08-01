# P3.5 C1 チェックリスト: セッションの保存と再開

経緯メモ(仕様の正はdocs/design/domain-model.md)。

着手前の人間決定(2026-08-01):
- Session内HashMap(roles/hands)・Character.stats → **BTreeMap化する**
- セーブファイル互換性方針 → **format_versionを埋め込み、不一致は拒否する**

## BTreeMap化(wasm-boundary-decisions.md 論点1の決定反映)

- [ ] ids.rs: id_type!マクロにOrd/PartialOrdを追加
- [ ] session.rs: `roles`/`hands` を HashMap→BTreeMap(proptest戦略もbtree_mapへ)
- [ ] character.rs: `stats` を HashMap→BTreeMap(proptest戦略も)
- [ ] event.rs: `SessionStarted.roles` を HashMap→BTreeMap(proptest戦略も)
- [ ] engine.rs: `HashMap::new()` → `BTreeMap::new()`(decide_start_session)
- [ ] テストファイル(patch_tests.rs / engine_tests.rs / invariant_tests.rs)の
      HashMap::new() → BTreeMap::new()
- [ ] `cargo build --features ts`でts-rs側の型生成が壊れないか確認
- [ ] domain-model.md「コレクションとidの規則」表・「セッション状態」struct・
      Event::SessionStarted定義をBTreeMap表記へ更新

## 保存ファイル形式の設計・文書化

- [ ] domain-model.md「フォーク出力」節の後に「セッションの保存と再開
      (CLIの決定)」節を追加。SaveFile{format_version, events}形式・
      versionの拒否方針・自己完結の理由(SessionStartedがscenario/party凍結
      コピーを含む)を記述
- [ ] future-requirements.md §1(中断・再開部分)の該当記述を上記へ移し、
      メモから削除(着手時の文書運用)

## 実装

- [ ] tabifuda-cli: `save.rs`モジュール新設(SaveFile読み書き・
      format_version検証・保存先パス発番)
  - [ ] Cargo.tomlにserde(derive)を直接依存追加
- [ ] play.rs: `run`のループ本体を`play_loop`へ抽出、`resume`エントリを追加
- [ ] play.rs: Running/Paused両方の入力に`q`(保存して中断)を追加
      (Paused中の中断→再開で裁定待ちに戻る経路が要件)
- [ ] main.rs: `play --resume <file>`の引数解釈を追加

## テスト

- [ ] 結合テスト: 保存→再開→勝利エンドまで
- [ ] 結合テスト: Paused中に中断→再開すると裁定待ちに戻り、そこから
      勝利エンドまで到達できる
- [ ] 結合テスト: format_version不一致のセーブファイルはエラーで拒否される
- [ ] 既存replay_tests.rsのfixtureがBTreeMap化後も壊れないか確認
      (壊れる場合はfixtureを同PRで更新)

## 終わり方

- [ ] cargo test --workspace / clippy / fmt
- [ ] design-syncで乖離チェック
- [ ] demo.mdへの影響確認(中断・再開の操作追加)
- [ ] agent-journal.mdへの追記(誤解があれば)
