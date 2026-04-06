# Geofence

PRECC bevat IP-geofence-nalevingscontrole voor gereguleerde omgevingen. Dit is een Pro-functie.

## Overzicht

Sommige organisaties vereisen dat ontwikkeltools alleen binnen goedgekeurde geografische regio's werken. De geofence-functie van PRECC verifieert dat het IP-adres van de huidige machine binnen een toegestane regiolijst valt.

## Nalevingscontrole

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Als de machine buiten de toegestane regio's is:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Geofence-gegevens vernieuwen

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Geofence-informatie bekijken

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

## Cache wissen

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configuratie

Het geofence-beleid wordt gedefinieerd in `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Stel `block_on_violation = true` in om te voorkomen dat PRECC werkt buiten toegestane regio's.
