# GitHub Actions-analys

`precc gha` analyserar misslyckade GitHub Actions-körningar och föreslår rättningar. Detta är en Pro-funktion.

## Användning

Ange URL:en till en misslyckad GitHub Actions-körning:

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

## Vad det gör

1. Tolkar GitHub Actions-körningens URL för att extrahera ägare, repo och run-ID.
2. Hämtar körningsloggarna via GitHub API (använder `GITHUB_TOKEN` om satt, annars offentlig åtkomst).
3. Identifierar det misslyckade steget och extraherar de relevanta felraderna.
4. Analyserar felet och föreslår en rättning baserat på vanliga CI-felmönster.

## Stödda felmönster

- Saknade tjänstecontainrar (databaser, Redis etc.)
- Felaktigt runner-OS eller arkitektur
- Saknade miljövariabler eller hemligheter
- Installationsfel för beroenden
- Testtimeouts
- Behörighetsfel
- Cache-missar som orsakar långsamma byggen
