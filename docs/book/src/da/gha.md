# GitHub Actions-analyse

`precc gha` analyserer fejlede GitHub Actions-kørsler og foreslår rettelser. Dette er en Pro-funktion.

## Brug

Angiv URL'en til en fejlet GitHub Actions-kørsel:

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

## Hvad det gør

1. Parser GitHub Actions-kørslens URL for at udtrække owner, repo og run-ID.
2. Henter kørslens logfiler via GitHub API (bruger `GITHUB_TOKEN` hvis sat, ellers offentlig adgang).
3. Identificerer det fejlede trin og udtrækker de relevante fejllinjer.
4. Analyserer fejlen og foreslår en rettelse baseret på almindelige CI-fejlmønstre.

## Understøttede fejlmønstre

- Manglende servicecontainere (databaser, Redis osv.)
- Forkert runner-OS eller arkitektur
- Manglende miljøvariabler eller hemmeligheder
- Fejl ved installation af afhængigheder
- Test-timeouts
- Tilladelsesfejl
- Cache-misser der forårsager langsomme builds
