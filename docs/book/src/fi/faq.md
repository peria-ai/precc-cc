# Usein kysytyt kysymykset

## Onko PRECC turvallista käyttää?

Kyllä. PRECC käyttää virallista Claude Code PreToolUse hook -mekanismia -- samaa laajennuspistettä, jonka Anthropic suunnitteli juuri tähän tarkoitukseen. Hook:

- Toimii täysin offline (ei verkkokutsuja kriittisellä polulla)
- Valmistuu alle 5 millisekunnissa
- On fail-open: jos jokin menee pieleen, alkuperäinen komento suoritetaan muuttumattomana
- Muokkaa vain komentoja, ei koskaan suorita niitä itse
- Tallentaa tiedot paikallisesti SQLite-tietokantoihin

## Toimiiko PRECC muiden AI-koodaustyökalujen kanssa?

PRECC on suunniteltu nimenomaan Claude Codelle. Se käyttää PreToolUse hook -protokollaa, jonka Claude Code tarjoaa. Se ei toimi Cursorin, Copilotin, Windsurfin tai muiden AI-koodaustyökalujen kanssa.

## Mitä tietoja telemetria lähettää?

Telemetria on vain opt-in. Kun käytössä, se lähettää:

- PRECC-version, käyttöjärjestelmän ja arkkitehtuurin
- Kootut laskurit (kaapatut komennot, aktivoidut taidot)
- Keskimääräisen hook-viiveen

Se **ei** lähetä komentotekstiä, tiedostopolkuja, projektinimiä tai henkilökohtaisesti tunnistettavaa tietoa. Voit esikatsella tarkan hyötykuorman komennolla `precc telemetry preview` ennen liittymistä. Katso [Telemetry](telemetry.md) täydelliset tiedot.

## Miten poistan PRECCin?

??faq_uninstall_a_intro??

1. Poista hook-rekisteröinti:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Poista binääri:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Poista tiedot (valinnainen):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Lisenssini on vanhentunut. Mitä tapahtuu?

PRECC palaa Community-tasolle. Kaikki ydintoiminnot jatkavat toimintaansa:

- Sisäänrakennetut taidot pysyvät aktiivisina
- Hook pipeline toimii normaalisti
- `precc savings` näyttää yhteenvetonäkymän
- `precc ingest` ja istuntojen analysointi toimivat

Pro-ominaisuudet eivät ole käytettävissä uusimiseen asti:

- `precc savings --all` (yksityiskohtainen erittely)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Sähköpostiraportit

## Hook ei näytä toimivan. Miten teen vianmäärityksen?

??faq_debug_a_intro??

1. Tarkista, että hook on rekisteröity:
   ```bash
   precc init
   ```

2. Testaa hookia manuaalisesti:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Tarkista, että binääri on PATH-muuttujassasi:
   ```bash
   which precc-hook
   ```

4. Tarkista Claude Coden hook-asetukset tiedostossa `~/.claude/settings.json`.

## Hidastaako PRECC Claude Codea?

Ei. Hook valmistuu alle 5 millisekunnissa (p99). Tämä on huomaamatonta verrattuna aikaan, jonka Claude käyttää päättelyyn ja vastausten generointiin.

## Voinko käyttää PRECCiä CI/CD:ssä?

PRECC on suunniteltu interaktiivisiin Claude Code -istuntoihin. CI/CD:ssä ei ole Claude Code -instanssia johon kytkeä. Kuitenkin `precc gha` voi analysoida epäonnistuneita GitHub Actions -ajoja mistä tahansa ympäristöstä.

## Miten opitut taidot eroavat sisäänrakennetuista?

Sisäänrakennetut taidot toimitetaan PRECCin mukana ja kattavat yleiset väärä-hakemisto-mallit. Opitut taidot poimitaan sinun istuntolokistasi -- ne tallentavat työnkulullesi ainutlaatuisia malleja. Molemmat tallennetaan SQLiteen ja arvioidaan identtisesti hook pipelinen toimesta.

## Voinko jakaa taitoja tiimini kanssa?

Kyllä. Vie mikä tahansa taito TOMLiin komennolla `precc skills export NAME` ja jaa tiedosto. Tiimin jäsenet voivat sijoittaa sen `skills/`-hakemistoonsa tai tuoda sen heuristiikkatietokantaansa.
