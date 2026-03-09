# PRECC Pitch Narratives — v2 (March 2026)

Five standalone sections, one per audience. Each is independently extractable.
Use the same terminal demo (`demo/demo.sh`) for all audiences — swap the spoken narrative.

**New in this revision:** skill portfolio (multi-skill per command), RTK v0.27.2 sync
(30+ new command rewrites), jj/Jujutsu translation, SMTP mail-agent, license key system,
and 万界方舟 pricing analysis (52% of Anthropic list price via API proxy).

---

## A. Technical VC / Angel Investor

### The Insight

Every Claude Code session involves Claude running bash commands. A significant fraction
fail immediately — wrong directory, missing dependency, wrong tool. Claude reads the
error, burns context tokens understanding it, retries. The failure-retry loop costs
tokens and accomplishes nothing.

PRECC intercepts each command *before* execution. In 2.93ms average, it rewrites the
command to succeed on the first try. But it does more than fix directories.

### What's New: Skill Portfolio

The original PRECC applied only the *first* matching correction per command.
The new portfolio engine applies **all compatible high-confidence skills simultaneously**:

```
Command: cargo clippy
  → prepend_cd (wrong dir, conf=0.92)   ← Pillar 1
  → suggest_fix: run warn-identify      ← Pillar 4
  → RTK: rtk cargo clippy               ← RTK compression
```

Three improvements from one command. Previously: only the cd-prepend fired.
Now: all three stack. Token savings compound.

### What's New: jj/Jujutsu Translation

PRECC now detects Jujutsu-colocated repos (`.jj/` present) and silently translates
git commands to their jj equivalents:

```
git status   → jj st         (30% shorter output)
git add .    → true          (jj stages implicitly — zero tokens)
git commit   → jj commit
git checkout → jj edit
```

`git add` becomes a no-op with an explanatory comment — the model never reads the
staging output because there is no staging. This alone saves ~40 tokens per add cycle
in jj repos.

### What's New: RTK v0.27.2 Sync

30 new command rewrites ported from upstream RTK v0.27.2:
`golangci-lint`, `ruff`, `mypy`, `aws`, `psql`, `find`, `tree`, `diff`,
`wget`, `docker exec`, `docker compose`, `git worktree`, `gh api/repo/release`, etc.

Every new rule is a new token-saving opportunity, automatically applied.

### Architecture

```
stdin (JSON)
     │
     ▼
┌─────────────────────────────────────────────────────────┐
│  precc-hook  (Rust, release binary)                     │
│                                                         │
│  Stage 1: Parse hook JSON from stdin                    │
│  Stage 2: Skills portfolio (heuristics.db RO)           │
│           └─ ALL matching skills, conf ≥ 0.7            │
│           └─ conflict rules: 1× prepend_cd, 1× rewrite  │
│           └─ suggest_fix: always additive               │
│  Stage 3: Context resolution (Pillar 1)                 │
│           └─ walk up to find Cargo.toml / package.json  │
│  Stage 4: GDB opportunity check (Pillar 2)              │
│  Stage 5: jj translation (if .jj/ present)             │
│  Stage 6: RTK output compression (30+ rules)           │
│                                                         │
│  Fail-open: any error → exit 0, unchanged              │
└─────────────────────────────────────────────────────────┘
     │
     ▼
stdout (modified JSON or silent)
```

### Technical Moat

**Latency:** 2.93ms average. Portfolio evaluation adds <0.2ms (HashSet dedup is O(1)).

**Learning moat:** `heuristics.db` grows with every session mined. Skills promoted from
candidate → active → trusted as confidence increases. The portfolio compounds: each
new skill stacks on top of existing ones rather than replacing them.

**License protection:** HMAC-SHA256 key system with machine fingerprinting. Pro/Team/Enterprise
tiers. Keys are machine-bound (4-byte SHA-256 fingerprint of hostname+username). The
build-time secret `PRECC_LICENSE_SECRET` is injected at CI build time — not in source.

**Mail-agent:** `precc mail report <email>` delivers savings reports via SMTP directly from
the CLI. Teams can automate weekly reports without dashboards.

**万界方舟 pricing leverage:** Chinese API proxy offering Claude Sonnet 4.6 at 52% of
Anthropic list price. PRECC + 万界方舟 stack multiplicatively:
`$878 × 0.52 (proxy) × 0.66 (PRECC) = $301` vs. $878 baseline — **66% total savings.**
PRECC is the efficiency layer that makes cheaper API proxies even more effective.

### Measured Numbers

