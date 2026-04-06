# GitHub Actions 分析

`precc gha` 分析失败的GitHub Actions运行并建议修复方案。这是Pro功能。

## 用法

传入失败的GitHub Actions运行的URL：

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

## 功能说明

1. 解析GitHub Actions运行URL以提取所有者、仓库和运行ID。
2. 通过GitHub API获取运行日志（如果设置了 `GITHUB_TOKEN` 则使用，否则公开访问）。
3. 识别失败步骤并提取相关错误行。
4. 分析错误并根据常见CI失败模式建议修复方案。

## 支持的失败模式

- 缺少服务容器（数据库、Redis等）
- 运行器OS或架构不正确
- 缺少环境变量或密钥
- 依赖安装失败
- 测试超时
- 权限错误
- 缓存未命中导致构建缓慢
