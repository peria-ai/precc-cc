# 地理圍欄

PRECC包含用於受監管環境的IP地理圍欄合規性檢查。這是Pro功能。

## 概述

一些組織要求開發工具僅在批准的地理區域內運行。PRECC的地理圍欄功能驗證當前機器的IP地址是否在允許的區域列表中。

## 檢查合規性

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

如果機器在允許的區域之外：

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## 刷新地理圍欄數據

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## 查看地理圍欄信息

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

## 清除緩存

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## 配置

地理圍欄策略在 `~/.config/precc/geofence.toml` 中定義：

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

設置 `block_on_violation = true` 以阻止PRECC在允許區域外運行。
