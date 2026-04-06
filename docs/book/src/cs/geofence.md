# Geofence

PRECC zahrnuje kontrolu shody IP geofence pro regulovaná prostředí. Toto je funkce Pro.

## Přehled

Některé organizace vyžadují, aby vývojové nástroje fungovaly pouze ve schválených geografických regionech. Funkce geofence v PRECC ověřuje, že IP adresa aktuálního počítače spadá do seznamu povolených regionů.

## Kontrola shody

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Pokud je počítač mimo povolené regiony:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Obnovení dat geofence

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Zobrazení informací o geofence

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

## Vymazání cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfigurace

Politika geofence je definována v `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Nastavte `block_on_violation = true` pro zabránění fungování PRECC mimo povolené regiony.
