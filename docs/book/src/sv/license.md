# Licens

PRECC erbjuder två nivåer: Community (gratis) och Pro.

## Community-nivå (gratis)

Community-nivån inkluderar:

- Alla inbyggda färdigheter (katalogkorrigering, jj-översättning etc.)
- Hook pipeline med fullt Pillar 1 och Pillar 4 stöd
- Grundläggande `precc savings`-sammanfattning
- Sessionsanalys med `precc ingest`
- Obegränsad lokal användning

## Pro-nivå

Pro låser upp ytterligare funktioner:

- **Detaljerad besparingsuppdelning** -- `precc savings --all` med per-kommando-analys
- **GIF-inspelning** -- `precc gif` för att skapa animerade terminal-GIF:ar
- **IP-geofence-efterlevnad** -- För reglerade miljöer
- **E-postrapporter** -- `precc mail report` för att skicka analyser
- **GitHub Actions-analys** -- `precc gha` för felsökning av misslyckade arbetsflöden
- **Kontextkomprimering** -- `precc compress` för optimering av CLAUDE.md
- **Prioriterad support**

## Aktivera licens

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Kontrollera licensstatus

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors-aktivering

Om du sponsrar PRECC via GitHub Sponsors aktiveras din licens automatiskt via din GitHub-e-post. Ingen nyckel krävs -- se bara till att din sponsor-e-post matchar:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Enhetsfingeravtryck

Varje licens är bunden till ett enhetsfingeravtryck. Visa ditt med:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Om du behöver överföra din licens till en ny maskin, avaktivera först:

```bash
precc license deactivate
```

Aktivera sedan på den nya maskinen.

## Licens utgången?

När en Pro-licens löper ut återgår PRECC till Community-nivån. Alla inbyggda färdigheter och kärnfunktionalitet fortsätter att fungera. Bara Pro-specifika funktioner blir otillgängliga. Se [FAQ](faq.md) för mer detaljer.
