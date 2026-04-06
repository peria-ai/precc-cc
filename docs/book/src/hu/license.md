# Licenc

A PRECC két szintet kínál: Community (ingyenes) és Pro.

## Community szint (ingyenes)

A Community szint tartalmazza:

- Minden beépített képesség (könyvtárjavítás, jj fordítás stb.)
- Hook pipeline teljes Pillar 1 és Pillar 4 támogatással
- Alapszintű `precc savings` összefoglaló
- Munkamenet-bányászat a `precc ingest` segítségével
- Korlátlan helyi használat

## Pro szint

A Pro további funkciókat nyit meg:

- **Részletes megtakarítás-bontás** -- `precc savings --all` parancsonkénti elemzéssel
- **GIF felvétel** -- `precc gif` animált terminál GIF-ek készítéséhez
- **IP geofence megfelelőség** -- Szabályozott környezetekhez
- **E-mail jelentések** -- `precc mail report` elemzések küldéséhez
- **GitHub Actions elemzés** -- `precc gha` sikertelen munkafolyamatok hibakereséséhez
- **Kontextus tömörítés** -- `precc compress` a CLAUDE.md optimalizálásához
- **Elsőbbségi támogatás**

## Licenc aktiválása

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Licenc állapot ellenőrzése

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors aktiválás

Ha a PRECC-et GitHub Sponsors-on keresztül szponzorálja, a licence automatikusan aktiválódik a GitHub e-mail címén keresztül. Nincs szükség kulcsra -- csak győződjön meg róla, hogy a szponzori e-mail egyezik:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Eszköz-ujjlenyomat

Minden licenc egy eszköz-ujjlenyomathoz van kötve. Tekintse meg az övét:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Ha át kell helyeznie a licencét egy új gépre, először deaktiválja:

```bash
precc license deactivate
```

Ezután aktiválja az új gépen.

## Lejárt a licenc?

Ha egy Pro licenc lejár, a PRECC visszaáll a Community szintre. Minden beépített képesség és alapfunkció tovább működik. Csak a Pro-specifikus funkciók válnak elérhetetlenné. Lásd a [FAQ](faq.md) oldalt további részletekért.
