# Geofence

PRECC includes IP geofence compliance checking for regulated environments. This is a Pro feature.

## Overview

Some organizations require that development tools only operate within approved geographic regions. PRECC's geofence feature verifies that the current machine's IP address falls within an allowed region list.

## Checking Compliance

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

If the machine is outside the allowed regions:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Refreshing Geofence Data

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Viewing Geofence Info

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

## Clearing Cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configuration

The geofence policy is defined in `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Set `block_on_violation = true` to prevent PRECC from operating when outside allowed regions.
