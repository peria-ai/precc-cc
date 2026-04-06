# Landsvæðaeftirlit

PRECC inniheldur IP-landsvæðasamræmisathugun fyrir stýrð umhverfi. Þetta er Pro-eiginleiki.

## Yfirlit

Sumar stofnanir krefjast þess að þróunartól virki aðeins innan viðurkenndra landfræðilegra svæða. Landsvæðaeiginleiki PRECC staðfestir að IP-tala núverandi tölvu falli innan leyfðs svæðalista.

## Samræmisathugun

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Ef tölvan er utan leyfilegra svæða:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Endurnýjun landsvæðagagna

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Skoðun landsvæðaupplýsinga

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

## Hreinsun skyndiminnis

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Stillingar

Landsvæðastefnan er skilgreind í `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Stilltu `block_on_violation = true` til að koma í veg fyrir að PRECC starfi utan leyfilegra svæða.
