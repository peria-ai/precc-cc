# Telemetrie

PRECC podporuje opt-in anonymní telemetrii pro zlepšení nástroje. Žádná data nejsou sbírána, pokud explicitně nesouhlasíte.

## Přihlášení

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Odhlášení

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Kontrola stavu

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Náhled dat, která by byla odeslána

Před přihlášením si můžete prohlédnout přesně jaká data by byla sbírána:

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

## Co se sbírá

- Verze PRECC, OS a architektura
- Agregované počty: zachycené příkazy, aktivované dovednosti, použité pilíře
- Průměrná latence hooku
- Počet relací

## Co se NESBÍRÁ

- Žádný text příkazů nebo argumenty
- Žádné cesty k souborům nebo názvy adresářů
- Žádné názvy projektů nebo URL repozitářů
- Žádné osobní identifikační údaje (PII)
- Žádné IP adresy (server je nezaznamenává)

## Přepsání proměnnou prostředí

Pro deaktivaci telemetrie bez spuštění příkazu (užitečné v CI nebo sdílených prostředích):

```bash
export PRECC_NO_TELEMETRY=1
```

Toto má přednost před nastavením souhlasu.

## Cíl dat

Telemetrická data jsou odesílána na `https://telemetry.peria.ai/v1/precc` přes HTTPS. Data jsou používána výhradně k pochopení vzorů využití a stanovení priorit vývoje.
