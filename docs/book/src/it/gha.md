# Analisi GitHub Actions

`precc gha` analizza le esecuzioni fallite di GitHub Actions e suggerisce correzioni. Questa è una funzionalità Pro.

## Utilizzo

Passa l'URL di un'esecuzione fallita di GitHub Actions:

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

## Cosa fa

1. Analizza l'URL dell'esecuzione GitHub Actions per estrarre owner, repo e run ID.
2. Recupera i log dell'esecuzione tramite l'API GitHub (usa `GITHUB_TOKEN` se impostato, altrimenti accesso pubblico).
3. Identifica lo step fallito ed estrae le righe di errore rilevanti.
4. Analizza l'errore e suggerisce una correzione basata su pattern comuni di fallimento CI.

## Pattern di fallimento supportati

- Container di servizio mancanti (database, Redis, ecc.)
- Sistema operativo o architettura del runner errati
- Variabili d'ambiente o segreti mancanti
- Fallimenti nell'installazione delle dipendenze
- Timeout dei test
- Errori di permesso
- Cache miss che causano build lente
