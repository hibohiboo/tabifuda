# タスク: 横断タスクの構造化(tasks/crosscutting/ 新設)+リンク規律の明文化

## 背景

[ssot-single-responsibility-rules.md](ssot-single-responsibility-rules.md)完了後、
`docs/tasks/plans/` 直下に「終わったか残っているか一目で分からない」状態が
生じた(実行系チケット4枚+マスター計画1枚+決定ログ2枚+記録2枚+リネーム後の
決定記録1枚が無区別に並ぶ)。

原因は、進捗追跡機構(YAML frontmatter + docs-site 進捗ビュー)を持つのが
`projects/phaseN/task.md`・`tools/*/task.md` に限られ、フェーズにもツールにも
属さない**横断の実行系タスク**の置き場が無いまま `plans/`(本来は決定ログ・記録・
plan mode出力の置き場)に紛れ込んでいたこと。

ユーザーとの議論(2026-08-31)で以下を決定:

1. 横断の実行系タスクは `tasks/crosscutting/` を新設し、既存の
   `projects/phaseN/`・`tools/<name>/` と同じ「1タスク=1ディレクトリ+
   task.md+frontmatter」に乗せる。追跡は既存機構1本に統一する
2. **現役文書(規範・索引・README等)から実行系チケット(工程文書)への
   直接リンクを禁じる。経緯を示したい場合はADR・決定ログ経由にする**
   (コード側の既存規約「コードコメントからdocs/tasks/を参照しない」の
   docs版。ADR 0007「帰結」へ追記する)
3. `tasks/plans/` は決定ログ(`*-decisions.md`)・記録・plan mode出力
   専用に戻す。実行系チケットは置かない

## 対象範囲の確定(2026-08-31の棚卸し)

`docs/tasks/plans/` 直下10ファイルを再分類した:

| ファイル | 区分 | 処置 |
|---|---|---|
| ssot-single-responsibility-rules.md | 実行系(完了) | crosscutting/へ移動 |
| rustdoc-references-to-domain-model.md | 実行系(完了) | crosscutting/へ移動 |
| normalize-document-classification-definitions.md | 実行系(完了) | crosscutting/へ移動 |
| reduce-redundant-declarations-frontmatter-rdra.md | 実行系(完了) | crosscutting/へ移動 |
| fix-lib-rs-docs-tasks-reference.md | 実行系(完了) | crosscutting/へ移動 |
| hand-card-removal.md | **決定/設計記録**(訂正: 当初は実行系と誤分類) | **plans/に残留**。domain-model.md「決定の経緯」表からの参照はADR/決定ログ相当の正規チャネルであり規律違反ではない |
| proposal-id-issuance-decisions.md | 決定ログ | plans/に残留 |
| post-p3.5-replanning-decisions.md | 決定ログ | plans/に残留 |
| docs-tasks-restructure.md | 記録 | plans/に残留(本タスク自身の前例) |
| post-p3.5-session-retrospective.md | 記録 | plans/に残留 |

**生きた参照の事前調査**(grep済み。移動時の張り替え対象はこれで全て):

- 4チケット単体は現役文書から参照ゼロ(plans/内部の追跡表からのみ参照)。
  張り替え不要
