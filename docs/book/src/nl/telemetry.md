# Telemetrie

PRECC ondersteunt optionele anonieme telemetrie om het hulpmiddel te verbeteren. Er worden geen gegevens verzameld tenzij u expliciet toestemt.

## Inschakelen

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Uitschakelen

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Status controleren

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Voorbeeld van wat verzonden zou worden

Voordat u inschakelt, kunt u precies zien welke gegevens verzameld zouden worden:

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

## Wat er verzameld wordt

- PRECC-versie, besturingssysteem en architectuur
- Geaggregeerde tellingen: onderschepte opdrachten, geactiveerde vaardigheden, gebruikte pijlers
- Gemiddelde hook-latentie
- Aantal sessies

## Wat NIET verzameld wordt

- Geen opdrachttekst of argumenten
- Geen bestandspaden of mapnamen
- Geen projectnamen of repository-URL's
- Geen persoonlijk identificeerbare informatie (PII)
- Geen IP-adressen (de server logt ze niet)

## Omgevingsvariabele overschrijving

Om telemetrie uit te schakelen zonder een opdracht uit te voeren (handig in CI of gedeelde omgevingen):

```bash
export PRECC_NO_TELEMETRY=1
```

Dit heeft voorrang op de toestemmingsinstelling.

## Gegevensbestemming

Telemetriegegevens worden via HTTPS verzonden naar `https://telemetry.peria.ai/v1/precc`. De gegevens worden uitsluitend gebruikt om gebruikspatronen te begrijpen en ontwikkeling te prioriteren.
