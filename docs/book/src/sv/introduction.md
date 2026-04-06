# Introduktion

## Vad är PRECC?

PRECC (Prediktiv felkorrigering för Claude Code) är ett Rust-verktyg som fångar upp Claude Code bash-kommandon via den officiella PreToolUse hook-mekanismen. Det rättar fel *innan de inträffar*, sparar tokens och eliminerar omförsöksslingor.

Gratis för community-användare.

## Problemet

Claude Code slösar betydande tokens på förebyggbara misstag:

- **Fel-katalog-fel** -- Körning av `cargo build` i en överordnad katalog utan `Cargo.toml`, sedan omförsök efter att ha läst felet.
- **Omförsöksslingor** -- Ett misslyckat kommando producerar ordrik utdata, Claude läser det, resonerar och försöker igen. Varje cykel bränner hundratals tokens.
- **Ordrik utdata** -- Kommandon som `find` eller `ls -R` dumpar tusentals rader som Claude måste bearbeta.

## De fyra pelarna

### Kontextkorrigering (cd-prepend)

Upptäcker när kommandon som `cargo build` eller `npm test` körs i fel katalog och lägger till `cd /correct/path &&` före körning.

### GDB-felsökning

Upptäcker möjligheter att koppla GDB för djupare felsökning av segfaults och krascher, och ger strukturerad debuginformation istället för råa core dumps.

### Sessionsanalys

Analyserar Claude Code-sessionsloggar för fel-rättningspar. När samma fel upprepas vet PRECC redan rättningen och tillämpar den automatiskt.

### Automatiseringsfärdigheter

Ett bibliotek av inbyggda och inlärda färdigheter som matchar kommandomönster och skriver om dem. Färdigheter definieras som TOML-filer eller SQLite-rader, vilket gör dem enkla att inspektera, redigera och dela.

## Hur det fungerar (30-sekundersversionen)

1. Claude Code ska köra ett bash-kommando.
2. PreToolUse hook skickar kommandot till `precc-hook` som JSON på stdin.
3. `precc-hook` kör kommandot genom pipeline (färdigheter, katalogkorrigering, komprimering) på under 3 millisekunder.
4. Det korrigerade kommandot returneras som JSON på stdout.
5. Claude Code kör det korrigerade kommandot istället.

Claude ser aldrig felet. Noll tokens slösade.

### Adaptiv komprimering

Om ett kommando misslyckas efter komprimering hoppar PRECC automatiskt över komprimering vid omförsöket så att Claude får den fullständiga okomprimerade utdatan för felsökning.

## Livestatistik

Aktuell version <span data-stat="current_version">--</span>:

| Mätvärde | Värde |
|---|---|
| Hook-anrop | <span data-stat="total_invocations">--</span> |
| Tokens sparade | <span data-stat="total_tokens_saved">--</span> |
| Besparingskvot | <span data-stat="saving_pct">--</span>% |
| RTK-omskrivningar | <span data-stat="rtk_rewrites">--</span> |
| CD-korrigeringar | <span data-stat="cd_prepends">--</span> |
| Hook-latens | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Användare | <span data-stat="unique_users">--</span> |

### Besparingar per version

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Användare</th><th>Hook-anrop</th><th>Tokens sparade</th><th>Besparingskvot</th></tr></thead>
<tbody><tr><td colspan="5"><em>Laddar...</em></td></tr></tbody>
</table>

<small>Siffror är uppskattningar. Varje förhindrat fel undviker en fullständig omförsökscykel: felutdata, modellresonemang och omförsökskommando. Dessa siffror uppdateras automatiskt från anonymiserad telemetri.</small>

## Länkar

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Webbplats: [https://peria.ai](https://peria.ai)
- Dokumentation: [https://precc.cc](https://precc.cc)
