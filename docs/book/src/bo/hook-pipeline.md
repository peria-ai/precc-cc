# Hook Pipeline

The `precc-hook` binary is the core of PRECC. It sits between Claude Code and the shell, processing every bash command in under 5 milliseconds.

## How Claude Code Invokes the Hook

Claude Code supports PreToolUse hooks -- external programs that can inspect and modify tool inputs before execution. When Claude is about to run a bash command, it sends JSON to `precc-hook` on stdin and reads the response from stdout.

## Pipeline Stages

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

## Example: JSON Input and Output

### Input (from Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC detects that the current directory has no `Cargo.toml`, but `./myapp/Cargo.toml` exists.

### Output (to Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

If no modification is needed, `updatedInput.command` is empty and Claude Code uses the original command.

## Stage Details

### Stage 1: Parse JSON

Reads the full JSON object from stdin. Extracts `tool_input.command`. If parsing fails, the hook exits immediately and Claude Code uses the original command (fail-open design).

### Stage 2: Skill Matching

Queries the SQLite heuristics database for skills whose trigger pattern matches the command. Skills are checked in priority order. Both built-in TOML skills and mined skills are evaluated.

### Stage 3: Directory Correction

For build commands (`cargo`, `go`, `make`, `npm`, `python`, etc.), checks whether the expected project file exists in the current directory. If not, scans nearby directories for the closest match and prepends `cd <dir> &&`.

The directory scan uses a cached filesystem index with a 5-second TTL to stay fast.

### Stage 4: GDB Check

If the command is likely to produce a crash (e.g., running a debug binary), PRECC can suggest or inject GDB wrappers to capture structured debug output instead of raw crash logs.

### Stage 5: RTK Rewriting

Applies RTK (Rewrite Toolkit) rules that shorten verbose commands, suppress noisy output, or restructure commands for token efficiency.

### Stage 6: Emit JSON

Serializes the modified command back to JSON and writes it to stdout. If no changes were made, the output signals Claude Code to use the original command.

## Performance

The entire pipeline completes in under 5 milliseconds (p99). Key optimizations:

- SQLite in WAL mode for lock-free concurrent reads
- Pre-compiled regex patterns for skill matching
- Cached filesystem scans (5-second TTL)
- No network calls in the hot path
- Fail-open: any error falls through to the original command

## Testing the Hook Manually

You can invoke the hook directly:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
