# Coğrafi sınır

PRECC, düzenlenmiş ortamlar için IP coğrafi sınır uyumluluk denetimi içerir. Bu bir Pro özelliğidir.

## Genel bakış

Bazı kuruluşlar, geliştirme araçlarının yalnızca onaylı coğrafi bölgelerde çalışmasını gerektirir. PRECC'nin coğrafi sınır özelliği, mevcut makinenin IP adresinin izin verilen bölge listesinde olduğunu doğrular.

## Uyumluluk kontrolü

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Makine izin verilen bölgelerin dışındaysa:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Coğrafi sınır verilerini yenileme

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Coğrafi sınır bilgilerini görüntüleme

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

## Önbelleği temizleme

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Yapılandırma

Coğrafi sınır politikası `~/.config/precc/geofence.toml` dosyasında tanımlanır:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

İzin verilen bölgelerin dışındayken PRECC'nin çalışmasını önlemek için `block_on_violation = true` olarak ayarlayın.
