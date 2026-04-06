# Fjarmæling

PRECC styður opt-in nafnlausa fjarmælingu til að bæta tólið. Engin gögn eru safnað nema þú samþykkir sérstaklega.

## Skráning

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Afskráning

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Stöðuathugun

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Forskoðun gagna sem yrðu send

Áður en þú skráir þig geturðu séð nákvæmlega hvaða gögn yrðu safnað:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Hvað er safnað

- PRECC útgáfa, stýrikerfi og arkitektúr
- Samanlögð teljari: fangaðar skipanir, virkjuð þekking, notaðir stoðar
- Meðal hook-leynd
- Lotufjöldi

## Hvað er EKKI safnað

- Enginn skipunatexti eða viðföng
- Engar skráarslóðir eða möppuheiti
- Engin verkefnaheiti eða geymslu-URL
- Engar persónugreinanlegar upplýsingar (PII)
- Engin IP-tölur (þjónninn skráir þær ekki)

## Umhverfisbreyta-yfirstjórnun

Til að slökkva á fjarmælingu án þess að keyra skipun (gagnlegt í CI eða samnýttum umhverfum):

```bash
export PRECC_NO_TELEMETRY=1
```

Þetta hefur forgang yfir samþykkisstillinguna.

## Gagnaafsetur

Fjarmælingargögn eru send á `https://telemetry.peria.ai/v1/precc` yfir HTTPS. Gögnin eru einungis notuð til að skilja notkunarmynstur og forgangsraða þróun.
