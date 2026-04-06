# GitHub Actions 分析

`precc gha` 分析失敗的GitHub Actions運行並建議修復方案。這是Pro功能。

## 用法

傳入失敗的GitHub Actions運行的URL：

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

## 功能說明

1. 解析GitHub Actions運行URL以提取所有者、倉庫和運行ID。
2. 通過GitHub API獲取運行日誌（如果設置了 `GITHUB_TOKEN` 則使用，否則公開訪問）。
3. 識別失敗步驟並提取相關錯誤行。
4. 分析錯誤並根據常見CI失敗模式建議修復方案。

## 支持的失敗模式

- 缺少服務容器（數據庫、Redis等）
- 運行器OS或架構不正確
- 缺少環境變量或密鑰
- 依賴安裝失敗
- 測試超時
- 權限錯誤
- 緩存未命中導致構建緩慢
