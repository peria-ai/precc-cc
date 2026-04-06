# Leyfi

PRECC býður upp á tvö stig: Community (ókeypis) og Pro.

## Community-stig (ókeypis)

Community-stigið inniheldur:

- Öll innbyggð þekking (möppuleiðrétting, jj-þýðing o.fl.)
- Hook pipeline með fullum Pillar 1 og Pillar 4 stuðningi
- Grunn `precc savings` yfirlit
- Lotugreining með `precc ingest`
- Ótakmörkuð staðbundin notkun

## Pro-stig

Pro opnar viðbótareiginleika:

- **Ítarleg sparnaðarsundurliðun** -- `precc savings --all` með skipanagreiningu
- **GIF-upptaka** -- `precc gif` til að búa til hreyfimynda-GIF af skjástöðvu
- **IP-landsvæðaeftirlit** -- Fyrir stýrð umhverfi
- **Tölvupóstsskýrslur** -- `precc mail report` til að senda greiningar
- **GitHub Actions greining** -- `precc gha` til kembi misheppnaðra verkflæða
- **Samhengsþjöppun** -- `precc compress` til hagræðingar á CLAUDE.md
- **Forgangsstuðningur**

## Virkjun leyfis

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Athugun á stöðu leyfis

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors virkjun

Ef þú styrkir PRECC í gegnum GitHub Sponsors er leyfið þitt virkjað sjálfkrafa í gegnum GitHub-netfangið þitt. Enginn lykill þarfnast -- passaðu bara að styrktarnetfangið þitt passi:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Fingrafar tækis

Hvert leyfi er bundið við fingrafar tækis. Skoðaðu þitt:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Ef þú þarft að flytja leyfið á nýja tölvu, afvirkjaðu fyrst:

```bash
precc license deactivate
```

Virkjaðu síðan á nýju tölvunni.

## Leyfi útrunnið?

Þegar Pro-leyfi rennur út snýr PRECC aftur í Community-stig. Öll innbyggð þekking og grunnvirkni heldur áfram að virka. Aðeins Pro-sérstakir eiginleikar verða óaðgengilegir. Sjá [FAQ](faq.md) fyrir frekari upplýsingar.
