# Hook Pipeline

A `precc-hook` bináris a PRECC magja. A Claude Code és a shell között helyezkedik el, és minden bash parancsot 5 milliszekundum alatt dolgoz fel.

## Hogyan hívja meg a Claude Code a Hookot

A Claude Code támogatja a PreToolUse hookokat -- külső programokat, amelyek a végrehajtás előtt megvizsgálhatják és módosíthatják az eszközbemeneteket. Amikor a Claude bash parancsot készül futtatni, JSON-t küld a `precc-hook`-nak stdin-en és kiolvassa a választ stdout-ról.

## Pipeline szakaszok

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

## Példa: JSON bemenet és kimenet

### Bemenet (Claude Code-tól)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

A PRECC észleli, hogy az aktuális könyvtárban nincs `Cargo.toml`, de `./myapp/Cargo.toml` létezik.

### Kimenet (Claude Code-nak)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Ha nincs szükség módosításra, az `updatedInput.command` üres, és a Claude Code az eredeti parancsot használja.

## Szakasz részletei

### 1. szakasz: JSON elemzés

Beolvassa a teljes JSON objektumot stdin-ről. Kinyeri a `tool_input.command` értéket. Ha az elemzés sikertelen, a hook azonnal kilép, és a Claude Code az eredeti parancsot használja (fail-open tervezés).

### 2. szakasz: Skill illesztés

Lekérdezi az SQLite heurisztikus adatbázisból azokat a skilleket, amelyek trigger mintája illeszkedik a parancsra. A skillek prioritás szerint kerülnek ellenőrzésre. Mind a beépített TOML skillek, mind a bányászott skillek kiértékelésre kerülnek.

### 3. szakasz: Könyvtárjavítás

Build parancsok (`cargo`, `go`, `make`, `npm`, `python` stb.) esetén ellenőrzi, hogy a várt projektfájl létezik-e az aktuális könyvtárban. Ha nem, a közeli könyvtárakat átvizsgálja a legközelebbi egyezésért és `cd <dir> &&` előtagot ad hozzá.

A könyvtárvizsgálat gyorsítótárazott fájlrendszer-indexet használ 5 másodperces TTL-lel a gyorsaság érdekében.

### 4. szakasz: GDB ellenőrzés

Ha a parancs valószínűleg összeomlást okoz (pl. debug bináris futtatása), a PRECC GDB-wrappereket javasolhat vagy injektálhat strukturált debug kimenet rögzítéséhez a nyers összeomlás-naplók helyett.

### 5. szakasz: RTK átírás

RTK (Rewrite Toolkit) szabályokat alkalmaz, amelyek rövidítik a bőbeszédű parancsokat, elnyomják a zajos kimenetet, vagy átstrukturálják a parancsokat a token-hatékonyság érdekében.

### 6. szakasz: JSON kibocsátás

A módosított parancsot JSON-be sorosítja és stdout-ra írja. Ha nem történt változás, a kimenet jelzi a Claude Code-nak, hogy az eredeti parancsot használja.

## Teljesítmény

A teljes pipeline 5 milliszekundum (p99) alatt fejeződik be. Főbb optimalizációk:

- SQLite WAL módban zárolás nélküli párhuzamos olvasáshoz
- Előre fordított regex minták skill illesztéshez
- Gyorsítótárazott fájlrendszer-vizsgálatok (5 másodperces TTL)
- Nincs hálózati hívás a hot path-ban
- Fail-open: bármely hiba esetén az eredeti parancs fut le

## A Hook manuális tesztelése

A hookot közvetlenül meghívhatja:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
