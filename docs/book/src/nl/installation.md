# Installatie

## Snelle installatie (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Dit downloadt de nieuwste release-binary voor uw platform, verifieert de SHA256-checksum en plaatst deze in `~/.local/bin/`.

Initialiseer PRECC na de installatie:

```bash
precc init
```

`precc init` registreert de PreToolUse-hook bij Claude Code, maakt de gegevensmappen aan en initialiseert de skills-database.

## Installatieopties

### SHA256-verificatie

Standaard verifieert het installatieprogramma de binaire checksum tegen de gepubliceerde SHA256-som. Om verificatie over te slaan (niet aanbevolen):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Aangepast installatieprefix

Installeren op een aangepaste locatie:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Bijkomende tools (--extras)

PRECC wordt geleverd met optionele bijkomende tools. Installeer ze met `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Dit installeert:

| Tool | Doel |
|------|---------|
| **RTK** | Toolkit voor herschrijven van commando's |
| **lean-ctx** | Contextcompressie voor CLAUDE.md en promptbestanden |
| **nushell** | Gestructureerde shell voor geavanceerde pipelines |
| **cocoindex-code** | Code-indexering voor snellere contextresolutie |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Vervolgens initialiseren:

```powershell
precc init
```

## Handmatige installatie

1. Download de release-binary voor uw platform van [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verifieer de SHA256-checksum tegen het `.sha256`-bestand in de release.
3. Plaats de binary in een map op uw `PATH` (bijv. `~/.local/bin/`).
4. Voer `precc init` uit.

## Bijwerken

```bash
precc update
```

Forceer een update naar een specifieke versie:

```bash
precc update --force --version 0.3.0
```

Automatische updates inschakelen:

```bash
precc update --auto
```

## Installatie verifiëren

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Als `precc` niet wordt gevonden, zorg ervoor dat `~/.local/bin` op uw `PATH` staat.