- ssot-single-responsibility-rules.md は3箇所から参照:
  - [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
    L17・L99(決定→実行タスクへの参照。ADRは経緯チャネルなので規律に適合。
    **パスのみ新場所へ更新**)
  - [docs/requirements/プロフェッショナルAI駆動開発.md](../../requirements/プロフェッショナルAI駆動開発.md)
    L23(チェックリストの一時的な進行中注記。完了した今、チケットパスへの
    詳細参照は不要。**要約に置き換え**)
- hand-card-removal.mdの3参照(domain-model.md・roadmap.md・tasks/README.md)は
  対象外(上記の通り残留のため無変更)

## 作業項目

### 1. ディレクトリ構造・規律の文書化

- [docs/tasks/README.md](../../README.md)に `crosscutting/` の節を追加
  (既存の `projects/`・`tools/` と並列。1タスク=1ディレクトリ+task.md+
  frontmatterである旨、plans/との違い)
- 同ファイルの「plans の振り分けルール」を「実行系タスクは
  crosscutting/へ。plans/は決定ログ・記録・plan mode出力専用」に改訂
- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  「帰結」に、現役文書→実行系チケットへの直接リンク禁止規律を追記
  (コード側規約との対応を明記)

### 2. 既存5ファイルの移動

各ファイルを `tasks/crosscutting/<slug>/task.md` へ `git mv`し、
frontmatter(`status: done`)を付与する。5ファイルは互いに参照し合う
親子関係(ssot-single-responsibility-rules.mdがマスター、他4つが子)
なので、slugディレクトリの持たせ方は以下のいずれかを実装時に決める:
- 案1: マスターを`crosscutting/ssot-single-responsibility/task.md`とし、
  4チケットは同ディレクトリ内`plans/`または直下ファイルとして従属させる
  (`projects/phaseN/plans/`と同じ構造)
- 案2: 5つを独立した`crosscutting/<slug>/task.md`として並列に置く

**推奨は案1**(1つの取り組みの記録であるため。既存のフェーズタスク構造と
一貫する)。

### 3. リンクの張り替え

- ADR 0007のL17・L99: 新パスへ更新
- プロフェッショナルAI駆動開発.md L23: 「棚卸しの是正は
  tasks/crosscutting/ssot-single-responsibility/(完了)で対応済み」程度に簡略化

## スコープ

- 変更は docs/ のみ(.claude/ の frontmatter仕様に既存踏襲。新規機構は作らない)
- docs-site 側の表示対応(進捗ビューにcrosscutting/を出す)は**別チケット**
  ([crosscutting-progress-view.md](../tools/docs-site/plans/crosscutting-progress-view.md))。
  frozen語彙追加時の教訓により、ルール改定とコード変更は同PRに畳み込まない

## 終わり方

1. 上記1〜3を実施
2. `git mv`後、リンク切れが無いか `grep -rn "tasks/plans/rustdoc-references\|tasks/plans/normalize-document\|tasks/plans/reduce-redundant\|tasks/plans/fix-lib-rs\|tasks/plans/ssot-single-responsibility" docs/` で確認
3. 誤解があれば agent-journal.md に1行記録
4. 作業ブランチで行い、マージは人間判断

## 関連

- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
- [docs-tasks-restructure.md](docs-tasks-restructure.md)(前回のtasks/構造再編の前例)
- [tasks/crosscutting/ssot-single-responsibility/task.md](../crosscutting/ssot-single-responsibility/task.md)(移動対象・きっかけ)

---

## 完了記録(2026-08-31)

### 1. ディレクトリ構造・規律の文書化

- [docs/tasks/README.md](../../README.md): `crosscutting/` の節を新設
  (projects/・tools/と並列、frontmatter機構の適用を明記)。
  「plans の振り分けルール」を「横断タスクの振り分けルール
  (crosscutting/ vs plans/)」に改訂し、「現役文書から実行系タスクへの
  直接リンク禁止(task.mdは対象外)」を追記
- [docs/adr/0007-ssot-single-responsibility.md](../../adr/0007-ssot-single-responsibility.md)
  「帰結」にリンク規律の決定を追記

### 2. 既存5ファイルの移動

**推奨案1(マスター+従属plans/)** を採用。
`tasks/crosscutting/ssot-single-responsibility/`を新設し:
- `ssot-single-responsibility-rules.md` → `task.md`(frontmatter
  `status: done`、cycles C1/C2/C3を`done`で追加。本文の見出し
  `### C1: ...`〜`### C3: ...`と一致させた)
- 4チケットは同ディレクトリの`plans/`へ移動

### 3. リンクの張り替え

- ADR 0007: L17・末尾2箇所を新パス`tasks/crosscutting/
  ssot-single-responsibility/task.md`へ更新
- プロフェッショナルAI駆動開発.md: 「フォルダ構成」チェックを完了に更新
  (棚卸し是正・フォルダ構成レビューが実際に全完了済みだったため)

### 副産物: 移動作業中に発見したリンク切れバグ

`git mv`後の相対パス機械的シフトで検証したところ、移動対象ファイル内の
`docs/tasks/tools/docs-site/...`への参照が、**移動前から**`tasks/`
セグメント抜けで壊れていたことが判明(実在ファイル確認で発覚。ラベルは
「docs/tasks/tools/docs-site/...」なのに相対パスの計算が1階層足りな
かった)。移動と同時に修正した(同一ファイルの同じリンク群の修正であり
別チケット化するまでもない軽微な誤り)。

### スコープ判断: 対象外

- `hand-card-removal.md`は当初「実行系」と誤分類していたが、内容は
  カード消費・除去の設計決定記録であり、domain-model.md「決定の経緯」
  表からの参照は規律が想定する「経緯はADR・決定ログ経由」の正規形。
  **移動せずplans/に残留**
- docs-site進捗ビューへの表示対応は別チケット
  ([crosscutting-progress-view.md](../tools/docs-site/plans/crosscutting-progress-view.md))
  へ分離(未着手)
