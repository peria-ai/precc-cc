# Licens

PRECC tilbyder to niveauer: Community (gratis) og Pro.

## Community-niveau (gratis)

Community-niveauet inkluderer:

- Alle indbyggede færdigheder (mappekorrektion, jj-oversættelse osv.)
- Hook pipeline med fuld Pillar 1 og Pillar 4 understøttelse
- Grundlæggende `precc savings`-oversigt
- Sessionsanalyse med `precc ingest`
- Ubegrænset lokal brug

## Pro-niveau

Pro låser op for yderligere funktioner:

- **Detaljeret besparelsesoversigt** -- `precc savings --all` med per-kommando-analyse
- **GIF-optagelse** -- `precc gif` til oprettelse af animerede terminal-GIF'er
- **IP-geofence-overholdelse** -- Til regulerede miljøer
- **E-mail-rapporter** -- `precc mail report` til afsendelse af analyser
- **GitHub Actions-analyse** -- `precc gha` til fejlfinding af fejlede workflows
- **Kontekstkomprimering** -- `precc compress` til optimering af CLAUDE.md
- **Prioriteret support**

## Aktivering af licens

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Kontrol af licensstatus

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors-aktivering

Hvis du sponsorerer PRECC via GitHub Sponsors, aktiveres din licens automatisk via din GitHub-e-mail. Ingen nøgle nødvendig -- sørg bare for at din sponsor-e-mail matcher:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Enhedsfingeraftryk

Hver licens er bundet til et enhedsfingeraftryk. Se dit med:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Hvis du har brug for at overføre din licens til en ny maskine, deaktiver først:

```bash
precc license deactivate
```

Aktiver derefter på den nye maskine.

## Licens udløbet?

Når en Pro-licens udløber, vender PRECC tilbage til Community-niveauet. Alle indbyggede færdigheder og kernefunktionalitet fortsætter med at virke. Kun Pro-specifikke funktioner bliver utilgængelige. Se [FAQ](faq.md) for flere detaljer.
