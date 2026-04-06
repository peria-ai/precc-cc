# Installation

## Hurtig installation (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Dette downloader den seneste release-binærfil til din platform, verificerer SHA256-checksummen og placerer den i `~/.local/bin/`.

Efter installation, initialiser PRECC:

```bash
precc init
```

`precc init` registrerer PreToolUse hook i Claude Code, opretter datamapper og initialiserer færdighedsdatabasen.

## Installationsmuligheder

### SHA256-verificering

Som standard verificerer installationsprogrammet binærens checksum mod den publicerede SHA256-sum. For at springe verificering over (anbefales ikke):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Brugerdefineret installationspræfiks

Installer til en brugerdefineret placering:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Ledsagerværktøjer (--extras)

PRECC leveres med valgfri ledsagerværktøjer. Installer dem med `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Dette installerer:

| Værktøj | Formål |
|------|---------|
| **RTK** | Kommandoomskrivningsværktøj |
| **lean-ctx** | Kontekstkomprimering for CLAUDE.md- og prompt-filer |
| **nushell** | Struktureret shell til avancerede pipelines |
| **cocoindex-code** | Kodeindeksering for hurtigere kontekstopløsning |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Derefter initialiser:

```powershell
precc init
```

## Manuel installation

1. Download release-binærfilen til din platform fra [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verificer SHA256-checksummen mod `.sha256`-filen i udgivelsen.
3. Placer binærfilen i en mappe i din `PATH` (f.eks. `~/.local/bin/`).
4. Kør `precc init`.

## Opdatering

```bash
precc update
```

Tving opdatering til en specifik version:

```bash
precc update --force --version 0.3.0
```

Aktiver automatiske opdateringer:

```bash
precc update --auto
```

## Verifikation af installation

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Hvis `precc` ikke findes, sørg for at `~/.local/bin` er i din `PATH`.
