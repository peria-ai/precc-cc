# Installation

## Schnellinstallation (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Dies lädt die neueste Release-Binary für Ihre Plattform herunter, verifiziert die SHA256-Prüfsumme und platziert sie in `~/.local/bin/`.

Nach der Installation initialisieren Sie PRECC:

```bash
precc init
```

`precc init` registriert den PreToolUse-Hook bei Claude Code, erstellt die Datenverzeichnisse und initialisiert die Skills-Datenbank.

## Installationsoptionen

### SHA256-Verifizierung

Standardmäßig verifiziert der Installer die Binär-Prüfsumme gegen die veröffentlichte SHA256-Summe. Um die Verifizierung zu überspringen (nicht empfohlen):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Benutzerdefiniertes Installationspräfix

In ein benutzerdefiniertes Verzeichnis installieren:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Begleit-Tools (--extras)

PRECC wird mit optionalen Begleit-Tools ausgeliefert. Installieren Sie diese mit `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Dies installiert:

| Tool | Zweck |
|------|---------|
| **RTK** | Befehlsumschreibungs-Toolkit |
| **lean-ctx** | Kontextkomprimierung für CLAUDE.md und Prompt-Dateien |
| **nushell** | Strukturierte Shell für erweiterte Pipelines |
| **cocoindex-code** | Code-Indexierung für schnellere Kontextauflösung |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Dann initialisieren:

```powershell
precc init
```

## Manuelle Installation

1. Laden Sie die Release-Binary für Ihre Plattform von [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) herunter.
2. Verifizieren Sie die SHA256-Prüfsumme anhand der `.sha256`-Datei im Release.
3. Platzieren Sie die Binary in einem Verzeichnis in Ihrem `PATH` (z.B. `~/.local/bin/`).
4. Führen Sie `precc init` aus.

## Aktualisierung

```bash
precc update
```

Erzwinge ein Update auf eine bestimmte Version:

```bash
precc update --force --version 0.3.0
```

Automatische Updates aktivieren:

```bash
precc update --auto
```

## Installation überprüfen

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Wenn `precc` nicht gefunden wird, stellen Sie sicher, dass `~/.local/bin` in Ihrem `PATH` ist.
