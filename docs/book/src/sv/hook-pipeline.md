# Hook Pipeline

Binärfilen `precc-hook` är kärnan i PRECC. Den sitter mellan Claude Code och skalet och bearbetar varje bash-kommando på under 5 millisekunder.

## Hur Claude Code anropar hook

Claude Code stöder PreToolUse hooks -- externa program som kan inspektera och modifiera verktygsinsdata före körning. När Claude ska köra ett bash-kommando skickar den JSON till `precc-hook` på stdin och läser svaret från stdout.

## Pipeline-faser

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

## Exempel: JSON indata och utdata

### Indata (från Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC upptäcker att den aktuella katalogen saknar `Cargo.toml`, men `./myapp/Cargo.toml` finns.

### Utdata (till Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Om ingen ändring behövs är `updatedInput.command` tom och Claude Code använder det ursprungliga kommandot.

## Fasdetaljer

### Fas 1: Tolka JSON

Läser det fullständiga JSON-objektet från stdin. Extraherar `tool_input.command`. Om tolkningen misslyckas avslutas hook omedelbart och Claude Code använder det ursprungliga kommandot (fail-open design).

### Fas 2: Färdighetsmatchning

Frågar SQLite-heuristikdatabasen efter färdigheter vars triggermönster matchar kommandot. Färdigheter kontrolleras i prioritetsordning. Både inbyggda TOML-färdigheter och inlärda färdigheter utvärderas.

### Fas 3: Katalogkorrigering

För byggkommandon (`cargo`, `go`, `make`, `npm`, `python` etc.) kontrolleras om den förväntade projektfilen finns i aktuell katalog. Om inte, skannas närliggande kataloger efter närmaste matchning och `cd <dir> &&` läggs till framför.

Katalogskanningen använder ett cachat filsystemindex med 5-sekunders TTL för att förbli snabb.

### Fas 4: GDB-kontroll

Om kommandot sannolikt producerar en krasch (t.ex. körning av en debugbinärfil) kan PRECC föreslå eller injicera GDB-wrappers för att fånga strukturerad debugutdata istället för råa kraschloggar.

### Fas 5: RTK-omskrivning

Tillämpar RTK (Rewrite Toolkit)-regler som förkortar ordrika kommandon, undertrycker brusig utdata eller omstrukturerar kommandon för tokeneffektivitet.

### Fas 6: Generera JSON

Serialiserar det modifierade kommandot tillbaka till JSON och skriver det till stdout. Om inga ändringar gjordes signalerar utdatan till Claude Code att använda det ursprungliga kommandot.

## Prestanda

Hela pipeline slutförs på under 5 millisekunder (p99). Nyckeloptimeringar:

- SQLite i WAL-läge för låsfria samtidiga läsningar
- Förkompilerade regex-mönster för färdighetsmatchning
- Cachade filsystemscanningar (5-sekunders TTL)
- Inga nätverksanrop i den kritiska sökvägen
- Fail-open: alla fel faller igenom till det ursprungliga kommandot

## Manuell test av hook

Du kan anropa hook direkt:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
