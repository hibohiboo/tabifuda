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
| screens.yaml | システム境界(画面。`status: implemented \| future`で未作成を区別) | あり(D4) |
| information.yaml | システム(情報モデル) | あり(C2) |
| states.yaml | システム(状態モデル。セッション状態機械の遷移も持つ) | あり(C2) |

## 形式

各要素は `id`(kebab-case、ファイル横断で一意)/ `name` / `description` /
`source`(docs/ からの相対パス+GitHub見出しアンカー)を持つ。
関係は参照側の要素に id 配列で持つ(例: usecase の `actors:` `information:`
`states:`、screen の `actors:` `usecases:`)。ビューア(tools/docs-site)の
`relatedIds`はusecase・requirement・業務フローステップ・screenを「関係を運ぶ
ノード」として扱い、どの層の要素をクリックしても1ホップ関係を辿って
ハイライトする(model.ts参照)。
`screens.yaml` の `status: implemented | future` は「まだ無い画面」を
一覧上で一目で分かるようにするためのもの(2026-08-02の進め方見直しQ6。
経緯: [../tasks/plans/post-p3.5-replanning-decisions.md](../tasks/plans/post-p3.5-replanning-decisions.md))。
存在しないidを参照するとビルドが落ちる(C3で `scripts/check-rdra-data.mjs`
によるCI検証を追加済み。id一意性・`source` のリンク先ファイル/アンカー存在も
同様に検証する)。

## 更新の規律

- 規範文書(特に domain-model.md)の該当節を変えたら、同PRでここも直す
- 見出し名(=アンカー)を変えたら `source` も直す(CIが検出する)
- **要素のフィールド(`actors:` `usecases:` 等)は、参照する他レイヤーの
  該当エントリ(usecases.yaml の該当id等)や実装コードを実際に開いて
  突き合わせてから値を確定する。**「だいたいこうだろう」で書かない
  (2026-08-02〜03、screens.yaml D4で actors の取り違えが2件発生した
  教訓。経緯: [../tasks/plans/post-p3.5-session-retrospective.md](../tasks/plans/post-p3.5-session-retrospective.md))
- **新しいレイヤー・要素種別(例: screens.yaml)を追加するときは、
  その手前のレイヤー(要求 requirements.yaml → 業務フロー
  business-flow.yaml)が対象アクター分すでに整備されているか先に確認する。**
  整備されていなければ、先にそちらを作ってから下流のレイヤーを作る
  (同上の教訓。screens.yamlをgm/authorの要求・業務フローが無いまま
  直接作った結果、レビューが「中身の妥当性」止まりになり画面自体の
  欠落を検知できなかった。経緯は同上、詳細は
  [../tasks/tools/docs-site/task.md](../tasks/tools/docs-site/task.md) D6)
- **未確定な要素を荒い段階(名前の列挙)で確認するときは、複合名・抽象度の
  高い1語をそのまま出さず、束ねている下位概念まで分解して提示する。**
  「・」区切りの名前(例: 「シナリオ執筆の流れ」が実際には執筆と他作者との
  関わりという別動機の2概念を束ねていた)や、1語で複数の文脈を指しうる
  名前(例: 「シェアワールド」がロケーション共有とルールブックという別物を
  束ねていた)は、列挙段階では見た目1件でも確認の過程で分解が必要になる
  ことがある。列挙時点で「この名前は単一概念か」を自問し、怪しければ
  最初から下位項目まで書き出す(2026-08-11、D6のbusiness-flow.yaml・
  screens.yaml列挙で3回発生。経緯: docs/agent-journal.md 2026-08-11、
  [../tasks/tools/docs-site/task.md](../tasks/tools/docs-site/task.md) D6)
