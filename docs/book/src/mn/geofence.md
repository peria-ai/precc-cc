# Геофенс

PRECC нь зохицуулалттай орчинд зориулсан IP geofence нийцлийн шалгалтыг агуулна. Энэ нь Pro боломж юм.

## Тойм

Зарим байгууллагууд хөгжүүлэлтийн хэрэгслүүд зөвхөн зөвшөөрөгдсөн газарзүйн бүс нутагт ажиллахыг шаарддаг. PRECC-ийн geofence боломж нь одоогийн машины IP хаяг зөвшөөрөгдсөн бүсийн жагсаалтад байгаа эсэхийг шалгана.

## Нийцлийн шалгалт

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Хэрэв машин зөвшөөрөгдсөн бүс нутгийн гадна байвал:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Геофенс өгөгдлийг шинэчлэх

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Геофенс мэдээлэл харах

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

## Кэш цэвэрлэх

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Тохиргоо

Геофенс бодлогыг `~/.config/precc/geofence.toml` файлд тодорхойлно:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Зөвшөөрөгдсөн бүс нутгийн гадна PRECC ажиллахаас сэргийлэхийн тулд `block_on_violation = true` гэж тохируулна.
