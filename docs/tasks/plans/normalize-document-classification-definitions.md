# タスク: 「規範/非規範/索引/記録」定義の一本化

## 概要

[docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
で確定した方針に基づき、「規範」「非規範」「索引」「記録」という文書区分の定義を
[docs/README.md](../../README.md)「文書区分の定義」に一本化し、後発文書(ふりかえり等)
から参照する形にする。同時に、発生源(作成手順・エージェント定義)に標準文言を組み込む。

## 背景

同じ概念(「規範」「非規範」など)が複数文書で個別に再定義されている SSoT違反が
見つかった。正をdocs/README.mdに統一し、後発文書は参照だけ添える形にすることで、
新規文書作成時の自動統一を実現する。

## 作業項目

### 1. 既存の規範/非規範宣言を確認して参照に変更(docs側)

以下の文書で「位置づけ」や「規範」「非規範」の説明をしている箇所に
docs/README.md への参照を添える(独立した再定義は削除):

- [docs/retrospectives/phase2.md](../../retrospectives/phase2.md) L4-5
- [docs/retrospectives/phase3.md](../../retrospectives/phase3.md) L7
- [docs/retrospectives/phase3.5.md](../../retrospectives/phase3.5.md) L5-7
- [docs/tasks/tools/docs-site/task.md](../../tools/docs-site/task.md) L35-40
- その他検索で見つかる箇所

### 2. 発生源への標準文言組み込み

新たに作られるふりかえりは自動的に統一した形式になるよう:

- [docs/agent-operations.md](../../agent-operations.md)「フェーズ完了時のふりかえり」
  手順の中に、「位置づけ: 記録文書(非規範)。詳細は docs/README.md『文書区分の定義』参照」
  という標準テンプレート文言を記載
- retrospective エージェント定義(docs/agent-operations.md参照の retrospective スキルの説明)
  に同様の文言を組み込む(スキル文書は .claude/skills/*.md の可能性が高い)

## スコープ

- 変更は docs/ のみ
- 削除・言い換えは行わない。docs/README.md への参照を既存記述に追加する形
- エージェント定義への変更は、スキル定義ファイルが見当たったら実施；
  見当たらなければ docs/agent-operations.md 手順のみ対応

## 終わり方

1. 既存後発文書への参照追記
2. agent-operations.md 手順への標準文言追加
3. 誤解があれば agent-journal.md に1行記録
4. 作業ブランチで行い、マージは人間判断

## 関連

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  (SSoT方針・定義の一本化)
- [docs/README.md](../../README.md)「文書区分の定義」(正の所在)
