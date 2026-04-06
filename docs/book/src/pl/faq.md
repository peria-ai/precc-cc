# FAQ

## Czy PRECC jest bezpieczny?

Tak. PRECC używa oficjalnego mechanizmu hooków PreToolUse Claude Code -- tego samego punktu rozszerzenia, który Anthropic zaprojektował do tego celu. Hook:

- Działa całkowicie offline (brak wywołań sieciowych na ścieżce krytycznej)
- Kończy się w mniej niż 5 milisekund
- Jest fail-open: jeśli coś pójdzie nie tak, oryginalne polecenie uruchomi się bez zmian
- Tylko modyfikuje polecenia, nigdy ich sam nie wykonuje
- Przechowuje dane lokalnie w bazach danych SQLite

## Czy PRECC działa z innymi narzędziami AI do kodowania?

PRECC jest zaprojektowany specjalnie dla Claude Code. Opiera się na protokole hooków PreToolUse, który zapewnia Claude Code. Nie działa z Cursor, Copilot, Windsurf ani innymi narzędziami AI do kodowania.

## Jakie dane wysyła telemetria?

Telemetria jest tylko opt-in. Po włączeniu wysyła:

- Wersję PRECC, system operacyjny i architekturę
- Zagregowane liczby (przechwycone polecenia, aktywowane umiejętności)
- Średnie opóźnienie hooka

**Nie** wysyła tekstu poleceń, ścieżek plików, nazw projektów ani żadnych danych osobowych. Przed włączeniem możesz podejrzeć dokładne dane za pomocą `precc telemetry preview`. Szczegóły w [Telemetria](telemetry.md).

## Jak odinstalować PRECC?

??faq_uninstall_a_intro??

1. Usuń rejestrację hooka:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Usuń plik binarny:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Usuń dane (opcjonalne):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Moja licencja wygasła. Co się stanie?

PRECC wraca do poziomu Community. Cała podstawowa funkcjonalność nadal działa:

- Wbudowane umiejętności pozostają aktywne
- Pipeline hooka działa normalnie
- `precc savings` pokazuje widok podsumowania
- `precc ingest` i eksploracja sesji działają

Funkcje Pro stają się niedostępne do momentu odnowienia:

- `precc savings --all` (szczegółowy podział)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Raporty e-mail

## Hook nie wydaje się działać. Jak debugować?

??faq_debug_a_intro??

1. Sprawdź, czy hook jest zarejestrowany:
   ```bash
   precc init
   ```

2. Przetestuj hook ręcznie:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Sprawdź, czy plik binarny jest w PATH:
   ```bash
   which precc-hook
   ```

4. Sprawdź konfigurację hooka Claude Code w `~/.claude/settings.json`.

## Czy PRECC spowalnia Claude Code?

Nie. Hook kończy się w mniej niż 5 milisekund (p99). Jest to niezauważalne w porównaniu z czasem, jaki Claude poświęca na rozumowanie i generowanie odpowiedzi.

## Czy mogę używać PRECC w CI/CD?

PRECC jest zaprojektowany do interaktywnych sesji Claude Code. W CI/CD nie ma instancji Claude Code, do której można się podłączyć. Jednak `precc gha` może analizować nieudane uruchomienia GitHub Actions z dowolnego środowiska.

## Czym różnią się wydobyte umiejętności od wbudowanych?

Wbudowane umiejętności są dostarczane z PRECC i obejmują typowe wzorce złego katalogu. Wydobyte umiejętności są uczone z Twoich konkretnych logów sesji -- przechwytują wzorce unikalne dla Twojego przepływu pracy. Oba typy są przechowywane w SQLite i oceniane identycznie przez pipeline hooka.

## Czy mogę udostępniać umiejętności zespołowi?

Tak. Wyeksportuj dowolną umiejętność do TOML za pomocą `precc skills export NAME` i udostępnij plik. Członkowie zespołu mogą umieścić go w katalogu `skills/` lub zaimportować do bazy danych heurystyk.
