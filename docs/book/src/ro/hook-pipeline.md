# Hook Pipeline

Binarul `precc-hook` este nucleul PRECC. Se poziționează între Claude Code și shell, procesând fiecare comandă bash în mai puțin de 5 milisecunde.

## Cum invocă Claude Code hook-ul

Claude Code suportă PreToolUse hooks -- programe externe care pot inspecta și modifica intrările instrumentelor înainte de execuție. Când Claude urmează să ruleze o comandă bash, trimite JSON la `precc-hook` pe stdin și citește răspunsul de pe stdout.

## Etapele pipeline

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

## Exemplu: intrare și ieșire JSON

### Intrare (de la Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC detectează că directorul curent nu are `Cargo.toml`, dar `./myapp/Cargo.toml` există.

### Ieșire (către Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Dacă nu este necesară nicio modificare, `updatedInput.command` este gol și Claude Code folosește comanda originală.

## Detalii etape

### Etapa 1: Parsare JSON

Citește obiectul JSON complet de pe stdin. Extrage `tool_input.command`. Dacă parsarea eșuează, hook-ul iese imediat și Claude Code folosește comanda originală (design fail-open).

### Etapa 2: Potrivirea abilităților

Interogă baza de date SQLite de euristici pentru abilități al căror tipar de declanșare se potrivește cu comanda. Abilitățile sunt verificate în ordinea priorității. Sunt evaluate atât abilitățile TOML integrate, cât și cele învățate.

### Etapa 3: Corecția directorului

Pentru comenzile de build (`cargo`, `go`, `make`, `npm`, `python` etc.), verifică dacă fișierul de proiect așteptat există în directorul curent. Dacă nu, scanează directoarele din apropiere pentru cea mai apropiată potrivire și adaugă `cd <dir> &&` în față.

Scanarea directorului folosește un index al sistemului de fișiere cu cache cu un TTL de 5 secunde pentru a rămâne rapidă.

### Etapa 4: Verificare GDB

Dacă comanda este probabil să producă o prăbușire (de ex., rularea unui binar de depanare), PRECC poate sugera sau injecta wrapper-e GDB pentru a captura ieșire de depanare structurată în loc de jurnale de prăbușire brute.

### Etapa 5: Rescriere RTK

Aplică regulile RTK (Rewrite Toolkit) care scurtează comenzile verbose, suprimă ieșirea zgomotoasă sau restructurează comenzile pentru eficiența tokenilor.

### Etapa 6: Emitere JSON

Serializează comanda modificată înapoi în JSON și o scrie pe stdout. Dacă nu s-au făcut modificări, ieșirea semnalează Claude Code să folosească comanda originală.

## Performanță

Întregul pipeline se finalizează în mai puțin de 5 milisecunde (p99). Optimizări cheie:

- SQLite în mod WAL pentru citiri concurente fără blocare
- Tipare regex precompilate pentru potrivirea abilităților
- Scanări ale sistemului de fișiere cu cache (TTL de 5 secunde)
- Fără apeluri de rețea pe calea critică
- Fail-open: orice eroare trece la comanda originală

## Testarea manuală a hook-ului

Puteți invoca hook-ul direct:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
