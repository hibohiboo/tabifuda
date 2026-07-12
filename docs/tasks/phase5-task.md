# Phase 5 実装タスク: AWSデプロイ

実行モデル: Sonnet 5。1サイクル=1セッション=1PR。
開始前の儀式は phase2-task.md 冒頭と同じ。
**IaCはOpus 4.8のセキュリティレビュー対象**(公開前必須)。

## 人間の事前決定(ADR化必須)
- IaCツール(推奨候補: AWS CDK。SST等との比較を提示して選択)
- ドメイン名・環境構成(prod のみ / prod+staging)

## サイクル

### C1: IaC骨格
- CloudFront+S3(apps/web 静的配信)、API Gateway+Lambda(Hono adapter)、
  Neon接続(serverlessドライバ、接続数に注意)
- シークレットはSSM Parameter Store(cross-cutting.md)。コード・環境定義に
  平文を置かない

### C2: デプロイパイプライン
- GitHub Actions: main→staging自動、prodは手動承認
- マイグレーションの適用手順(デプロイとの順序)を文書化して自動化

### C3: 観測と保護
- CloudWatch構造化ログ(種別+IDのみの規律を本番でも維持)、
  エラー率・レイテンシの最小アラーム
- API GatewayスロットリングとCloudFrontの基本保護、コスト予算アラート

### C4: 公開前チェック
- デプロイ後スモーク(セッション開始→カード1枚→冒険記取得のcurl)
- Opusセキュリティレビュー(IAM最小権限、公開範囲、CORS、シークレット)
- cargo/pnpm audit を必須ゲートに昇格(cross-cutting.md の予定どおり)

## 完了条件
本番URLで通しプレイ可能 / パイプラインが再現可能 /
セキュリティレビュー指摘ゼロまたは対応済み
