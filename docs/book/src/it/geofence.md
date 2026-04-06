# Geofence

PRECC include il controllo di conformità IP geofence per ambienti regolamentati. Questa è una funzionalità Pro.

## Panoramica

Alcune organizzazioni richiedono che gli strumenti di sviluppo operino solo all'interno di regioni geografiche approvate. La funzionalità geofence di PRECC verifica che l'indirizzo IP della macchina corrente rientri in un elenco di regioni consentite.

## Verifica conformità

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Se la macchina è fuori dalle regioni consentite:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Aggiornamento dati geofence

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Visualizzazione info geofence

```bash
$ precc geofence info
Geofence Configuration
======================
Policy file:    ~/.config/precc/geofence.toml
Allowed regions: us-east-1, us-west-2, eu-west-1
Cache age:      2h 14m
Last check:     2026-04-03 09:12:00 UTC
Status:         COMPLIANT
```

## Pulizia cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configurazione

La policy di geofence è definita in `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Imposta `block_on_violation = true` per impedire a PRECC di operare quando ci si trova fuori dalle regioni consentite.
