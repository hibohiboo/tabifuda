---
name: feedback-no-code-without-ticket
description: ソースコード変更はチケット(タスク文書)発行が前提。議論・文書タスク中にその場でコードを触らない
metadata: 
  node_type: memory
  type: feedback
  originSessionId: e2194cb4-23c8-4347-9889-8ce10d4913f7
  modified: 2026-08-02T01:14:06.801Z
---

議論・文書整理・決定ログ反映のタスク中に、ソースコード(crates/・apps/・packages/・tools/)の修正が必要になっても、その場で修正してはならない。タスク(既存 task.md のサイクル、または docs/tasks/plans/ の計画文書)を先に発行し、別途着手する。docs/ のみの反映は対象外。

**Why:** 2026-08-02、進め方見直し(post-p3.5-replanning-decisions.md)のQ1反映で、frozen語彙追加に伴う docs-site のコード修正(ts/css)へタスク発行なしに着手し、ユーザーに「トークンがもったいない。議論は議論として終わらせたい」と指摘され中断した。「ビルドが壊れるから」という技術的必然があっても、スコープ外のコード修正を畳み込むと議論が中断しトークンも浪費される。[[feedback-scope-boundary-bugs]]と同じ構図。

**How to apply:** コード修正の必要に気づいたら、(1)壊れる旨・修正内容・マージ順の制約をタスク文書か決定ログに書き残す、(2)修正自体は別ブランチ・別タスクに退避する、(3)元のタスク(議論)を最後まで続ける。正は docs/agent-operations.md「ソースコード変更はタスク(チケット)発行が前提」。
