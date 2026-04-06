# Геозонирование

PRECC включает проверку соответствия IP-геозонированию для регулируемых сред. Это функция Pro.

## Обзор

Некоторые организации требуют, чтобы инструменты разработки работали только в одобренных географических регионах. Функция геозонирования PRECC проверяет, что IP-адрес текущей машины попадает в список разрешённых регионов.

## Проверка соответствия

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Если машина находится за пределами разрешённых регионов:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Обновление данных геозонирования

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Просмотр информации о геозонировании

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

## Очистка кэша

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Конфигурация

Политика геозонирования определена в `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Установите `block_on_violation = true`, чтобы запретить PRECC работать за пределами разрешённых регионов.
