# Télémétrie

PRECC prend en charge la télémétrie anonyme optionnelle pour aider à améliorer l'outil. Aucune donnée n'est collectée sans votre consentement explicite.

## Activer

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Désactiver

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Vérifier le statut

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Aperçu des données qui seraient envoyées

Avant d'activer, vous pouvez voir exactement quelles données seraient collectées :

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Ce qui est collecté

- Version de PRECC, système d'exploitation et architecture
- Compteurs agrégés : commandes interceptées, compétences activées, piliers utilisés
- Latence moyenne du hook
- Nombre de sessions

## Ce qui N'est PAS collecté

- Pas de texte de commande ni d'arguments
- Pas de chemins de fichiers ni de noms de répertoires
- Pas de noms de projets ni d'URLs de dépôts
- Aucune information personnellement identifiable (PII)
- Pas d'adresses IP (le serveur ne les enregistre pas)

## Remplacement par variable d'environnement

Pour désactiver la télémétrie sans exécuter de commande (utile en CI ou environnements partagés) :

```bash
export PRECC_NO_TELEMETRY=1
```

Ceci a priorité sur le paramètre de consentement.

## Destination des données

Les données de télémétrie sont envoyées à `https://telemetry.peria.ai/v1/precc` via HTTPS. Les données sont utilisées uniquement pour comprendre les schémas d'utilisation et prioriser le développement.
