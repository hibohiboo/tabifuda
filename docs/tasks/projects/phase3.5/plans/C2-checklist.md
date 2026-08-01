# P3.5 C2 チェックリスト: パーティのload/save

経緯メモ(仕様の正はdocs/design/domain-model.md)。

着手時のスコープ確認: task.mdのC2には「人間の事前決定」の指定なし
(C1のformat_version方針・BTreeMap化のような明示ゲートが無い)。
書き戻し(finalize)はC4が担うため、C2は読み込みとファイル検証まで。

## 設計判断(CLIの決定として文書化する)

- パーティファイル形式: `Vec<Character>`をそのままJSON化(SaveFileのような
  format_versionラッパーは持たない。task.mdの明記通り)
- ファイル検証: 空配列を拒否、`CharacterId`の重複を拒否
  (domain-model.md「コレクションとidの規則」がparty内CharacterIdの一意性を
  不変条件と明記しているが、これまでCLIが常に単一キャラのpartyを構築して
  いたため未検証のまま到達不能だった。外部ファイル入力を導入するC2で
  初めて到達可能になるため、CLI層で検証する)
- ソロプレイの操作対象キャラ: パーティ先頭(`party[0]`)をCLIが操作する
  (CLIには複数キャラ切り替えUIが無いため。無指定時の既定パーティ
  (「旅人」1人)でも同じ規則が成立する)

## 実装

- [ ] tabifuda-cli: `party.rs`モジュール新設(読み込み+検証)
- [ ] play.rs: `run`が`party: Option<Vec<Character>>`を受け取れるように変更。
      Noneなら従来の既定ソロパーティ、Someならそれを使い`party[0]`を
      操作キャラとする
- [ ] main.rs: `play <file> --party <party-file>`の引数解釈を追加

## テスト

- [ ] 結合テスト: `--party`で読み込んだパーティのキャラ名が画面に表示される
- [ ] 結合テスト: 空配列のパーティファイルは拒否される
- [ ] 結合テスト: CharacterId重複のパーティファイルは拒否される
- [ ] party.rsの単体テスト(読み込み成功・各拒否系)

## 終わり方

- [ ] cargo test --workspace / clippy / fmt
- [ ] design-syncで乖離チェック
- [ ] demo.mdへの影響確認(--partyの使い方を追記するか検討)
- [ ] agent-journal.mdへの追記(誤解があれば)
