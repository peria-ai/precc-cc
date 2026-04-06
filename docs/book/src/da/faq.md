# Ofte stillede spørgsmål

## Er PRECC sikkert at bruge?

Ja. PRECC bruger den officielle Claude Code PreToolUse hook-mekanisme -- det samme udvidelsespunkt som Anthropic har designet til netop dette formål. Hook:

- Kører helt offline (ingen netværkskald i den kritiske sti)
- Fuldfører på under 5 millisekunder
- Er fail-open: hvis noget går galt, køres den originale kommando uændret
- Ændrer kun kommandoer, udfører dem aldrig selv
- Gemmer data lokalt i SQLite-databaser

## Virker PRECC med andre AI-kodningsværktøjer?

PRECC er designet specifikt til Claude Code. Det bruger PreToolUse hook-protokollen fra Claude Code. Det virker ikke med Cursor, Copilot, Windsurf eller andre AI-kodningsværktøjer.

## Hvilke data sender telemetrien?

Telemetri er kun opt-in. Når aktiveret, sender den:

- PRECC-version, OS og arkitektur
- Aggregerede tællinger (opfangede kommandoer, aktiverede færdigheder)
- Gennemsnitlig hook-latens

Den sender **ikke** kommandotekst, filstier, projektnavne eller personligt identificerbare oplysninger. Du kan forhåndsvise den nøjagtige payload med `precc telemetry preview` før tilmelding. Se [Telemetry](telemetry.md) for alle detaljer.

## Hvordan afinstallerer jeg PRECC?

??faq_uninstall_a_intro??

1. Fjern hook-registreringen:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Fjern binærfilen:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Fjern data (valgfrit):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Min licens er udløbet. Hvad sker der?

PRECC vender tilbage til Community-niveauet. Al kernefunktionalitet fortsætter med at virke:

- Indbyggede færdigheder forbliver aktive
- Hook pipeline kører normalt
- `precc savings` viser oversigtvisningen
- `precc ingest` og sessionsanalyse virker

Pro-funktioner bliver utilgængelige indtil fornyelse:

- `precc savings --all` (detaljeret oversigt)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mail-rapporter

## Hook ser ikke ud til at køre. Hvordan fejlfinder jeg?

??faq_debug_a_intro??

1. Kontroller at hook er registreret:
   ```bash
   precc init
   ```

2. Test hook manuelt:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Kontroller at binærfilen er i din PATH:
   ```bash
   which precc-hook
   ```

4. Kontroller Claude Codes hook-konfiguration i `~/.claude/settings.json`.

## Gør PRECC Claude Code langsommere?

Nej. Hook fuldfører på under 5 millisekunder (p99). Dette er umærkeligt sammenlignet med den tid, Claude bruger på at ræsonnere og generere svar.

## Kan jeg bruge PRECC i CI/CD?

PRECC er designet til interaktive Claude Code-sessioner. I CI/CD er der ingen Claude Code-instans at tilslutte. Dog kan `precc gha` analysere fejlede GitHub Actions-kørsler fra ethvert miljø.

## Hvordan adskiller lærte færdigheder sig fra indbyggede?

Indbyggede færdigheder leveres med PRECC og dækker almindelige forkert-mappe-mønstre. Lærte færdigheder udvindes fra dine specifikke sessionslogfiler -- de fanger mønstre unikke for din arbejdsgang. Begge gemmes i SQLite og evalueres identisk af hook pipeline.

## Kan jeg dele færdigheder med mit team?

Ja. Eksporter enhver færdighed til TOML med `precc skills export NAME` og del filen. Teammedlemmer kan placere den i deres `skills/`-mappe eller importere den i deres heuristikdatabase.
