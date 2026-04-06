# GitHub Actionsの分析

`precc gha`は失敗したGitHub Actionsの実行を分析し、修正を提案します。これはPro機能です。

## 使い方

失敗したGitHub Actionsの実行URLを渡します：

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## 機能

1. GitHub ActionsのランURLを解析して、オーナー、リポジトリ、ランIDを抽出します。
2. GitHub API経由でランログを取得します（`GITHUB_TOKEN`が設定されている場合はそれを使用、そうでなければパブリックアクセス）。
3. 失敗したステップを特定し、関連するエラー行を抽出します。
4. エラーを分析し、一般的なCI失敗パターンに基づいて修正を提案します。

## サポートされる障害パターン

- 不足しているサービスコンテナ（データベース、Redisなど）
- ランナーのOSまたはアーキテクチャの不一致
- 環境変数またはシークレットの不足
- 依存関係のインストール失敗
- テストタイムアウト
- 権限エラー
- キャッシュミスによるビルドの遅延
