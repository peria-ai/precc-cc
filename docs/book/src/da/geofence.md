# Geofence

PRECC inkluderer IP-geofence-overensstemmelseskontrol for regulerede miljøer. Dette er en Pro-funktion.

## Oversigt

Nogle organisationer kræver, at udviklingsværktøjer kun fungerer inden for godkendte geografiske regioner. PRECCs geofence-funktion verificerer, at den aktuelle maskines IP-adresse er inden for en tilladt regionliste.

## Overensstemmelseskontrol

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Hvis maskinen er uden for de tilladte regioner:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Opdatering af geofence-data

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Visning af geofence-info

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

## Rydning af cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfiguration

Geofence-politikken er defineret i `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Sæt `block_on_violation = true` for at forhindre PRECC i at fungere uden for tilladte regioner.
