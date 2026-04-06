# Johdanto

## Mikä on PRECC?

PRECC (Ennakoiva virheenkorjaus Claude Codelle) on Rust-työkalu, joka kaappaa Claude Coden bash-komennot virallisen PreToolUse hook -mekanismin kautta. Se korjaa virheet *ennen niiden tapahtumista*, säästäen tokeneita ja eliminoiden uudelleenyrityssilmukat.

Ilmainen yhteisön käyttäjille.

## Ongelma

Claude Code tuhlaa merkittävästi tokeneita estettävissä oleviin virheisiin:

- **Väärä-hakemisto-virheet** -- `cargo build` suoritetaan ylähakemistossa jossa ei ole `Cargo.toml`-tiedostoa, sitten uudelleenyritys virheen lukemisen jälkeen.
- **Uudelleenyrityssilmukat** -- Epäonnistunut komento tuottaa monisanaista tulostetta, Claude lukee sen, päättelee ja yrittää uudelleen. Jokainen sykli kuluttaa satoja tokeneita.
- **Monisanainen tuloste** -- Komennot kuten `find` tai `ls -R` tulostavat tuhansia rivejä, jotka Clauden on käsiteltävä.

## Neljä pilaria

### Kontekstikorjaus (cd-prepend)

Havaitsee, kun komennot kuten `cargo build` tai `npm test` suoritetaan väärässä hakemistossa ja lisää `cd /correct/path &&` ennen suoritusta.

### GDB-virheenkorjaus

Havaitsee mahdollisuudet liittää GDB syvempään segfault- ja kaatumisvirheenkorjaukseen, tarjoten rakenteellista debug-tietoa raakojen core dump -tiedostojen sijaan.

### Istuntojen analysointi

Analysoi Claude Coden istuntolokeja virhe-korjaus-parien löytämiseksi. Kun sama virhe toistuu, PRECC tietää jo korjauksen ja soveltaa sen automaattisesti.

### Automaatiotaidot

Kirjasto sisäänrakennettuja ja opittuja taitoja, jotka vastaavat komentomalleja ja kirjoittavat ne uudelleen. Taidot määritellään TOML-tiedostoina tai SQLite-riveinä, mikä tekee niistä helppoja tarkastella, muokata ja jakaa.

## Miten se toimii (30 sekunnin versio)

1. Claude Code aikoo suorittaa bash-komennon.
2. PreToolUse hook lähettää komennon `precc-hook`:lle JSONina stdiniin.
3. `precc-hook` ajaa komennon pipelinen läpi (taidot, hakemistokorjaus, pakkaus) alle 3 millisekunnissa.
4. Korjattu komento palautetaan JSONina stdoutiin.
5. Claude Code suorittaa korjatun komennon.

Claude ei koskaan näe virhettä. Nolla hukkaan mennyttä tokenia.

### Mukautuva pakkaus

Jos komento epäonnistuu pakkauksen jälkeen, PRECC ohittaa pakkauksen automaattisesti uudelleenyrityksessä, jotta Claude saa täyden pakkaamattoman tulosteen virheenkorjausta varten.

## Reaaliaikaiset käyttötilastot

Nykyinen versio <span data-stat="current_version">--</span>:

| Mittari | Arvo |
|---|---|
| Hook-kutsut | <span data-stat="total_invocations">--</span> |
| Säästetyt tokenit | <span data-stat="total_tokens_saved">--</span> |
| Säästösuhde | <span data-stat="saving_pct">--</span>% |
| RTK-uudelleenkirjoitukset | <span data-stat="rtk_rewrites">--</span> |
| CD-korjaukset | <span data-stat="cd_prepends">--</span> |
| Hook-viive | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Käyttäjät | <span data-stat="unique_users">--</span> |

### Measured Savings (Ground Truth)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Mittari</th><th>Arvo</th></tr></thead>
<tbody>
<tr><td>Original output tokens (without PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Actual output tokens (with PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Säästetyt tokenit</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Säästösuhde</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Ground-truth measurements</td><td><span data-measured="ground_truth_count">--</span> measurements</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### By Rewrite Type

<table id="rewrite-type-table">
<thead><tr><th>Type</th><th>Count</th><th>Avg Savings %</th><th>Säästetyt tokenit</th></tr></thead>
<tbody><tr><td colspan="4"><em>Ladataan...</em></td></tr></tbody>
</table>
</div>

### Säästöt versioittain

<table id="version-breakdown" style="display:none">
<thead><tr><th>Versio</th><th>Käyttäjät</th><th>Hook-kutsut</th><th>Säästetyt tokenit</th><th>Säästösuhde</th></tr></thead>
<tbody><tr><td colspan="5"><em>Ladataan...</em></td></tr></tbody>
</table>

<small>Nämä luvut päivittyvät automaattisesti anonymisoidusta telemetriasta.</small>

## Linkit

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Verkkosivusto: [https://peria.ai](https://peria.ai)
- Dokumentaatio: [https://precc.cc](https://precc.cc)
