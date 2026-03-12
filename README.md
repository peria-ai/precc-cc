# PRECC — Predictive Error Correction for Claude Code

PRECC saves **~34% of Claude Code costs** by fixing bash commands before they fail and compressing tool output.

## Install

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash
precc init
```

**Windows (PowerShell):**

```powershell
iwr -useb https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.ps1 | iex
```

Also available via [OpenClaw](https://clawhub.ai/skills/precc-token-saver) (`clawdhub install precc-token-saver`), [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) (`zeroclaw skills install https://github.com/yijunyu/precc-cc`), or the Claude Code plugin marketplace (`claude plugin install precc`).

## How It Works

Once installed, PRECC intercepts every bash command Claude Code runs through a <3ms hook. No configuration needed.

- **Fixes wrong-directory commands** — auto-prepends `cd /correct/path &&` when cargo, npm, go, python, git, or make runs outside its project root
- **Prevents repeated failures** — learns from past sessions and auto-corrects commands that would fail the same way
- **Compresses output** — rewrites commands via [RTK](https://github.com/rtk-ai/rtk) for 60-90% smaller output
- **Optimizes Rust workflows** — caches `cargo doc`, substitutes `cargo check` for `cargo build`, and slices test output to show only failures

## Usage

```bash
precc report                # view savings dashboard
precc skills list           # see all active skills
precc skills advise         # get suggestions for new skills
precc savings               # detailed token savings breakdown (Pro)
```

### Skill advisor

PRECC learns from your usage and suggests ways to save more:

```bash
precc skills advise         # suggest new skills, flag ineffective ones
```

The advisor analyzes your failure patterns, identifies skills with high failure rates, and recommends token optimizations (output slicing, caching, command substitution). It can auto-disable broken skills and let you accept suggestions in one command:

```bash
precc skills advise --auto-disable    # disable ineffective mined skills
precc skills advise --accept <name>   # create a suggested skill
```

### Skill sharing

Share useful skills with your team and earn token credits:

```bash
precc skills advise --share <name>
```

Creators earn 10% of the total token savings their shared skills generate across all adopters.

## Measured Results

| Metric | Value |
|--------|-------|
| **Cost savings** | **$296 / $878 (34%)** |
| **Failures prevented** | **352 / 358 (98%)** |
| **Bash calls improved** | **894 / 5,384 (17%)** |
| **Hook latency** | **2.93ms avg** |

Measured across 29 real sessions, 5 projects, 5,384 bash calls.

## Pricing

| | Free | Pro |
|---|---|---|
| Error correction & output compression | ✓ | ✓ |
| 15 built-in skills (incl. Rust actionbook) | ✓ | ✓ |
| Skill advisor, sharing & reports | ✓ | ✓ |
| Session mining | 1 session | Unlimited |
| Mined skills in hook | 3 max | Unlimited |
| `precc savings` / `precc gif` / `precc mail` | — | ✓ |

```bash
precc license activate PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX
```

## Privacy

PRECC collects **no data by default**. All databases are AES-256 encrypted and machine-bound.

- **Update ping** (opt-out): `precc update` sends version/OS/arch only. Disable with `PRECC_NO_TELEMETRY=1`.
- **Usage analytics** (opt-in): aggregated skill counts and latency — no commands, paths, usernames, or code. Requires explicit `precc telemetry consent`.

```bash
precc telemetry consent     # opt in
precc telemetry preview     # inspect exact payload
precc telemetry revoke      # opt out
```

## Requirements

- Claude Code (with hooks support)
- [RTK](https://github.com/rtk-ai/rtk) (optional, for output compression)

## License

MIT

---

<details>
<summary>Changelog</summary>

### v0.1.9

- Skill advisor (`precc skills advise`): suggests new skills from uncovered failure patterns, flags ineffective skills, recommends token optimizations
- Skill sharing with credit tracking (10% creator earnings on downstream savings)
- Per-skill ablation in `precc report` and `precc savings`
- Opt-in anonymous telemetry with versioned consent
- Rust actionbook skills: `rust-doc-cache`, `rust-check-before-build`, `rust-test-slice`

### v0.1.5

- Anonymous update-check ping (version/OS/arch, opt-out via `PRECC_NO_TELEMETRY=1`)

### v0.1.4

- Free/Pro tier licensing, OpenClaw and ZeroClaw plugins

### v0.1.3

- Self-update, GIF generation, email reports, license key system

### v0.6.0

- Prefix cache pre-filter for <3ms hook latency on non-matching commands

### v0.5.0

- Builtin skills embedded in binary, subagent session mining

### v0.4.0

- Skill export/edit, git wrong-dir skill, GDB debugging pillar

### v0.3.0

- Skill confidence lifecycle (candidate → active → trusted), live metrics

### v0.2.0

- AES-256 database encryption, binary hardening, pre-built releases, install scripts

### v0.1.0

- Initial release: hook pipeline, session mining, built-in skills

</details>
