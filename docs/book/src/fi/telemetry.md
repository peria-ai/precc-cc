# Telemetria

PRECC tukee opt-in-anonyymiä telemetriaa työkalun parantamiseksi. Mitään tietoja ei kerätä ellei nimenomaisesti anna suostumustasi.

## Liittyminen

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Poistuminen

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Tilan tarkistus

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Lähetettävien tietojen esikatselu

Ennen liittymistä voit nähdä tarkalleen mitä tietoja kerättäisiin:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Mitä kerätään

- PRECC-versio, käyttöjärjestelmä ja arkkitehtuuri
- Kootut laskurit: kaapatut komennot, aktivoidut taidot, käytetyt pilarit
- Keskimääräinen hook-viive
- Istuntojen määrä

## Mitä EI kerätä

- Ei komentotekstiä tai argumentteja
- Ei tiedostopolkuja tai hakemistonimiä
- Ei projektinimiä tai repositorio-URL-osoitteita
- Ei henkilökohtaisesti tunnistettavaa tietoa (PII)
- Ei IP-osoitteita (palvelin ei kirjaa niitä)

## Ympäristömuuttujan ohitus

Telemetrian poistaminen käytöstä komentoa suorittamatta (hyödyllistä CI:ssä tai jaetuissa ympäristöissä):

```bash
export PRECC_NO_TELEMETRY=1
```

Tämä ohittaa suostumuksen asetuksen.

## Tietojen kohde

Telemetriatiedot lähetetään osoitteeseen `https://telemetry.peria.ai/v1/precc` HTTPS:n kautta. Tietoja käytetään ainoastaan käyttömallien ymmärtämiseen ja kehityksen priorisointiin.
