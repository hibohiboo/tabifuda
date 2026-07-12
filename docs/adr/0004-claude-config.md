# ADR 0004: Claude Codeハーネス設定(.claude/)の構成

状態: 採用 / 日付: 2026-07-13

## 文脈

`.claude/settings.json` は `plansDirectory` のみが暗黙に置かれ、設定の根拠を
記録した文書がなかった(CLAUDE.md最重要ルール1の趣旨に反する状態)。
また、エージェントの自動メモリ(auto-memory)は既定で
`~/.claude/projects/<プロジェクト別ディレクトリ>/memory/` に保存され、
リポジトリ外にあるためgit管理できない。本プロジェクトは
「ハーネス改良もコードと同じくレビュー対象」(agent-operations.md)を掲げており、
プラン・メモリ・設定を可能な限りgit管理下に置きたい。

## 決定

### ファイル構成(3層)

| ファイル | git | 役割 |
|---|---|---|
| `.claude/settings.json` | 管理する | プロジェクト共有設定。変更は本ADR更新とセット |
| `.claude/settings.local.json` | 除外 | マシン固有設定(絶対パスを含むもの)。`settings.local.json.example` からコピーして作る |
| `.claude/settings.local.json.example` | 管理する | ローカル設定の雛形。新しいマシンでのセットアップ手順を兼ねる |

### プラン置き場: `plansDirectory: "docs/tasks/plans"`

plan modeの成果物は設計の中間生成物であり、docs/ 配下でレビュー・追跡する。
既定の `~/.claude/plans/` はリポジトリ外でgit管理できないため変更する。

### メモリ置き場: `.claude/memory/`(autoMemoryDirectory)

自動メモリの実体をリポジトリ内 `.claude/memory/` に置き、git管理する。
メモリの中身(エージェントが学習した事実・注意点)もPRでレビュー可能になり、
失敗ジャーナル(agent-journal.md)と併せてハーネス改良の材料にする。

**制約**: `autoMemoryDirectory` はセキュリティ上の理由により、git管理される
`.claude/settings.json` に書いても無視される(リポジトリ側からメモリの
読み書き先を注入されるのを防ぐ仕様)。そのため各マシンの
`.claude/settings.local.json` に**絶対パス**で書く。これが settings.local.json
と example ファイルが存在する主因。

### 権限(permissions)

- allow: 品質ゲートに必要な cargo サブコマンド(build / check / clippy /
  fmt / test)と、読み取り専用の git コマンド(status / diff / log / show)。
  コミット前必須コマンド(CLAUDE.md)を確認なしで回せるようにする
- 状態を変える操作(`git add` / `commit` / `push`、`cargo publish` 等)は
  allowに**入れない**。既定の確認プロンプトに任せる
- deny: `.env` 系の読み取り。シークレットをエージェントのコンテキストに
  入れない(cross-cutting.md のシークレット方針と整合)
- `defaultMode` は変更しない(既定の確認動作を維持)

### その他の明示設定

- `autoMemoryEnabled: true` — メモリ運用を明示的にオンと宣言
- `language: "japanese"` — 本プロジェクトの文書・応答言語
- `$schema` — エディタ補完・検証用(schemastore)

### 採用しなかったもの

- **PostToolUseフックでの cargo fmt 自動実行**: コミット前ゲート
  (CLAUDE.md「作業の終わり方」)で十分。編集毎のフックはレイテンシを増やし、
  失敗が暗黙化する。ジャーナルでfmt忘れが頻発したら再検討
- **`defaultMode: "acceptEdits"` 等の緩和**: 設計文書先行の運用では
  人間の確認ポイントを残す価値が大きい

## 帰結

- `.claude/settings.json` を変更する場合は、先に本ADRを更新してから行う
- 新しいマシンでは `settings.local.json.example` を `settings.local.json` に
  コピーし、`autoMemoryDirectory` のパスを自分の環境に合わせて書き換える
- メモリの変更はコミット差分に現れる。週次棚卸し(agent-operations.md)で
  ジャーナルと一緒に確認する
- 許可コマンドの追加(頻出プロンプトの削減)は許可リストへの追記で対応し、
  状態変更系を追加する場合のみ本ADRの表明を更新する
