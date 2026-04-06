# Geofence

PRECC zawiera sprawdzanie zgodności geofence IP dla środowisk regulowanych. To funkcja Pro.

## Przegląd

Niektóre organizacje wymagają, aby narzędzia programistyczne działały tylko w zatwierdzonych regionach geograficznych. Funkcja geofence PRECC sprawdza, czy adres IP bieżącej maszyny znajduje się na liście dozwolonych regionów.

## Sprawdzanie zgodności

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Jeśli maszyna jest poza dozwolonymi regionami:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Odświeżanie danych geofence

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Wyświetlanie informacji geofence

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

## Czyszczenie pamięci podręcznej

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfiguracja

Polityka geofence jest zdefiniowana w `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Ustaw `block_on_violation = true`, aby zapobiec działaniu PRECC poza dozwolonymi regionami.
