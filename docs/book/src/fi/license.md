# Lisenssi

PRECC tarjoaa kaksi tasoa: Community (ilmainen) ja Pro.

## Community-taso (ilmainen)

Community-taso sisältää:

- Kaikki sisäänrakennetut taidot (hakemistokorjaus, jj-käännös jne.)
- Hook pipeline täydellä Pillar 1 ja Pillar 4 tuella
- Perus `precc savings` -yhteenveto
- Istuntojen analysointi `precc ingest` -komennolla
- Rajoittamaton paikallinen käyttö

## Pro-taso

Pro avaa lisäominaisuuksia:

- **Yksityiskohtainen säästöerittely** -- `precc savings --all` komentokohtaisella analyysillä
- **GIF-tallennus** -- `precc gif` animoitujen terminaali-GIFien luomiseen
- **IP-geoaitaus** -- Säännellyille ympäristöille
- **Sähköpostiraportit** -- `precc mail report` analytiikan lähettämiseen
- **GitHub Actions -analyysi** -- `precc gha` epäonnistuneiden työnkulkujen virheenkorjaukseen
- **Kontekstin pakkaus** -- `precc compress` CLAUDE.md:n optimointiin
- **Ensisijainen tuki**

## Lisenssin aktivointi

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Lisenssin tilan tarkistus

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors -aktivointi

Jos sponsoroit PRECCiä GitHub Sponsorsin kautta, lisenssisi aktivoidaan automaattisesti GitHub-sähköpostisi kautta. Avainta ei tarvita -- varmista vain, että sponsori-sähköpostisi täsmää:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Laitteen sormenjälki

Jokainen lisenssi on sidottu laitteen sormenjälkeen. Näytä omasi:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Jos sinun tarvitsee siirtää lisenssi uudelle koneelle, poista käytöstä ensin:

```bash
precc license deactivate
```

Aktivoi sitten uudella koneella.

## Lisenssi vanhentunut?

Kun Pro-lisenssi vanhenee, PRECC palaa Community-tasolle. Kaikki sisäänrakennetut taidot ja ydintoiminnot jatkavat toimintaansa. Vain Pro-erityisominaisuudet eivät ole käytettävissä. Katso [FAQ](faq.md) lisätietoja.
