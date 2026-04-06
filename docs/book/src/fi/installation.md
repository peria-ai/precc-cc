# Asennus

## Pika-asennus (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Tämä lataa uusimman julkaisun binäärin alustallesi, vahvistaa SHA256-tarkistussumman ja sijoittaa sen hakemistoon `~/.local/bin/`.

Asennuksen jälkeen alusta PRECC:

```bash
precc init
```

`precc init` rekisteröi PreToolUse hookin Claude Codeen, luo datahakemistot ja alustaa taitotietokannan.

## Asennusvaihtoehdot

### SHA256-vahvistus

Oletuksena asennusohjelma vahvistaa binäärin tarkistussumman julkaistua SHA256-summaa vasten. Ohittaaksesi vahvistuksen (ei suositella):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Mukautettu asennusetuliite

Asenna mukautettuun sijaintiin:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Lisätyökalut (--extras)

PRECC toimitetaan valinnaisten lisätyökalujen kanssa. Asenna ne `--extras`-lipulla:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Tämä asentaa:

| Työkalu | Tarkoitus |
|------|---------|
| **RTK** | Komentojen uudelleenkirjoitustyökalu |
| **lean-ctx** | Kontekstin pakkaus CLAUDE.md- ja prompt-tiedostoille |
| **nushell** | Rakenteellinen kuori edistyneille putkille |
| **cocoindex-code** | Koodin indeksointi nopeampaa kontekstin ratkaisua varten |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Sitten alusta:

```powershell
precc init
```

## Manuaalinen asennus

1. Lataa julkaisun binääri alustallesi [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) -sivulta.
2. Vahvista SHA256-tarkistussumma julkaisun `.sha256`-tiedostoa vasten.
3. Sijoita binääri hakemistoon `PATH`-muuttujassasi (esim. `~/.local/bin/`).
4. Suorita `precc init`.

## Päivitys

```bash
precc update
```

Pakota päivitys tiettyyn versioon:

```bash
precc update --force --version 0.3.0
```

Ota käyttöön automaattiset päivitykset:

```bash
precc update --auto
```

## Asennuksen vahvistus

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Jos `precc` ei löydy, varmista että `~/.local/bin` on `PATH`-muuttujassasi.
