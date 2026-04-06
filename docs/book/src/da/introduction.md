# Introduktion

## Hvad er PRECC?

PRECC (Forudsigelig fejlkorrektion for Claude Code) er et Rust-værktøj der opfanger Claude Code bash-kommandoer via den officielle PreToolUse hook-mekanisme. Det retter fejl *før de sker*, sparer tokens og eliminerer genforsøgsløkker.

Gratis for community-brugere.

## Problemet

Claude Code spilder betydelige tokens på forebyggelige fejl:

- **Forkert-mappe-fejl** -- Kørsel af `cargo build` i en overordnet mappe uden `Cargo.toml`, derefter genforsøg efter at have læst fejlen.
- **Genforsøgsløkker** -- En fejlet kommando producerer omstændeligt output, Claude læser det, ræsonnerer og prøver igen. Hver cyklus brænder hundredvis af tokens.
- **Omstændeligt output** -- Kommandoer som `find` eller `ls -R` dumper tusindvis af linjer som Claude skal behandle.

## De fire søjler

### Kontekstkorrektion (cd-prepend)

Registrerer når kommandoer som `cargo build` eller `npm test` køres i den forkerte mappe og sætter `cd /correct/path &&` foran før udførelse.

### GDB-fejlfinding

Registrerer muligheder for at tilslutte GDB til dybere fejlfinding af segfaults og nedbrud, og giver struktureret debug-information i stedet for rå core dumps.

### Sessionsanalyse

Analyserer Claude Code-sessionslogfiler for fejl-rettelsespar. Når den samme fejl gentager sig, kender PRECC allerede rettelsen og anvender den automatisk.

### Automatiseringsfærdigheder

Et bibliotek af indbyggede og lærte færdigheder der matcher kommandomønstre og omskriver dem. Færdigheder defineres som TOML-filer eller SQLite-rækker, hvilket gør dem nemme at inspicere, redigere og dele.

## Sådan virker det (30-sekunders version)

1. Claude Code er ved at køre en bash-kommando.
2. PreToolUse hook sender kommandoen til `precc-hook` som JSON på stdin.
3. `precc-hook` kører kommandoen gennem pipeline (færdigheder, mappekorrektion, komprimering) på under 3 millisekunder.
4. Den korrigerede kommando returneres som JSON på stdout.
5. Claude Code udfører den korrigerede kommando i stedet.

Claude ser aldrig fejlen. Nul tokens spildt.

### Adaptiv komprimering

Hvis en kommando fejler efter komprimering, springer PRECC automatisk komprimering over ved genforsøget, så Claude får det fulde ukomprimerede output til fejlfinding.

## Live brugsstatistik

Nuværende version <span data-stat="current_version">--</span>:

| Metrik | Værdi |
|---|---|
| Hook-invokationer | <span data-stat="total_invocations">--</span> |
| Tokens sparet | <span data-stat="total_tokens_saved">--</span> |
| Besparelsesforhold | <span data-stat="saving_pct">--</span>% |
| RTK-omskrivninger | <span data-stat="rtk_rewrites">--</span> |
| CD-korrektioner | <span data-stat="cd_prepends">--</span> |
| Hook-latens | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Brugere | <span data-stat="unique_users">--</span> |

### Besparelser per version

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Brugere</th><th>Hook-invokationer</th><th>Tokens sparet</th><th>Besparelsesforhold</th></tr></thead>
<tbody><tr><td colspan="5"><em>Indlæser...</em></td></tr></tbody>
</table>

<small>Tal er estimater. Hver forhindret fejl undgår en fuld genforsøgscyklus: fejloutput, modelræsonnement og genforsøgskommando. Disse tal opdateres automatisk fra anonymiseret telemetri.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Hjemmeside: [https://peria.ai](https://peria.ai)
- Dokumentation: [https://precc.cc](https://precc.cc)
