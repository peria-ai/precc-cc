# Telemetria

PRECC supporta telemetria anonima opt-in per migliorare lo strumento. Nessun dato viene raccolto a meno che tu non dia esplicitamente il consenso.

## Adesione

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Rinuncia

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Verifica dello stato

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Anteprima dei dati che verrebbero inviati

Prima di aderire, puoi vedere esattamente quali dati verrebbero raccolti:

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

## Cosa viene raccolto

- Versione PRECC, sistema operativo e architettura
- Conteggi aggregati: comandi intercettati, skill attivate, pillar utilizzati
- Latenza media dell'hook
- Conteggio sessioni

## Cosa NON viene raccolto

- Nessun testo di comandi o argomenti
- Nessun percorso di file o nomi di directory
- Nessun nome di progetto o URL di repository
- Nessuna informazione personale identificabile (PII)
- Nessun indirizzo IP (il server non li registra)

## Override con variabile d'ambiente

Per disabilitare la telemetria senza eseguire un comando (utile in CI o ambienti condivisi):

```bash
export PRECC_NO_TELEMETRY=1
```

Questa ha la precedenza sull'impostazione di consenso.

## Destinazione dati

I dati di telemetria vengono inviati a `https://telemetry.peria.ai/v1/precc` tramite HTTPS. I dati vengono utilizzati esclusivamente per comprendere i pattern di utilizzo e dare priorità allo sviluppo.
