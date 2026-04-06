# GitHub Actions Analysis

`precc gha` analyzes failed GitHub Actions runs and suggests fixes. This is a Pro feature.

## Usage

Pass the URL of a failed GitHub Actions run:

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

## What It Does

1. Parses the GitHub Actions run URL to extract the owner, repo, and run ID.
2. Fetches the run logs via the GitHub API (uses `GITHUB_TOKEN` if set, otherwise public access).
3. Identifies the failed step and extracts the relevant error lines.
4. Analyzes the error and suggests a fix based on common CI failure patterns.

## Supported Failure Patterns

- Missing service containers (databases, Redis, etc.)
- Incorrect runner OS or architecture
- Missing environment variables or secrets
- Dependency installation failures
- Test timeouts
- Permission errors
- Cache misses causing slow builds
