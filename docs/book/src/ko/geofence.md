# 지오펜스

PRECC는 규제 환경을 위한 IP 지오펜스 규정 준수 검사를 포함합니다. Pro 기능입니다.

## 개요

일부 조직은 개발 도구가 승인된 지리적 리전 내에서만 작동하도록 요구합니다. PRECC의 지오펜스 기능은 현재 기기의 IP 주소가 허용된 리전 목록 내에 있는지 확인합니다.

## 규정 준수 확인

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

기기가 허용된 리전 외부에 있는 경우:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## 지오펜스 데이터 새로고침

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## 지오펜스 정보 보기

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

## 캐시 지우기

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## 구성

지오펜스 정책은 `~/.config/precc/geofence.toml`에 정의됩니다:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

허용된 리전 외부에서 PRECC가 작동하지 않도록 `block_on_violation = true`를 설정하세요.
