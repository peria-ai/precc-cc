# Licentie

PRECC biedt twee niveaus: Community (gratis) en Pro.

## Community-niveau (gratis)

Het Community-niveau omvat:

- Alle ingebouwde vaardigheden (directoryfouten, jj-vertaling, enz.)
- Hook-pipeline met volledige Pillar 1- en Pillar 4-ondersteuning
- Basis `precc savings` samenvatting
- Sessiemining met `precc ingest`
- Onbeperkt lokaal gebruik

## Pro-niveau

Pro ontgrendelt extra functies:

- **Gedetailleerd besparingsoverzicht** -- `precc savings --all` met analyse per commando
- **GIF-opname** -- `precc gif` voor het maken van geanimeerde terminal-GIF's
- **IP-geofence compliance** -- Voor gereguleerde omgevingen
- **E-mailrapporten** -- `precc mail report` om analyses te versturen
- **GitHub Actions-analyse** -- `precc gha` voor foutopsporing in mislukte workflows
- **Contextcompressie** -- `precc compress` voor CLAUDE.md-optimalisatie
- **Prioriteitsondersteuning**

## Een licentie activeren

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Licentiestatus controleren

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors-activering

Als u PRECC sponsort via GitHub Sponsors, wordt uw licentie automatisch geactiveerd via uw GitHub-e-mail. Geen sleutel nodig -- zorg er alleen voor dat uw sponsor-e-mail overeenkomt:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Apparaat-vingerafdruk

Elke licentie is gekoppeld aan een apparaat-vingerafdruk. Bekijk de uwe met:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Als u uw licentie naar een nieuwe machine wilt overzetten, deactiveer dan eerst:

```bash
precc license deactivate
```

Activeer vervolgens op de nieuwe machine.

## Licentie verlopen?

Wanneer een Pro-licentie verloopt, keert PRECC terug naar het Community-niveau. Alle ingebouwde vaardigheden en kernfunctionaliteit blijven werken. Alleen Pro-specifieke functies worden onbeschikbaar. Zie de [FAQ](faq.md) voor meer details.
