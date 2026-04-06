# Veelgestelde vragen

## Is PRECC veilig om te gebruiken?

Ja. PRECC gebruikt het officiële Claude Code PreToolUse hook-mechanisme -- hetzelfde uitbreidingspunt dat Anthropic precies hiervoor heeft ontworpen. De hook:

- Draait volledig offline (geen netwerkoproepen in het kritieke pad)
- Voltooit in minder dan 5 milliseconden
- Is fail-open: als er iets misgaat, wordt het oorspronkelijke commando ongewijzigd uitgevoerd
- Wijzigt alleen commando's, voert ze nooit zelf uit
- Slaat gegevens lokaal op in SQLite-databases

## Werkt PRECC met andere AI-coderingstools?

PRECC is specifiek ontworpen voor Claude Code. Het is afhankelijk van het PreToolUse hook-protocol dat Claude Code biedt. Het werkt niet met Cursor, Copilot, Windsurf of andere AI-coderingstools.

## Welke gegevens stuurt de telemetrie?

Telemetrie is alleen opt-in. Indien ingeschakeld, stuurt het:

- PRECC-versie, besturingssysteem en architectuur
- Geaggregeerde tellingen (onderschepte opdrachten, geactiveerde vaardigheden)
- Gemiddelde hook-latentie

Het stuurt **geen** opdrachttekst, bestandspaden, projectnamen of persoonlijk identificeerbare informatie. U kunt de exacte payload bekijken met `precc telemetry preview` voordat u zich aanmeldt. Zie [Telemetrie](telemetry.md) voor details.

## Hoe verwijder ik PRECC?

??faq_uninstall_a_intro??

1. Hook-registratie verwijderen:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Binair bestand verwijderen:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Gegevens verwijderen (optioneel):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Mijn licentie is verlopen. Wat gebeurt er?

PRECC keert terug naar het Community-niveau. Alle kernfunctionaliteit blijft werken:

- Ingebouwde vaardigheden blijven actief
- Hook-pipeline draait normaal
- `precc savings` toont de samenvattingsweergave
- `precc ingest` en sessiemining werken

Pro-functies worden niet beschikbaar tot verlenging:

- `precc savings --all` (gedetailleerd overzicht)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mailrapporten

## De hook lijkt niet te werken. Hoe debug ik?

??faq_debug_a_intro??

1. Controleer of de hook geregistreerd is:
   ```bash
   precc init
   ```

2. Test de hook handmatig:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Controleer of het binaire bestand in uw PATH staat:
   ```bash
   which precc-hook
   ```

4. Controleer de hook-configuratie van Claude Code in `~/.claude/settings.json`.

## Vertraagt PRECC Claude Code?

Nee. De hook wordt in minder dan 5 milliseconden (p99) voltooid. Dit is onmerkbaar vergeleken met de tijd die Claude besteedt aan redeneren en het genereren van antwoorden.

## Kan ik PRECC gebruiken in CI/CD?

PRECC is ontworpen voor interactieve Claude Code-sessies. In CI/CD is er geen Claude Code-instantie om aan te haken. Echter, `precc gha` kan mislukte GitHub Actions-runs vanuit elke omgeving analyseren.

## Hoe verschillen gedolven vaardigheden van ingebouwde vaardigheden?

Ingebouwde vaardigheden worden meegeleverd met PRECC en dekken veelvoorkomende verkeerde-map-patronen. Gedolven vaardigheden worden geleerd uit uw specifieke sessielogs -- ze vangen patronen die uniek zijn voor uw werkstroom. Beide worden opgeslagen in SQLite en identiek geëvalueerd door de hook-pipeline.

## Kan ik vaardigheden delen met mijn team?

Ja. Exporteer een vaardigheid naar TOML met `precc skills export NAME` en deel het bestand. Teamleden kunnen het in hun `skills/`-map plaatsen of importeren in hun heuristieken-database.
