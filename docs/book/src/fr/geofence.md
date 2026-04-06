# Géorepérage

PRECC inclut la vérification de conformité de géorepérage IP pour les environnements réglementés. C'est une fonctionnalité Pro.

## Vue d'ensemble

Certaines organisations exigent que les outils de développement ne fonctionnent que dans des régions géographiques approuvées. La fonctionnalité de géorepérage de PRECC vérifie que l'adresse IP de la machine actuelle se trouve dans une liste de régions autorisées.

## Vérification de la conformité

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Si la machine est en dehors des régions autorisées :

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Actualisation des données de géorepérage

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Affichage des informations de géorepérage

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

## Vider le cache

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Configuration

La politique de géorepérage est définie dans `~/.config/precc/geofence.toml` :

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Définissez `block_on_violation = true` pour empêcher PRECC de fonctionner en dehors des régions autorisées.
