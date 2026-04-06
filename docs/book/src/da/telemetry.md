# Telemetri

PRECC understøtter opt-in anonym telemetri for at forbedre værktøjet. Ingen data indsamles medmindre du eksplicit samtykker.

## Tilmelding

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Framelding

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Statuskontrol

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Forhåndsvisning af data der ville blive sendt

Før tilmelding kan du se præcis hvilke data der ville blive indsamlet:

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

## Hvad der indsamles

- PRECC-version, OS og arkitektur
- Aggregerede tællinger: opfangede kommandoer, aktiverede færdigheder, anvendte pillar
- Gennemsnitlig hook-latens
- Sessionsantal

## Hvad der IKKE indsamles

- Ingen kommandotekst eller argumenter
- Ingen filstier eller mappenavne
- Ingen projektnavne eller repository-URL'er
- Ingen personligt identificerbare oplysninger (PII)
- Ingen IP-adresser (serveren logger dem ikke)

## Miljøvariabel-override

For at deaktivere telemetri uden at køre en kommando (nyttigt i CI eller delte miljøer):

```bash
export PRECC_NO_TELEMETRY=1
```

Dette har forrang over samtykkeindstillingen.

## Datadestination

Telemetridata sendes til `https://telemetry.peria.ai/v1/precc` over HTTPS. Dataene bruges udelukkende til at forstå brugsmønstre og prioritere udvikling.
