---
name: precc-token-saver
description: Before running any shell command, pipe it through the PRECC hook to fix wrong-directory errors, compress CLI output, and apply learned heuristics — saving 34%+ of API token spend automatically.
version: 0.1.3
emoji: ⚡
user-invocable: true
disable-model-invocation: false
metadata:
  openclaw:
    emoji: ⚡
    homepage: https://github.com/yijunyu/precc-cc
    os:
      - darwin
      - linux
    requires:
      bins:
        - precc-hook
        - precc
    install:
      - kind: brew
        tap: yijunyu/tap
        formula: precc
        bins: [precc, precc-hook]
---

# PRECC Token Saver

PRECC (Predictive Error Correction) intercepts every shell command before it runs.
In under 3ms it:

- **Fixes wrong-directory errors** — detects the correct project root and prepends `cd /correct/path &&`
- **Compresses CLI output** — rewrites commands through RTK (60–90% smaller output)
- **Applies learned skills** — uses patterns mined from past sessions to prevent repeated failures
- **Translates git→jj** — auto-converts git commands in Jujutsu repos (saves ~40 tokens per `git add`)

**Measured results** (29 real sessions, 5,384 bash calls):
- 34% cost saving ($296 saved from $878 baseline)
- 98% of failures prevented (352/358)
- 2.93ms average hook latency

## How it works in OpenClaw

When PRECC Token Saver is active, every `shell` tool call is first piped through
`precc-hook`. The hook reads the command as JSON on stdin and writes a (possibly
rewritten) command back to stdout. OpenClaw then executes the rewritten command.

```
OpenClaw agent wants to run: cargo build
     ↓
precc-hook receives:  {"tool_input": {"command": "cargo build"}}
precc-hook outputs:   {"hookSpecificOutput": {"updatedInput": {"command": "cd /path/to/project && rtk cargo build"}}}
     ↓
OpenClaw executes:    cd /path/to/project && rtk cargo build
```

## Setup

```bash
# Install PRECC
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash

# Initialise (one-time)
precc init

# Mine existing session history (optional)
precc ingest --all
```

## Commands

Once the skill is loaded, you can address the agent directly:

- `@precc report` — show token savings and command statistics
- `@precc skills list` — list active correction skills
- `@precc update` — update PRECC binaries to the latest release
- `@precc savings` — show dollar-value savings estimate

## Skill behaviour

This skill instructs the agent to:

1. **Always** route shell/bash tool calls through `precc-hook` by constructing the
   hook JSON payload and parsing the response before execution.
2. **On session start**, run `precc init` if the databases are absent.
3. **On session end**, optionally run `precc report` and summarise savings.
4. **Never** skip the hook even for simple commands — the latency is <3ms and the
   hook is fail-open (any error → original command is used unchanged).

## Hook invocation pattern

```bash
# Wrap any command through precc-hook:
echo '{"tool_input":{"command":"YOUR_COMMAND_HERE"}}' | precc-hook
```

Parse the JSON response:
- If `.hookSpecificOutput.updatedInput.command` is non-empty → use it
- Otherwise → use the original command unchanged

## Advanced: savings report

```bash
precc report          # full analytics dashboard
precc savings         # dollar-value breakdown
precc skills list     # active skills
precc skills show <name>   # detail for one skill
```

## License

MIT — https://github.com/yijunyu/precc-cc
