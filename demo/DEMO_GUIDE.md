# PRECC Demo Guide — 5-Minute Presenter Script

This guide walks you through the `demo/demo.sh` terminal demo. Run the same script
for every audience; swap the spoken narrative using the per-audience callout boxes.

---

## Before You Start

```bash
# One-time setup: build release binary
cargo build --release

# Verify the demo runs cleanly (CI mode)
bash demo/demo.sh --fast --no-real-sessions

# Full live demo with pauses
bash demo/demo.sh
```

**Terminal setup:** 120-column width, font size ≥ 14pt, dark background.
Color output requires a TTY (iTerm, alacritty, gnome-terminal all work).

---

## [0:00] SETUP — `precc init`

**Run:** `bash demo/demo.sh` (let it scroll through Section 0 automatically)

**Say:**
> "PRECC is a Rust binary that hooks into Claude Code's bash execution pipeline.
> The very first thing it does is create isolated, encrypted databases for this session.
> Notice the temp directory — this demo never touches your real data."

**Point at:** The `PRECC_DATA_DIR=/tmp/precc-demo-...` line and the `Encryption: AES-256` output from `precc init`.

> **[ENTERPRISE CTO]** Pause here. Point out: "The key is derived from your machine ID. The database is unreadable on any other machine — your session history never leaves your laptop."

---

## [0:30] THE PROBLEM — Section 1 output

**Run:** Script continues automatically through Section 1.

**Say:**
> "Here's the problem PRECC solves. Claude Code often runs bash commands from the wrong
> directory — there's no Cargo.toml, no package.json. The command fails, Claude reads
> the error, burns tokens understanding it, retries. This costs real money and fills
> Claude's context with noise."

**Point at:** The three red failure lines: `cargo build` → exit 1, `npm test` → exit 1,
`cargo clippy` → exit 0 but with token noise.

> **[TECHNICAL VC]** "Each of these failures is 200–4000 wasted context tokens. Multiply
> by 17% of all bash calls across a project and you're burning cash on nothing."

> **[BUSINESS VC]** "We measured $878 in API spend across 29 sessions. $296 of that was
> recoverable — commands that failed and had to be retried. PRECC prevents the retries."

---

## [1:30] THE HOOK — Section 2

**Run:** Script continues. Watch the transformation output.

**Say:**
> "Here's the hook in action. The exact same three commands pass through precc-hook
> before Claude executes them. The hook resolves the correct project directory and
> rewrites the command in-place — silently, before Claude even sees a failure."

**Point at:** The `Input → Output` pairs. Note: if running from the repo root (which has
a Cargo.toml), the hook may approve cargo commands unchanged — that's correct behavior.
For a visually dramatic transformation, show the JSON output directly:

```bash
echo '{"tool_input":{"command":"cargo build"}}' | \
  PRECC_DATA_DIR=/tmp/demo ./target/release/precc-hook
```

**Point at:** The timing lines (~2–4ms each).

> **[TECHNICAL VC]** "2.93ms average. The Claude Code SDK timeout for hooks is 60 seconds.
> We're using 0.005% of that budget. We leave 59,940ms for your actual work."

> **[DEV ADVOCATE]** "This is the wow moment — a wrong-dir cargo build that would have
> wasted 800 tokens gets silently fixed in 3ms. Your future self is being protected by
> your past failures."

> **[END USER]** "You don't configure anything. PRECC finds your Cargo.toml, your
> package.json, your Makefile. It just works."

---

## [2:30] MINING — Section 3

**Run:** Script continues. Watch `precc ingest demo/session.jsonl` output.

**Say:**
> "PRECC doesn't just fix known patterns — it learns. The mining step reads your past
> Claude Code sessions and extracts failure-fix pairs: 'cargo build failed here,
> then Claude ran cd /app && cargo build and it worked.' That pattern gets stored and
> becomes a new automatic correction."

**Point at:** The mining summary: `X events, Y failure-fix pairs found`.

If real sessions are mined, let them run and point at the count.

> **[TECHNICAL VC]** "The mined patterns are hashed, normalized, and deduplicated.
> Running cargo build from /tmp and from /home/user/other-project both hash to the
> same pattern — so the fix generalizes."

> **[DEV ADVOCATE]** "It gets smarter across every session. Your mistakes today teach
> it to prevent your next mistakes tomorrow. The heuristics.db grows with your history."

> **[ENTERPRISE CTO]** "All mining is local. No data ever leaves the machine. The session
> logs are read-only and the pattern database is encrypted. Your code patterns and
> project structure are yours."

---

## [3:30] THE NUMBERS — Section 4

**Run:** Script continues. `precc report` and `precc skills list` appear.

**Say:**
> "Every PRECC installation tracks its own impact. The report shows hook latency,
> skill activations, failure-fix pairs mined, and estimated token savings. The skills
> list shows all active corrections — both built-in and ones mined from your sessions."

**Point at:** The report output. For the synthetic demo, point at the mined pairs from
`session.jsonl`. If real sessions were available, point at the larger stats.

> **[BUSINESS VC]** Stay on this screen. "This is the audit trail. Every engineer running
> PRECC gets their own `precc report` showing exactly how much was saved. At $200/month
> Claude Code Pro, 34% savings is $68/month per developer, $816/year."

> **[ENTERPRISE CTO]** "The `precc savings` subcommand breaks down token savings by
> category — cache reads, output compression, prevented retries. This is defensible ROI
> for procurement."

---

## [4:30] SECURITY — Section 5

**Run:** Script continues. The `xxd` output of the encrypted database appears.

**Say:**
> "A plain SQLite file starts with the ASCII text 'SQLite format 3'. What you're seeing
> here is the encrypted header — random-looking bytes. This is AES-256 via SQLCipher,
> with the key derived from your machine's unique ID. Drag this database to another
> machine and it will not open."

**Point at:** The hex dump. Specifically note the absence of `53 51 4c 69 74 65` (SQLite magic).

> **[ENTERPRISE CTO]** Pause here for longest. "There are no network calls in the PRECC
> binary. It reads your bash commands and writes to local SQLite files. The attack surface
> is: an attacker would need to compromise the binary itself, which is a standard software
> supply-chain problem — same as any other tool you install. The data at rest is
> machine-bound."

---

## [4:50] CLOSE — Section 6 Summary

**Run:** Script continues to final banner.

**Say:**
> "Those are the production numbers from 29 real Claude Code sessions across 5 projects.
> One line to install. Zero configuration. Works today."

**Point at:** Each of the five metrics in the box.

> **[TECHNICAL VC]** "These aren't projections. These are measured from real billing data."

> **[BUSINESS VC]** "$296 saved from $878. 34%. If your team runs Claude Code at scale,
> the math compounds quickly."

> **[DEV ADVOCATE]** "17% of all bash calls improved — silently, in 3ms. That's not a
> tool you have to remember to use. It's infrastructure."

> **[END USER]** Point at the install line: "One curl. That's it."

---

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `precc-hook not found` | Run `cargo build --release` or pass `--no-build` if binary exists elsewhere |
| Colors not showing | Run in a proper terminal emulator, not a piped shell |
| Real sessions not found | Use `--no-real-sessions` to skip; or point at demo/session.jsonl |
| Slow startup (~500ms) | Using debug binary — switch to `./target/release/precc-hook` |
| `xxd` not available | Install with `sudo apt install xxd` or `brew install xxd`; demo falls back to `od` |
