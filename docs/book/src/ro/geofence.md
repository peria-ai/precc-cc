# Geofence

PRECC include verificarea conformității geofence IP pentru medii reglementate. Aceasta este o funcție Pro.

## Prezentare generală

Unele organizații necesită ca instrumentele de dezvoltare să funcționeze doar în regiuni geografice aprobate. Funcția geofence a PRECC verifică că adresa IP a mașinii curente se încadrează într-o listă de regiuni permise.

## Verificare conformitate

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Dacă mașina este în afara regiunilor permise:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Reîmprospătare date geofence

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Vizualizare informații geofence

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

## Curățare cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configurare

Politica geofence este definită în `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Setați `block_on_violation = true` pentru a preveni PRECC să funcționeze în afara regiunilor permise.