| Metric | Measured | Methodology |
|--------|----------|-------------|
| Cost savings | 34% ($296/$878) | 29 sessions, real billing data |
| Failures prevented | 98% (352/358) | Exit-code tracking |
| Commands improved | 17% (894/5,384) | Hook activation log |
| Hook p99 latency | < 5ms | 1,000 activations |
| New rewrites (RTK sync) | +30 rules | v0.22 → v0.27.2 |
| jj token savings (git add) | ~40 tokens/call | jj staging model |

---

## B. Business VC / Angel Investor

### The Problem in One Sentence

Every Claude Code developer wastes 34% of their API budget on commands that fail
and retry — PRECC prevents it, automatically, in 3ms.

### The Numbers That Matter

**Baseline (29 real sessions, 5 projects):**
- API spend without PRECC: $878
- API spend with PRECC: $582
- Saved: $296 (34%)

**With 万界方舟 API proxy (52% of Anthropic pricing):**
- $878 × 0.52 × 0.66 = **$301 total** vs. $878 baseline
- Combined saving: **66%** — two independent multipliers

### Business Model

**Open-core (current):**
Core error prevention is open source. Monetisation layers:
1. **License keys** (Pro/Team/Enterprise) — HMAC-SHA256, machine-bound, tier-gated
2. **Skills marketplace** — teams publish/subscribe correction packages for their stacks
3. **Mail-agent** — automated weekly savings reports to procurement teams, enabling
   data-driven renewal decisions

**Per-seat SaaS target:** $10–20/month per developer.
- At 34% savings on $200/month Claude Pro: **3–7× ROI on subscription**
- At 66% savings with proxy: **10–20× ROI**

**Enterprise:** Centralized dashboard, team heuristics sync, SOC-2 audit log, SMTP
report delivery. One install command, zero reconfiguration of existing Claude Code.

### Market

| Segment | Size | PRECC's Angle |
|---------|------|---------------|
| Claude Code Pro | $200/month × growing install base | 34% savings day one |
| Enterprise Claude API | $1,000+/month per team | 34-66% savings + audit |
| API-proxy users (万界方舟 etc.) | 52% price, needs efficiency | PRECC compounds savings |

### Why Now

Claude Code is the fastest-growing AI coding tool. Enterprise adoption is accelerating.
The wrong-directory problem scales linearly with usage: more sessions → more failures →
more waste. PRECC's fixes also scale linearly, and the learning compounds.

---

## C. Enterprise CTO / Buyer

### The Problem PRECC Solves for Your Teams

Your developers are running Claude Code. 17% of all bash commands need correction.
98% of failures are preventable. Your teams are paying for the retry loop.

**New protections since v0.1:**

**License key system:** Machine-bound HMAC-SHA256 keys enforce tier entitlements
(Community / Pro / Team / Enterprise). Keys are tied to `SHA-256(hostname+username)` —
a license key copied to another machine fails silently. Your seat count is auditable.

**SMTP reporting:** `precc mail setup` + `precc mail report <finance@yourco.com>`
delivers per-engineer savings reports directly to procurement. No dashboard required.
Reports include: commands corrected, tokens saved, dollar estimate.

### Security Architecture

| Concern | Mitigation |
|---------|-----------|
| Data at rest | AES-256 via SQLCipher; key derived from machine-id+username via HKDF-SHA256 |
| Network calls | Zero at runtime; SMTP only when explicitly invoked |
| Fail-open | Hook crash → Claude Code continues unaffected (exit 0) |
| License leakage | Machine-bound keys; copying binary ≠ copying entitlement |
| Audit | `precc report` exports per-command audit log; `precc mail report` delivers it |
| Binary hardening | `obfstr` compile-time string obfuscation; `strip = true`; `lto = true` |

### Deployment

```bash
# Install (all platforms, one command)
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash
precc init

# Activate enterprise license
precc license activate PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX
precc license status

# Configure email reporting
precc mail setup      # creates ~/.config/precc/mail.toml
precc mail report procurement@yourco.com
```

**Supported platforms:** Linux x86_64/ARM64, macOS Intel/Apple Silicon, Windows x86_64.

### ROI by Headcount

| Team size | Monthly Claude spend | PRECC saving (34%) | PRECC saving (66% w/ proxy) |
|-----------|---------------------|--------------------|-----------------------------|
| 10 devs | $2,000 | $680/month | $1,320/month |
| 50 devs | $10,000 | $3,400/month | $6,600/month |
| 100 devs | $20,000 | $6,800/month | $13,200/month |

