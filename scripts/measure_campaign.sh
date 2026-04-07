#!/usr/bin/env bash
# measure_campaign.sh — Run safe historical commands through every PRECC
# compression mode and record real measurements.
#
# Approach: bypass the hook entirely. For each safe command class:
#   1. Run the command in basic form, measure output bytes
#   2. Run with rtk (if rtk has a rule), measure
#   3. Run with diet (if diet has a rule), measure
#   4. Run with nu wrapper (if nushell has a rule), measure
#   5. Run with lean-ctx wrapper, measure
#   6. Append to savings_measurements.jsonl with real numbers
#
# Reuses precc-core directly via a Rust binary so we don't fight the hook.

set -euo pipefail

LIMIT="${1:-30}"
TOP_CLASSES="${2:-15}"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

GREEN='\033[0;32m'; CYAN='\033[36m'; YELLOW='\033[0;33m'; NC='\033[0m'

echo -e "${CYAN}=== PRECC Measurement Campaign ===${NC}"
echo "Top ${TOP_CLASSES} classes, max ${LIMIT} measurements"
echo ""

DATA_DIR="${HOME}/.local/share/precc"
SAVINGS_LOG="${DATA_DIR}/savings_measurements.jsonl"
SAVINGS_BEFORE=$(wc -l < "${SAVINGS_LOG}" 2>/dev/null || echo 0)

# Build the campaign runner binary
PROBE_DIR=/tmp/precc_campaign_runner
if [ ! -f "${PROBE_DIR}/target/release/precc_campaign_runner" ] || [ "${PROJECT_DIR}/crates/precc-core/src/post_observe.rs" -nt "${PROBE_DIR}/target/release/precc_campaign_runner" ]; then
    mkdir -p "${PROBE_DIR}/src"
    cat > "${PROBE_DIR}/Cargo.toml" <<EOF
[package]
name = "precc_campaign_runner"
version = "0.0.1"
edition = "2021"
[dependencies]
precc-core = { path = "${PROJECT_DIR}/crates/precc-core" }
serde_json = "1"
EOF
    cat > "${PROBE_DIR}/src/main.rs" <<'EOF'
//! Campaign runner: takes JSON commands on stdin (one per line) of the form
//!   {"cmd_class": "...", "command": "..."}
//! For each, runs the command in every applicable mode and writes
//! measurements to ~/.local/share/precc/savings_measurements.jsonl

use std::io::BufRead;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const TIMEOUT_SECS: u64 = 30;
const MAX_BYTES: usize = 256 * 1024;

