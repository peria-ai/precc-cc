# Licence

PRECC propose deux niveaux : Community (gratuit) et Pro.

## Niveau Community (gratuit)

Le niveau Community comprend :

- Toutes les compétences intégrées (correction de répertoire, traduction jj, etc.)
- Pipeline de hooks avec support complet Pillar 1 et Pillar 4
- Résumé de base de `precc savings`
- Exploration de sessions avec `precc ingest`
- Utilisation locale illimitée

## Niveau Pro

Pro débloque des fonctionnalités supplémentaires :

- **Ventilation détaillée des économies** -- `precc savings --all` avec analyse par commande
- **Enregistrement GIF** -- `precc gif` pour créer des GIFs animés de terminal
- **Conformité géobarrière IP** -- Pour les environnements réglementés
- **Rapports par e-mail** -- `precc mail report` pour envoyer des analyses
- **Analyse GitHub Actions** -- `precc gha` pour le débogage des workflows échoués
- **Compression de contexte** -- `precc compress` pour l'optimisation de CLAUDE.md
- **Support prioritaire**

## Activer une licence

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Vérifier le statut de la licence

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Activation GitHub Sponsors

Si vous parrainez PRECC via GitHub Sponsors, votre licence est activée automatiquement via votre e-mail GitHub. Pas de clé requise -- assurez-vous simplement que votre e-mail de parrainage correspond :

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Empreinte de l'appareil

Chaque licence est liée à une empreinte d'appareil. Consultez la vôtre avec :

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Si vous devez transférer votre licence sur une nouvelle machine, désactivez d'abord :

```bash
precc license deactivate
```

Puis activez sur la nouvelle machine.

## Licence expirée ?

Lorsqu'une licence Pro expire, PRECC revient au niveau Community. Toutes les compétences intégrées et les fonctionnalités de base continuent de fonctionner. Seules les fonctionnalités Pro deviennent indisponibles. Voir la [FAQ](faq.md) pour plus de détails.
