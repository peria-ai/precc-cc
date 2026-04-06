# Pipeline Hooka

Plik binarny `precc-hook` jest rdzeniem PRECC. Znajduje się między Claude Code a powłoką, przetwarzając każde polecenie bash w mniej niż 5 milisekund.

## Jak Claude Code wywołuje Hook

Claude Code obsługuje hooki PreToolUse -- zewnętrzne programy, które mogą sprawdzać i modyfikować dane wejściowe narzędzi przed wykonaniem. Gdy Claude zamierza uruchomić polecenie bash, wysyła JSON do `precc-hook` na stdin i odczytuje odpowiedź ze stdout.

## Etapy Pipeline

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

## Przykład: Wejście i wyjście JSON

### Wejście (z Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC wykrywa, że bieżący katalog nie ma `Cargo.toml`, ale `./myapp/Cargo.toml` istnieje.

### Wyjście (do Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Jeśli nie jest potrzebna modyfikacja, `updatedInput.command` jest pusty i Claude Code używa oryginalnego polecenia.

## Szczegóły etapów

### Etap 1: Parsowanie JSON

Odczytuje pełny obiekt JSON ze stdin. Wyodrębnia `tool_input.command`. Jeśli parsowanie się nie powiedzie, hook natychmiast się kończy i Claude Code używa oryginalnego polecenia (projekt fail-open).

### Etap 2: Dopasowanie umiejętności

Odpytuje bazę heurystyk SQLite o umiejętności, których wzorzec wyzwalania pasuje do polecenia. Umiejętności sprawdzane są w kolejności priorytetów. Oceniane są zarówno wbudowane umiejętności TOML, jak i wydobyte.

### Etap 3: Korekta katalogu

Dla poleceń budowania (`cargo`, `go`, `make`, `npm`, `python` itd.) sprawdza, czy oczekiwany plik projektu istnieje w bieżącym katalogu. Jeśli nie, skanuje pobliskie katalogi w poszukiwaniu najbliższego dopasowania i dodaje `cd <dir> &&` na początku.

Skanowanie katalogów wykorzystuje buforowany indeks systemu plików z TTL wynoszącym 5 sekund, aby zachować szybkość.

### Etap 4: Sprawdzenie GDB

Jeśli polecenie prawdopodobnie spowoduje awarię (np. uruchomienie binarki debug), PRECC może zasugerować lub wstrzyknąć wrappery GDB, aby przechwycić ustrukturyzowane dane debugowania zamiast surowych logów awarii.

### Etap 5: Przepisywanie RTK

Stosuje reguły RTK (Rewrite Toolkit), które skracają rozwlekłe polecenia, tłumią hałaśliwe wyjście lub restrukturyzują polecenia dla wydajności tokenów.

### Etap 6: Emisja JSON

Serializuje zmodyfikowane polecenie z powrotem do JSON i zapisuje je na stdout. Jeśli nie wprowadzono zmian, wyjście sygnalizuje Claude Code, aby użył oryginalnego polecenia.

## Wydajność

Cały pipeline kończy się w mniej niż 5 milisekund (p99). Kluczowe optymalizacje:

- SQLite w trybie WAL dla odczytów współbieżnych bez blokad
- Prekompilowane wzorce regex do dopasowywania umiejętności
- Buforowane skany systemu plików (TTL 5 sekund)
- Brak wywołań sieciowych na gorącej ścieżce
- Fail-open: każdy błąd przechodzi do oryginalnego polecenia

## Ręczne testowanie Hooka

Możesz wywołać hook bezpośrednio:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
