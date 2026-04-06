# Telepítés

## Gyors telepítés (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Ez letölti a legújabb kiadás binárisát az Ön platformjára, ellenőrzi a SHA256 ellenőrző összeget, és a `~/.local/bin/` könyvtárba helyezi.

A telepítés után inicializálja a PRECC-et:

```bash
precc init
```

A `precc init` regisztrálja a PreToolUse hookot a Claude Code-ban, létrehozza az adatkönyvtárakat és inicializálja a készségadatbázist.

## Telepítési lehetőségek

### SHA256 ellenőrzés

Alapértelmezés szerint a telepítő ellenőrzi a bináris ellenőrző összegét a közzétett SHA256 összeg alapján. Az ellenőrzés kihagyásához (nem ajánlott):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Egyéni telepítési előtag

Telepítés egyéni helyre:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Kiegészítő eszközök (--extras)

A PRECC opcionális kiegészítő eszközökkel érkezik. Telepítse őket a `--extras` kapcsolóval:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Ez a következőket telepíti:

| Eszköz | Cél |
|------|---------|
| **RTK** | Parancs-újraíró eszközkészlet |
| **lean-ctx** | Kontextus tömörítés CLAUDE.md és prompt fájlokhoz |
| **nushell** | Strukturált shell fejlett pipeline-okhoz |
| **cocoindex-code** | Kódindexelés a gyorsabb kontextusfeloldáshoz |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Ezután inicializálja:

```powershell
precc init
```

## Kézi telepítés

1. Töltse le a kiadás binárisát platformjához a [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) oldalról.
2. Ellenőrizze a SHA256 ellenőrző összeget a kiadás `.sha256` fájljával.
3. Helyezze a binárist a `PATH`-on lévő könyvtárba (pl. `~/.local/bin/`).
4. Futtassa a `precc init` parancsot.

## Frissítés

```bash
precc update
```

Kényszerített frissítés egy adott verzióra:

```bash
precc update --force --version 0.3.0
```

Automatikus frissítések engedélyezése:

```bash
precc update --auto
```

## Telepítés ellenőrzése

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Ha a `precc` nem található, győződjön meg arról, hogy a `~/.local/bin` rajta van a `PATH`-on.
