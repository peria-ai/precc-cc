# Compétences

Les compétences sont les règles de correspondance de motifs que PRECC utilise pour détecter et corriger les commandes. Elles peuvent être intégrées (livrées en fichiers TOML) ou extraites des journaux de session.

## Compétences intégrées

| Compétence | Déclencheur | Action |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` en dehors d'un projet Rust | Ajouter `cd` vers le répertoire `Cargo.toml` le plus proche |
| `git-wrong-dir` | `git *` en dehors d'un dépôt git | Ajouter `cd` vers le répertoire `.git` le plus proche |
| `go-wrong-dir` | `go build/test` en dehors d'un module Go | Ajouter `cd` vers le répertoire `go.mod` le plus proche |
| `make-wrong-dir` | `make` sans Makefile dans le répertoire courant | Ajouter `cd` vers le répertoire Makefile le plus proche |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` en dehors d'un projet Node | Ajouter `cd` vers le répertoire `package.json` le plus proche |
| `python-wrong-dir` | `python/pytest/pip` en dehors d'un projet Python | Ajouter `cd` vers le projet Python le plus proche |
| `jj-translate` | `git *` dans un dépôt jj colocalisé | Réécrire en commande `jj` équivalente |
| `asciinema-gif` | `asciinema rec` | Réécrire en `precc gif` |

## Lister les compétences

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
  9 fix-pytest-path    mined     pytest with wrong test path
```

## Afficher les détails d'une compétence

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## Exporter une compétence en TOML

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## Modifier une compétence

```bash
$ precc skills edit cargo-wrong-dir
```

Cela ouvre la définition de la compétence dans votre `$EDITOR`. Après sauvegarde, la compétence est rechargée automatiquement.

## La commande Advise

`precc skills advise` analyse votre session récente et suggère de nouvelles compétences basées sur des motifs répétés :

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## Regrouper les compétences

```bash
$ precc skills cluster
```

Regroupe les compétences extraites similaires pour aider à identifier les motifs redondants ou chevauchants.

## Compétences extraites vs. intégrées

Les compétences intégrées sont livrées avec PRECC et définies dans `skills/builtin/*.toml`. Elles couvrent les erreurs de mauvais répertoire les plus courantes.

Les compétences extraites sont créées par `precc ingest` ou le démon `precc-learner` à partir de vos journaux de session. Elles sont stockées dans `~/.local/share/precc/heuristics.db` et sont spécifiques à votre flux de travail. Voir [Extraction](mining.md) pour les détails.
