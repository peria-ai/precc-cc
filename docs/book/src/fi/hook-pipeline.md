# Hook Pipeline

`precc-hook`-binääri on PRECCin ydin. Se sijaitsee Claude Coden ja kuoren välissä käsitellen jokaisen bash-komennon alle 5 millisekunnissa.

## Miten Claude Code kutsuu hookia

Claude Code tukee PreToolUse hookeja -- ulkoisia ohjelmia, jotka voivat tarkastella ja muokata työkalujen syötteitä ennen suoritusta. Kun Claude aikoo suorittaa bash-komennon, se lähettää JSONin `precc-hook`:lle stdiniin ja lukee vastauksen stdoutista.

## Pipeline-vaiheet

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## Esimerkki: JSON-syöte ja -tuloste

### Syöte (Claude Codelta)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC havaitsee, ettei nykyisessä hakemistossa ole `Cargo.toml`-tiedostoa, mutta `./myapp/Cargo.toml` löytyy.

### Tuloste (Claude Codelle)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Jos muutosta ei tarvita, `updatedInput.command` on tyhjä ja Claude Code käyttää alkuperäistä komentoa.

## Vaiheiden yksityiskohdat

### Vaihe 1: JSON-jäsennys

Lukee täyden JSON-objektin stdinistä. Poimii `tool_input.command`. Jos jäsennys epäonnistuu, hook poistuu välittömästi ja Claude Code käyttää alkuperäistä komentoa (fail-open-suunnittelu).

### Vaihe 2: Taitojen sovitus

Kyselee SQLite-heuristiikkatietokantaa taidoista, joiden laukaisumalli vastaa komentoa. Taidot tarkistetaan prioriteettijärjestyksessä. Sekä sisäänrakennetut TOML-taidot että opitut taidot arvioidaan.

### Vaihe 3: Hakemistokorjaus

Koontikomennoille (`cargo`, `go`, `make`, `npm`, `python` jne.) tarkistaa, onko odotettu projektitiedosto nykyisessä hakemistossa. Jos ei, skannaa lähihakemistoja lähimmän vastaavuuden löytämiseksi ja lisää `cd <dir> &&` eteen.

Hakemistoskannaus käyttää välimuistiin tallennettua tiedostojärjestelmäindeksiä 5 sekunnin TTL:llä nopeuden säilyttämiseksi.

### Vaihe 4: GDB-tarkistus

Jos komento todennäköisesti aiheuttaa kaatumisen (esim. debug-binäärin ajaminen), PRECC voi ehdottaa tai syöttää GDB-käärejä strukturoidun debug-tulosteen kaappaamiseksi raakojen kaatumislokien sijaan.

### Vaihe 5: RTK-uudelleenkirjoitus

Soveltaa RTK (Rewrite Toolkit) -sääntöjä, jotka lyhentävät monisanaisia komentoja, vaimentavat kohinaista tulostetta tai uudelleenjärjestävät komentoja tokenitehokkuuden saavuttamiseksi.

### Vaihe 6: JSON-tuloste

Serialisoi muokatun komennon takaisin JSONiksi ja kirjoittaa sen stdoutiin. Jos muutoksia ei tehty, tuloste signaloi Claude Codelle käyttää alkuperäistä komentoa.

## Suorituskyky

Koko pipeline valmistuu alle 5 millisekunnissa (p99). Keskeiset optimoinnit:

- SQLite WAL-tilassa lukitusvapaita samanaikaisia lukuja varten
- Esikäännetyt regex-mallit taitojen sovitukseen
- Välimuistiin tallennetut tiedostojärjestelmän skannaukset (5 sekunnin TTL)
- Ei verkkokutsuja kriittisellä polulla
- Fail-open: mikä tahansa virhe putoaa alkuperäiseen komentoon

## Hookin manuaalinen testaus

Voit kutsua hookia suoraan:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
