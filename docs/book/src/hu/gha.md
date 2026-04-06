# GitHub Actions elemzés

A `precc gha` elemzi a sikertelen GitHub Actions-futásokat és javításokat javasol. Ez egy Pro funkció.

## Használat

Adja meg egy sikertelen GitHub Actions-futás URL-jét:

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

## Mit csinál

1. Elemzi a GitHub Actions futási URL-t a tulajdonos, tároló és futási ID kinyeréséhez.
2. Lekéri a futási naplókat a GitHub API-n keresztül (használja a `GITHUB_TOKEN`-t ha be van állítva, egyébként nyilvános hozzáférés).
3. Azonosítja a sikertelen lépést és kivonja a releváns hibasorokat.
4. Elemzi a hibát és javítást javasol a gyakori CI-hibaminták alapján.

## Támogatott hibaminták

- Hiányzó szolgáltatás-konténerek (adatbázisok, Redis, stb.)
- Hibás runner OS vagy architektúra
- Hiányzó környezeti változók vagy titkok
- Függőségtelepítési hibák
- Teszt időtúllépések
- Jogosultsági hibák
- Cache-hiányok, amelyek lassú buildeket okoznak
