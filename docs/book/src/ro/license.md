# Licență

PRECC oferă două niveluri: Community (gratuit) și Pro.

## Nivelul Community (gratuit)

Nivelul Community include:

- Toate abilitățile integrate (corecția directorului, traducerea jj etc.)
- Hook pipeline cu suport complet Pillar 1 și Pillar 4
- Sumar de bază `precc savings`
- Analiza sesiunilor cu `precc ingest`
- Utilizare locală nelimitată

## Nivelul Pro

Pro deblochează funcții suplimentare:

- **Detaliere completă a economiilor** -- `precc savings --all` cu analiză per comandă
- **Înregistrare GIF** -- `precc gif` pentru crearea de GIF-uri animate de terminal
- **Conformitate geofence IP** -- Pentru medii reglementate
- **Rapoarte prin e-mail** -- `precc mail report` pentru trimiterea analiticilor
- **Analiză GitHub Actions** -- `precc gha` pentru depanarea fluxurilor de lucru eșuate
- **Comprimare context** -- `precc compress` pentru optimizarea CLAUDE.md
- **Suport prioritar**

## Activarea licenței

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Verificarea stării licenței

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Activare GitHub Sponsors

Dacă sponsorizați PRECC prin GitHub Sponsors, licența dvs. este activată automat prin e-mailul GitHub. Nu este necesară nicio cheie -- asigurați-vă doar că e-mailul de sponsor se potrivește:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Amprenta dispozitivului

Fiecare licență este legată de o amprentă a dispozitivului. Vizualizați-o cu:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Dacă trebuie să transferați licența pe o nouă mașină, dezactivați mai întâi:

```bash
precc license deactivate
```

Apoi activați pe noua mașină.

## Licență expirată?

Când o licență Pro expiră, PRECC revine la nivelul Community. Toate abilitățile integrate și funcționalitatea de bază continuă să funcționeze. Doar funcțiile specifice Pro devin indisponibile. Vedeți [FAQ](faq.md) pentru detalii.
