# GitHub Actions Analyse

`precc gha` analysiert fehlgeschlagene GitHub Actions-Läufe und schlägt Korrekturen vor. Dies ist eine Pro-Funktion.

## Verwendung

Übergeben Sie die URL eines fehlgeschlagenen GitHub Actions-Laufs:

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

## Was es tut

1. Parst die GitHub Actions-Run-URL, um den Eigentümer, das Repository und die Run-ID zu extrahieren.
2. Ruft die Run-Logs über die GitHub-API ab (verwendet `GITHUB_TOKEN` falls gesetzt, sonst öffentlichen Zugang).
3. Identifiziert den fehlgeschlagenen Schritt und extrahiert die relevanten Fehlerzeilen.
4. Analysiert den Fehler und schlägt eine Korrektur basierend auf häufigen CI-Fehlermustern vor.

## Unterstützte Fehlermuster

- Fehlende Service-Container (Datenbanken, Redis, etc.)
- Falsches Runner-Betriebssystem oder falsche Architektur
- Fehlende Umgebungsvariablen oder Secrets
- Fehler bei der Abhängigkeitsinstallation
- Test-Timeouts
- Berechtigungsfehler
- Cache-Misses, die langsame Builds verursachen
