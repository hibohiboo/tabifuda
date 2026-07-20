# docs/rdra/ — RDRAモデルデータ(非規範の索引)

設計文書を RDRA(https://www.rdra.jp/)のレイヤー構造で可視化するための
手動キュレーションデータ。[tools/rdra-viewer](../../tools/rdra-viewer/) が
読み込み、GitHub Pages に表示する。
タスクの正: [../tasks/tools/rdra-viewer/task.md](../tasks/tools/rdra-viewer/task.md)。

## 位置づけ(重要)

- **非規範の索引**。規範は従来どおり design/ の文書
  ([../README.md](../README.md)「文書間の優先順位」)
- 規範文書と食い違ったら**このYAML側を直す**(正を二重化しない)
- `description` は1〜2行の要約に留め、規範の内容を複製しない

## ファイル構成

| ファイル | RDRAレイヤー | 状態 |
|---|---|---|
| actors.yaml | システム価値(アクター) | あり |
| usecases.yaml | システム境界(ユースケース=Command) | あり |
| requirements.yaml | システム価値(要求) | C2で追加 |
| business-flow.yaml | システム外部環境(業務フロー) | C2で追加 |
| information.yaml | システム(情報モデル) | C2で追加 |
| states.yaml | システム(状態モデル) | C2で追加 |

## 形式

各要素は `id`(kebab-case、ファイル横断で一意)/ `name` / `description` /
`source`(docs/ からの相対パス+GitHub見出しアンカー)を持つ。
関係は参照側の要素に id 配列で持つ(例: usecase の `actors:` `information:`
`states:`)。未整備ファイルの id を先行参照してよい(ビューアは未解決idを
プレーン表示する。C3でCI検証が入ったら解消必須)。

## 更新の規律

- 規範文書(特に domain-model.md)の該当節を変えたら、同PRでここも直す
- 見出し名(=アンカー)を変えたら `source` も直す(C3以降はCIが検出する)
