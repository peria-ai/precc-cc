# Telemetri

PRECC stöder opt-in anonym telemetri för att förbättra verktyget. Ingen data samlas in om du inte uttryckligen samtycker.

## Anmälan

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Avregistrering

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Statuskontroll

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Förhandsgranskning av data som skulle skickas

Innan anmälan kan du se exakt vilken data som skulle samlas in:

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

## Vad som samlas in

- PRECC-version, OS och arkitektur
- Aggregerade räknare: uppfångade kommandon, aktiverade färdigheter, använda pelarpunkter
- Genomsnittlig hook-latens
- Sessionsantal

## Vad som INTE samlas in

- Ingen kommandotext eller argument
- Inga filsökvägar eller katalognamn
- Inga projektnamn eller repository-URL:er
- Ingen personligt identifierbar information (PII)
- Inga IP-adresser (servern loggar dem inte)

## Miljövariabel-override

För att inaktivera telemetri utan att köra ett kommando (användbart i CI eller delade miljöer):

```bash
export PRECC_NO_TELEMETRY=1
```

Detta har företräde framför samtyckesinställningen.

## Datadestination

Telemetridata skickas till `https://telemetry.peria.ai/v1/precc` över HTTPS. Data används enbart för att förstå användningsmönster och prioritera utveckling.
