# Hook Pipeline

`precc-hook` keyrsluskráin er kjarni PRECC. Hún situr á milli Claude Code og skeljarinnar og vinnur úr hverri bash-skipun á undir 5 millisekúndum.

## Hvernig Claude Code kallar á hook

Claude Code styður PreToolUse hooks -- ytri forrit sem geta skoðað og breytt tólsinntökum fyrir keyrslu. Þegar Claude ætlar að keyra bash-skipun sendir það JSON til `precc-hook` á stdin og les svarið frá stdout.

## Pipeline-stig

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

## Dæmi: JSON inntak og úttak

### Inntak (frá Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC greinir að núverandi mappa hefur ekki `Cargo.toml`, en `./myapp/Cargo.toml` er til.

### Úttak (til Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Ef engrar breytingar er þörf er `updatedInput.command` tómt og Claude Code notar upprunalegu skipunina.

## Smáatriði stiga

### Stig 1: Þátta JSON

Les fullt JSON-hlut frá stdin. Dregur út `tool_input.command`. Ef þáttun mistekst hættir hook strax og Claude Code notar upprunalegu skipunina (fail-open hönnun).

### Stig 2: Þekkingarsamsvarun

Fyrirspurnir í SQLite-heuristics gagnagrunn eftir þekkingu þar sem triggermynstur passar við skipunina. Þekking er skoðuð í forgangsröð. Bæði innbyggð TOML-þekking og lærð þekking er metin.

### Stig 3: Möppuleiðrétting

Fyrir smíðaskipanir (`cargo`, `go`, `make`, `npm`, `python` o.fl.) athugar hvort vænt verkefnisskrá er til í núverandi möppu. Ef ekki, skannar nálægar möppur eftir næstu samsvarun og bætir `cd <dir> &&` framan á.

Möppuskönnunin notar vistuð skráarkerfisyfirlit með 5-sekúndna TTL til að vera hröð.

### Stig 4: GDB-athugun

Ef skipunin er líkleg til að valda hruni (t.d. keyrsla á kembikeyrsluskrá), getur PRECC stungið upp á eða sett inn GDB-umbúðir til að fanga skipulögð kembiúttak í stað hrárra hrunskráa.

### Stig 5: RTK-endurskrifun

Beitir RTK (Rewrite Toolkit) reglum sem stytta orðmargar skipanir, bæla niður hávaðasamt úttak eða endurskipuleggja skipanir fyrir táknaskilvirkni.

### Stig 6: Gefa út JSON

Raðar breyttri skipun aftur í JSON og skrifar á stdout. Ef engar breytingar voru gerðar gefur úttakið Claude Code merki um að nota upprunalegu skipunina.

## Afköst

Allt pipeline lýkur á undir 5 millisekúndum (p99). Lykilhagræðingar:

- SQLite í WAL-ham fyrir læsingarlausar samhliða lesanir
- Fyrirfram þýdd regex-mynstur fyrir þekkingarsamsvarun
- Vistuð skráarkerfisskön (5-sekúndna TTL)
- Engin netköll á skiptu leiðinni
- Fail-open: öll villa fer í gegn til upprunalegu skipunarinnar

## Handvirk prófun á hook

Þú getur kallað á hook beint:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
