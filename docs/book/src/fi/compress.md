# Pakkaus

`precc compress` pienentää CLAUDE.md:n ja muut kontekstitiedostot tokenien käytön vähentämiseksi, kun Claude Code lataa ne. Tämä on Pro-ominaisuus.

## Peruskäyttö

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## Testiajo

Esikatsele muutoksia muokkaamatta tiedostoja:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Palautus

Alkuperäiset varmuuskopioidaan automaattisesti. Palauttaaksesi ne:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Mitä pakataan

Pakkaaja soveltaa useita muunnoksia:

- Poistaa ylimääräiset välilyönnit ja tyhjät rivit
- Lyhentää monisanaisia ilmaisuja merkityksen säilyttäen
- Tiivistää taulukot ja listat
- Poistaa kommentit ja koristeellisen muotoilun
- Säilyttää kaikki koodilohkot, polut ja tekniset tunnisteet

Pakattu tuloste on edelleen ihmisen luettavissa -- sitä ei ole minifoitu tai obfuskoitu.

## Tietyt tiedostot

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
