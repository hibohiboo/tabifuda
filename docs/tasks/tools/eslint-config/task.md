---
status: done
cycles:
  C1: done
---

# ツールタスク: ESLint設定の共通化(packages/eslint-config)

実行モデル: Sonnet 5。どのフェーズにも属さないツール系タスク。

## 目的

apps/webとpackages/uiでほぼ同一のまま重複していたESLint flat configを
`packages/eslint-config`へ切り出す。あわせて`eslint-plugin-sonarjs`を
導入する(人間との相談で決定。エージェントが実装主体のプロジェクトに
おける自動品質ガードレールとして採用)。

## 前提となる設計決定

- パッケージ追加の経緯・sonarjs採用理由は
  [../../../adr/0002-package-manager.md](../../../adr/0002-package-manager.md)
  「追記(2026-08-01): packages/eslint-config追加」が正
- tools/docs-siteはTypeScript 7系を使用しており、typescript-eslintが
  現時点で未対応(ADR 0006)のため、本タスクでは対象外とする
  (docs-siteへのlint導入は別途判断)

## サイクル

### C1: 共通化+sonarjs導入

- `packages/eslint-config`新設(`base.js`: js recommended+typescript-eslint
  recommended+sonarjs recommended。`frontend.js`: base+react-hooks+UGC禁止
  ルール)
- apps/web・packages/uiの`eslint.config.js`を`@tabifuda/eslint-config/frontend`
  のre-exportのみに置き換え、両パッケージのpackage.jsonから重複した
  ESLint関連devDependency(`@eslint/js`・`eslint-plugin-react-hooks`・
  `typescript-eslint`)を削除し`@tabifuda/eslint-config`に一本化
- sonarjs recommendedを適用した結果、現状のコードで違反が出ないことを確認
  (追加の除外設定は不要だった)
- UGC禁止ルール(dangerouslySetInnerHTML検出)が共通化後も機能することを
  一時的な違反コードで再検証
- 検証: 両パッケージのtypecheck/lint/build、apps/webのPlaywrightスモーク
  がすべて通過

## 完了条件

apps/web・packages/uiのESLint設定が`packages/eslint-config`からの
参照のみになっている / `eslint-plugin-sonarjs`のrecommended設定が
両パッケージに適用されている / 既存のtypecheck/lint/build/スモークが
すべて通過する
