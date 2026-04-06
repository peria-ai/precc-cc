# Installazione

## Installazione rapida (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Questo scarica l'ultimo binario della release per la tua piattaforma, verifica il checksum SHA256 e lo posiziona in `~/.local/bin/`.

Dopo l'installazione, inizializza PRECC:

```bash
precc init
```

`precc init` registra il PreToolUse hook con Claude Code, crea le directory dati e inizializza il database delle skill.

## Opzioni di installazione

### Verifica SHA256

Per impostazione predefinita, l'installer verifica il checksum del binario rispetto alla somma SHA256 pubblicata. Per saltare la verifica (non raccomandato):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Prefisso di installazione personalizzato

Installa in una posizione personalizzata:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Strumenti companion (--extras)

PRECC include strumenti companion opzionali. Installali con `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Questo installa:

| Strumento | Scopo |
|------|---------|
| **RTK** | Toolkit di riscrittura comandi |
| **lean-ctx** | Compressione del contesto per file CLAUDE.md e prompt |
| **nushell** | Shell strutturata per pipeline avanzate |
| **cocoindex-code** | Indicizzazione del codice per una risoluzione del contesto più veloce |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Poi inizializza:

```powershell
precc init
```

## Installazione manuale

1. Scarica il binario della release per la tua piattaforma da [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verifica il checksum SHA256 rispetto al file `.sha256` nella release.
3. Posiziona il binario in una directory nel tuo `PATH` (es. `~/.local/bin/`).
4. Esegui `precc init`.

## Aggiornamento

```bash
precc update
```

Forza l'aggiornamento a una versione specifica:

```bash
precc update --force --version 0.3.0
```

Abilita gli aggiornamenti automatici:

```bash
precc update --auto
```

## Verifica dell'installazione

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Se `precc` non viene trovato, assicurati che `~/.local/bin` sia nel tuo `PATH`.
