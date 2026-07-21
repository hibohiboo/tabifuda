# docs/rdra/ — RDRAモデルデータ(非規範の索引)

設計文書を RDRA(https://www.rdra.jp/)のレイヤー構造で可視化するための
手動キュレーションデータ。[tools/docs-site](../../tools/docs-site/) が
読み込み、GitHub Pages に表示する。
タスクの正: [../tasks/tools/docs-site/task.md](../tasks/tools/docs-site/task.md)。

## 位置づけ(重要)

- **非規範の索引**。規範は従来どおり design/ の文書
  ([../README.md](../README.md)「文書間の優先順位」)
- 規範文書と食い違ったら**このYAML側を直す**(正を二重化しない)
- `description` は1〜2行の要約に留め、規範の内容を複製しない

## ファイル構成

| ファイル | RDRAレイヤー | 状態 |
|---|---|---|
| actors.yaml | システム価値(アクター) | あり |
| requirements.yaml | システム価値(要求。`status: realized \| future`で実装済み/将来要望を区別) | あり(C2) |
| business-flow.yaml | システム外部環境(業務フロー。ステップは`branch: true`で任意分岐を表現) | あり(C2) |
| usecases.yaml | システム境界(ユースケース=Command) | あり |
| information.yaml | システム(情報モデル) | あり(C2) |
| states.yaml | システム(状態モデル。セッション状態機械の遷移も持つ) | あり(C2) |

## 形式

各要素は `id`(kebab-case、ファイル横断で一意)/ `name` / `description` /
`source`(docs/ からの相対パス+GitHub見出しアンカー)を持つ。
関係は参照側の要素に id 配列で持つ(例: usecase の `actors:` `information:`
`states:`)。ビューア(tools/docs-site)の`relatedIds`はusecase・requirement・
業務フローステップを「関係を運ぶノード」として扱い、どの層の要素をクリックしても
1ホップ関係を辿ってハイライトする(model.ts参照)。
存在しないidを参照してもビルドは通る(未解決idはプレーン表示。C3でCI検証を追加する)。

## 更新の規律

- 規範文書(特に domain-model.md)の該当節を変えたら、同PRでここも直す
- 見出し名(=アンカー)を変えたら `source` も直す(C3以降はCIが検出する)
