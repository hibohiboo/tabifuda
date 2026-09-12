# ハンドオフ: grillingスキル導入 → P7着手

## 目標

grillingスキルを導入・試用したのち、次フェーズ(P7 依頼/シナリオ選択画面)へ
コンテキストをリセットしてから進む。

## 完了済み

- grillingスキルを導入(mattpocock/skillsから移植)。ブランチ`grilling-skill`
  コミット `d27d3e8`
- grillingスキルを試用し、future-requirements.mdに「12. ロール制」を追記。
  同ブランチ コミット `b7f451a`
- roadmap.md確認の上、次フェーズはユーザーとの合意で **P7(依頼/シナリオ
  選択画面)** に決定(roadmap.mdの「今/次」は本来未選定だった)

## 残作業

- P7は`docs/tasks/projects/phase7/task.md`に「サイクル未設計」の入れ物
  として起票済みのみ。着手時にサイクルを設計し、frontmatterに`cycles:`を
  追加する必要がある(まだ未着手)

## 次の一手

1. `grilling-skill`ブランチの2コミットの扱いを人間に確認する(そのまま
   PR化するか、masterへマージしてからP7用ブランチを切るか)
2. phase-cycleスキル「0. 開始の儀式」からP7 task.mdを対象に着手
3. task.md「未検討事項(着手時に決める)」の3点
   (シナリオを複数持たせる方法/`StartSession`変更の要否/張り紙カードの
   掲載情報)を、grillingスキルで質問を尽くしてから詰める
4. 詰めた結果でサイクルを設計し、task.mdのfrontmatterを更新する

## ハマりどころ

- ローカルに`phase7-kickoff`という別ブランチが残っているが、これは
  P7起票コミット(旧作業)がmasterへマージ済みの残骸で、今回のP7実装作業
  とは無関係(`git log master..phase7-kickoff`が空)。削除候補だが今回は
  未着手のまま残した
- 今回の変更(grillingスキル+ロール制メモ)はP7そのものではなく別ブランチ
  上にある。P7着手時にこのブランチをベースにするかmasterに戻すかで
  ブランチ運用が変わる

## 関連ファイルパス

- .claude/skills/grilling/SKILL.md
- .claude/skills/phase-cycle/SKILL.md
- docs/agent-operations.md
- docs/prompt-sample.md
- docs/requirements/future-requirements.md(§12)
- docs/tasks/projects/phase7/task.md
- docs/roadmap.md
