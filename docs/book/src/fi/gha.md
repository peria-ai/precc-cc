# GitHub Actions -analyysi

`precc gha` analysoi epäonnistuneet GitHub Actions -ajot ja ehdottaa korjauksia. Tämä on Pro-ominaisuus.

## Käyttö

Anna epäonnistuneen GitHub Actions -ajon URL:

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

## Mitä se tekee

1. Jäsentää GitHub Actions -ajon URL:n omistajan, repon ja ajo-ID:n poimimiseksi.
2. Hakee ajon lokit GitHub API:n kautta (käyttää `GITHUB_TOKEN`:ia jos asetettu, muuten julkinen pääsy).
3. Tunnistaa epäonnistuneen vaiheen ja poimii virherivit.
4. Analysoi virheen ja ehdottaa korjausta yleisten CI-virhemallien perusteella.

## Tuetut virhemallit

- Puuttuvat palvelukontit (tietokannat, Redis jne.)
- Väärä runner-käyttöjärjestelmä tai arkkitehtuuri
- Puuttuvat ympäristömuuttujat tai salaisuudet
- Riippuvuuksien asennusvirheet
- Testien aikakatkaisut
- Oikeusvirheet
- Välimuistiosumattomuudet aiheuttaen hitaita koontiversioita
