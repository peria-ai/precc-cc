# FAQ

## Is PRECC safe to use?

Yes. PRECC uses the official Claude Code PreToolUse hook mechanism -- the same extension point that Anthropic designed for exactly this purpose. The hook:

- Runs entirely offline (no network calls in the hot path)
- Completes in under 5 milliseconds
- Is fail-open: if anything goes wrong, the original command runs unmodified
- Only modifies commands, never executes them itself
- Stores data locally in SQLite databases

## Does PRECC work with other AI coding tools?

PRECC is designed specifically for Claude Code. It relies on the PreToolUse hook protocol that Claude Code provides. It does not work with Cursor, Copilot, Windsurf, or other AI coding tools.

## What data does telemetry send?

Telemetry is opt-in only. When enabled, it sends:

- PRECC version, OS, and architecture
- Aggregate counts (commands intercepted, skills activated)
- Average hook latency

It does **not** send command text, file paths, project names, or any personally identifiable information. You can preview the exact payload with `precc telemetry preview` before opting in. See [Telemetry](telemetry.md) for full details.

## How do I uninstall PRECC?

??faq_uninstall_a_intro??

1. Remove the hook registration:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Remove the binary:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Remove data (optional):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## My license expired. What happens?

PRECC reverts to the Community tier. All core functionality continues to work:

- Built-in skills remain active
- Hook pipeline runs normally
- `precc savings` shows the summary view
- `precc ingest` and session mining work

Pro features become unavailable until you renew:

- `precc savings --all` (detailed breakdown)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Email reports

## The hook does not seem to be running. How do I debug?

??faq_debug_a_intro??

1. Check that the hook is registered:
   ```bash
   precc init
   ```

2. Test the hook manually:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Check that the binary is on your PATH:
   ```bash
   which precc-hook
   ```

4. Check Claude Code's hook configuration in `~/.claude/settings.json`.

## Does PRECC slow down Claude Code?

No. The hook completes in under 5 milliseconds (p99). This is imperceptible compared to the time Claude spends reasoning and generating responses.

## Can I use PRECC in CI/CD?

PRECC is designed for interactive Claude Code sessions. In CI/CD, there is no Claude Code instance to hook into. However, `precc gha` can analyze failed GitHub Actions runs from any environment.

## How do mined skills differ from built-in skills?

Built-in skills ship with PRECC and cover common wrong-directory patterns. Mined skills are learned from your specific session logs -- they capture patterns unique to your workflow. Both are stored in SQLite and evaluated identically by the hook pipeline.

## Can I share skills with my team?

Yes. Export any skill to TOML with `precc skills export NAME` and share the file. Team members can place it in their `skills/` directory or import it into their heuristics database.
