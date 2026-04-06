# Instalace

## Rychlá instalace (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Toto stáhne nejnovější binární soubor vydání pro vaši platformu, ověří SHA256 kontrolní součet a umístí ho do `~/.local/bin/`.

Po instalaci inicializujte PRECC:

```bash
precc init
```

`precc init` zaregistruje PreToolUse hook v Claude Code, vytvoří datové adresáře a inicializuje databázi dovedností.

## Možnosti instalace

### Ověření SHA256

Ve výchozím nastavení instalátor ověřuje kontrolní součet binárního souboru proti publikovanému SHA256 součtu. Pro přeskočení ověření (nedoporučuje se):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Vlastní prefix instalace

Instalace na vlastní umístění:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Doplňkové nástroje (--extras)

PRECC je dodáván s volitelnými doplňkovými nástroji. Nainstalujte je pomocí `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Toto nainstaluje:

| Nástroj | Účel |
|------|---------|
| **RTK** | Sada nástrojů pro přepis příkazů |
| **lean-ctx** | Komprese kontextu pro soubory CLAUDE.md a prompt |
| **nushell** | Strukturovaný shell pro pokročilé pipeline |
| **cocoindex-code** | Indexování kódu pro rychlejší rozlišení kontextu |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Poté inicializujte:

```powershell
precc init
```

## Ruční instalace

1. Stáhněte binární soubor vydání pro vaši platformu z [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Ověřte SHA256 kontrolní součet proti souboru `.sha256` ve vydání.
3. Umístěte binární soubor do adresáře ve vašem `PATH` (např. `~/.local/bin/`).
4. Spusťte `precc init`.

## Aktualizace

```bash
precc update
```

Vynuťte aktualizaci na konkrétní verzi:

```bash
precc update --force --version 0.3.0
```

Povolte automatické aktualizace:

```bash
precc update --auto
```

## Ověření instalace

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Pokud `precc` není nalezen, ujistěte se, že `~/.local/bin` je ve vašem `PATH`.
