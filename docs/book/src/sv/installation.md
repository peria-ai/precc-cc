# Installation

## Snabbinstallation (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Detta laddar ner den senaste releasebinärfilen för din plattform, verifierar SHA256-checksumman och placerar den i `~/.local/bin/`.

Efter installation, initiera PRECC:

```bash
precc init
```

`precc init` registrerar PreToolUse hook i Claude Code, skapar datakataloger och initierar färdighetsdatabasen.

## Installationsalternativ

### SHA256-verifiering

Som standard verifierar installationsprogrammet binärens checksumma mot den publicerade SHA256-summan. För att hoppa över verifiering (rekommenderas inte):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Anpassat installationsprefix

Installera till en anpassad plats:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Kompletteringsverktyg (--extras)

PRECC levereras med valfria kompletteringsverktyg. Installera dem med `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Detta installerar:

| Verktyg | Syfte |
|------|---------|
| **RTK** | Kommandomskrivningsverktyg |
| **lean-ctx** | Kontextkomprimering för CLAUDE.md och promptfiler |
| **nushell** | Strukturerat skal för avancerade pipelines |
| **cocoindex-code** | Kodindexering för snabbare kontextupplösning |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Initiera sedan:

```powershell
precc init
```

## Manuell installation

1. Ladda ner releasebinärfilen för din plattform från [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verifiera SHA256-checksumman mot `.sha256`-filen i releasen.
3. Placera binärfilen i en katalog i din `PATH` (t.ex. `~/.local/bin/`).
4. Kör `precc init`.

## Uppdatering

```bash
precc update
```

Tvinga uppdatering till en specifik version:

```bash
precc update --force --version 0.3.0
```

Aktivera automatiska uppdateringar:

```bash
precc update --auto
```

## Verifiering av installation

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Om `precc` inte hittas, se till att `~/.local/bin` finns i din `PATH`.