Auditable, per-machine, emailable.

---

## D. Developer Advocate / DevRel Partner

### The Wow Moment

```bash
# In /tmp, Claude issues:
cargo build
# Without PRECC: error: could not find Cargo.toml (200 token read, retry)
# With PRECC:    cd /path/to/project && rtk cargo build  (silent, 3ms)
```

But there are now *three* wow moments stacked:

**Wow 1 — Context fix:** PRECC finds the right project root in <3ms.

**Wow 2 — Portfolio:** Multiple skills fire simultaneously.
`cargo clippy` now gets: cd-fix + warn-identify suggestion + RTK compression — all in one hook call.

**Wow 3 — jj translation:** In a Jujutsu repo, `git add .` becomes
`true # jj: changes are implicitly staged` — zero tokens, model instantly understands why.
`git status` becomes `jj st` — shorter output, fewer tokens consumed.

### What's Shareable

**New commands to demo:**

```bash
# License management
precc license status
precc license activate PRECC-XXXXXXXX-...
precc license fingerprint   # machine tag for generating bound keys

# Email reports
precc mail setup            # configure SMTP
precc mail report you@team.com --attach report.pdf

# jj workflow (in a colocated repo)
# Automatically: git status → jj st, git add → no-op, git commit → jj commit

# GIF generation (for your own demos!)
precc gif demo/gif/dev.sh 60s
```

**The portfolio story:** "PRECC used to apply one fix per command. Now it applies all
compatible fixes in a single pass — context resolution, skill suggestions, and RTK
compression stack on top of each other. The savings compound."

### Skills to Highlight

| Skill | What it does |
|-------|-------------|
| `cargo-wrong-dir` | Finds Cargo.toml and prepends cd |
| `warn-identify` | After `cargo clippy`, ranks warnings by frequency |
| `zerowarns` | Runs auto-fix then suggests semantic fixes |
| `jj-translate` | In jj repos, suggests jj equivalents to git commands |
| `mail-report` | After `precc report`, suggests emailing results |
| `asciinema-gif` | Intercepts `asciinema rec`, routes through `precc gif` |

### The 万界方舟 Angle (for Chinese developer community)

万界方舟 offers Claude Sonnet 4.6 at 52% of official price via `ANTHROPIC_BASE_URL`.
PRECC makes that cheaper proxy *even more efficient*:

```
Official price:              $878
+ 万界方舟 proxy (52%):     $456
+ PRECC on top (66%):       $301
```

Frame it as: "Choose your API source. PRECC maximises whatever you're paying for."

---

## E. End-User (Claude Code Power User)

### One Line to Install

```bash
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash
precc init
```

No configuration. No restart. Your next Claude Code session: PRECC is active.

### What's New Since You Last Looked

**Skill portfolio:** PRECC now applies *all* relevant fixes per command, not just the
first. You get context-fix + RTK compression + a suggestion, all in one hook call.

**jj support:** Use Jujutsu? PRECC auto-translates your git habits to jj commands.
`git add` becomes a silent no-op (jj stages everything automatically). `git status`
becomes `jj st` (shorter, fewer tokens). No config needed — PRECC detects `.jj/`.

**More RTK rules:** 30 new commands now get compressed output:
`golangci-lint`, `ruff`, `mypy`, `aws`, `psql`, `find`, `tree`, `diff`, `wget`,
`docker compose`, `git worktree`, `gh api`, and more.

**Email your savings:** Configure SMTP once, then `precc mail report you@email.com`
delivers your savings report directly to your inbox. Good for sharing with your manager
or tracking your own spend over time.

**License tiers:** Community (free, all core features). Pro/Team/Enterprise for
priority support and team sync features. `precc license status` shows your tier.

### Your Numbers After a Week

```bash
precc report    # per-command audit: what was fixed, when, how much saved
precc savings   # dollar breakdown: RTK savings + PRECC savings vs. baseline
precc mail report you@email.com   # email it to yourself
```

Measured baseline: **34% cheaper for the same work.** With a price-discounted API
proxy: up to **66% cheaper.**

### The Compounding Effect

| Sessions | What happens |
|----------|-------------|
| 1–5 | Built-in skills fix wrong-dir, RTK compresses output |
| 5–20 | `precc ingest --all` mines your history; new skills emerge |
| 20+ | Portfolio of skills tuned to *your* projects and patterns |
| Ongoing | Skill confidence increases; more auto-applies, fewer misses |

PRECC doesn't just save tokens — it learns. The ROI compounds over time, automatically.
