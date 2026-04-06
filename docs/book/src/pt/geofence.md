# Geocerca

PRECC inclui verificação de conformidade de geocerca IP para ambientes regulados. Este é um recurso Pro.

## Visão geral

Algumas organizações exigem que ferramentas de desenvolvimento operem apenas dentro de regiões geográficas aprovadas. O recurso de geocerca do PRECC verifica se o endereço IP da máquina atual está dentro de uma lista de regiões permitidas.

## Verificando conformidade

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Se a máquina estiver fora das regiões permitidas:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Atualizando dados de geocerca

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Visualizando informações de geocerca

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

## Limpando cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configuração

A política de geocerca é definida em `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Defina `block_on_violation = true` para impedir que PRECC opere fora das regiões permitidas.
