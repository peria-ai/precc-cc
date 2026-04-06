# ངོ་སྤྲོད།

## PRECC ཅི་ཞིག་ཡིན།

PRECC (Claude Code ཡི་སྔོན་བརྟག་ནོར་འཆུག་བཅོས་སྒྲིག) ནི་ཆོག་ཅན་གྱི PreToolUse hook བརྒྱུད་ནས Claude Code bash བཀའ་རྒྱ་བར་གཅོད་བྱེད་པའི Rust ལག་ཆ་ཞིག་ཡིན། ནོར་འཆུག་*མ་བྱུང་གོང་*བཅོས་སྒྲིག་བྱས་ནས token འགྲོ་གྲོན་ཉུང་དུ་གཏོང་།

སྤྱི་ཚོགས་བཀོལ་མཁན་ལ་རིན་མེད།

## དཀའ་ངལ།

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## རྩ་དོན་བཞི།

### Context Fix (cd-prepend)

Detects when commands like `cargo build` or `npm test` run in the wrong directory and prepends `cd /correct/path &&` before execution.

### GDB Debugging

Detects opportunities to attach GDB for deeper debugging of segfaults and crashes, providing structured debug information instead of raw core dumps.

### Session Mining

Mines Claude Code session logs for failure-fix pairs. When the same mistake recurs, PRECC already knows the fix and applies it automatically.

### Automation Skills

A library of built-in and mined skills that match command patterns and rewrite them. Skills are defined as TOML files or SQLite rows, making them easy to inspect, edit, and share.

## ལས་ཀ་ཇི་ལྟར་བྱེད།

1. Claude Code is about to run a bash command.
2. The PreToolUse hook sends the command to `precc-hook` as JSON on stdin.
3. `precc-hook` runs the command through the pipeline (skills, directory correction, compression) in under 3 milliseconds.
4. The corrected command is returned as JSON on stdout.
5. Claude Code executes the corrected command instead.

Claude never sees the error. No tokens wasted.

### Adaptive Compression

If a command fails after compression, PRECC automatically skips compression on the retry so Claude gets the full uncompressed output to debug with.

## དུས་ཐོག་བཀོལ་སྤྱོད་གྲངས་ཐོ།

Current version <span data-stat="current_version">--</span>:

| ཚད་གཞི། | གྲངས་ཀ |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Unique users | <span data-stat="unique_users">--</span> |

### Savings by Release

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Unique users</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Loading...</em></td></tr></tbody>
</table>

<small>Figures are estimates. Each prevented failure avoids a full retry cycle: error output, model reasoning, and retry command. These numbers update automatically from anonymized telemetry.</small>

## སྦྲེལ་མཐུད།

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