fn run(cmd: &str, cwd: &str) -> Option<u64> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
        .ok()?;
    let deadline = Instant::now() + Duration::from_secs(TIMEOUT_SECS);
    loop {
        match child.try_wait().ok()? {
            Some(_) => break,
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
    let out = child.wait_with_output().ok()?;
    let total = out.stdout.len() + out.stderr.len();
    let capped = total.min(MAX_BYTES);
    Some((capped / 4) as u64)
}

fn main() {
    let data_dir = std::path::PathBuf::from(std::env::var("HOME").unwrap())
        .join(".local/share/precc");
    let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let v: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let cmd_class = v.get("cmd_class").and_then(|s| s.as_str()).unwrap_or("");
        let original_cmd = v.get("command").and_then(|s| s.as_str()).unwrap_or("");
        if original_cmd.is_empty() { continue; }

        // Re-check safety with current code
        if !precc_core::post_observe::is_safe_to_rerun(original_cmd) {
            eprintln!("SKIP unsafe: {}", &original_cmd[..original_cmd.len().min(60)]);
            continue;
        }
        let safe_cmd = match precc_core::post_observe::rerunnable_form(original_cmd) {
            Some(c) => c,
            None => continue,
        };

        // 1. Basic
        let basic = match run(&safe_cmd, &cwd) {
            Some(t) => t,
            None => {
                eprintln!("SKIP timeout: {}", &safe_cmd[..safe_cmd.len().min(60)]);
                continue;
            }
        };
        // Skip commands whose output is too small to bother compressing.
        // Default: 250 tokens (~1KB). Override via PRECC_CAMPAIGN_MIN_TOKENS.
        let min_tokens: u64 = std::env::var("PRECC_CAMPAIGN_MIN_TOKENS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(250);
        if basic < min_tokens {
            eprintln!("SKIP small:   {} ({} tokens)", &safe_cmd[..safe_cmd.len().min(60)], basic);
            continue;
        }

        eprint!("MEAS {} basic={}", &cmd_class[..cmd_class.len().min(30)], basic);

        // Self-comparison: basic vs basic (sanity row)
        precc_core::post_observe::append_savings_measurement(
            &data_dir, cmd_class, "campaign-basic", "basic", "probe",
            "campaign", basic, basic, "campaign", 0.0,
        );

        // 2. RTK
        if precc_core::rtk::has_rule(&safe_cmd) {
            let rtk_cmd = format!("rtk {}", safe_cmd);
            if let Some(rtk_tokens) = run(&rtk_cmd, &cwd) {
                let pct = if basic > 0 { (basic.saturating_sub(rtk_tokens)) as f64 / basic as f64 * 100.0 } else { 0.0 };
                eprint!(" rtk={}({}%)", rtk_tokens, pct as i32);
                precc_core::post_observe::append_savings_measurement(
                    &data_dir, cmd_class, "campaign-rtk", "rtk", "probe",
                    "campaign", basic, rtk_tokens, "campaign", 0.0,
                );
            }
        }

        // 3. Diet
        if let Some((diet_cmd, _est)) = precc_core::diet::apply(&safe_cmd) {
            if let Some(diet_tokens) = run(&diet_cmd, &cwd) {
                let pct = if basic > 0 { (basic.saturating_sub(diet_tokens)) as f64 / basic as f64 * 100.0 } else { 0.0 };
                eprint!(" diet={}({}%)", diet_tokens, pct as i32);
                precc_core::post_observe::append_savings_measurement(
                    &data_dir, cmd_class, "campaign-diet", "diet", "probe",
                    "campaign", basic, diet_tokens, "campaign", 0.0,
                );
            }
        }

        // 4. Nushell
        if precc_core::nushell::has_rule(&safe_cmd) {
            if let Some(nu_cmd) = precc_core::nushell::wrap(&safe_cmd) {
                if let Some(nu_tokens) = run(&nu_cmd, &cwd) {
                    let pct = if basic > 0 { (basic.saturating_sub(nu_tokens)) as f64 / basic as f64 * 100.0 } else { 0.0 };
                    eprint!(" nu={}({}%)", nu_tokens, pct as i32);
                    precc_core::post_observe::append_savings_measurement(
                        &data_dir, cmd_class, "campaign-nu", "nushell", "probe",
                        "campaign", basic, nu_tokens, "campaign", 0.0,
                    );
                }
            }
        }

        // 5. Lean-ctx
        if let Some(lc_cmd) = precc_core::lean_ctx::wrap(&safe_cmd) {
            if let Some(lc_tokens) = run(&lc_cmd, &cwd) {
                let pct = if basic > 0 { (basic.saturating_sub(lc_tokens)) as f64 / basic as f64 * 100.0 } else { 0.0 };
                eprint!(" lc={}({}%)", lc_tokens, pct as i32);
                precc_core::post_observe::append_savings_measurement(
                    &data_dir, cmd_class, "campaign-leanctx", "lean-ctx", "probe",
                    "campaign", basic, lc_tokens, "campaign", 0.0,
                );
            }
        }

        eprintln!();
    }
}
EOF
    echo "Building campaign runner..."
    cargo build --release --quiet --manifest-path "${PROBE_DIR}/Cargo.toml" 2>&1 | tail -5
fi

# Step 1: extract top safe candidates
python3 <<PYEOF > /tmp/precc_campaign_targets.jsonl
import json, glob, os
from collections import Counter, defaultdict

class_to_cmds = defaultdict(list)
for path in glob.glob(os.path.expanduser("~/.claude/projects/**/*.jsonl"), recursive=True):
    try:
        with open(path) as f:
            for line in f:
                try: msg = json.loads(line)
                except: continue
                blocks = msg.get("message", {}).get("content", [])
                if not isinstance(blocks, list): continue
                for b in blocks:
                    if isinstance(b, dict) and b.get("type") == "tool_use" and b.get("name") == "Bash":
                        cmd = b.get("input", {}).get("command", "").strip()
                        if not cmd: continue
                        # Skip ssh to remote hosts I likely can't reach
                        if "ssh" in cmd.split()[:2]: continue
                        c = cmd
                        if c.startswith("cd ") and " && " in c:
                            c = c.split(" && ", 1)[1]
                        words = c.split()
                        while words:
                            w = words[0]
                            if "=" in w and w.split("=")[0] and all(ch.isupper() or ch.isdigit() or ch=="_" for ch in w.split("=")[0]):
                                words.pop(0)
                            elif w in ("sudo","time","exec","nohup"):
                                words.pop(0)
                            else: break
                        if not words: continue
                        cls = " ".join(words[:2]) if len(words) >= 2 else words[0]
                        class_to_cmds[cls].append(cmd.replace('\n', ' '))
    except: pass

top = sorted(class_to_cmds.items(), key=lambda x: -len(x[1]))[:${TOP_CLASSES}]
import sys
print(f"Top classes:", file=sys.stderr)
n = 0
for cls, cmds in top:
    if n >= ${LIMIT}: break
    counter = Counter(cmds)
    rep = counter.most_common(1)[0][0]
    print(f"  {len(cmds):5d}  {cls}", file=sys.stderr)
    print(json.dumps({"cmd_class": cls, "command": rep}))
    n += 1
PYEOF

echo ""
echo -e "${CYAN}=== Running campaign ===${NC}"

# Pipe the targets through the campaign runner
cat /tmp/precc_campaign_targets.jsonl | timeout 600 "${PROBE_DIR}/target/release/precc_campaign_runner"

echo ""
SAVINGS_AFTER=$(wc -l < "${SAVINGS_LOG}" 2>/dev/null || echo 0)
NEW=$((SAVINGS_AFTER - SAVINGS_BEFORE))
echo "New measurement rows: ${NEW}"
echo ""

# Aggregate
python3 <<PYEOF
import json
from collections import defaultdict

with open("${SAVINGS_LOG}") as f:
    data = [json.loads(line) for line in f if line.strip()]

# Only the new campaign rows
recent = [r for r in data[-${NEW}:] if r.get('measurement_method') == 'campaign']
if not recent:
    print("No campaign rows recorded.")
    exit()

# Per (class, mode) summary
by_cm = defaultdict(lambda: {'orig': 0, 'actual': 0, 'n': 0})
for r in recent:
    key = (r['cmd_class'], r.get('compression_mode', 'unknown'))
    by_cm[key]['orig'] += r.get('original_output_tokens', 0) or 0
    by_cm[key]['actual'] += r.get('actual_output_tokens', 0) or 0
    by_cm[key]['n'] += 1

# Group by class
classes = sorted(set(k[0] for k in by_cm))
print(f"{'CLASS':<25} {'MODE':<10} {'ORIG':>8} {'ACTUAL':>8} {'PCT':>8}")
print("-" * 70)
for cls in classes:
    for mode in ['basic', 'rtk', 'diet', 'nushell', 'lean-ctx']:
        s = by_cm.get((cls, mode))
        if not s: continue
        saved = max(0, s['orig'] - s['actual'])
        pct = saved / s['orig'] * 100 if s['orig'] > 0 else 0
        marker = "  *" if mode != 'basic' and pct > 30 else ""
        print(f"{cls:<25} {mode:<10} {s['orig']:>8} {s['actual']:>8} {pct:>7.1f}%{marker}")
    print()

# Per-mode totals (excluding basic)
print()
print("=== Per-mode aggregates ===")
by_mode = defaultdict(lambda: {'orig_total': 0, 'actual_total': 0, 'classes_with_savings': 0, 'classes_total': 0})
basic_per_class = {k[0]: v for k, v in by_cm.items() if k[1] == 'basic'}
for (cls, mode), s in by_cm.items():
    if mode == 'basic': continue
    basic = basic_per_class.get(cls, {}).get('orig', 0)
    if basic == 0: continue
    by_mode[mode]['classes_total'] += 1
    by_mode[mode]['orig_total'] += basic
    by_mode[mode]['actual_total'] += s['actual']
    if s['actual'] < basic:
        by_mode[mode]['classes_with_savings'] += 1

print(f"{'MODE':<10} {'CLASSES':>9} {'WINS':>6} {'TOTAL ORIG':>12} {'TOTAL ACTUAL':>14} {'PCT':>8}")
print("-" * 65)
for mode in sorted(by_mode):
    s = by_mode[mode]
    saved = max(0, s['orig_total'] - s['actual_total'])
    pct = saved / s['orig_total'] * 100 if s['orig_total'] > 0 else 0
    print(f"{mode:<10} {s['classes_total']:>9} {s['classes_with_savings']:>6} {s['orig_total']:>12,} {s['actual_total']:>14,} {pct:>7.1f}%")
PYEOF
