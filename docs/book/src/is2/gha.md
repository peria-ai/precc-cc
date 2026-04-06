# GitHub Actions greining

`precc gha` greinir misheppnaðar GitHub Actions keyrslur og stingur upp á lagfæringum. Þetta er Pro-eiginleiki.

## Notkun

Gefðu URL misheppnaðrar GitHub Actions keyrslu:

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

## Hvað það gerir

1. Þáttar GitHub Actions keyrslu-URL til að draga út eiganda, repo og keyrslu-ID.
2. Sækir keyrslulogs gegnum GitHub API (notar `GITHUB_TOKEN` ef stillt, annars opinn aðgangur).
3. Auðkennir misheppnaða skrefið og dregur út viðeigandi villulínur.
4. Greinir villuna og stingur upp á lagfæringu byggt á algengum CI-villumunstur.

## Studd villumunstur

- Vantar þjónustugáma (gagnagrunna, Redis o.fl.)
- Rangt keyrslu-OS eða arkitektúr
- Vantar umhverfisbreytur eða leyndarmál
- Bilun í uppsetningu háða pakka
- Tímamörk prófana
- Heimildavillur
- Skyndiminnis-missi sem veldur hægum smíðum
