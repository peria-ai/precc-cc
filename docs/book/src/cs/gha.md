# Analýza GitHub Actions

`precc gha` analyzuje neúspěšné běhy GitHub Actions a navrhuje opravy. Toto je funkce Pro.

## Použití

Zadejte URL neúspěšného běhu GitHub Actions:

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

## Co to dělá

1. Parsuje URL běhu GitHub Actions pro extrakci vlastníka, repozitáře a ID běhu.
2. Stáhne logy běhu přes GitHub API (používá `GITHUB_TOKEN` pokud je nastaven, jinak veřejný přístup).
3. Identifikuje neúspěšný krok a extrahuje relevantní chybové řádky.
4. Analyzuje chybu a navrhne opravu na základě běžných vzorů CI selhání.

## Podporované vzory selhání

- Chybějící servisní kontejnery (databáze, Redis atd.)
- Nesprávný OS nebo architektura runneru
- Chybějící proměnné prostředí nebo tajemství
- Selhání instalace závislostí
- Timeouty testů
- Chyby oprávnění
- Cache miss způsobující pomalé buildy
