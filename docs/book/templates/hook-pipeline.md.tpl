# {{i18n:hp_title}}

{{i18n:hp_intro}}

## {{i18n:hp_invocation_title}}

{{i18n:hp_invocation_body}}

## {{i18n:hp_stages_title}}

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

## {{i18n:hp_example_title}}

### {{i18n:hp_example_input}}

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

{{i18n:hp_example_explanation}}

### {{i18n:hp_example_output}}

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

{{i18n:hp_no_modification_note}}

## {{i18n:hp_stage_details_title}}

### {{i18n:hp_stage1_title}}

{{i18n:hp_stage1_body}}

### {{i18n:hp_stage2_title}}

{{i18n:hp_stage2_body}}

### {{i18n:hp_stage3_title}}

{{i18n:hp_stage3_body}}

{{i18n:hp_stage3_cache_note}}

### {{i18n:hp_stage4_title}}

{{i18n:hp_stage4_body}}

### {{i18n:hp_stage5_title}}

{{i18n:hp_stage5_body}}

### {{i18n:hp_stage6_title}}

{{i18n:hp_stage6_body}}

## {{i18n:hp_performance_title}}

{{i18n:hp_performance_intro}}

- {{i18n:hp_perf_wal}}
- {{i18n:hp_perf_regex}}
- {{i18n:hp_perf_cache}}
- {{i18n:hp_perf_no_network}}
- {{i18n:hp_perf_failopen}}

## {{i18n:hp_testing_title}}

{{i18n:hp_testing_body}}

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
