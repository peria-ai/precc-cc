# Installation

## Installation rapide (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Cela télécharge le dernier binaire pour votre plateforme, vérifie la somme de contrôle SHA256 et le place dans `~/.local/bin/`.

Après l'installation, initialisez PRECC :

```bash
precc init
```

`precc init` enregistre le hook PreToolUse avec Claude Code, crée les répertoires de données et initialise la base de données des compétences.

## Options d'installation

### Vérification SHA256

Par défaut, l'installateur vérifie la somme de contrôle du binaire par rapport à la somme SHA256 publiée. Pour ignorer la vérification (non recommandé) :

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Préfixe d'installation personnalisé

Installer dans un emplacement personnalisé :

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Outils compagnons (--extras)

PRECC est livré avec des outils compagnons optionnels. Installez-les avec `--extras` :

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Cela installe :

| Outil | Fonction |
|------|---------|
| **RTK** | Kit de réécriture de commandes |
| **lean-ctx** | Compression de contexte pour CLAUDE.md et les fichiers de prompt |
| **nushell** | Shell structuré pour les pipelines avancés |
| **cocoindex-code** | Indexation du code pour une résolution de contexte plus rapide |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Puis initialisez :

```powershell
precc init
```

## Installation manuelle

1. Téléchargez le binaire pour votre plateforme depuis [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Vérifiez la somme de contrôle SHA256 par rapport au fichier `.sha256` de la version.
3. Placez le binaire dans un répertoire de votre `PATH` (par ex., `~/.local/bin/`).
4. Exécutez `precc init`.

## Mise à jour

```bash
precc update
```

Forcer la mise à jour vers une version spécifique :

```bash
precc update --force --version 0.3.0
```

Activer les mises à jour automatiques :

```bash
precc update --auto
```

## Vérification de l'installation

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Si `precc` n'est pas trouvé, assurez-vous que `~/.local/bin` est dans votre `PATH`.
