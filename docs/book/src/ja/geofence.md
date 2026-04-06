# ジオフェンス

PRECCは規制環境向けのIPジオフェンスコンプライアンスチェックを含みます。これはPro機能です。

## 概要

一部の組織は、開発ツールが承認された地理的リージョン内でのみ動作することを要求しています。PRECCのジオフェンス機能は、現在のマシンのIPアドレスが許可されたリージョンリスト内にあることを検証します。

## コンプライアンスチェック

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

マシンが許可されたリージョンの外にある場合：

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## ジオフェンスデータの更新

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## ジオフェンス情報の表示

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

## キャッシュのクリア

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## 設定

ジオフェンスポリシーは `~/.config/precc/geofence.toml` で定義されます：

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

`block_on_violation = true` を設定すると、許可されたリージョン外でPRECCの動作を防止します。
