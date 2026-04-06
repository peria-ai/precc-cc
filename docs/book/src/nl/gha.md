# GitHub Actions-analyse

`precc gha` analyseert mislukte GitHub Actions-runs en stelt oplossingen voor. Dit is een Pro-functie.

## Gebruik

Geef de URL van een mislukte GitHub Actions-run door:

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

## Wat het doet

1. Parseert de GitHub Actions-run-URL om de eigenaar, repository en run-ID te extraheren.
2. Haalt de run-logs op via de GitHub API (gebruikt `GITHUB_TOKEN` indien ingesteld, anders openbare toegang).
3. Identificeert de mislukte stap en extraheert de relevante foutregels.
4. Analyseert de fout en stelt een oplossing voor op basis van veelvoorkomende CI-foutpatronen.

## Ondersteunde foutpatronen

- Ontbrekende servicecontainers (databases, Redis, enz.)
- Onjuist runner-besturingssysteem of -architectuur
- Ontbrekende omgevingsvariabelen of secrets
- Fouten bij de installatie van afhankelijkheden
- Test-timeouts
- Machtigingsfouten
- Cache-misses die trage builds veroorzaken
