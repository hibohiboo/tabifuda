---
status: done
---

# Phase 0 初期化タスク(エージェント指示文)

実行モデル: Sonnet 5 / Claude Code。plan modeで計画を提示し、承認後に実行すること。

## 目的

リポジトリ骨格・CI・エージェントハーネスを整備し、Phase 1(コアのドメイン
モデル実装)を開始できる状態にする。**ドメインロジックは一切実装しない。**

## 前提ファイル(リポジトリルートに同梱済み)

CLAUDE.md, domain-model-v0.2.md, future-requirements.md, agent-operations.md,
test-strategy.md, cross-cutting.md, adr-0001-methodology.md, agent-journal.md

## タスク

1. **docs/ 配置**: 上記文書を CLAUDE.md の「リポジトリ構成」に従い移動する
   - design/ ← domain-model-v0.2.md(domain-model.md にリネーム。版は
     git履歴で管理に切替)、test-strategy.md、cross-cutting.md
   - requirements/ ← future-requirements.md
   - adr/ ← adr-0001-methodology.md(0001-methodology.md にリネーム)
   - docs/ 直下 ← agent-operations.md、agent-journal.md
2. **CLAUDE.md 小修正**: 「必読文書」に2行追加
   - 横断方針(権限・ログ・UGC・削除)に触れる → docs/design/cross-cutting.md
   - 手法・構造の是非を判断する → docs/adr/0001-methodology.md
   また、文書内のファイル名参照を移動後のパスに合わせて更新する
3. **cargo workspace**: crates/core と crates/engine-cli を作成
   - core: lib。依存は serde, serde_json, thiserror のみ。
     置くのは lib.rs に「このクレートの原則」docコメント(CLAUDE.md
     最重要ルール2,3の要約)と、ダミーの型1つ+テスト1本(CI疎通用)
   - engine-cli: bin。main.rs は挨拶出力のみ
   - apps/ packages/ は作らない(P3/P4で作る。空ディレクトリを残さない)
4. **CI**(GitHub Actions想定): push/PRで
   - cargo fmt --check / cargo clippy --workspace -- -D warnings /
     cargo test --workspace
   - gitleaks(シークレットスキャン)
   - cargo audit(当面 warning 扱い=失敗させない。cross-cutting.md参照)
5. **その他**: .gitignore(Rust+node+.env)、rust-toolchain.toml(stable固定)、
   README.md(プロジェクト一行説明と docs/ への案内のみ。仕様を書かない)

## 受け入れ条件

- ローカルで fmt/clippy/test がすべて通る
- CLAUDE.md の「必読文書」参照先が全て実在するパスになっている
- ドメインモデル由来の型(Card, Session等)が一切存在しない
- コミットは意味単位で分割されている(docs配置 / workspace / CI / 雑務)

## 終わり方

CLAUDE.md「作業の終わり方」に従うこと。特に、この指示で誤解した点が
あれば docs/agent-journal.md への追記を忘れない(修正済みでも記録する)。
