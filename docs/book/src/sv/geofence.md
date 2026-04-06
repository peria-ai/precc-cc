# Geofence

PRECC inkluderar IP-geofence-efterlevnadskontroll för reglerade miljöer. Detta är en Pro-funktion.

## Översikt

Vissa organisationer kräver att utvecklingsverktyg bara fungerar inom godkända geografiska regioner. PRECCs geofence-funktion verifierar att den aktuella maskinens IP-adress faller inom en tillåten regionlista.

## Efterlevnadskontroll

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Om maskinen är utanför de tillåtna regionerna:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Uppdatera geofence-data

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Visa geofence-info

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

## Rensa cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfiguration

Geofence-policyn definieras i `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Ställ in `block_on_violation = true` för att förhindra PRECC från att fungera utanför tillåtna regioner.
