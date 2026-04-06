# GitHub Actions 분석

`precc gha`는 실패한 GitHub Actions 실행을 분석하고 수정 사항을 제안합니다. Pro 기능입니다.

## 사용법

실패한 GitHub Actions 실행의 URL을 전달합니다:

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

## 기능 설명

1. GitHub Actions 실행 URL을 파싱하여 소유자, 리포지토리, 실행 ID를 추출합니다.
2. GitHub API를 통해 실행 로그를 가져옵니다(`GITHUB_TOKEN`이 설정된 경우 사용, 그렇지 않으면 공개 접근).
3. 실패한 단계를 식별하고 관련 오류 줄을 추출합니다.
4. 오류를 분석하고 일반적인 CI 실패 패턴에 기반한 수정을 제안합니다.

## 지원되는 실패 패턴

- 누락된 서비스 컨테이너(데이터베이스, Redis 등)
- 잘못된 러너 OS 또는 아키텍처
- 누락된 환경 변수 또는 시크릿
- 종속성 설치 실패
- 테스트 시간 초과
- 권한 오류
- 느린 빌드를 유발하는 캐시 미스
