# Inngangur

## Hvað er PRECC?

PRECC (Fyrirbyggjandi villuleiðrétting fyrir Claude Code) er Rust-tól sem fangar bash-skipanir Claude Code í gegnum opinberu PreToolUse hook-kerfið. Það lagar villur *áður en þær gerast*, sparar tákn og útilokar endurtilraunalokkur.

Ókeypis fyrir samfélagsnotendur.

## Vandamálið

Claude Code sóar verulegum táknum á fyrirbyggjanlegar villur:

- **Röng-möppu villur** -- Keyrsla á `cargo build` í yfirmöppu sem hefur ekki `Cargo.toml`, svo reynt aftur eftir lestur villunnar.
- **Endurtilraunalokkur** -- Misheppnuð skipun framleiðir orðmörg úttak, Claude les þau, rökhugar og reynir aftur. Hvert hringrás brennir hundruð tákna.
- **Orðmörg úttak** -- Skipanir eins og `find` eða `ls -R` hella út þúsundum lína sem Claude þarf að vinna úr.

## Fjórir stoðir

### Samhengisleiðrétting (cd-prepend)

Greinir þegar skipanir eins og `cargo build` eða `npm test` keyra í rangri möppu og bætir `cd /correct/path &&` framan á fyrir keyrslu.

### GDB-kembivinnsla

Greinir tækifæri til að tengja GDB fyrir dýpri kembivinnslu segfault og hruns, veitir skipulagðar kembiupplýsingar í stað hrárra core dump.

### Lotugreining

Greinir lotunarskrár Claude Code eftir villu-lagfæringar pörum. Þegar sama villan endurtekur sig veit PRECC þegar lagfæringuna og beitir henni sjálfkrafa.

### Sjálfvirkniþekking

Safn innbyggðrar og lærðrar þekkingar sem passar við skipanamunstur og endurskrifar þær. Þekking er skilgreind sem TOML-skrár eða SQLite-raðir, sem gerir þær auðveldar til að skoða, breyta og deila.

## Hvernig það virkar (30-sekúndna útgáfa)

1. Claude Code er að fara að keyra bash-skipun.
2. PreToolUse hook sendir skipunina til `precc-hook` sem JSON á stdin.
3. `precc-hook` keyrir skipunina í gegnum pipeline (þekkingu, möppuleiðréttingu, þjöppun) á undir 3 millisekúndum.
4. Leiðrétta skipunin er skilað sem JSON á stdout.
5. Claude Code keyrir leiðréttu skipunina í staðinn.

Claude sér aldrei villuna. Engin tákn sóuð.

### Aðlögunarþjöppun

Ef skipun mistekst eftir þjöppun, sleppir PRECC sjálfkrafa þjöppun við endurtilraun svo Claude fái fullt óþjappað úttak til villuleitar.

## Lifandi notkunartölfræði

Current version <span data-stat="current_version">--</span>:

| Mælikvarði | Gildi |
|---|---|
| Hook-köll | <span data-stat="total_invocations">--</span> |
| Tákn sparuð | <span data-stat="total_tokens_saved">--</span> |
| Sparnaðarhlutfall | <span data-stat="saving_pct">--</span>% |
| RTK-endurskriftir | <span data-stat="rtk_rewrites">--</span> |
| CD-leiðréttingar | <span data-stat="cd_prepends">--</span> |
| Hook-leynd | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Unique users | <span data-stat="unique_users">--</span> |

### Savings by Release

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Unique users</th><th>Hook-köll</th><th>Tákn sparuð</th><th>Sparnaðarhlutfall</th></tr></thead>
<tbody><tr><td colspan="5"><em>Loading...</em></td></tr></tbody>
</table>

<small>Tölur eru áætlanir. Hver komin í veg fyrir villa forðast fulla endurtilraunalokku: villuúttak, líkans rökstuðningur og endurtilraunaskipun. Þessar tölur uppfærast sjálfkrafa úr nafnlausri fjarmælingu.</small>

## Tenglar

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Vefsíða: [https://peria.ai](https://peria.ai)
- Skjöl: [https://precc.cc](https://precc.cc)
