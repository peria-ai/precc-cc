# {{i18n:geo_title}}

{{i18n:geo_intro}}

## {{i18n:geo_overview_title}}

{{i18n:geo_overview_body}}

## {{i18n:geo_check_title}}

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

{{i18n:geo_check_noncompliant}}

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## {{i18n:geo_refresh_title}}

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## {{i18n:geo_info_title}}

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

## {{i18n:geo_clear_title}}

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## {{i18n:geo_config_title}}

{{i18n:geo_config_body}}

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

{{i18n:geo_config_block}}
