# Geofence

A PRECC IP geofence megfelelőségi ellenőrzést tartalmaz szabályozott környezetekhez. Ez egy Pro funkció.

## Áttekintés

Egyes szervezetek megkövetelik, hogy a fejlesztőeszközök csak jóváhagyott földrajzi régiókban működjenek. A PRECC geofence funkciója ellenőrzi, hogy a jelenlegi gép IP-címe az engedélyezett régiók listáján belül van-e.

## Megfelelőségi ellenőrzés

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Ha a gép az engedélyezett régiókon kívül van:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Geofence adatok frissítése

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Geofence információk megtekintése

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

## Gyorsítótár törlése

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Konfiguráció

A geofence-szabályzat a `~/.config/precc/geofence.toml` fájlban van meghatározva:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Állítsa a `block_on_violation = true` értéket, hogy megakadályozza a PRECC működését az engedélyezett régiókon kívül.
