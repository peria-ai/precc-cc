# Analiză GitHub Actions

`precc gha` analizează rulările eșuate de GitHub Actions și sugerează corecții. Aceasta este o funcție Pro.

## Utilizare

Furnizați URL-ul unei rulări eșuate de GitHub Actions:

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

## Ce face

1. Parsează URL-ul rulării GitHub Actions pentru a extrage proprietarul, repo-ul și ID-ul rulării.
2. Descarcă jurnalele rulării prin API-ul GitHub (folosește `GITHUB_TOKEN` dacă este setat, altfel acces public).
3. Identifică pasul eșuat și extrage liniile de eroare relevante.
4. Analizează eroarea și sugerează o corecție bazată pe tipare comune de eșec CI.

## Tipare de eșec suportate

- Containere de servicii lipsă (baze de date, Redis etc.)
- SO sau arhitectură incorectă a runner-ului
- Variabile de mediu sau secrete lipsă
- Eșecuri la instalarea dependențelor
- Timeout-uri ale testelor
- Erori de permisiuni
- Cache miss-uri care cauzează build-uri lente
