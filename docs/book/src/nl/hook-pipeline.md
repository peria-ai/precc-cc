# Hook Pipeline

Het `precc-hook` binary is de kern van PRECC. Het zit tussen Claude Code en de shell en verwerkt elk bash-commando in minder dan 5 milliseconden.

## Hoe Claude Code de Hook aanroept

Claude Code ondersteunt PreToolUse hooks -- externe programma's die tool-invoer kunnen inspecteren en wijzigen vóór uitvoering. Wanneer Claude een bash-commando gaat uitvoeren, stuurt het JSON naar `precc-hook` op stdin en leest het antwoord van stdout.

## Pipeline Stappen

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

## Voorbeeld: JSON invoer en uitvoer

### Invoer (van Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC detecteert dat de huidige map geen `Cargo.toml` heeft, maar `./myapp/Cargo.toml` wel bestaat.

### Uitvoer (naar Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Als er geen wijziging nodig is, is `updatedInput.command` leeg en gebruikt Claude Code het oorspronkelijke commando.

## Stap Details

### Stap 1: JSON Parsen

Leest het volledige JSON-object van stdin. Extraheert `tool_input.command`. Als het parsen mislukt, stopt de hook onmiddellijk en gebruikt Claude Code het oorspronkelijke commando (fail-open ontwerp).

### Stap 2: Skill Matching

Bevraagt de SQLite heuristics-database voor skills waarvan het triggerpatroon overeenkomt met het commando. Skills worden in prioriteitsvolgorde gecontroleerd. Zowel ingebouwde TOML-skills als geminde skills worden geëvalueerd.

### Stap 3: Directory Correctie

Voor build-commando's (`cargo`, `go`, `make`, `npm`, `python`, enz.) controleert of het verwachte projectbestand bestaat in de huidige map. Zo niet, scant nabijgelegen mappen voor de dichtstbijzijnde match en voegt `cd <dir> &&` toe aan het begin.

De directoryscan gebruikt een gecachte bestandssysteemindex met een 5 seconden TTL om snel te blijven.

### Stap 4: GDB Controle

Als het commando waarschijnlijk een crash veroorzaakt (bijv. het uitvoeren van een debug binary), kan PRECC GDB-wrappers voorstellen of injecteren om gestructureerde debug-uitvoer vast te leggen in plaats van ruwe crash-logs.

### Stap 5: RTK Herschrijving

Past RTK (Rewrite Toolkit) regels toe die uitgebreide commando's verkorten, ruis in uitvoer onderdrukken of commando's herstructureren voor token-efficiëntie.

### Stap 6: JSON Uitvoer

Serialiseert het gewijzigde commando terug naar JSON en schrijft het naar stdout. Als er geen wijzigingen zijn aangebracht, signaleert de uitvoer Claude Code om het oorspronkelijke commando te gebruiken.

## Prestaties

De volledige pipeline wordt voltooid in minder dan 5 milliseconden (p99). Belangrijke optimalisaties:

- SQLite in WAL-modus voor lockvrije gelijktijdige leesbewerkingen
- Voorgecompileerde regex-patronen voor skill matching
- Gecachte bestandssysteemscans (5 seconden TTL)
- Geen netwerkoproepen in het hot path
- Fail-open: elke fout valt terug op het oorspronkelijke commando

## De Hook handmatig testen

U kunt de hook direct aanroepen:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
