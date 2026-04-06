# Vanliga frågor

## Är PRECC säkert att använda?

Ja. PRECC använder den officiella Claude Code PreToolUse hook-mekanismen -- samma tilläggspunkt som Anthropic designade för just detta ändamål. Hook:

- Körs helt offline (inga nätverksanrop i den kritiska sökvägen)
- Slutförs på under 5 millisekunder
- Är fail-open: om något går fel körs det ursprungliga kommandot oförändrat
- Ändrar bara kommandon, kör dem aldrig själv
- Lagrar data lokalt i SQLite-databaser

## Fungerar PRECC med andra AI-kodningsverktyg?

PRECC är designat specifikt för Claude Code. Det förlitar sig på PreToolUse hook-protokollet som Claude Code tillhandahåller. Det fungerar inte med Cursor, Copilot, Windsurf eller andra AI-kodningsverktyg.

## Vilken data skickar telemetrin?

Telemetri är enbart opt-in. När aktiverad skickar den:

- PRECC-version, OS och arkitektur
- Aggregerade räknare (uppfångade kommandon, aktiverade färdigheter)
- Genomsnittlig hook-latens

Den skickar **inte** kommandotext, filsökvägar, projektnamn eller personligt identifierbar information. Du kan förhandsgranska den exakta nyttolasten med `precc telemetry preview` innan du anmäler dig. Se [Telemetry](telemetry.md) för alla detaljer.

## Hur avinstallerar jag PRECC?

??faq_uninstall_a_intro??

1. Ta bort hook-registreringen:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Ta bort binärfilen:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Ta bort data (valfritt):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Min licens har löpt ut. Vad händer?

PRECC återgår till Community-nivån. All kärnfunktionalitet fortsätter att fungera:

- Inbyggda färdigheter förblir aktiva
- Hook pipeline körs normalt
- `precc savings` visar sammanfattningsvyn
- `precc ingest` och sessionsanalys fungerar

Pro-funktioner blir otillgängliga tills du förnyar:

- `precc savings --all` (detaljerad uppdelning)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-postrapporter

## Hook verkar inte köras. Hur felsöker jag?

??faq_debug_a_intro??

1. Kontrollera att hook är registrerad:
   ```bash
   precc init
   ```

2. Testa hook manuellt:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Kontrollera att binärfilen finns i din PATH:
   ```bash
   which precc-hook
   ```

4. Kontrollera Claude Codes hook-konfiguration i `~/.claude/settings.json`.

## Gör PRECC Claude Code långsammare?

Nej. Hook slutförs på under 5 millisekunder (p99). Detta är omärkbart jämfört med tiden Claude lägger på att resonera och generera svar.

## Kan jag använda PRECC i CI/CD?

PRECC är designat för interaktiva Claude Code-sessioner. I CI/CD finns det ingen Claude Code-instans att koppla till. Dock kan `precc gha` analysera misslyckade GitHub Actions-körningar från vilken miljö som helst.

## Hur skiljer sig inlärda färdigheter från inbyggda?

Inbyggda färdigheter levereras med PRECC och täcker vanliga fel-katalog-mönster. Inlärda färdigheter extraheras från dina specifika sessionsloggar -- de fångar mönster unika för ditt arbetsflöde. Båda lagras i SQLite och utvärderas identiskt av hook pipeline.

## Kan jag dela färdigheter med mitt team?

Ja. Exportera vilken färdighet som helst till TOML med `precc skills export NAME` och dela filen. Teammedlemmar kan placera den i sin `skills/`-katalog eller importera den i sin heuristikdatabas.
