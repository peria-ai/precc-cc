# Uppsetning

## Hröð uppsetning (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Þetta sækir nýjustu keyrsluskrá útgáfunnar fyrir þinn vettvang, staðfestir SHA256-gáttalykil og setur hana í `~/.local/bin/`.

Eftir uppsetningu, ræstu PRECC:

```bash
precc init
```

`precc init` skráir PreToolUse hook í Claude Code, býr til gagnamöppur og frumstillir þekkingargagnagrunn.

## Uppsetningarvalkostir

### SHA256-staðfesting

Sjálfgefið staðfestir uppsetningarforritið gáttalykil keyrsluskráar á móti opinberum SHA256-lykli. Til að sleppa staðfestingu (ekki ráðlagt):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Sérsniðið uppsetningarforskeyti

Setja upp á sérsniðinni staðsetningu:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Viðbótartól (--extras)

PRECC fylgir valfrjáls viðbótartól. Settu þau upp með `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Þetta setur upp:

| Tól | Tilgangur |
|------|---------|
| **RTK** | Skipanaendurskrifunartól |
| **lean-ctx** | Samhengsþjöppun fyrir CLAUDE.md og prompt skrár |
| **nushell** | Skipulagt skel fyrir framhaldsröðir |
| **cocoindex-code** | Kóðavísitala fyrir hraðari samhengisskil |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Ræstu síðan:

```powershell
precc init
```

## Handvirk uppsetning

1. Sæktu keyrsluskrá útgáfunnar fyrir þinn vettvang frá [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Staðfestu SHA256-gáttalykil á móti `.sha256`-skránni í útgáfunni.
3. Settu keyrsluskrána í möppu á `PATH` (t.d. `~/.local/bin/`).
4. Keyrðu `precc init`.

## Uppfærsla

```bash
precc update
```

Þvinga uppfærslu í ákveðna útgáfu:

```bash
precc update --force --version 0.3.0
```

Virkja sjálfvirkar uppfærslur:

```bash
precc update --auto
```

## Staðfesting uppsetningar

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Ef `precc` finnst ekki, gakktu úr skugga um að `~/.local/bin` sé á `PATH`.
