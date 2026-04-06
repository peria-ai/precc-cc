# 地理围栏

PRECC包含用于受监管环境的IP地理围栏合规性检查。这是Pro功能。

## 概述

一些组织要求开发工具仅在批准的地理区域内运行。PRECC的地理围栏功能验证当前机器的IP地址是否在允许的区域列表中。

## 检查合规性

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

如果机器在允许的区域之外：

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## 刷新地理围栏数据

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## 查看地理围栏信息

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

## 清除缓存

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## 配置

地理围栏策略在 `~/.config/precc/geofence.toml` 中定义：

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

设置 `block_on_violation = true` 以阻止PRECC在允许区域外运行。
