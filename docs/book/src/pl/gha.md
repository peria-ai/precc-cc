# Analiza GitHub Actions

`precc gha` analizuje nieudane uruchomienia GitHub Actions i sugeruje poprawki. To funkcja Pro.

## Użycie

Podaj URL nieudanego uruchomienia GitHub Actions:

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

## Co robi

1. Parsuje URL uruchomienia GitHub Actions, aby wyodrębnić właściciela, repozytorium i ID uruchomienia.
2. Pobiera logi uruchomienia przez API GitHub (używa `GITHUB_TOKEN` jeśli ustawiony, w przeciwnym razie dostęp publiczny).
3. Identyfikuje nieudany krok i wyodrębnia odpowiednie linie błędów.
4. Analizuje błąd i sugeruje poprawkę na podstawie typowych wzorców awarii CI.

## Obsługiwane wzorce awarii

- Brakujące kontenery usług (bazy danych, Redis itp.)
- Nieprawidłowy system operacyjny lub architektura runnera
- Brakujące zmienne środowiskowe lub sekrety
- Błędy instalacji zależności
- Przekroczenia limitu czasu testów
- Błędy uprawnień
- Braki w cache powodujące wolne kompilacje
