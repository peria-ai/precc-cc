# Hook Pipeline

Binární soubor `precc-hook` je jádrem PRECC. Sedí mezi Claude Code a shellem a zpracovává každý bash příkaz za méně než 5 milisekund.

## Jak Claude Code volá hook

Claude Code podporuje PreToolUse hooky -- externí programy, které mohou kontrolovat a upravovat vstupy nástrojů před spuštěním. Když Claude chystá spustit bash příkaz, pošle JSON do `precc-hook` na stdin a čte odpověď ze stdout.

## Fáze pipeline

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

## Příklad: JSON vstup a výstup

### Vstup (z Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC detekuje, že aktuální adresář nemá `Cargo.toml`, ale `./myapp/Cargo.toml` existuje.

### Výstup (do Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Pokud není potřeba žádná úprava, `updatedInput.command` je prázdný a Claude Code použije původní příkaz.

## Podrobnosti fází

### Fáze 1: Parsování JSON

Načte úplný JSON objekt ze stdin. Extrahuje `tool_input.command`. Pokud parsování selže, hook okamžitě skončí a Claude Code použije původní příkaz (fail-open design).

### Fáze 2: Porovnávání dovedností

Dotazuje SQLite databázi heuristik na dovednosti, jejichž spouštěcí vzor odpovídá příkazu. Dovednosti jsou kontrolovány v pořadí priority. Jsou vyhodnoceny jak vestavěné TOML dovednosti, tak naučené.

### Fáze 3: Korekce adresáře

Pro build příkazy (`cargo`, `go`, `make`, `npm`, `python` atd.) kontroluje, zda očekávaný projektový soubor existuje v aktuálním adresáři. Pokud ne, skenuje blízké adresáře pro nejbližší shodu a předřadí `cd <dir> &&`.

Skenování adresáře používá cachovaný index souborového systému s 5sekundovým TTL pro rychlost.

### Fáze 4: Kontrola GDB

Pokud příkaz pravděpodobně způsobí pád (např. spuštění debug binárního souboru), PRECC může navrhnout nebo vložit GDB wrappery pro zachycení strukturovaného debug výstupu místo surových crash logů.

### Fáze 5: Přepis RTK

Aplikuje pravidla RTK (Rewrite Toolkit), která zkracují rozvláčné příkazy, potlačují šumový výstup nebo restrukturalizují příkazy pro efektivitu tokenů.

### Fáze 6: Výstup JSON

Serializuje upravený příkaz zpět do JSON a zapíše na stdout. Pokud nebyly provedeny žádné změny, výstup signalizuje Claude Code, aby použil původní příkaz.

## Výkon

Celý pipeline se dokončí za méně než 5 milisekund (p99). Klíčové optimalizace:

- SQLite v režimu WAL pro souběžné čtení bez zámků
- Předkompilované regex vzory pro porovnávání dovedností
- Cachované skenování souborového systému (5sekundový TTL)
- Žádná síťová volání na kritické cestě
- Fail-open: jakákoli chyba projde k původnímu příkazu

## Ruční testování hooku

Hook můžete vyvolat přímo:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
