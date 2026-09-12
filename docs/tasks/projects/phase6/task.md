---
status: done
cycles:
  C1: done
  C2: done
  C3: done
---

# Phase 6 実装タスク: カードUI改善

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
**開始前の儀式(全フェーズ共通)**: CLAUDE.md と docs/design/ の関連文書を読む。

## 経緯

P3.5完了後の見直し
([tasks/plans/post-p3.5-replanning-decisions.md](../../plans/post-p3.5-replanning-decisions.md)
Q2)で、「Webでプレイできるようになったが、わくわく感が無い」の主因を
**(c) 演出の薄さ**と判定した。ただし決定した打ち手はモーション演出の追加
ではなく、**カードそのもののビジュアル仕様**(文章メインの差別化・種類別
既定アイコン・白銀比縦長・2サイズ)。roadmap.md の Now/Next では長らく
「カードUI・演出強化」と呼んでいたが、「演出強化」は決定内容を正しく
表していなかったため、本タスク起票時(2026-08-12)に「カードUI改善」へ
呼称を整理した。

## 目的

`packages/ui` にカードコンポーネントを新設し、apps/web のカード表示
(手札・冒険記・GM裁定パネル等)を差し替える。現状(`Hand.tsx`)はカードが
ただのボタンリストで、カードらしい見た目が無い状態。

## 前提となる設計決定

- カードのビジュアル仕様は
  [design/ui-visual-design.md](../../../design/ui-visual-design.md)
  「カードのビジュアル方針」が正(文章メイン・種類別既定アイコン・
  白銀比縦長・小/大2サイズ)
- コンポーネントの置き場・カタログ掲載は
  [design/client-conventions.md](../../../design/client-conventions.md)
  「UIコンポーネントの置き場(packages/ui)」に従う。新設するカード
  コンポーネントも `packages/ui` に置き、
  [tasks/tools/component-catalog/task.md](../../tools/component-catalog/task.md)
  で追加済みのコンポーネントカタログ(`tools/docs-site` `#/components`)に
  掲載する
- 対象の `CardKind` は6種(`crates/tabifuda-core/src/card.rs`):
  `Action` / `Scenario` / `Dialogue` / `Proposal` / `Item` / `Marker`。
  ただし `Marker` は手札表示から除外済み
  (client-conventions.md「手札表示からの Marker 除外」)なので、
  既定アイコンは主に手札以外(GM裁定パネル等)での表示要否を見て判断する

## 人間の事前決定(該当サイクルの前に確認)

- **アイコンの調達方法**(C1着手前): 種類別既定アイコンをどう用意するか
  (自作SVG / アイコンフォント・ライブラリ導入 / 絵文字暫定)。
  ui-visual-design.md「未決事項」の一部と重なるため、C1着手時に決定し
  同文書へ追記する

## サイクル

### C1: カードコンポーネント新設(小サイズ)

- `packages/ui` にカードコンポーネントを新設。白銀比(1:√2)縦長、
  小サイズ(タイトル+アイコン。CardWirthの74x94相当が収まる)
- `CardKind` → 既定アイコンのマッピングを実装(人間の事前決定に従う)
- コンポーネントカタログ(`tools/docs-site` `#/components`)に追加し、
  6種類(Marker含む)の見本を並べて確認できるようにする
- 既存コンポーネントは変更しない(このサイクルは新設のみ)

### C2: 大サイズ+手札UIへの適用

- カードコンポーネントに大サイズ(スマホでも読みやすい詳細表示。
  カード名・本文・自由入力欄等)を追加
- `Hand.tsx` を新カードコンポーネントに置き換え、手札選択がカードらしい
  見た目・操作感になるようにする(Dialogue の自由入力フローは維持)
- apps/web の Playwright スモークが引き続き通ることを確認

### C3: 残箇所への適用+振り返り

- 冒険記(`chronicle/eventRenderers.tsx` 等)・GM裁定パネル
  (`GmJudgePanel.tsx`)など、カードが登場する残りの箇所に新コンポーネントを
  適用する
- 目視確認(Playwright): 手札・冒険記・GM裁定パネルでカードUIが崩れなく
  表示されコンソールエラーが無いことを確認
- **Q4再訪**(post-p3.5-replanning-decisions.md): 身近な1〜2人に見せるかを
  ここで再判断し、結果を本タスクの `plans/` に記録する

## 完了条件

- 手札・冒険記・GM裁定パネルのカードが、白銀比縦長・種類別既定アイコンを
  持つカードコンポーネントで表示される(小/大2サイズとも実装済み)
- コンポーネントカタログでカードの全種類(Marker含む)を確認できる
- 既存の Playwright スモークが通る
- Q4(身近な人に見せるか)の再判断結果が記録されている

## やらないこと

- 依頼(シナリオ)選択の張り紙カードUI: シナリオ選択機能自体が未実装のため
  スコープ外(future-requirements.md §11)
- 装飾目的のアニメーション・エフェクト: ui-visual-design.md「演出の強さ」
  の既定方針(視認性・操作性優先)どおり、状態変化の把握を助ける最小限を
  超えて作り込まない
- PC版の作り込み: ui-visual-design.md「対応優先度」どおりモバイル優先
- カード1枚ごとのイラスト制作: 既定アイコン方式を維持する
