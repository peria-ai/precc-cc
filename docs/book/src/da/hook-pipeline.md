# Hook Pipeline

Binærfilen `precc-hook` er kernen i PRECC. Den sidder mellem Claude Code og shellen og behandler hver bash-kommando på under 5 millisekunder.

## Hvordan Claude Code kalder hook

Claude Code understøtter PreToolUse hooks -- eksterne programmer der kan inspicere og ændre værktøjsinput før udførelse. Når Claude er ved at køre en bash-kommando, sender den JSON til `precc-hook` på stdin og læser svaret fra stdout.

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

## Eksempel: JSON input og output

### Input (fra Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC registrerer, at den aktuelle mappe ikke har `Cargo.toml`, men `./myapp/Cargo.toml` findes.

### Output (til Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Hvis ingen ændring er nødvendig, er `updatedInput.command` tom, og Claude Code bruger den originale kommando.

## Fasedetaljer

### Fase 1: Parse JSON

Læser det fulde JSON-objekt fra stdin. Udtrækker `tool_input.command`. Hvis parsing fejler, afsluttes hook øjeblikkeligt, og Claude Code bruger den originale kommando (fail-open design).

### Fase 2: Færdighedsmatching

Forespørger SQLite-heuristikdatabasen for færdigheder, hvis triggermønster matcher kommandoen. Færdigheder kontrolleres i prioritetsrækkefølge. Både indbyggede TOML-færdigheder og lærte færdigheder evalueres.

### Fase 3: Mappekorrektion

For build-kommandoer (`cargo`, `go`, `make`, `npm`, `python` osv.) kontrolleres, om den forventede projektfil findes i den aktuelle mappe. Hvis ikke, scannes nærliggende mapper for det nærmeste match, og `cd <dir> &&` sættes foran.

Mappescanningen bruger et cachelagret filsystemindeks med et 5-sekunders TTL for at forblive hurtig.

### Fase 4: GDB-kontrol

Hvis kommandoen sandsynligvis producerer et nedbrud (f.eks. kørsel af en debug-binærfil), kan PRECC foreslå eller indsætte GDB-wrappers for at fange struktureret debug-output i stedet for rå crash-logfiler.

### Fase 5: RTK-omskrivning

Anvender RTK (Rewrite Toolkit)-regler der forkorter omstændelige kommandoer, undertrykker støjende output eller omstrukturerer kommandoer for tokeneffektivitet.

### Fase 6: Udsend JSON

Serialiserer den ændrede kommando tilbage til JSON og skriver den til stdout. Hvis ingen ændringer blev foretaget, signalerer outputtet Claude Code at bruge den originale kommando.

## Ydeevne

Hele pipeline fuldfører på under 5 millisekunder (p99). Nøgleoptimeringer:

- SQLite i WAL-tilstand for låsefri samtidige læsninger
- Prækompilerede regex-mønstre til færdighedsmatching
- Cachelagrede filsystemscanninger (5-sekunders TTL)
- Ingen netværkskald i den kritiske sti
- Fail-open: enhver fejl falder igennem til den originale kommando

## Manuel test af hook

Du kan kalde hook direkte:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
