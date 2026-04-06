# Hook Pipeline

Il binario `precc-hook` è il cuore di PRECC. Si posiziona tra Claude Code e la shell, elaborando ogni comando bash in meno di 5 millisecondi.

## Come Claude Code invoca l'hook

Claude Code supporta i PreToolUse hook -- programmi esterni che possono ispezionare e modificare gli input degli strumenti prima dell'esecuzione. Quando Claude sta per eseguire un comando bash, invia JSON a `precc-hook` su stdin e legge la risposta da stdout.

## Fasi della pipeline

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

## Esempio: input e output JSON

### Input (da Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC rileva che la directory corrente non ha `Cargo.toml`, ma `./myapp/Cargo.toml` esiste.

### Output (verso Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Se non è necessaria alcuna modifica, `updatedInput.command` è vuoto e Claude Code usa il comando originale.

## Dettagli delle fasi

### Fase 1: Parsing JSON

Legge l'oggetto JSON completo da stdin. Estrae `tool_input.command`. Se il parsing fallisce, l'hook esce immediatamente e Claude Code usa il comando originale (design fail-open).

### Fase 2: Matching delle skill

Interroga il database SQLite delle euristiche per le skill il cui pattern trigger corrisponde al comando. Le skill vengono controllate in ordine di priorità. Vengono valutate sia le skill TOML integrate che quelle apprese.

### Fase 3: Correzione directory

Per i comandi di build (`cargo`, `go`, `make`, `npm`, `python`, ecc.), verifica se il file di progetto atteso esiste nella directory corrente. In caso contrario, scansiona le directory vicine per la corrispondenza più prossima e prepone `cd <dir> &&`.

La scansione delle directory usa un indice del filesystem con cache e un TTL di 5 secondi per restare veloce.

### Fase 4: Controllo GDB

Se il comando potrebbe produrre un crash (es. esecuzione di un binario di debug), PRECC può suggerire o iniettare wrapper GDB per catturare output di debug strutturato invece di log di crash grezzi.

### Fase 5: Riscrittura RTK

Applica le regole RTK (Rewrite Toolkit) che accorciano i comandi verbose, sopprimono l'output rumoroso o ristrutturano i comandi per l'efficienza dei token.

### Fase 6: Emissione JSON

Serializza il comando modificato in JSON e lo scrive su stdout. Se non sono state apportate modifiche, l'output segnala a Claude Code di usare il comando originale.

## Prestazioni

L'intera pipeline si completa in meno di 5 millisecondi (p99). Ottimizzazioni chiave:

- SQLite in modalità WAL per letture concorrenti senza lock
- Pattern regex pre-compilati per il matching delle skill
- Scansioni del filesystem con cache (TTL di 5 secondi)
- Nessuna chiamata di rete nel percorso critico
- Fail-open: qualsiasi errore passa al comando originale

## Test manuale dell'hook

Puoi invocare l'hook direttamente:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
