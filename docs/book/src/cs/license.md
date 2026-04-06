# Licence

PRECC nabízí dvě úrovně: Community (zdarma) a Pro.

## Úroveň Community (zdarma)

Úroveň Community zahrnuje:

- Všechny vestavěné dovednosti (korekce adresáře, překlad jj atd.)
- Hook pipeline s plnou podporou Pillar 1 a Pillar 4
- Základní souhrn `precc savings`
- Analýzu relací pomocí `precc ingest`
- Neomezené lokální použití

## Úroveň Pro

Pro odemyká další funkce:

- **Podrobný rozpis úspor** -- `precc savings --all` s analýzou po příkazech
- **Nahrávání GIF** -- `precc gif` pro vytváření animovaných terminálových GIF
- **IP geofence compliance** -- Pro regulovaná prostředí
- **E-mailové zprávy** -- `precc mail report` pro odesílání analytik
- **Analýza GitHub Actions** -- `precc gha` pro ladění neúspěšných workflow
- **Komprese kontextu** -- `precc compress` pro optimalizaci CLAUDE.md
- **Prioritní podpora**

## Aktivace licence

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Kontrola stavu licence

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Aktivace GitHub Sponsors

Pokud sponzorujete PRECC přes GitHub Sponsors, vaše licence je aktivována automaticky přes váš GitHub e-mail. Žádný klíč není potřeba -- jen se ujistěte, že váš sponzorský e-mail odpovídá:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Otisk zařízení

Každá licence je vázána na otisk zařízení. Zobrazit svůj:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Pokud potřebujete přenést licenci na nový počítač, nejprve deaktivujte:

```bash
precc license deactivate
```

Poté aktivujte na novém počítači.

## Licence vypršela?

Když licence Pro vyprší, PRECC se vrátí na úroveň Community. Všechny vestavěné dovednosti a základní funkčnost fungují dál. Pouze specifické funkce Pro se stanou nedostupnými. Viz [FAQ](faq.md) pro podrobnosti.
