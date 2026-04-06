# Geofence

PRECC enthält IP-Geofence-Compliance-Prüfung für regulierte Umgebungen. Dies ist eine Pro-Funktion.

## Überblick

Einige Organisationen verlangen, dass Entwicklungswerkzeuge nur innerhalb genehmigter geografischer Regionen betrieben werden. Die Geofence-Funktion von PRECC überprüft, ob die IP-Adresse des aktuellen Rechners in einer Liste erlaubter Regionen liegt.

## Compliance-Prüfung

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Wenn sich der Rechner außerhalb der erlaubten Regionen befindet:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Geofence-Daten aktualisieren

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Geofence-Informationen anzeigen

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

## Cache leeren

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfiguration

Die Geofence-Richtlinie wird in `~/.config/precc/geofence.toml` definiert:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Setzen Sie `block_on_violation = true`, um zu verhindern, dass PRECC außerhalb der erlaubten Regionen arbeitet.
