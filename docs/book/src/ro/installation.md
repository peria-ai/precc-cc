# Instalare

## Instalare rapidă (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Aceasta descarcă cel mai recent binar release pentru platforma dvs., verifică suma de control SHA256 și îl plasează în `~/.local/bin/`.

După instalare, inițializați PRECC:

```bash
precc init
```

`precc init` înregistrează hook-ul PreToolUse în Claude Code, creează directoarele de date și inițializează baza de date a abilităților.

## Opțiuni de instalare

### Verificare SHA256

Implicit, programul de instalare verifică suma de control a binarului față de suma SHA256 publicată. Pentru a sări verificarea (nerecomandat):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Prefix de instalare personalizat

Instalați într-o locație personalizată:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Instrumente companion (--extras)

PRECC vine cu instrumente companion opționale. Instalați-le cu `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Aceasta instalează:

| Instrument | Scop |
|------|---------|
| **RTK** | Set de instrumente pentru rescrierea comenzilor |
| **lean-ctx** | Comprimare context pentru fișierele CLAUDE.md și prompt |
| **nushell** | Shell structurat pentru pipeline-uri avansate |
| **cocoindex-code** | Indexare cod pentru rezolvare mai rapidă a contextului |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Apoi inițializați:

```powershell
precc init
```

## Instalare manuală

1. Descărcați binarul release pentru platforma dvs. de pe [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verificați suma de control SHA256 față de fișierul `.sha256` din release.
3. Plasați binarul într-un director din `PATH` (de ex., `~/.local/bin/`).
4. Rulați `precc init`.

## Actualizare

```bash
precc update
```

Forțați actualizarea la o versiune specifică:

```bash
precc update --force --version 0.3.0
```

Activați actualizările automate:

```bash
precc update --auto
```

## Verificarea instalării

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Dacă `precc` nu este găsit, asigurați-vă că `~/.local/bin` este în `PATH`.
