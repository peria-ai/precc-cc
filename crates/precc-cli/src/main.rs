//! precc: CLI tool for Predictive Error Correction for Claude Code.
//!
//! Subcommands:
//! - `precc init` — Setup hook and databases
//! - `precc ingest [file|--all]` — Mine session logs for failure patterns
//! - `precc skills [list|show|export|edit]` — Manage automation skills
//! - `precc debug <binary> [args]` — GDB-based debugging helper
//! - `precc report` — Analytics dashboard

use anyhow::{bail, Context, Result};
use clap::Parser;
use precc_core::{
    advisor, compress, consent, db, gdb, geofence, gha, license, metrics, mining, nushell, promote,
    rtk, sharing, skill_advisor, skills, telemetry, update_check,
};
#[allow(unused_imports)] // needed for writeln! on impl Write params
use std::io::Write;

mod gif;
mod mail;
mod webhook;

#[derive(Parser)]
#[command(
    name = "precc",
    about = "Predictive Error Correction for Claude Code",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Mine session logs for failure-fix patterns
    Ingest {
        /// Session file to mine (or --all for batch)
        file: Option<String>,
        /// Mine all unmined sessions
        #[arg(long)]
        all: bool,
        /// Re-mine sessions even if already recorded (overwrites prior results)
        #[arg(long)]
        force: bool,
    },
    /// Manage automation skills
    Skills {
        #[command(subcommand)]
        action: Option<SkillsAction>,
    },
    /// GDB-based debugging helper
    Debug {
        /// Binary to debug
        binary: Option<String>,
        /// Arguments to pass to the binary
        args: Vec<String>,
    },
    /// Analytics dashboard
    Report {
        /// Email the report (Pro: to your registered email)
        #[arg(long)]
        email: bool,
    },
    /// Estimate token savings from PRECC over RTK alone
    Savings {
        /// Show full detailed breakdown (Pro only)
        #[arg(long)]
        all: bool,
    },
    /// Setup hook and databases
    Init,
    /// Compress CLAUDE.md and context files to reduce token usage
    Compress {
        /// Show what would change without modifying files
        #[arg(long)]
        dry_run: bool,
        /// Restore original files from backups
        #[arg(long)]
        revert: bool,
        /// Project directory (defaults to current directory)
        dir: Option<String>,
    },
    /// Convert a bash script to an animated GIF at a target duration
    Gif {
        /// Bash script to animate
        script: String,
        /// Target GIF length, e.g. "30s" or "2m"
        length: String,
        /// Expected user inputs (quoted strings, piped to script stdin)
        inputs: Vec<String>,
    },
    /// Analyze a failed GitHub Actions run
    Gha {
        /// GitHub Actions URL (e.g. https://github.com/owner/repo/actions/runs/123)
        url: String,
    },
    /// Manage PRECC license key
    License {
        #[command(subcommand)]
        action: LicenseAction,
    },
    /// Send reports and documents via email
    Mail {
        #[command(subcommand)]
        action: MailAction,
    },
    /// Stripe webhook server and license delivery
    Webhook {
        #[command(subcommand)]
        action: WebhookAction,
    },
    /// Update PRECC binaries to the latest release
    Update {
        /// Force update even if already on the latest version
        #[arg(long)]
        force: bool,
        /// Install a specific version (e.g. v0.2.0) instead of latest
        #[arg(long)]
        version: Option<String>,
        /// Enable silent auto-update via the background miner daemon
        #[arg(long)]
        auto: bool,
    },
    /// Print a cache_control systemPrompt block for Anthropic API users (prompt caching)
    CacheHint,
    /// Manage anonymous telemetry consent
    Telemetry {
        #[command(subcommand)]
        action: TelemetryAction,
    },
    /// IP geofence compliance guard (Pro)
    Geofence {
        #[command(subcommand)]
        action: GeofenceAction,
    },
    /// Experimental nushell integration (alternative to RTK)
    Nushell {
        #[command(subcommand)]
        action: NushellAction,
    },
}

#[derive(clap::Subcommand)]
enum GeofenceAction {
    /// Check current geofence status (reads cached result)
    Check,
    /// Refresh the geofence cache (probes IP geolocation API)
    Refresh,
    /// Clear the geofence cache
    Clear,
    /// Show blocked regions and alternative LLM providers
    Info,
}

#[derive(clap::Subcommand)]
enum NushellAction {
    /// Check if nushell is available and show version
    Check,
    /// Show nushell translation for a command (dry-run)
    Translate {
        /// Bash command to translate
        command: String,
    },
    /// Run benchmark comparing bash vs RTK vs nushell token savings
    Benchmark {
        /// Specific command to benchmark (default: run all standard scenarios)
        #[arg(long)]
        command: Option<String>,
        /// Number of iterations per command
        #[arg(long, default_value = "3")]
        iterations: u32,
    },
    /// List all nushell translation rules
    Rules,
    /// Retrospective what-if analysis: compare bash vs RTK vs nushell on historical sessions
    WhatIf {
        /// Export results as CSV
        #[arg(long)]
        csv: bool,
    },
}

#[derive(clap::Subcommand)]
enum MailAction {
    /// Configure SMTP settings (creates ~/.config/precc/mail.toml)
    Setup,
    /// Send a savings report to an email address
    Report {
        /// Recipient email address
        to: String,
        /// Files to attach (PDF, PPTX, GIF, etc.)
        #[arg(long = "attach", short = 'a')]
        attachments: Vec<std::path::PathBuf>,
    },
    /// Send an arbitrary file to an email address
    Send {
        /// Recipient email address
        to: String,
        /// Subject line
        #[arg(long, short, default_value = "From PRECC")]
        subject: String,
        /// Body text
        #[arg(long, short, default_value = "")]
        body: String,
        /// Files to attach
        #[arg(long = "attach", short = 'a')]
        attachments: Vec<std::path::PathBuf>,
    },
}

#[derive(clap::Subcommand)]
enum WebhookAction {
    /// Start the Stripe webhook server
    Serve {
        /// Port to listen on (default: 8090)
        #[arg(long)]
        port: Option<u16>,
        /// Stripe webhook signing secret (whsec_XXXXX)
        #[arg(long)]
        stripe_secret: Option<String>,
    },
    /// Check and send any pending expiry reminder emails (run daily via cron)
    CheckReminders,
}

#[derive(clap::Subcommand)]
enum LicenseAction {
    /// Activate a license key
    Activate {
        /// License key (format: PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX)
        key: Option<String>,
        /// Email address used for purchase (for email-bound keys)
        #[arg(long)]
        email: Option<String>,
        /// Activate via GitHub Sponsors (uses GITHUB_TOKEN or gh CLI)
        #[arg(long)]
        github: bool,
    },
    /// Show current license status
    Status,
    /// Deactivate (remove) the stored license key
    Deactivate,
    /// Show this machine's fingerprint (for generating machine-bound keys)
    Fingerprint,
    /// Generate a license key (hidden, restricted to authorized machines)
    #[command(hide = true)]
    Generate {
        /// Edition: pro, team, or enterprise
        #[arg(long, default_value = "pro")]
        edition: String,
        /// Machine fingerprint (hex, e.g. f29c7d98). Omit for unbound key.
        #[arg(long)]
        fingerprint: Option<String>,
        /// Generate key bound to this email instead of machine
        #[arg(long, conflicts_with = "fingerprint")]
        email: Option<String>,
        /// Expiry in days from now. 0 = never expires.
        #[arg(long, default_value = "0")]
        expiry_days: u32,
    },
}

#[derive(clap::Subcommand)]
enum SkillsAction {
    /// List all skills
    List,
    /// Show details of a skill
    Show { name: String },
    /// Export a skill to TOML format (stdout)
    Export { name: String },
    /// Edit a skill's triggers/actions in $EDITOR and reimport on save
    Edit { name: String },
    /// Suggest new skills and flag ineffective ones
    Advise {
        /// Auto-disable ineffective mined skills
        #[arg(long)]
        auto_disable: bool,
        /// Accept and create a suggested skill by name
        #[arg(long)]
        accept: Option<String>,
        /// Share a skill to the team repository (generates TOML and token credits)
        #[arg(long)]
        share: Option<String>,
    },
    /// Cluster installed skills by function and recommend token-efficient replacements
    Cluster {
        /// Similarity threshold for clustering (0.0-1.0, default 0.3)
        #[arg(long, default_value = "0.3")]
        threshold: f64,
    },
}

#[derive(clap::Subcommand)]
enum TelemetryAction {
    /// Opt in to anonymous usage telemetry (requires explicit consent)
    Consent,
    /// Revoke telemetry consent
    Revoke,
    /// Show current telemetry consent status
    Status,
    /// Preview the exact data that would be sent (dry-run)
    Preview,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Show update banner on any command (except update itself, which handles it)
    if !matches!(cli.command, Some(Commands::Update { .. })) {
        show_update_banner();
    }

    match cli.command {
        Some(Commands::Init) => cmd_init(),
        Some(Commands::Ingest { file, all, force }) => cmd_ingest(file, all, force),
        Some(Commands::Skills { action }) => cmd_skills(action),
        Some(Commands::Debug {
            binary: Some(binary),
            args,
        }) => cmd_debug(binary, args),
        Some(Commands::Debug { binary: None, .. }) => {
            println!("Usage: precc debug <binary> [args...]");
            println!();
            println!("Generates a .gdbinit-precc script and launches GDB on the given binary.");
            println!();
            println!("Examples:");
            println!("  precc debug target/debug/myapp");
            println!("  precc debug target/debug/myapp --arg1 value");
            Ok(())
        }
        Some(Commands::Report { email }) => cmd_report_with_email(email),
        Some(Commands::Compress {
            dry_run,
            revert,
            dir,
        }) => cmd_compress(dry_run, revert, dir),
        Some(Commands::Savings { all }) => cmd_savings(all),
        Some(Commands::Gif {
            script,
            length,
            inputs,
        }) => {
            if license::tier() == license::Tier::Free {
                return Err(license::require_paid("GIF generation"));
            }
            gif::cmd_gif(script, length, inputs)
        }
        Some(Commands::Gha { url }) => cmd_gha(url),
        Some(Commands::License { action }) => cmd_license(action),
        Some(Commands::Mail { action }) => cmd_mail(action),
        Some(Commands::Webhook { action }) => match action {
            WebhookAction::Serve {
                port,
                stripe_secret,
            } => webhook::serve(port, stripe_secret),
            WebhookAction::CheckReminders => {
                let sent = webhook::check_reminders()?;
                println!("{sent} reminder(s) sent.");
                Ok(())
            }
        },
        Some(Commands::Update {
            force,
            version,
            auto,
        }) => cmd_update(force, version, auto),
        Some(Commands::CacheHint) => cmd_cache_hint(),
        Some(Commands::Telemetry { action }) => cmd_telemetry(action),
        Some(Commands::Geofence { action }) => cmd_geofence(action),
        Some(Commands::Nushell { action }) => cmd_nushell(action),
        None => {
            println!("precc — Predictive Error Correction for Claude Code");
            println!();
            println!("Run `precc --help` for available commands.");
            Ok(())
        }
    }
}

// =============================================================================
// precc init
// =============================================================================

fn cmd_init() -> Result<()> {
    let data_dir = db::data_dir()?;

    // Migrate any existing unencrypted databases before opening them with a key.
    for db_name in &["heuristics.db", "history.db", "metrics.db"] {
        let path = data_dir.join(db_name);
        match db::migrate_to_encrypted(&path) {
            Ok(true) => println!("  Migrated {} to AES-256 encryption", db_name),
            Ok(false) => {}
            Err(e) => eprintln!("  Warning: could not migrate {}: {e:#}", db_name),
        }
    }

    // Initialize all three databases
    println!("Initializing databases in {}...", data_dir.display());

    db::open_heuristics(&data_dir).context("failed to initialize heuristics.db")?;
    println!("  heuristics.db — OK");

    db::open_history(&data_dir).context("failed to initialize history.db")?;
    println!("  history.db    — OK");

    db::open_metrics(&data_dir).context("failed to initialize metrics.db")?;
    println!("  metrics.db    — OK");

    // Show encryption confirmation (first 4 bytes of the derived key)
    let key = db::master_key();
    println!(
        "  Encryption: AES-256 (machine-bound key, first 4 bytes: {})",
        &key[..8]
    );

    // Load builtin skills (embedded at compile time — no external files needed)
    let heuristics_conn = db::open_heuristics(&data_dir)?;
    const BUILTIN_SKILLS: &[(&str, &str)] = &[
        (
            "cargo-wrong-dir",
            include_str!("../../../skills/builtin/cargo-wrong-dir.toml"),
        ),
        (
            "git-wrong-dir",
            include_str!("../../../skills/builtin/git-wrong-dir.toml"),
        ),
        (
            "go-wrong-dir",
            include_str!("../../../skills/builtin/go-wrong-dir.toml"),
        ),
        (
            "make-wrong-dir",
            include_str!("../../../skills/builtin/make-wrong-dir.toml"),
        ),
        (
            "npm-wrong-dir",
            include_str!("../../../skills/builtin/npm-wrong-dir.toml"),
        ),
        (
            "python-wrong-dir",
            include_str!("../../../skills/builtin/python-wrong-dir.toml"),
        ),
        (
            "asciinema-gif",
            include_str!("../../../skills/builtin/asciinema-gif.toml"),
        ),
        (
            "warn-identify",
            include_str!("../../../skills/builtin/warn-identify.toml"),
        ),
        (
            "warn-reduce",
            include_str!("../../../skills/builtin/warn-reduce.toml"),
        ),
        (
            "zerowarns",
            include_str!("../../../skills/builtin/zerowarns.toml"),
        ),
        (
            "jj-translate",
            include_str!("../../../skills/builtin/jj-translate.toml"),
        ),
        (
            "mail-report",
            include_str!("../../../skills/builtin/mail-report.toml"),
        ),
        (
            "rust-doc-cache",
            include_str!("../../../skills/builtin/rust-doc-cache.toml"),
        ),
        (
            "rust-check-before-build",
            include_str!("../../../skills/builtin/rust-check-before-build.toml"),
        ),
        (
            "rust-test-slice",
            include_str!("../../../skills/builtin/rust-test-slice.toml"),
        ),
        (
            "block-comment-cmd",
            include_str!("../../../skills/builtin/block-comment-cmd.toml"),
        ),
    ];
    let loaded = skills::load_builtin_skills_embedded(&heuristics_conn, BUILTIN_SKILLS)?;
    if loaded > 0 {
        println!("  Loaded {} builtin skill(s)", loaded);
    } else {
        println!("  Builtin skills already loaded");
    }

    // Write prefix cache so the hook can skip heuristics.db for non-matching commands
    skills::write_skill_prefixes(&heuristics_conn, &data_dir)?;

    // Print hook setup instructions
    println!();
    println!("Hook setup:");
    println!("  Add to ~/.claude/settings.json:");
    println!();
    println!("  {{");
    println!("    \"hooks\": {{");
    println!("      \"PreToolUse\": [");
    println!("        {{");
    println!("          \"matcher\": \"Bash\",");
    println!("          \"hooks\": [");
    println!("            {{");
    println!("              \"type\": \"command\",");

    // Try to find precc-hook binary
    if let Ok(exe) = std::env::current_exe() {
        let hook_path = exe
            .parent()
            .map(|p| p.join("precc-hook"))
            .unwrap_or_else(|| std::path::PathBuf::from("precc-hook"));
        println!("              \"command\": \"{}\"", hook_path.display());
    } else {
        println!("              \"command\": \"precc-hook\"");
    }

    println!("            }}");
    println!("          ]");
    println!("        }}");
    println!("      ]");
    println!("    }}");
    println!("  }}");
    println!();

    // Write compact ~/.claude/CLAUDE.md with skill trigger index
    if let Err(e) = write_claude_md() {
        eprintln!("  Warning: could not write ~/.claude/CLAUDE.md: {e:#}");
    }

    // Ask about auto-updates if not already configured
    if !update_check::has_auto_update_consent() {
        println!();
        let _ = update_check::prompt_auto_update_consent();
    }

    println!();
    println!("Prompt caching (Anthropic API users only):");
    println!("  Run `precc cache-hint` to print a ready-to-paste systemPrompt block");
    println!("  with cache_control: ephemeral markers for ~/.claude/settings.json.");
    println!();
    println!("Init complete.");

    Ok(())
}

// =============================================================================
// precc ingest
// =============================================================================

/// Free-tier session ingest limit.
const FREE_INGEST_LIMIT: usize = 1;

fn cmd_ingest(file: Option<String>, all: bool, force: bool) -> Result<()> {
    let data_dir = db::data_dir()?;
    let conn = db::open_history(&data_dir)?;

    // License gate: Free tier is limited to FREE_INGEST_LIMIT sessions.
    // Listing (no file, no --all) is always allowed; only actual mining is gated.
    if license::tier() == license::Tier::Free && (file.is_some() || all) {
        let mined_count: usize = conn
            .query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))
            .unwrap_or(0);
        if mined_count >= FREE_INGEST_LIMIT {
            eprintln!(
                "Free tier: session mining is limited to {} session(s). \
                 You have already mined {}.\n{}",
                FREE_INGEST_LIMIT,
                mined_count,
                license::require_paid("Unlimited session mining")
            );
            std::process::exit(1);
        }
    }

    if let Some(path) = file {
        // Mine a single session file
        let path = std::path::PathBuf::from(&path);
        if !path.exists() {
            bail!("session file not found: {}", path.display());
        }

        println!("Mining {}...", path.display());
        match mining::mine_session(&conn, &path, force)? {
            mining::MineResult::Skipped => println!("  Session already mined or has no events"),
            mining::MineResult::Processed { pairs, events } => {
                println!("  Found {} event(s), {} failure-fix pair(s)", events, pairs);
            }
        }
    } else if all {
        // Mine all sessions (or re-mine if --force)
        if force {
            println!("Scanning all sessions (force re-mine)...");
        } else {
            println!("Scanning for unmined sessions...");
        }
        let summary = mining::mine_all(&conn, force)?;
        println!();
        println!("Mining summary:");
        println!("  Sessions processed: {}", summary.sessions_processed);
        println!("  Sessions skipped:   {}", summary.sessions_skipped);
        println!("  Events found:       {}", summary.events_found);
        println!("  Pairs found:        {}", summary.pairs_found);
    } else {
        // List available session files
        let files = mining::find_session_files()?;
        if files.is_empty() {
            println!("No session files found in ~/.claude/projects/");
            println!("Run Claude Code to generate session logs first.");
        } else {
            println!("Found {} session file(s):", files.len());
            // Check which are already mined
            let mut mined = 0;
            let mut unmined = 0;
            for file in &files {
                let session_id = file
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let already: bool = conn
                    .query_row(
                        "SELECT COUNT(*) > 0 FROM sessions WHERE session_id = ?1",
                        [session_id],
                        |r| r.get(0),
                    )
                    .unwrap_or(false);

                if already {
                    mined += 1;
                } else {
                    unmined += 1;
                }
            }
            println!("  {} already mined, {} new", mined, unmined);
            if unmined > 0 {
                println!();
                println!("Run `precc ingest --all` to mine new sessions.");
            }
        }
    }

    Ok(())
}

// =============================================================================
// precc skills
// =============================================================================

fn cmd_skills(action: Option<SkillsAction>) -> Result<()> {
    let data_dir = db::data_dir()?;
    let conn = db::open_heuristics(&data_dir)?;

    match action {
        Some(SkillsAction::List) | None => cmd_skills_list(&conn),
        Some(SkillsAction::Show { name }) => cmd_skills_show(&conn, &name),
        Some(SkillsAction::Export { name }) => cmd_skills_export(&conn, &name),
        Some(SkillsAction::Edit { name }) => cmd_skills_edit(&conn, &name),
        Some(SkillsAction::Advise {
            auto_disable,
            accept,
            share,
        }) => cmd_skills_advise(&data_dir, &conn, auto_disable, accept, share),
        Some(SkillsAction::Cluster { threshold }) => cmd_skills_cluster(threshold),
    }
}

fn cmd_skills_list(conn: &rusqlite::Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name, s.description, s.source, s.enabled, s.priority,
                COALESCE(st.activated, 0), COALESCE(st.succeeded, 0), COALESCE(st.failed, 0),
                st.last_used
         FROM skills s
         LEFT JOIN skill_stats st ON st.skill_id = s.id
         ORDER BY s.priority ASC, s.name ASC",
    )?;

    let rows: Vec<SkillRow> = stmt
        .query_map([], |row: &rusqlite::Row| {
            Ok(SkillRow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                source: row.get(3)?,
                enabled: row.get(4)?,
                priority: row.get(5)?,
                activated: row.get(6)?,
                succeeded: row.get(7)?,
                failed: row.get(8)?,
                last_used: row.get(9)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    if rows.is_empty() {
        println!("No skills registered.");
        println!(
            "Run `precc init` to load builtin skills, or `precc ingest --all` to mine patterns."
        );
        return Ok(());
    }

    // Table header
    println!(
        "{:<4} {:<25} {:<8} {:<8} {:<6} {:<6} {:<6}",
        "ID", "Name", "Source", "Enabled", "Pri", "Acts", "Succ"
    );
    println!("{}", "-".repeat(70));

    for row in &rows {
        println!(
            "{:<4} {:<25} {:<8} {:<8} {:<6} {:<6} {:<6}",
            row.id,
            truncate_str(&row.name, 24),
            truncate_str(&row.source, 7),
            if row.enabled { "yes" } else { "no" },
            row.priority,
            row.activated,
            row.succeeded,
        );
    }

    println!();
    println!("{} skill(s) total", rows.len());

    Ok(())
}

struct SkillRow {
    id: i64,
    name: String,
    description: String,
    source: String,
    enabled: bool,
    priority: i64,
    activated: i64,
    succeeded: i64,
    failed: i64,
    last_used: Option<String>,
}

fn cmd_skills_show(conn: &rusqlite::Connection, name: &str) -> Result<()> {
    let row: Option<SkillRow> = conn
        .query_row(
            "SELECT s.id, s.name, s.description, s.source, s.enabled, s.priority,
                    COALESCE(st.activated, 0), COALESCE(st.succeeded, 0), COALESCE(st.failed, 0),
                    st.last_used
             FROM skills s
             LEFT JOIN skill_stats st ON st.skill_id = s.id
             WHERE s.name = ?1",
            [name],
            |row: &rusqlite::Row| {
                Ok(SkillRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    source: row.get(3)?,
                    enabled: row.get(4)?,
                    priority: row.get(5)?,
                    activated: row.get(6)?,
                    succeeded: row.get(7)?,
                    failed: row.get(8)?,
                    last_used: row.get(9)?,
                })
            },
        )
        .ok();

    let row = match row {
        Some(r) => r,
        None => {
            bail!("skill '{}' not found", name);
        }
    };

    println!("Skill: {}", row.name);
    println!("  Description: {}", row.description);
    println!("  Source:      {}", row.source);
    println!("  Priority:    {}", row.priority);
    println!("  Enabled:     {}", if row.enabled { "yes" } else { "no" });
    println!();

    // Show triggers
    let mut stmt = conn
        .prepare("SELECT trigger_type, pattern, weight FROM skill_triggers WHERE skill_id = ?1")?;
    let triggers: Vec<(String, String, f64)> = stmt
        .query_map([row.id], |r: &rusqlite::Row| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })?
        .filter_map(Result::ok)
        .collect();

    if !triggers.is_empty() {
        println!("  Triggers:");
        for (ttype, pattern, weight) in &triggers {
            println!("    [{ttype}] {pattern} (weight={weight:.1})");
        }
        println!();
    }

    // Show actions
    let mut stmt = conn.prepare(
        "SELECT action_type, template, confidence FROM skill_actions WHERE skill_id = ?1",
    )?;
    let actions: Vec<(String, String, f64)> = stmt
        .query_map([row.id], |r: &rusqlite::Row| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })?
        .filter_map(Result::ok)
        .collect();

    if !actions.is_empty() {
        println!("  Actions:");
        for (atype, template, conf) in &actions {
            println!("    [{atype}] {template} (confidence={conf:.1})");
        }
        println!();
    }

    // Show stats
    println!("  Stats:");
    println!("    Activated: {}", row.activated);
    println!("    Succeeded: {}", row.succeeded);
    println!("    Failed:    {}", row.failed);
    if let Some(last) = &row.last_used {
        println!("    Last used: {}", last);
    }

    Ok(())
}

fn cmd_skills_export(conn: &rusqlite::Connection, name: &str) -> Result<()> {
    write_skill_toml(conn, name, &mut std::io::stdout())
}

fn cmd_skills_edit(conn: &rusqlite::Connection, name: &str) -> Result<()> {
    // Verify the skill exists before opening an editor
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM skills WHERE name = ?1",
            [name],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if !exists {
        bail!("skill '{}' not found", name);
    }

    // Write current skill TOML to a temp file
    let tmp_path = std::env::temp_dir().join(format!("precc-skill-{}.toml", name));
    {
        // Reuse export logic by capturing its output into a string
        let mut buf = Vec::new();
        write_skill_toml(conn, name, &mut buf)?;
        std::fs::write(&tmp_path, &buf)
            .with_context(|| format!("failed to write temp file {}", tmp_path.display()))?;
    }

    let original =
        std::fs::read_to_string(&tmp_path).context("failed to read temp file before edit")?;

    // Launch $EDITOR (fallback: vi)
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    let status = std::process::Command::new(&editor)
        .arg(&tmp_path)
        .status()
        .with_context(|| format!("failed to launch editor '{editor}'"))?;

    if !status.success() {
        let _ = std::fs::remove_file(&tmp_path);
        bail!("editor exited with non-zero status — no changes saved");
    }

    let edited =
        std::fs::read_to_string(&tmp_path).context("failed to read temp file after edit")?;
    let _ = std::fs::remove_file(&tmp_path);

    if edited == original {
        println!("No changes detected — skill '{}' unchanged.", name);
        return Ok(());
    }

    // Validate TOML parses before writing to DB
    toml::from_str::<toml::Value>(&edited).context("edited file is not valid TOML")?;

    // Update skill in DB
    skills::update_skill_toml(conn, name, &edited)
        .with_context(|| format!("failed to update skill '{name}'"))?;

    println!("Skill '{}' updated.", name);
    Ok(())
}

/// Render a skill's current DB state as TOML bytes (shared by export and edit).
fn write_skill_toml(
    conn: &rusqlite::Connection,
    name: &str,
    out: &mut impl std::io::Write,
) -> Result<()> {
    let row: Option<(i64, String, String, String, bool, i64)> = conn
        .query_row(
            "SELECT id, name, description, source, enabled, priority
             FROM skills WHERE name = ?1",
            [name],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        )
        .ok();

    let (skill_id, skill_name, description, source, _enabled, priority) = match row {
        Some(r) => r,
        None => bail!("skill '{}' not found", name),
    };

    let mut stmt = conn.prepare(
        "SELECT trigger_type, pattern, weight FROM skill_triggers WHERE skill_id = ?1 ORDER BY id",
    )?;
    let triggers: Vec<(String, String, f64)> = stmt
        .query_map([skill_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .filter_map(Result::ok)
        .collect();

    let mut stmt = conn.prepare(
        "SELECT action_type, template, confidence FROM skill_actions WHERE skill_id = ?1 ORDER BY id",
    )?;
    let actions: Vec<(String, String, f64)> = stmt
        .query_map([skill_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .filter_map(Result::ok)
        .collect();

    writeln!(out, "[skill]")?;
    writeln!(out, "name = {:?}", skill_name)?;
    writeln!(out, "description = {:?}", description)?;
    writeln!(out, "source = {:?}", source)?;
    writeln!(out, "priority = {}", priority)?;

    for (ttype, pattern, weight) in &triggers {
        writeln!(out)?;
        writeln!(out, "[[triggers]]")?;
        writeln!(out, "type = {:?}", ttype)?;
        if pattern.contains('\\') {
            writeln!(out, "pattern = '{}'", pattern)?;
        } else {
            writeln!(out, "pattern = {:?}", pattern)?;
        }
        writeln!(out, "weight = {}", weight)?;
    }

    for (atype, template, confidence) in &actions {
        writeln!(out)?;
        writeln!(out, "[[actions]]")?;
        writeln!(out, "type = {:?}", atype)?;
        writeln!(out, "template = {:?}", template)?;
        writeln!(out, "confidence = {}", confidence)?;
    }

    Ok(())
}

// =============================================================================
// precc skills cluster
// =============================================================================

fn cmd_skills_cluster(threshold: f64) -> Result<()> {
    if !license::tier().is_paid() {
        return Err(license::require_paid("Skill cluster analysis"));
    }

    println!("Scanning installed skills...");
    let skills = skill_advisor::scan_installed_skills()?;

    if skills.is_empty() {
        println!("No installed skills found.");
        println!("Skills are scanned from:");
        println!("  ~/.claude/plugins/**/SKILL.md");
        println!("  ~/.claude/skills/*/SKILL.md");
        println!("  ./skills/*/SKILL.md (project-local)");
        return Ok(());
    }

    println!("Found {} installed skills", skills.len());
    println!();

    // Show inventory
    println!(
        "{:<30} {:>6} {:>8}  {:<35}",
        "Skill", "Tokens", "Source", "Description"
    );
    println!("{}", "-".repeat(90));
    for s in &skills {
        let desc = if s.description.chars().count() > 35 {
            let truncated: String = s.description.chars().take(32).collect();
            format!("{truncated}...")
        } else {
            s.description.clone()
        };
        println!(
            "{:<30} {:>4}t {:>8}  {}",
            if s.name.len() > 29 {
                &s.name[..29]
            } else {
                &s.name
            },
            s.context_tokens,
            s.source,
            desc,
        );
    }
    let total_tokens: u64 = skills.iter().map(|s| s.context_tokens).sum();
    println!();
    println!("Total context cost: {} tokens/session", total_tokens);

    // Cluster
    let clusters = skill_advisor::cluster_skills(&skills, threshold);

    if clusters.is_empty() {
        println!();
        println!(
            "No overlapping skill clusters found (threshold={:.1}).",
            threshold
        );
        println!("All installed skills appear to serve distinct functions.");
        return Ok(());
    }

    // Scan usage
    println!();
    println!("Scanning session logs for usage data...");
    let skill_names: Vec<String> = skills.iter().map(|s| s.name.clone()).collect();
    let usage = skill_advisor::scan_skill_usage(&skill_names)?;

    // Generate recommendations
    let recs = skill_advisor::recommend(&skills, &clusters, &usage);

    println!();
    println!(
        "Skill Clusters ({} clusters with overlapping skills)",
        clusters.len()
    );
    println!("{}", "=".repeat(70));

    for rec in &recs {
        println!();
        println!(
            "Cluster: {} ({} skills, {} context tokens)",
            rec.cluster_label,
            1 + rec.remove.len(),
            rec.keep.context_tokens + rec.remove.iter().map(|r| r.context_tokens).sum::<u64>(),
        );

        // Keep
        let u = usage.get(&rec.keep.name);
        let acts = u.map(|u| u.activations).unwrap_or(0);
        let rate = u
            .map(|u| {
                if u.success_count + u.failure_count > 0 {
                    format!("{:.0}%", u.success_rate() * 100.0)
                } else {
                    "-".to_string()
                }
            })
            .unwrap_or_else(|| "-".to_string());
        println!(
            "  {:<28} {:>4}t  {:>3} activations  {} success  <- KEEP",
            rec.keep.name, rec.keep.context_tokens, acts, rate,
        );

        // Remove
        for r in &rec.remove {
            let u = usage.get(&r.name);
            let acts = u.map(|u| u.activations).unwrap_or(0);
            let rate = u
                .map(|u| {
                    if u.success_count + u.failure_count > 0 {
                        format!("{:.0}%", u.success_rate() * 100.0)
                    } else {
                        "-".to_string()
                    }
                })
                .unwrap_or_else(|| "-".to_string());
            println!(
                "  {:<28} {:>4}t  {:>3} activations  {} success  -> REMOVE (saves {}t/session)",
                r.name, r.context_tokens, acts, rate, r.context_tokens,
            );
        }
    }

    let total_savings: u64 = recs.iter().map(|r| r.tokens_saved_per_session).sum();
    let removable: usize = recs.iter().map(|r| r.remove.len()).sum();

    if total_savings > 0 {
        println!();
        println!(
            "Potential savings: {} tokens/session ({} skills removable)",
            total_savings, removable,
        );
    }

    Ok(())
}

// =============================================================================
// precc skills advise
// =============================================================================

fn cmd_skills_advise(
    data_dir: &std::path::Path,
    heuristics_conn: &rusqlite::Connection,
    auto_disable: bool,
    accept: Option<String>,
    share: Option<String>,
) -> Result<()> {
    // Handle --share: export a skill and record sharing credits
    if let Some(ref skill_name) = share {
        let toml_content = sharing::export_skill_toml(heuristics_conn, skill_name)?;
        sharing::record_share(heuristics_conn, skill_name, &toml_content)?;
        sharing::update_credits(heuristics_conn)?;

        println!("Shared skill: {skill_name}");
        println!();
        println!("TOML (copy to your team skill repository):");
        println!("---");
        println!("{toml_content}");
        println!("---");
        println!();

        let summary = sharing::credit_summary(heuristics_conn)?;
        println!("Sharing Credits");
        println!("---------------");
        println!("  Skills shared       : {}", summary.skills_shared);
        println!("  Total activations   : {}", summary.total_activations);
        println!("  Total tokens saved  : {:.0}", summary.total_tokens_saved);
        println!(
            "  Credits earned (10%): {:.0} tokens",
            summary.total_credits_earned
        );
        return Ok(());
    }

    // Handle --accept: create a suggested skill from failure-fix pairs
    if let Some(ref suggested_name) = accept {
        let history_conn = db::open_history(data_dir)?;
        let report = advisor::advise(heuristics_conn, &history_conn, false)?;

        let suggestion = report
            .suggestions
            .iter()
            .find(|s| s.name == *suggested_name);

        match suggestion {
            Some(s) => {
                // Promote this pattern immediately
                promote::promote_patterns(
                    &history_conn,
                    heuristics_conn,
                    Some(1), // lower threshold to force promotion
                )?;
                println!("Accepted suggestion: {}", s.name);
                println!("  Trigger: {}", s.trigger_pattern);
                println!("  Fix:     {}", s.fix_template);
                println!(
                    "  Est. savings: {:.0} tok/hit × {} hits = {:.0} tok",
                    s.est_tokens_per_hit,
                    s.occurrences,
                    s.est_tokens_per_hit * s.occurrences as f64
                );
            }
            None => {
                bail!(
                    "No suggestion named '{}'. Run `precc skills advise` to see available suggestions.",
                    suggested_name
                );
            }
        }
        return Ok(());
    }

    // Default: show the full advisor report
    let history_conn = db::open_history(data_dir)?;
    let report = advisor::advise(heuristics_conn, &history_conn, auto_disable)?;

    // ── Suggested new skills ────────────────────────────────────────────────
    println!("Skill Advisor");
    println!("=============");
    println!();

    if report.suggestions.is_empty() {
        println!("Suggestions: none (all known patterns are covered by existing skills)");
    } else {
        println!("Suggested Skills ({} found)", report.suggestions.len());
        println!("{}", "-".repeat(50));
        for (i, s) in report.suggestions.iter().enumerate() {
            let proj = s.project_type.as_deref().unwrap_or("?");
            println!("  {}. {} [{}]", i + 1, s.name, proj);
            println!("     {}", s.reason);
            println!("     Trigger:  {}", s.trigger_pattern);
            println!("     Fix:      {}", truncate_str(&s.fix_template, 60));
            println!(
                "     Savings:  ~{:.0} tok/hit × {} hits = ~{:.0} tok total",
                s.est_tokens_per_hit,
                s.occurrences,
                s.est_tokens_per_hit * s.occurrences as f64
            );
            println!();
        }
        println!("  Accept a suggestion:  precc skills advise --accept <name>");
    }
    println!();

    // ── Ineffective skills ──────────────────────────────────────────────────
    if report.ineffective.is_empty() {
        println!("Ineffective Skills: none (all enabled skills are performing well)");
    } else {
        println!("Ineffective Skills ({} found)", report.ineffective.len());
        println!("{}", "-".repeat(50));
        println!(
            "  {:<25} {:<8} {:>5} {:>5} {:>5} {:>7}  Status",
            "Name", "Source", "Acts", "OK", "Fail", "Rate"
        );
        for s in &report.ineffective {
            let status = if s.auto_disabled {
                "DISABLED"
            } else {
                "flagged"
            };
            println!(
                "  {:<25} {:<8} {:>5} {:>5} {:>5} {:>6.0}%  {}",
                truncate_str(&s.name, 25),
                truncate_str(&s.source, 7),
                s.activated,
                s.succeeded,
                s.failed,
                s.failure_rate * 100.0,
                status,
            );
        }
        println!();

        if report.disabled_count > 0 {
            println!("  {} skill(s) auto-disabled.", report.disabled_count);
        } else if !auto_disable {
            println!(
                "  To auto-disable ineffective mined skills: precc skills advise --auto-disable"
            );
        }
    }
    println!();

    // ── Token optimization analysis ─────────────────────────────────────────
    if let Ok(skill_rows) = telemetry::per_skill_stats(heuristics_conn) {
        if !skill_rows.is_empty() {
            let total_tokens: f64 = skill_rows.iter().map(|s| s.est_tokens_saved).sum();
            let total_acts: i64 = skill_rows.iter().map(|s| s.activated).sum();

            println!("Token Optimization Analysis");
            println!("--------------------------");
            for row in &skill_rows {
                let efficiency = if row.activated > 0 {
                    row.est_tokens_saved / row.activated as f64
                } else {
                    0.0
                };
                let pct = if total_tokens > 0.0 {
                    row.est_tokens_saved / total_tokens * 100.0
                } else {
                    0.0
                };
                let fail_rate = if row.activated > 0 {
                    row.failed as f64 / row.activated as f64 * 100.0
                } else {
                    0.0
                };

                let rating = if fail_rate > 30.0 {
                    "POOR"
                } else if fail_rate > 10.0 {
                    "FAIR"
                } else if pct > 20.0 {
                    "HIGH"
                } else {
                    "OK"
                };

                println!(
                    "  {:<25} {:>5.1}% of savings  {:>5.0} tok/act  fail:{:>4.0}%  [{}]",
                    truncate_str(&row.skill_name, 25),
                    pct,
                    efficiency,
                    fail_rate,
                    rating,
                );
            }

            println!();
            println!(
                "  Total: {:.0} tokens saved across {} activations ({:.0} tok/act avg)",
                total_tokens,
                total_acts,
                if total_acts > 0 {
                    total_tokens / total_acts as f64
                } else {
                    0.0
                },
            );
            println!();
        }
    }

    // ── Token optimization recommendations ────────────────────────────────
    if !report.optimizations.is_empty() {
        println!(
            "Token Optimization Recommendations ({} found)",
            report.optimizations.len()
        );
        println!("{}", "-".repeat(50));
        for (i, opt) in report.optimizations.iter().enumerate() {
            println!("  {}. [{}]", i + 1, opt.skill_name);
            println!("     {}", opt.optimization);
            println!(
                "     Est. saving: ~{:.0} tok/act × {} acts = ~{:.0} tok",
                opt.est_savings_per_act,
                opt.activations,
                opt.est_savings_per_act * opt.activations as f64,
            );
            println!();
        }
    }

    // ── Sharing credits summary ─────────────────────────────────────────────
    let credit_summary = sharing::credit_summary(heuristics_conn)?;
    if credit_summary.skills_shared > 0 {
        sharing::update_credits(heuristics_conn)?;
        let updated = sharing::credit_summary(heuristics_conn)?;
        println!("Sharing Credits");
        println!("---------------");
        println!("  Skills shared       : {}", updated.skills_shared);
        println!("  Total activations   : {}", updated.total_activations);
        println!("  Total tokens saved  : {:.0}", updated.total_tokens_saved);
        println!(
            "  Credits earned (10%): {:.0} tokens",
            updated.total_credits_earned
        );
        println!();
        println!("  Share a skill: precc skills advise --share <name>");
    } else {
        println!("Sharing: no skills shared yet.  Share with: precc skills advise --share <name>");
    }

    Ok(())
}

// =============================================================================
// precc debug
// =============================================================================

fn cmd_debug(binary: String, args: Vec<String>) -> Result<()> {
    if !gdb::gdb_available() {
        bail!("GDB is not available. Install GDB first: sudo apt install gdb");
    }

    let binary_path = std::path::Path::new(&binary);
    if !binary_path.exists() {
        bail!("binary not found: {}", binary);
    }

    // Generate a .gdbinit script for debugging
    let gdbinit_content = generate_gdbinit(&binary, &args);
    let gdbinit_path = std::env::current_dir()?.join(".gdbinit-precc");

    std::fs::write(&gdbinit_path, &gdbinit_content).context("failed to write .gdbinit-precc")?;

    println!("Generated {}", gdbinit_path.display());
    println!();
    println!("GDB commands file created with:");
    println!("  - Breakpoints on common error paths (panic, abort)");
    println!("  - Backtrace on stop");
    println!("  - Auto-display of local variables");
    println!();

    // Build GDB command
    let mut gdb_args = vec![
        "-x".to_string(),
        gdbinit_path.to_string_lossy().to_string(),
        "--args".to_string(),
        binary.clone(),
    ];
    gdb_args.extend(args.iter().cloned());

    println!("Running: gdb {}", gdb_args.join(" "));
    println!();

    let status = std::process::Command::new("gdb")
        .args(&gdb_args)
        .status()
        .context("failed to launch GDB")?;

    // Clean up
    let _ = std::fs::remove_file(&gdbinit_path);

    if !status.success() {
        bail!("GDB exited with status {}", status);
    }

    Ok(())
}

fn generate_gdbinit(binary: &str, _args: &[String]) -> String {
    let is_rust = binary.contains("target/debug")
        || binary.contains("target/release")
        || std::path::Path::new("Cargo.toml").exists();

    let mut script = String::new();
    script.push_str("# Generated by precc debug\n");
    script.push_str("set pagination off\n");
    script.push_str("set print pretty on\n");
    script.push_str("set print array on\n");
    script.push_str("set confirm off\n");
    script.push('\n');

    if is_rust {
        // Rust-specific breakpoints
        script.push_str("# Rust panic/abort breakpoints\n");
        script.push_str("break rust_panic\n");
        script.push_str("break rust_begin_unwind\n");
        script.push_str("break std::panicking::begin_panic\n");
        script.push_str("break std::panicking::rust_panic_with_hook\n");
    } else {
        // Generic C/C++ breakpoints
        script.push_str("# Error breakpoints\n");
        script.push_str("break abort\n");
        script.push_str("break exit\n");
    }

    script.push('\n');
    script.push_str("# Show backtrace on stop\n");
    script.push_str("define hook-stop\n");
    script.push_str("  bt 10\n");
    script.push_str("  info locals\n");
    script.push_str("end\n");
    script.push('\n');
    script.push_str("run\n");

    script
}

// =============================================================================
// precc report
// =============================================================================

fn cmd_report_with_email(email: bool) -> Result<()> {
    // Generate the text report
    let report_text = generate_report_text()?;

    if email {
        if license::tier() == license::Tier::Free {
            return Err(license::require_paid("Monthly usage report via email"));
        }
        let to = license::stored_email()
            .ok_or_else(|| anyhow::anyhow!(
                "No email found in license. Activate with:\n  precc license activate <key> --email you@example.com"
            ))?;

        // Send via local sendmail
        let message = format!(
            "From: PRECC <support@peria.ai>\n\
             To: {to}\n\
             Subject: Your PRECC Monthly Usage Report\n\
             Content-Type: text/plain; charset=utf-8\n\
             \n\
             {report_text}\n\
             \n\
             ---\n\
             This report was generated by PRECC.\n\
             https://github.com/peria-ai/precc-cc\n"
        );

        let mut child = std::process::Command::new("sendmail")
            .args(["-t", "-f", "support@peria.ai"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to run sendmail")?;

        use std::io::Write;
        child
            .stdin
            .take()
            .context("no stdin")?
            .write_all(message.as_bytes())?;
        let status = child.wait()?;
        if !status.success() {
            bail!("sendmail failed with status {}", status);
        }
        println!("Monthly usage report sent to {to}");
        Ok(())
    } else {
        // Print to terminal; if day 30 and not telemetry, show reminder
        println!("{report_text}");

        if license::tier().is_paid() && !consent::is_telemetry_enabled() {
            let day_of_month = {
                let secs = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                // Approximate day of month (good enough for a reminder)
                ((secs / 86400) % 31) + 1
            };
            if day_of_month >= 28 {
                println!();
                println!("Tip: Email this report to yourself with: precc report --email");
                println!("Or enable telemetry to receive monthly reports automatically:");
                println!("  precc telemetry consent");
            }
        }
        Ok(())
    }
}

fn generate_report_text() -> Result<String> {
    let data_dir = db::data_dir()?;
    let mut out = String::new();

    out.push_str("PRECC Monthly Usage Report\n");
    out.push_str("=========================\n\n");

    // Hook latency
    if let Ok(conn) = db::open_metrics(&data_dir) {
        if let Some(s) = metrics::summary(&conn, metrics::MetricType::HookLatency)? {
            out.push_str(&format!(
                "Hook Latency: {:.2}ms avg, {:.2}ms max ({} calls)\n",
                s.avg, s.max, s.count
            ));
        }
        if let Some(s) = metrics::summary(&conn, metrics::MetricType::RtkRewrite)? {
            out.push_str(&format!("RTK Rewrites: {} total\n", s.count));
        }
        if let Some(s) = metrics::summary(&conn, metrics::MetricType::CdPrepend)? {
            out.push_str(&format!("CD Prepends: {} (failures prevented)\n", s.count));
        }
        if let Some(s) = metrics::summary(&conn, metrics::MetricType::SkillActivation)? {
            out.push_str(&format!("Skill Activations: {}\n", s.count));
        }
    }

    // Skill stats
    if let Ok(conn) = db::open_heuristics(&data_dir) {
        let total: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(activated), 0) FROM skill_stats",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        out.push_str(&format!("\nTotal skill activations: {total}\n"));
    }

    // Mined preventions
    if let Ok(conn) = db::open_history(&data_dir) {
        let preventions: i64 = conn.query_row(
            "SELECT COALESCE(SUM(occurrences - 1), 0) FROM failure_fix_pairs WHERE occurrences > 1",
            [], |r| r.get(0)
        ).unwrap_or(0);
        out.push_str(&format!("Mined failure preventions: {preventions}\n"));
    }

    out.push_str(&format!("\nEdition: {}\n", license::tier().name()));

    Ok(out)
}

fn cmd_report() -> Result<()> {
    let data_dir = db::data_dir()?;

    println!("PRECC Analytics Report");
    println!("======================");
    println!();

    // Hook latency metrics
    if let Ok(metrics_conn) = db::open_metrics(&data_dir) {
        report_section(
            &metrics_conn,
            "Hook Latency (ms)",
            metrics::MetricType::HookLatency,
        )?;
        report_section(
            &metrics_conn,
            "Skill Activations",
            metrics::MetricType::SkillActivation,
        )?;
        report_section(&metrics_conn, "CD Prepends", metrics::MetricType::CdPrepend)?;
        report_section(
            &metrics_conn,
            "GDB Suggestions",
            metrics::MetricType::GdbSuggestion,
        )?;
        report_section(
            &metrics_conn,
            "RTK Rewrites",
            metrics::MetricType::RtkRewrite,
        )?;
    } else {
        println!("  (metrics.db not available)");
        println!();
    }

    // Skills summary
    if let Ok(heuristics_conn) = db::open_heuristics(&data_dir) {
        let skill_count: i64 = heuristics_conn
            .query_row("SELECT COUNT(*) FROM skills", [], |r| r.get(0))
            .unwrap_or(0);
        let enabled_count: i64 = heuristics_conn
            .query_row("SELECT COUNT(*) FROM skills WHERE enabled = 1", [], |r| {
                r.get(0)
            })
            .unwrap_or(0);
        let total_activations: i64 = heuristics_conn
            .query_row(
                "SELECT COALESCE(SUM(activated), 0) FROM skill_stats",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);

        println!("Skills");
        println!("------");
        println!("  Total:       {}", skill_count);
        println!("  Enabled:     {}", enabled_count);
        println!("  Activations: {}", total_activations);
        println!();

        // Top 5 most activated skills
        let mut stmt = heuristics_conn.prepare(
            "SELECT s.name, st.activated FROM skills s
             JOIN skill_stats st ON st.skill_id = s.id
             WHERE st.activated > 0
             ORDER BY st.activated DESC LIMIT 5",
        )?;
        let top_skills: Vec<(String, i64)> = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();

        if !top_skills.is_empty() {
            println!("  Top skills:");
            for (name, count) in &top_skills {
                println!("    {name:<25} {count} activations");
            }
            println!();
        }

        // Per-skill ablation breakdown
        if let Ok(skill_rows) = telemetry::per_skill_stats(&heuristics_conn) {
            if !skill_rows.is_empty() {
                let total_tokens: f64 = skill_rows.iter().map(|s| s.est_tokens_saved).sum();

                println!("Per-Skill Savings Breakdown");
                println!("--------------------------");
                println!(
                    "  {:<25} {:>6} {:>7} {:>5} {:>15}",
                    "Skill", "Acts", "OK", "Fail", "Est. Tokens"
                );
                for row in &skill_rows {
                    println!(
                        "  {:<25} {:>6} {:>7} {:>5} {:>15.0}",
                        truncate_str(&row.skill_name, 25),
                        row.activated,
                        row.succeeded,
                        row.failed,
                        row.est_tokens_saved,
                    );
                }
                println!(
                    "  {:<25} {:>6} {:>7} {:>5} {:>15.0}",
                    "TOTAL",
                    skill_rows.iter().map(|s| s.activated).sum::<i64>(),
                    skill_rows.iter().map(|s| s.succeeded).sum::<i64>(),
                    skill_rows.iter().map(|s| s.failed).sum::<i64>(),
                    total_tokens,
                );
                println!();

                // Ablation bar chart
                if total_tokens > 0.0 {
                    println!("Ablation (skill contribution to savings)");
                    println!("-----------------------------------------");
                    for row in &skill_rows {
                        let pct = row.est_tokens_saved / total_tokens * 100.0;
                        let bar_len = (pct / 2.5).round() as usize; // 40 chars = 100%
                        let bar = "|".repeat(bar_len);
                        println!(
                            "  {:<25} {:>5.1}%  {bar}",
                            truncate_str(&row.skill_name, 25),
                            pct,
                        );
                    }
                    println!();
                }
            }
        }
    }

    // History summary
    if let Ok(history_conn) = db::open_history(&data_dir) {
        let session_count: i64 = history_conn
            .query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))
            .unwrap_or(0);
        let pair_count: i64 = history_conn
            .query_row("SELECT COUNT(*) FROM failure_fix_pairs", [], |r| r.get(0))
            .unwrap_or(0);
        let top_pair_count: i64 = history_conn
            .query_row(
                "SELECT COALESCE(MAX(occurrences), 0) FROM failure_fix_pairs",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);

        println!("History Mining");
        println!("--------------");
        println!("  Sessions mined:     {}", session_count);
        println!("  Failure-fix pairs:  {}", pair_count);
        println!("  Max occurrences:    {}", top_pair_count);
        println!();

        // Top 5 most frequent failure patterns
        if pair_count > 0 {
            let mut stmt = history_conn.prepare(
                "SELECT failure_command, fix_command, occurrences, project_type
                 FROM failure_fix_pairs
                 ORDER BY occurrences DESC LIMIT 5",
            )?;
            let top_patterns: Vec<(String, String, i64, Option<String>)> = stmt
                .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
                .filter_map(|r| r.ok())
                .collect();

            if !top_patterns.is_empty() {
                println!("  Top failure patterns:");
                for (fail_cmd, fix_cmd, occ, proj) in &top_patterns {
                    let proj_tag = proj.as_deref().unwrap_or("?");
                    println!(
                        "    [{proj_tag}] {} -> {} ({occ}x)",
                        truncate_str(fail_cmd, 30),
                        truncate_str(fix_cmd, 30)
                    );
                }
                println!();
            }
        }
    }

    // PostToolUse observability
    if let Ok(metrics_conn) = db::open_metrics(&data_dir) {
        let post_tokens = metrics::summary(
            &metrics_conn,
            metrics::MetricType::Custom("post_output_tokens"),
        )
        .ok()
        .flatten();
        let post_dupes = metrics::summary(
            &metrics_conn,
            metrics::MetricType::Custom("post_duplicate_detected"),
        )
        .ok()
        .flatten();
        let post_large = metrics::summary(
            &metrics_conn,
            metrics::MetricType::Custom("post_large_output"),
        )
        .ok()
        .flatten();

        if let Some(tok) = post_tokens {
            println!("PostToolUse Observability");
            println!("-------------------------");
            println!("  Output observations  : {:>8}", tok.count);
            println!("  Avg output (tokens)  : {:>8.0}", tok.avg);
            println!("  Total output (tokens): {:>8.0}", tok.total);
            if let Some(ref large) = post_large {
                println!("  Large outputs (>10K) : {:>8}", large.count);
            }
            if let Some(ref dupes) = post_dupes {
                println!("  Duplicate commands   : {:>8}", dupes.count);
                let est_waste = dupes.count as f64 * tok.avg;
                println!("  Est. wasted tokens   : {:>8.0}", est_waste);
            }

            // Per-tool breakdown
            let tool_types = ["Bash", "Read", "Grep"];
            let mut has_tool_data = false;
            for tool in &tool_types {
                let key = format!("post_tool_{}", tool);
                if let Some(s) = metrics::summary(&metrics_conn, metrics::MetricType::Custom(&key))
                    .ok()
                    .flatten()
                {
                    if !has_tool_data {
                        println!();
                        println!("  Per-tool output:");
                        has_tool_data = true;
                    }
                    println!(
                        "    {:<8} {:>6} calls, {:>8.0} avg tok, {:>10.0} total tok",
                        tool, s.count, s.avg, s.total
                    );
                }
            }
            println!();
        }
    }

    // Hook latency percentiles
    if let Ok(metrics_conn) = db::open_metrics(&data_dir) {
        let lat = telemetry::hook_latency_percentiles(&metrics_conn)?;
        if lat.count > 0 {
            println!("Hook Latency Percentiles");
            println!("------------------------");
            println!("  p50: {:.2} ms", lat.p50_ms);
            println!("  p99: {:.2} ms", lat.p99_ms);
            println!("  Samples: {}", lat.count);
            println!();
        }
    }

    // Database sizes
    println!("Database Sizes");
    println!("--------------");
    for name in &["heuristics.db", "history.db", "metrics.db"] {
        let path = data_dir.join(name);
        if path.exists() {
            if let Ok(meta) = std::fs::metadata(&path) {
                let size_kb = meta.len() as f64 / 1024.0;
                println!("  {name:<16} {size_kb:>8.1} KB");
            }
        } else {
            println!("  {name:<16} (not created)");
        }
    }

    // Telemetry: send anonymous aggregated data if consented (rate-limited)
    let _ = telemetry::maybe_send(&data_dir);

    Ok(())
}

fn report_section(
    conn: &rusqlite::Connection,
    label: &str,
    metric_type: metrics::MetricType,
) -> Result<()> {
    match metrics::summary(conn, metric_type)? {
        Some(s) => {
            println!("{label}");
            println!("{}", "-".repeat(label.len()));
            println!("  Count: {}", s.count);
            println!("  Avg:   {:.2}", s.avg);
            println!("  Min:   {:.2}", s.min);
            println!("  Max:   {:.2}", s.max);
            println!("  Total: {:.2}", s.total);
            println!();
        }
        None => {
            println!("{label}: no data");
            println!();
        }
    }
    Ok(())
}

// =============================================================================
// precc savings
// =============================================================================

/// Token-savings estimates per event type.
///
/// RTK baseline (output compression only):
///   Per-category estimates stored in `RewriteRule::est_tokens_saved`.
///   For records without per-command metadata, we query the weighted average
///   across all rules (≈175 tok) as the blended estimate.
///
/// PRECC-over-RTK gains (error prevention — these do NOT overlap with RTK):
///   • CD prepend (Pillar 1): a wrong-dir failure produces ~1 failed tool call
///     (~80 tokens output) + ~1 retry tool call + model re-reasoning ≈ 300 tokens
///     saved per prevented miss.
///   • Skill auto-fix (Pillar 4): each auto-applied skill prevents ~1 failure
///     cycle (fail output + model re-read + retry) ≈ 250 tokens saved.
///   • Mined pattern occurrences (Pillar 3): similar to skill auto-fix when the
///     pattern has been promoted; each additional occurrence prevented ≈ 200 tokens.
///
/// All figures are deliberately conservative; real savings depend on model,
/// session length, and verbosity settings.
struct TokenModel {
    /// Tokens saved per RTK rewrite (weighted average across all rule categories).
    rtk_per_rewrite_avg: f64,
    /// PRECC-over-RTK tokens saved per prevented wrong-dir failure.
    precc_per_cd_prepend: f64,
    /// PRECC-over-RTK tokens saved per skill auto-activation.
    precc_per_skill_activation: f64,
    /// PRECC-over-RTK tokens saved per mined pattern occurrence (above 1st).
    precc_per_mined_occurrence: f64,
}

impl Default for TokenModel {
    fn default() -> Self {
        // Compute weighted average of est_tokens_saved across all RTK rules
        // (assumes uniform distribution of matched commands across rule categories).
        let avg = rtk_weighted_avg_tokens();
        Self {
            rtk_per_rewrite_avg: avg,
            precc_per_cd_prepend: 300.0,
            precc_per_skill_activation: 250.0,
            precc_per_mined_occurrence: 200.0,
        }
    }
}

/// Compute the weighted average of `est_tokens_saved` across all RTK rules.
fn rtk_weighted_avg_tokens() -> f64 {
    // Use the public tokens_saved function to query a representative set of commands.
    // We sample one command per rule by querying the rule's `from` string directly.
    // Since we can't access the private RULES array from precc-cli, we use a hardcoded
    // representative set that matches the rule categories.
    let samples: &[(&str, u32)] = &[
        ("cargo build", 420),
        ("cargo test", 420),
        ("cargo clippy", 420),
        ("cargo check", 300),
        ("cargo run", 200),
        ("cargo fmt", 60),
        ("git status", 160),
        ("git diff", 160),
        ("git log", 160),
        ("git add", 60),
        ("git commit", 60),
        ("git push", 60),
        ("git pull", 60),
        ("git branch", 60),
        ("git fetch", 60),
        ("git stash", 60),
        ("git show", 60),
        ("gh pr", 120),
        ("gh issue", 120),
        ("gh run", 120),
        ("npm test", 420),
        ("npm run", 180),
        ("npm install", 150),
        ("yarn test", 420),
        ("yarn add", 100),
        ("pytest", 380),
        ("python -m pytest", 380),
        ("pip install", 150),
        ("go test", 380),
        ("go build", 300),
        ("cat", 50),
        ("ls", 40),
        ("rg", 90),
        ("grep", 90),
        ("pnpm test", 180),
        ("vitest", 180),
        ("tsc", 180),
        ("eslint", 180),
        ("prettier", 180),
        ("playwright", 180),
        ("prisma", 180),
        ("docker build", 500),
        ("docker run", 200),
        ("docker ps", 150),
        ("docker images", 150),
        ("docker logs", 150),
        ("kubectl describe", 300),
        ("kubectl apply", 150),
        ("kubectl get", 180),
        ("kubectl logs", 180),
        ("curl", 200),
        ("pnpm list", 100),
        ("pnpm ls", 100),
        ("pnpm outdated", 100),
        ("make", 400),
    ];
    let total: u64 = samples.iter().map(|(_, t)| *t as u64).sum();
    let count = samples.len() as f64;
    // Verify a sample against the actual rtk module
    let _ = rtk::tokens_saved("cargo build"); // compile-time check
    total as f64 / count
}

fn cmd_savings(all: bool) -> Result<()> {
    if all && license::tier() == license::Tier::Free {
        return Err(license::require_paid("Detailed savings breakdown (--all)"));
    }
    let data_dir = db::data_dir()?;
    let model = TokenModel::default();

    println!("PRECC Token Savings Estimate");
    println!("============================");
    println!();

    // ---- RTK baseline (from metrics.db) --------------------------------
    let rtk_rewrite_count: i64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::RtkRewrite)?
            .map(|s| s.count as i64)
            .unwrap_or(0)
    } else {
        0
    };

    let rtk_tokens = rtk_rewrite_count as f64 * model.rtk_per_rewrite_avg;

    println!("RTK baseline (output compression)");
    println!("---------------------------------");
    println!("  RTK rewrites recorded : {:>8}", rtk_rewrite_count);
    println!(
        "  Est. tokens/rewrite   : {:>8.0}  (per-category weighted avg)",
        model.rtk_per_rewrite_avg
    );
    println!("  RTK gain (tokens)     : {:>8.0}", rtk_tokens);
    println!();

    // ---- PRECC-over-RTK gains (from metrics.db + heuristics.db + history.db) --
    let cd_count: i64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::CdPrepend)?
            .map(|s| s.count as i64)
            .unwrap_or(0)
    } else {
        0
    };

    let skill_activations: i64 = if let Ok(conn) = db::open_heuristics(&data_dir) {
        conn.query_row(
            "SELECT COALESCE(SUM(activated), 0) FROM skill_stats",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0)
    } else {
        0
    };

    // Mined pattern occurrences: sum of (occurrences - 1) for all patterns
    // where occurrences > 1 (the first occurrence is the "learning" event,
    // subsequent occurrences are preventions), plus any PRECC-prevented failures
    // detected retroactively from session logs.
    let mined_preventions: i64 = if let Ok(conn) = db::open_history(&data_dir) {
        conn.query_row(
            "SELECT COALESCE(SUM(occurrences - 1), 0) + COALESCE(SUM(precc_prevented), 0)
             FROM failure_fix_pairs
             WHERE occurrences > 1 OR precc_prevented > 0",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| {
            // Fallback: precc_prevented column may not exist on older DBs
            conn.query_row(
                "SELECT COALESCE(SUM(occurrences - 1), 0) FROM failure_fix_pairs WHERE occurrences > 1",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0)
        })
    } else {
        0
    };

    let cd_tokens = cd_count as f64 * model.precc_per_cd_prepend;
    let skill_tokens = skill_activations as f64 * model.precc_per_skill_activation;
    let mined_tokens = mined_preventions as f64 * model.precc_per_mined_occurrence;
    let precc_over_rtk = cd_tokens + skill_tokens + mined_tokens;

    println!("PRECC gain over RTK (error prevention)");
    println!("--------------------------------------");
    println!(
        "  CD prepends (Pillar 1): {:>8}  × {:>4.0} tok = {:>8.0} tok",
        cd_count, model.precc_per_cd_prepend, cd_tokens
    );
    println!(
        "  Skill activations (P4): {:>8}  × {:>4.0} tok = {:>8.0} tok",
        skill_activations, model.precc_per_skill_activation, skill_tokens
    );
    println!(
        "  Mined preventions (P3): {:>8}  × {:>4.0} tok = {:>8.0} tok",
        mined_preventions, model.precc_per_mined_occurrence, mined_tokens
    );
    println!();
    println!("  PRECC-over-RTK total  : {:>8.0} tokens", precc_over_rtk);
    println!();

    // ---- CCC semantic search savings (Pillar 2b) -------------------------
    let ccc_count: i64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::Custom("ccc_redirect"))?
            .map(|s| s.count as i64)
            .unwrap_or(0)
    } else {
        0
    };
    let ccc_saved_bytes: f64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::Custom("ccc_redirect"))?
            .map(|s| s.total)
            .unwrap_or(0.0)
    } else {
        0.0
    };
    let ccc_tokens = ccc_saved_bytes / 4.0;

    if ccc_count > 0 {
        println!("CCC semantic search savings (Pillar 2b)");
        println!("---------------------------------------");
        println!("  grep/rg redirected    : {:>8}", ccc_count);
        println!("  Est. tokens saved     : {:>8.0}", ccc_tokens);
        println!();
    }

    // ---- Context compression savings (Pillar 3) --------------------------
    let compress_count: i64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::Custom("context_compress"))?
            .map(|s| s.count as i64)
            .unwrap_or(0)
    } else {
        0
    };
    let compress_tokens: f64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::Custom("context_compress"))?
            .map(|s| s.total)
            .unwrap_or(0.0)
    } else {
        0.0
    };

    if compress_count > 0 {
        println!("Context file compression savings (Pillar 3)");
        println!("--------------------------------------------");
        println!("  Compression runs      : {:>8}", compress_count);
        println!("  Tokens saved          : {:>8.0}", compress_tokens);
        println!();
    }

    // ---- lean-ctx output compression (external) --------------------------
    let lean_ctx_count: i64 = if let Ok(conn) = db::open_metrics(&data_dir) {
        metrics::summary(&conn, metrics::MetricType::Custom("lean_ctx_wrap"))?
            .map(|s| s.count as i64)
            .unwrap_or(0)
    } else {
        0
    };

    let lean_ctx_tokens = lean_ctx_count as f64 * 350.0; // conservative: 70-80% compression

    if lean_ctx_count > 0 {
        println!("lean-ctx output compression (external)");
        println!("--------------------------------------");
        println!("  Commands wrapped      : {:>8}", lean_ctx_count);
        println!("  Est. tokens/wrap      :      350  (conservative avg)");
        println!("  Est. tokens saved     : {:>8.0}", lean_ctx_tokens);
        println!();
    }

    // ---- Grand total ---------------------------------------------------
    let grand_total = rtk_tokens + precc_over_rtk + ccc_tokens + compress_tokens + lean_ctx_tokens;
    let precc_pct = if grand_total > 0.0 {
        precc_over_rtk / grand_total * 100.0
    } else {
        0.0
    };

    // ---- Token breakdown by tool type (from session log content) -----------
    let breakdown = mining::session_token_breakdown().ok();

    println!("Summary");
    println!("-------");
    println!("  RTK baseline          : {:>8.0} tokens", rtk_tokens);
    println!("  PRECC additional gain : {:>8.0} tokens", precc_over_rtk);
    println!("  Grand total saved     : {:>8.0} tokens", grand_total);
    if grand_total > 0.0 {
        println!("  PRECC share of savings: {:>7.1}%", precc_pct);
    }
    println!();

    if !all {
        println!("Run `precc savings --all` for full per-tool and per-skill breakdown (Pro).");
        println!();
        println!("Note: figures are estimates based on conservative medians per event.");
        return Ok(());
    }

    if let Some(ref bd) = breakdown {
        let bash = bd.tool("Bash");
        let bash_total = (bash.input_bytes + bash.output_bytes) as f64 / 4.0;
        let api_tokens = bd.api_relevant_bytes() as f64 / 4.0;

        let bash_saving_ratio = if bash_total > 0.0 {
            grand_total / bash_total * 100.0
        } else {
            0.0
        };
        let api_saving_ratio = if api_tokens > 0.0 {
            grand_total / api_tokens * 100.0
        } else {
            0.0
        };

        println!("Saving Ratio (API-relevant tokens only)");
        println!("---------------------------------------");
        if api_tokens > 0.0 {
            println!("  API-relevant tokens             : {:>10.0}", api_tokens);
            println!("  Tokens saved                    : {:>10.0}", grand_total);
            println!(
                "  vs all API content              : {:>9.1}%",
                api_saving_ratio
            );
        }
        if bash_total > 0.0 {
            println!(
                "  vs Bash tool spend              : {:>9.1}%",
                bash_saving_ratio
            );
        }
        println!();

        // ---- Per-tool token distribution table ------------------------------
        println!("Token Distribution by Category (API-relevant)");
        println!("----------------------------------------------");
        println!(
            "  {:<14} {:>8} {:>10} {:>10} {:>10}  {:>5}",
            "Category", "Calls", "Input tok", "Output tok", "Total tok", "Share"
        );
        println!("  {}", "-".repeat(67));

        // Collect and sort tools by total bytes descending
        let mut tool_rows: Vec<(&str, &mining::ToolBytes)> =
            bd.tools.iter().map(|(k, v)| (k.as_str(), v)).collect();
        tool_rows.sort_by(|a, b| {
            (b.1.input_bytes + b.1.output_bytes).cmp(&(a.1.input_bytes + a.1.output_bytes))
        });

        for (name, tb) in &tool_rows {
            let input_tok = tb.input_bytes as f64 / 4.0;
            let output_tok = tb.output_bytes as f64 / 4.0;
            let total_tok = input_tok + output_tok;
            let pct = if api_tokens > 0.0 {
                total_tok / api_tokens * 100.0
            } else {
                0.0
            };
            let optimized = match *name {
                "Bash" | "Read" | "Grep" | "Agent" => " ✓",
                _ => "",
            };
            println!(
                "  {:<14} {:>8} {:>10.0} {:>10.0} {:>10.0}  {:>4.1}%{}",
                name, tb.invocations, input_tok, output_tok, total_tok, pct, optimized
            );
        }

        // Thinking, assistant text, user text
        let think_tok = bd.thinking_bytes as f64 / 4.0;
        let asst_tok = bd.assistant_text_bytes as f64 / 4.0;
        let user_tok = bd.user_text_bytes as f64 / 4.0;
        let pct = |v: f64| -> f64 {
            if api_tokens > 0.0 {
                v / api_tokens * 100.0
            } else {
                0.0
            }
        };
        println!("  {}", "-".repeat(67));
        println!(
            "  {:<14} {:>8} {:>10} {:>10.0} {:>10.0}  {:>4.1}%",
            "Thinking",
            "",
            "",
            think_tok,
            think_tok,
            pct(think_tok)
        );
        println!(
            "  {:<14} {:>8} {:>10} {:>10.0} {:>10.0}  {:>4.1}%",
            "Asst. text",
            "",
            "",
            asst_tok,
            asst_tok,
            pct(asst_tok)
        );
        println!(
            "  {:<14} {:>8} {:>10} {:>10.0} {:>10.0}  {:>4.1}%",
            "User text",
            "",
            "",
            user_tok,
            user_tok,
            pct(user_tok)
        );
        println!("  {}", "-".repeat(67));
        println!(
            "  {:<14} {:>8} {:>10} {:>10} {:>10.0}  100.0%",
            "API TOTAL", "", "", "", api_tokens
        );
        println!();
    }

    // ---- Per-tool optimization rates (from metrics.log) --------------------
    if let Ok(conn) = db::open_metrics(&data_dir) {
        let read_filter_count: i64 =
            metrics::summary(&conn, metrics::MetricType::Custom("read_filter"))
                .ok()
                .flatten()
                .map(|s| s.count as i64)
                .unwrap_or(0);
        let grep_filter_count: i64 =
            metrics::summary(&conn, metrics::MetricType::Custom("grep_filter"))
                .ok()
                .flatten()
                .map(|s| s.count as i64)
                .unwrap_or(0);
        let agent_propagate_count: i64 =
            metrics::summary(&conn, metrics::MetricType::Custom("agent_propagate"))
                .ok()
                .flatten()
                .map(|s| s.count as i64)
                .unwrap_or(0);

        if read_filter_count > 0 || grep_filter_count > 0 || agent_propagate_count > 0 {
            println!("Token Optimization by Tool");
            println!("--------------------------");
            println!(
                "  Bash   : optimized (RTK rewrite + skills) [{} rewrites]",
                rtk_rewrite_count
            );
            if read_filter_count > 0 {
                println!(
                    "  Read   : optimized (binary block + limit inject) [{} interventions]",
                    read_filter_count
                );
            }
            if grep_filter_count > 0 {
                println!(
                    "  Grep   : optimized (head_limit + type filter) [{} interventions]",
                    grep_filter_count
                );
            }
            if agent_propagate_count > 0 {
                println!(
                    "  Agent  : propagated to {} subagents",
                    agent_propagate_count
                );
            }
            println!();
        }
    }

    // ---- Per-skill ablation (PRECC-over-RTK only) -------------------------
    if let Ok(heuristics_conn) = db::open_heuristics(&data_dir) {
        if let Ok(skill_rows) = telemetry::per_skill_stats(&heuristics_conn) {
            if !skill_rows.is_empty() {
                let total_skill_tokens: f64 = skill_rows.iter().map(|s| s.est_tokens_saved).sum();
                println!("Per-Skill Ablation (PRECC-over-RTK)");
                println!("-----------------------------------");
                for row in &skill_rows {
                    let pct = if total_skill_tokens > 0.0 {
                        row.est_tokens_saved / total_skill_tokens * 100.0
                    } else {
                        0.0
                    };
                    println!(
                        "  {:<25} : {:>8.0} tok  ({:>5.1}%)",
                        truncate_str(&row.skill_name, 25),
                        row.est_tokens_saved,
                        pct,
                    );
                }
                println!();
            }
        }
    }

    println!("Note: figures are estimates based on conservative medians per event.");
    println!(
        "      RTK ~{:.0} tok/rewrite (weighted avg), CD-miss ~{:.0} tok, skill ~{:.0} tok, pattern ~{:.0} tok.",
        model.rtk_per_rewrite_avg,
        model.precc_per_cd_prepend,
        model.precc_per_skill_activation,
        model.precc_per_mined_occurrence,
    );

    // Telemetry: send anonymous aggregated data if consented (rate-limited)
    let _ = telemetry::maybe_send(&data_dir);

    Ok(())
}

// =============================================================================
// Compress
// =============================================================================

fn cmd_compress(dry_run: bool, revert: bool, dir: Option<String>) -> Result<()> {
    let project_dir = match dir {
        Some(d) => std::path::PathBuf::from(d),
        None => std::env::current_dir()?,
    };

    if revert {
        let count = compress::revert_files(&project_dir)?;
        if count > 0 {
            println!("Reverted {} file(s).", count);
        } else {
            println!("No backups found.");
        }
        return Ok(());
    }

    let files = compress::discover_files(&project_dir);
    if files.is_empty() {
        println!("No context files found (CLAUDE.md, .claude/memory/*.md).");
        return Ok(());
    }

    println!("Compressing {} context file(s)...", files.len());
    println!();

    let results = compress::compress_files(&project_dir, dry_run)?;

    for r in &results {
        let rel = r
            .file
            .strip_prefix(&project_dir)
            .unwrap_or(&r.file)
            .display();
        println!(
            "  {}: {} -> {} tokens (saved {}, {}%)",
            rel, r.original_tokens, r.compressed_tokens, r.saved_tokens, r.pct_saved
        );
    }

    if results.is_empty() {
        println!("All files already compact. Nothing to do.");
        return Ok(());
    }

    let total_saved: usize = results.iter().map(|r| r.saved_tokens).sum();
    let total_orig: usize = results.iter().map(|r| r.original_tokens).sum();
    let total_pct = if total_orig > 0 {
        total_saved * 100 / total_orig
    } else {
        0
    };

    println!();
    println!("Total: {} tokens saved ({}%)", total_saved, total_pct);

    if dry_run {
        println!("(dry run -- no files modified)");
    } else {
        // Log to metrics.db
        if let Ok(data_dir) = db::data_dir() {
            if let Ok(conn) = db::open_metrics(&data_dir) {
                let _ = metrics::record(
                    &conn,
                    metrics::MetricType::Custom("context_compress"),
                    total_saved as f64,
                    Some(&format!(
                        "{{\"files\":{},\"pct\":{}}}",
                        results.len(),
                        total_pct
                    )),
                );
            }
        }
        println!("Backups saved as *.backup. Revert with: precc compress --revert");
    }

    Ok(())
}

// =============================================================================
// GHA (GitHub Actions analysis)
// =============================================================================

fn cmd_gha(url: String) -> Result<()> {
    let diagnosis = gha::analyze(&url)?;
    println!("{}", serde_json::to_string_pretty(&diagnosis)?);
    Ok(())
}

// =============================================================================
// License
// =============================================================================

fn cmd_license(action: LicenseAction) -> Result<()> {
    match action {
        LicenseAction::Activate { key, email, github } => {
            let lic = if github {
                // GitHub Sponsors — verify via GitHub API
                license::activate_github()?
            } else {
                let key = key.ok_or_else(|| {
                    anyhow::anyhow!(
                        "License key required. Use --github for GitHub Sponsors, or provide a key."
                    )
                })?;
                if key.starts_with("PRECC-") {
                    // PRECC native key
                    if let Some(ref email) = email {
                        license::activate_with_email(&key, email)?
                    } else {
                        license::activate(&key)?
                    }
                } else if license::is_stripe_key(&key) {
                    // Stripe key — verify online
                    license::activate_stripe(&key)?
                } else {
                    // Gumroad key — verify online
                    license::activate_gumroad(&key)?
                }
            };
            println!("License activated successfully.");
            println!("  Edition:        {}", lic.edition_name());
            println!(
                "  Machine-bound:  {}",
                if lic.machine_bound { "yes" } else { "no" }
            );
            println!("  Expires:        {}", lic.expiry_date());
            if let Some(remaining) = lic.days_remaining() {
                if remaining > 0 {
                    println!("  Days remaining: {remaining}");
                } else {
                    println!("  Status:         EXPIRED");
                }
            }
            Ok(())
        }
        LicenseAction::Status => {
            match license::load()? {
                Some(lic) => {
                    println!("License status: ACTIVE");
                    println!("  Edition:        {}", lic.edition_name());
                    println!(
                        "  Machine-bound:  {}",
                        if lic.machine_bound { "yes" } else { "no" }
                    );
                    if lic.expiry_days > 0 {
                        println!(
                            "  Expires:        day {} (Unix epoch days)",
                            lic.expiry_days
                        );
                    } else {
                        println!("  Expires:        never");
                    }
                }
                None => {
                    println!("License status: COMMUNITY (no key activated)");
                    println!("  All core features available. Activate a Pro/Team key for");
                    println!("  priority support and enterprise features.");
                }
            }
            Ok(())
        }
        LicenseAction::Deactivate => {
            license::deactivate()?;
            println!("License key removed. Running in community mode.");
            Ok(())
        }
        LicenseAction::Fingerprint => {
            let fp = license::machine_fingerprint();
            println!(
                "Machine fingerprint: {:02x}{:02x}{:02x}{:02x}",
                fp[0], fp[1], fp[2], fp[3]
            );
            println!("(Provide this to generate a machine-bound license key)");
            Ok(())
        }
        LicenseAction::Generate {
            edition,
            fingerprint,
            email,
            expiry_days,
        } => {
            // Restricted: only authorized machines can generate keys
            const AUTHORIZED_FPS: &[[u8; 4]] = &[
                [0xF2, 0x9C, 0x7D, 0x98], // dev machine
                [0xE5, 0x95, 0x35, 0xC6], // peria.ai server (old)
                [0x86, 0x29, 0x4B, 0x99], // peria.ai server
            ];
            let local_fp = license::machine_fingerprint();
            if !AUTHORIZED_FPS.contains(&local_fp) {
                bail!(
                    "Key generation is not available on this machine (fp={:02x}{:02x}{:02x}{:02x})",
                    local_fp[0],
                    local_fp[1],
                    local_fp[2],
                    local_fp[3]
                );
            }

            let machine_tag = if let Some(ref email) = email {
                license::email_fingerprint(email)
            } else {
                match &fingerprint {
                    Some(hex_str) => {
                        if hex_str.len() != 8 {
                            bail!("fingerprint must be exactly 8 hex chars (4 bytes)");
                        }
                        let parse_byte = |i: usize| -> Result<u8> {
                            u8::from_str_radix(&hex_str[i..i + 2], 16)
                                .context("invalid hex in fingerprint")
                        };
                        [
                            parse_byte(0)?,
                            parse_byte(2)?,
                            parse_byte(4)?,
                            parse_byte(6)?,
                        ]
                    }
                    None => [0u8; 4], // unbound
                }
            };

            let edition_flags: u32 = match edition.to_lowercase().as_str() {
                "pro" => 1,
                "team" => 3,       // pro + team
                "enterprise" => 7, // pro + team + enterprise
                other => bail!("unknown edition: {other} (use pro, team, or enterprise)"),
            };

            let expiry = if expiry_days > 0 {
                let now_days = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    / 86400;
                (now_days as u32) + expiry_days
            } else {
                0
            };

            let key = license::generate(machine_tag, expiry, edition_flags);
            println!("{key}");
            Ok(())
        }
    }
}

// =============================================================================
// Mail
// =============================================================================

fn cmd_mail(action: MailAction) -> Result<()> {
    if license::tier() == license::Tier::Free {
        return Err(license::require_paid("Email sending"));
    }
    match action {
        MailAction::Setup => mail::cmd_mail_setup(),
        MailAction::Report { to, attachments } => mail::cmd_mail_report(&to, &attachments),
        MailAction::Send {
            to,
            subject,
            body,
            attachments,
        } => {
            mail::send_mail(&to, &subject, &body, &attachments)?;
            println!("Email sent to {to}");
            Ok(())
        }
    }
}

// =============================================================================
// Telemetry
// =============================================================================

fn cmd_telemetry(action: TelemetryAction) -> Result<()> {
    match action {
        TelemetryAction::Consent => {
            println!("PRECC Anonymous Telemetry");
            println!("========================");
            println!();
            println!("By opting in, PRECC will periodically send aggregated, anonymous");
            println!("usage data to help improve the tool.  Data includes:");
            println!();
            println!("  - Per-skill activation, success, and failure counts");
            println!("  - Token savings estimates per pillar (aggregated totals)");
            println!("  - Hook latency percentiles (p50, p99)");
            println!("  - PRECC version, OS, architecture, license tier");
            println!();
            println!("NO personally identifiable information is collected:");
            println!("  - No commands, file paths, or project names");
            println!("  - No usernames, hostnames, or machine IDs");
            println!("  - No session content or code");
            println!();
            println!("You can revoke consent at any time with: precc telemetry revoke");
            println!("You can also disable all telemetry with: PRECC_NO_TELEMETRY=1");
            println!();
            print!("Do you consent to anonymous telemetry? [yes/no]: ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let answer = input.trim().to_lowercase();

            if answer == "yes" || answer == "y" {
                consent::save(true)?;
                println!("Telemetry enabled.  Thank you!");
            } else {
                println!("Telemetry NOT enabled.  No data will be sent.");
            }
            Ok(())
        }
        TelemetryAction::Revoke => {
            consent::save(false)?;
            println!("Telemetry consent revoked.  No data will be sent.");
            Ok(())
        }
        TelemetryAction::Status => {
            if std::env::var("PRECC_NO_TELEMETRY").is_ok() {
                println!("Telemetry: DISABLED (PRECC_NO_TELEMETRY is set)");
            } else if consent::is_telemetry_enabled() {
                println!("Telemetry: ENABLED (consent v{})", consent::CONSENT_VERSION);
            } else {
                println!("Telemetry: NOT ENABLED");
                println!("  Run `precc telemetry consent` to opt in.");
            }
            Ok(())
        }
        TelemetryAction::Preview => {
            let data_dir = db::data_dir()?;
            let payload = telemetry::build_payload(&data_dir)?;
            let json = serde_json::to_string_pretty(&payload)?;
            println!("Telemetry payload preview (this is exactly what would be sent):");
            println!();
            println!("{json}");
            Ok(())
        }
    }
}

// =============================================================================
// Geofence
// =============================================================================

fn cmd_geofence(action: GeofenceAction) -> Result<()> {
    if !license::tier().is_paid() {
        return Err(license::require_paid("Geofence compliance guard"));
    }

    match action {
        GeofenceAction::Check => {
            match geofence::read_cache()? {
                Some(cache) => {
                    println!("Geofence status:");
                    println!("  IP:      {}", cache.ip);
                    println!("  Country: {} ({})", cache.country_name, cache.country_code);
                    if cache.blocked {
                        println!("  Status:  BLOCKED");
                        println!();
                        println!("{}", geofence::format_deny_message(&cache));
                    } else {
                        println!("  Status:  OK (allowed region)");
                    }
                    let age = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        .saturating_sub(cache.timestamp);
                    println!("  Age:     {}s ago", age);
                }
                None => {
                    println!("No geofence cache found.");
                    println!("Run `precc geofence refresh` to probe your IP.");
                }
            }
            Ok(())
        }
        GeofenceAction::Refresh => {
            println!("Probing IP geolocation...");
            let cache = geofence::refresh_cache()?;
            println!("  IP:      {}", cache.ip);
            println!("  Country: {} ({})", cache.country_name, cache.country_code);
            if cache.blocked {
                println!("  Status:  BLOCKED");
                println!();
                println!("{}", geofence::format_deny_message(&cache));
            } else {
                println!("  Status:  OK (allowed region)");
            }
            println!("Cache updated.");
            Ok(())
        }
        GeofenceAction::Clear => {
            geofence::clear_cache()?;
            println!("Geofence cache cleared.");
            Ok(())
        }
        GeofenceAction::Info => {
            println!("Blocked regions (Anthropic API restricted):");
            for code in geofence::blocked_countries() {
                println!("  {code}");
            }
            println!();
            println!("Alternative LLM providers for blocked regions:");
            for alt in geofence::ALTERNATIVES {
                println!("  {} ({}) — {}", alt.name, alt.provider, alt.notes);
                println!("    API: {}", alt.api_url);
            }
            println!();
            println!("Override: set PRECC_GEOFENCE_OVERRIDE=1 to bypass (at your own risk)");
            Ok(())
        }
    }
}

// =============================================================================
// Nushell
// =============================================================================

fn cmd_nushell(action: NushellAction) -> Result<()> {
    match action {
        NushellAction::Check => {
            if nushell::nu_available() {
                // Try to get version
                let version = std::process::Command::new("nu")
                    .arg("--version")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_else(|| "unknown".to_string());
                println!("Nushell: AVAILABLE (v{})", version.trim());
                println!(
                    "Rules:   {} translation rules loaded",
                    nushell::rule_count()
                );
                println!(
                    "Mode:    {}",
                    if nushell::nushell_mode_enabled() {
                        "ENABLED (PRECC_NUSHELL is set)"
                    } else {
                        "DISABLED (set PRECC_NUSHELL=1 to enable)"
                    }
                );
            } else {
                println!("Nushell: NOT FOUND");
                println!("Install: cargo install nu");
                println!("         or https://www.nushell.sh/book/installation.html");
            }
            Ok(())
        }
        NushellAction::Translate { command } => {
            match nushell::translate_preview(&command) {
                Some((from, to, baseline, ratio)) => {
                    println!("Bash:    {}", command);
                    println!("Nushell: {}", to);
                    println!("Matched: \"{}\" rule", from);
                    println!("RTK est: {} tokens saved", baseline);
                    println!("Nu est:  {:.0}% output reduction", (1.0 - ratio) * 100.0);
                }
                None => {
                    if !nushell::is_nu_safe(&command) {
                        println!("Command contains bash-specific syntax (not nu-safe)");
                    } else {
                        println!("No nushell translation rule for: {}", command);
                    }
                    println!("Fallback: RTK rewriting would be used");
                }
            }
            Ok(())
        }
        NushellAction::Benchmark {
            command,
            iterations,
        } => {
            if !nushell::nu_available() {
                bail!("Nushell is not installed. Install with: cargo install nu");
            }

            let scenarios: Vec<String> = if let Some(cmd) = command {
                vec![cmd]
            } else {
                // Default benchmark scenarios (commands likely to exist in a dev environment)
                vec![
                    "cargo build".to_string(),
                    "git status".to_string(),
                    "git log".to_string(),
                    "git diff".to_string(),
                    "ls".to_string(),
                ]
            };

            println!(
                "{:<20} {:>8} {:>8} {:>8} {:>7} {:>7}",
                "Command", "Bash", "RTK", "Nushell", "RTK %", "Nu %"
            );
            println!("{}", "-".repeat(70));

            for cmd in &scenarios {
                let nu_preview = nushell::translate_preview(cmd);
                let rtk_preview = precc_core::rtk::tokens_saved(cmd);

                let mut bash_tokens = 0u64;
                let mut nu_tokens = 0u64;

                // Run bash version
                for _ in 0..iterations {
                    if let Ok(output) = std::process::Command::new("bash")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                    {
                        let text = String::from_utf8_lossy(&output.stdout).to_string()
                            + &String::from_utf8_lossy(&output.stderr);
                        bash_tokens += nushell::estimate_tokens(&text) as u64;
                    }
                }
                bash_tokens /= iterations as u64;

                // Run nushell version
                if let Some((_, nu_cmd, _, _)) = nu_preview {
                    for _ in 0..iterations {
                        if let Ok(output) = std::process::Command::new("bash")
                            .arg("-c")
                            .arg(nu_cmd)
                            .output()
                        {
                            let text = String::from_utf8_lossy(&output.stdout).to_string()
                                + &String::from_utf8_lossy(&output.stderr);
                            nu_tokens += nushell::estimate_tokens(&text) as u64;
                        }
                    }
                    nu_tokens /= iterations as u64;
                }

                // RTK estimated tokens (bash - estimated savings)
                let rtk_tokens = bash_tokens.saturating_sub(rtk_preview as u64);

                let rtk_pct = if bash_tokens > 0 {
                    ((bash_tokens as f64 - rtk_tokens as f64) / bash_tokens as f64 * 100.0) as i32
                } else {
                    0
                };
                let nu_pct = if bash_tokens > 0 {
                    ((bash_tokens as f64 - nu_tokens as f64) / bash_tokens as f64 * 100.0) as i32
                } else {
                    0
                };

                let nu_display = if nu_preview.is_some() {
                    format!("{:>5}t", nu_tokens)
                } else {
                    "  n/a".to_string()
                };
                let nu_pct_display = if nu_preview.is_some() {
                    format!("{:>5}%", nu_pct)
                } else {
                    "  n/a".to_string()
                };

                println!(
                    "{:<20} {:>5}t {:>5}t {} {:>5}% {}",
                    if cmd.len() > 20 { &cmd[..20] } else { cmd },
                    bash_tokens,
                    rtk_tokens,
                    nu_display,
                    rtk_pct,
                    nu_pct_display,
                );
            }

            println!();
            println!("Note: RTK tokens are estimated (bash - RTK est. savings).");
            println!("      Nushell tokens are measured from actual nu output.");
            Ok(())
        }
        NushellAction::Rules => {
            println!("{:<25} {:<60} {:>6}", "Match", "Nushell Command", "RTK Δ");
            println!("{}", "-".repeat(95));
            for (from, to, baseline, ratio) in nushell::rules_summary() {
                let to_display = if to.len() > 50 {
                    format!("{}...", &to[..47])
                } else {
                    to.to_string()
                };
                println!(
                    "{:<25} {:<52} {:>5}t {:>4.0}%",
                    from,
                    to_display,
                    baseline,
                    (1.0 - ratio) * 100.0
                );
            }
            println!();
            println!("{} rules total", nushell::rule_count());
            Ok(())
        }
        NushellAction::WhatIf { csv } => {
            println!("Scanning historical session logs...");
            let analysis = nushell::what_if_analysis()?;

            if analysis.session_count == 0 {
                println!("No session logs found in ~/.claude/projects/");
                println!("Run some Claude Code sessions first, then re-run this command.");
                return Ok(());
            }

            // Compute totals
            let mut total = nushell::WhatIfResult::default();
            for r in analysis.by_category.values() {
                total.total_commands += r.total_commands;
                total.bash_tokens += r.bash_tokens;
                total.rtk_tokens += r.rtk_tokens;
                total.nushell_tokens += r.nushell_tokens;
                total.nushell_matched += r.nushell_matched;
                total.nushell_unmatched += r.nushell_unmatched;
            }

            if csv {
                // CSV output
                println!("Category,Commands,Bash_Tokens,RTK_Tokens,Nushell_Tokens,RTK_Saving_%,Nushell_Saving_%,Nu_Matched,Nu_Unmatched");
                for cat in nushell::UsageCategory::all() {
                    let r = analysis.by_category.get(cat).cloned().unwrap_or_default();
                    if r.total_commands == 0 {
                        continue;
                    }
                    let rtk_pct = if r.bash_tokens > 0 {
                        (r.bash_tokens as f64 - r.rtk_tokens as f64) / r.bash_tokens as f64 * 100.0
                    } else {
                        0.0
                    };
                    let nu_pct = if r.bash_tokens > 0 {
                        (r.bash_tokens as f64 - r.nushell_tokens as f64) / r.bash_tokens as f64
                            * 100.0
                    } else {
                        0.0
                    };
                    println!(
                        "{},{},{},{},{},{:.1},{:.1},{},{}",
                        cat.name(),
                        r.total_commands,
                        r.bash_tokens,
                        r.rtk_tokens,
                        r.nushell_tokens,
                        rtk_pct,
                        nu_pct,
                        r.nushell_matched,
                        r.nushell_unmatched,
                    );
                }
                // Total row
                let rtk_pct_t = if total.bash_tokens > 0 {
                    (total.bash_tokens as f64 - total.rtk_tokens as f64) / total.bash_tokens as f64
                        * 100.0
                } else {
                    0.0
                };
                let nu_pct_t = if total.bash_tokens > 0 {
                    (total.bash_tokens as f64 - total.nushell_tokens as f64)
                        / total.bash_tokens as f64
                        * 100.0
                } else {
                    0.0
                };
                println!(
                    "TOTAL,{},{},{},{},{:.1},{:.1},{},{}",
                    total.total_commands,
                    total.bash_tokens,
                    total.rtk_tokens,
                    total.nushell_tokens,
                    rtk_pct_t,
                    nu_pct_t,
                    total.nushell_matched,
                    total.nushell_unmatched,
                );
                // Dev sub-breakdown CSV
                if !analysis.dev_sub.is_empty() {
                    println!();
                    println!("Dev_Subcategory,Commands,Bash_Tokens,RTK_Tokens,Nushell_Tokens");
                    for sub in &analysis.dev_sub {
                        println!(
                            "{},{},{},{},{}",
                            sub.label,
                            sub.commands,
                            sub.bash_tokens,
                            sub.rtk_tokens,
                            sub.nushell_tokens,
                        );
                    }
                }
                return Ok(());
            }

            // Table output
            println!();
            println!("PRECC Nushell What-If Analysis");
            println!("==============================");
            println!("Historical sessions: {}", analysis.session_count);
            println!("Total Bash commands: {}", total.total_commands);
            println!();

            println!(
                "{:<16} {:>8} {:>10} {:>10} {:>10} {:>6} {:>6}",
                "Category", "Commands", "Bash Tok", "RTK Tok", "Nu Tok", "RTK %", "Nu %"
            );
            println!("{}", "-".repeat(72));

            for cat in nushell::UsageCategory::all() {
                let r = analysis.by_category.get(cat).cloned().unwrap_or_default();
                if r.total_commands == 0 {
                    continue;
                }
                let rtk_pct = if r.bash_tokens > 0 {
                    ((r.bash_tokens as f64 - r.rtk_tokens as f64) / r.bash_tokens as f64 * 100.0)
                        as i32
                } else {
                    0
                };
                let nu_pct = if r.bash_tokens > 0 {
                    ((r.bash_tokens as f64 - r.nushell_tokens as f64) / r.bash_tokens as f64
                        * 100.0) as i32
                } else {
                    0
                };
                println!(
                    "{:<16} {:>8} {:>10} {:>10} {:>10} {:>5}% {:>5}%",
                    cat.name(),
                    r.total_commands,
                    r.bash_tokens,
                    r.rtk_tokens,
                    r.nushell_tokens,
                    rtk_pct,
                    nu_pct,
                );
            }

            println!("{}", "-".repeat(72));
            let rtk_pct_t = if total.bash_tokens > 0 {
                ((total.bash_tokens as f64 - total.rtk_tokens as f64) / total.bash_tokens as f64
                    * 100.0) as i32
            } else {
                0
            };
            let nu_pct_t = if total.bash_tokens > 0 {
                ((total.bash_tokens as f64 - total.nushell_tokens as f64)
                    / total.bash_tokens as f64
                    * 100.0) as i32
            } else {
                0
            };
            println!(
                "{:<16} {:>8} {:>10} {:>10} {:>10} {:>5}% {:>5}%",
                "TOTAL",
                total.total_commands,
                total.bash_tokens,
                total.rtk_tokens,
                total.nushell_tokens,
                rtk_pct_t,
                nu_pct_t,
            );

            println!();
            println!(
                "Nu coverage: {} matched / {} unmatched ({:.0}% of commands)",
                total.nushell_matched,
                total.nushell_unmatched,
                if total.total_commands > 0 {
                    total.nushell_matched as f64 / total.total_commands as f64 * 100.0
                } else {
                    0.0
                }
            );

            // Dev sub-breakdown
            if !analysis.dev_sub.is_empty() {
                println!();
                println!("Software Dev Breakdown:");
                for sub in &analysis.dev_sub {
                    if sub.commands == 0 {
                        continue;
                    }
                    let rtk_pct = if sub.bash_tokens > 0 {
                        ((sub.bash_tokens as f64 - sub.rtk_tokens as f64) / sub.bash_tokens as f64
                            * 100.0) as i32
                    } else {
                        0
                    };
                    let nu_pct = if sub.bash_tokens > 0 {
                        ((sub.bash_tokens as f64 - sub.nushell_tokens as f64)
                            / sub.bash_tokens as f64
                            * 100.0) as i32
                    } else {
                        0
                    };
                    println!(
                        "  {:<24} {:>5} cmds  RTK:{:>4}%  Nu:{:>4}%",
                        sub.label, sub.commands, rtk_pct, nu_pct,
                    );
                }
            }

            Ok(())
        }
    }
}

// =============================================================================
// Update
// =============================================================================

/// Self-update PRECC binaries to the latest (or specified) GitHub release.
/// Fire an anonymous update-check ping in the background.
///
/// Sends a single GET request to the configured ping URL with these
/// query parameters (no PII, no cookies):
///   - `v`  — current version (e.g. "0.1.4")
///   - `os` — operating system slug ("linux" / "macos" / "windows")
///   - `arch` — CPU architecture ("x86_64" / "aarch64")
///
/// The request is fire-and-forget: spawned as a background process so it
/// never delays the update flow. Any error is silently ignored.
///
/// Opt-out: set `PRECC_NO_TELEMETRY=1` in the environment.
/// Ping URL: compile-time `PRECC_PING_URL` env var, falling back to
/// the GoatCounter endpoint `https://precc.goatcounter.com/count`.
fn fire_update_ping(current_version: &str) {
    if std::env::var("PRECC_NO_TELEMETRY").is_ok() {
        return;
    }

    const PING_URL: &str = match option_env!("PRECC_PING_URL") {
        Some(u) => u,
        None => "https://precc.goatcounter.com/count",
    };

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    // GoatCounter /count accepts: p (path), t (title), q (query string carries extra data)
    // We encode version+os+arch in the path so each variant is a separate counter.
    let path = format!("/update/{}/{}/{}", current_version, os, arch);
    let url = format!("{}?p={}&t=update-ping", PING_URL, urlencoding_simple(&path));

    // Spawn detached curl — no wait, no stdout/stderr capture.
    let _ = std::process::Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "5",
            "--silent",
            "--output",
            "/dev/null",
            &url,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn(); // spawn() not wait() — fire and forget
}

/// Minimal percent-encoding for path segments (encodes space and non-ASCII only).
fn urlencoding_simple(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '/' => {
                vec![c]
            }
            c => format!("%{:02X}", c as u32).chars().collect(),
        })
        .collect()
}

fn cmd_update(force: bool, requested_version: Option<String>, auto: bool) -> Result<()> {
    use std::io::Write;
    use std::process::Command;

    const REPO: &str = "yijunyu/precc-cc";
    const CURRENT: &str = env!("CARGO_PKG_VERSION");

    // Handle --auto flag: persist auto-update preference and return
    if auto {
        update_check::set_auto_update(true)?;
        println!("Auto-update enabled. The miner daemon will silently upgrade PRECC when new versions are released.");
        println!("To disable: edit ~/.config/precc/config.toml and set [update] auto = false");
        return Ok(());
    }

    // ── 0. Fire anonymous update-check ping (non-blocking) ───────────────────
    fire_update_ping(CURRENT);

    // ── 1. Detect platform triple ────────────────────────────────────────────
    let target_triple = update_target_triple().ok_or_else(|| {
        anyhow::anyhow!(
            "Unsupported platform {}/{}. \
             Download manually from https://github.com/{REPO}/releases",
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    })?;

    // ── 2. Fetch release metadata from GitHub API ────────────────────────────
    let api_url = match &requested_version {
        Some(v) => {
            let tag = update_normalise_version(v);
            format!("https://api.github.com/repos/{REPO}/releases/tags/{tag}")
        }
        None => {
            println!("Checking latest release...");
            format!("https://api.github.com/repos/{REPO}/releases/latest")
        }
    };

    let api_out = Command::new("curl")
        .args(["-fsSL", &api_url])
        .output()
        .context("curl not found — install curl to use `precc update`")?;
    if !api_out.status.success() {
        bail!("Failed to reach GitHub API ({})", api_url);
    }

    let release: serde_json::Value =
        serde_json::from_slice(&api_out.stdout).context("parsing GitHub API response")?;

    let tag_name = release["tag_name"]
        .as_str()
        .context("missing tag_name in GitHub API response")?
        .to_string();

    // ── 3. Compare with running version ─────────────────────────────────────
    let tag_bare = tag_name.trim_start_matches('v');
    if !force && tag_bare == CURRENT {
        println!("Already on the latest version (v{CURRENT}). Use --force to reinstall.");
        return Ok(());
    }
    if force && tag_bare == CURRENT {
        println!("Reinstalling v{CURRENT} (--force)...");
    } else {
        println!("Updating v{CURRENT} → {tag_name}...");
    }

    // ── 4. Pick the right asset for this platform ────────────────────────────
    // The asset name may embed a different version string than the tag (deploy
    // script sometimes mismatches), so we match by triple suffix rather than
    // constructing the name from the tag.
    let assets = release["assets"]
        .as_array()
        .context("no assets in release")?;

    let (asset_name, download_url) =
        update_pick_asset(assets, target_triple).with_context(|| {
            let names: Vec<&str> = assets.iter().filter_map(|a| a["name"].as_str()).collect();
            format!(
                "No asset found for {target_triple} in release {tag_name}.\nAvailable: {names:?}"
            )
        })?;

    // ── 5. Locate install directory ───────────────────────────────────────────
    let current_exe = std::env::current_exe().context("cannot determine current binary path")?;
    let bin_dir = current_exe
        .parent()
        .context("binary has no parent directory")?;

    // ── 6. Download archive ───────────────────────────────────────────────────
    let tmp_dir = tempfile::tempdir().context("creating temp dir")?;
    let archive_path = tmp_dir.path().join(asset_name);

    println!("Downloading {download_url}...");
    let status = Command::new("curl")
        .args([
            "-fsSL",
            "--progress-bar",
            "-o",
            archive_path.to_str().unwrap(),
            download_url,
        ])
        .status()
        .context("curl download failed")?;
    if !status.success() {
        bail!(
            "Download failed. Check https://github.com/{REPO}/releases/tag/{tag_name} \
             for available assets."
        );
    }

    // ── 7. Extract and discover inner directory name ──────────────────────────
    println!("Extracting...");
    let status = Command::new("tar")
        .args([
            "-xzf",
            archive_path.to_str().unwrap(),
            "-C",
            tmp_dir.path().to_str().unwrap(),
        ])
        .status()
        .context("tar extraction failed")?;
    if !status.success() {
        bail!("Extraction failed");
    }

    // The inner dir is the asset name minus ".tar.gz"
    let inner_dir = asset_name.trim_end_matches(".tar.gz");
    let extracted = tmp_dir.path().join(inner_dir);

    // ── 8. Replace binaries ───────────────────────────────────────────────────
    for bin in ["precc", "precc-hook", "precc-miner"] {
        let src = extracted.join(bin);
        let dst = bin_dir.join(bin);
        if !src.exists() {
            continue;
        }
        // Rename current binary to .old first (Linux allows replacing a running binary this way)
        let old = bin_dir.join(format!("{bin}.old"));
        if dst.exists() {
            std::fs::rename(&dst, &old).with_context(|| {
                format!("cannot move {dst:?} — try running with sudo or check permissions")
            })?;
        }
        std::fs::copy(&src, &dst).with_context(|| format!("cannot write {dst:?}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dst, std::fs::Permissions::from_mode(0o755))?;
        }
        if old.exists() {
            let _ = std::fs::remove_file(&old);
        }
        println!("  Updated {}", dst.display());
    }

    // ── 9. Verify ─────────────────────────────────────────────────────────────
    println!();
    print!("Verifying... ");
    std::io::stdout().flush().ok();
    if let Ok(out) = Command::new(&current_exe).arg("--version").output() {
        print!("{}", String::from_utf8_lossy(&out.stdout).trim());
    }
    println!();
    println!("PRECC updated to {tag_name}. Run `precc init` if schemas changed.");

    // Clear the update-available marker so the banner stops showing
    if let Ok(data_dir) = db::data_dir() {
        update_check::clear_update_marker(&data_dir);
    }

    // Ask about auto-updates if not already configured
    if !update_check::has_auto_update_consent() {
        println!();
        let _ = update_check::prompt_auto_update_consent();
    }

    Ok(())
}

// =============================================================================
// Update banner
// =============================================================================

/// Show a yellow update-available banner on stderr if a newer version exists.
fn show_update_banner() {
    if let Ok(data_dir) = db::data_dir() {
        if let Some(ver) = update_check::read_update_available(&data_dir) {
            eprintln!(
                "\x1b[33m[precc] Update available: v{ver} — run `precc update` to upgrade\x1b[0m"
            );
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}

// =============================================================================
// Update helpers (testable, extracted from cmd_update)
// =============================================================================

/// Parse the platform target triple for the current OS/arch.
fn update_target_triple() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-gnu"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        _ => None,
    }
}

/// Given a slice of GitHub API asset objects, find the `.tar.gz` asset
/// whose name contains `triple`. Returns `(asset_name, download_url)`.
fn update_pick_asset<'a>(
    assets: &'a [serde_json::Value],
    triple: &str,
) -> Option<(&'a str, &'a str)> {
    assets.iter().find_map(|a| {
        let name = a["name"].as_str()?;
        let url = a["browser_download_url"].as_str()?;
        if name.contains(triple) && name.ends_with(".tar.gz") {
            Some((name, url))
        } else {
            None
        }
    })
}

/// Normalise a version string to always have a leading `v`.
fn update_normalise_version(v: &str) -> String {
    if v.starts_with('v') {
        v.to_string()
    } else {
        format!("v{v}")
    }
}

// =============================================================================
// precc cache-hint  +  write_claude_md helpers
// =============================================================================

const PRECC_START: &str = "<!-- PRECC:start -->";
const PRECC_END: &str = "<!-- PRECC:end -->";

fn strip_precc_block_owned(content: &str) -> String {
    if let (Some(s), Some(e)) = (content.find(PRECC_START), content.rfind(PRECC_END)) {
        let end_pos = e + PRECC_END.len();
        let prefix = content[..s].trim_end();
        let suffix = content[end_pos..].trim_start();
        if suffix.is_empty() {
            prefix.to_string()
        } else {
            format!("{}\n\n{}", prefix, suffix)
        }
    } else {
        content.to_string()
    }
}

fn generate_precc_block() -> String {
    format!(
        "{start}\n\
         <!-- generated by precc init v{ver} — https://github.com/yijunyu/precc-cc -->\n\
         # PRECC hook\n\n\
         Routes every Bash command through `precc-hook` (< 3 ms) for error correction and\n\
         token savings. Run `precc skills list` for the full active-skill list.\n\n\
         ## Skill trigger index\n\n\
         | Skill | Triggers on | Action |\n\
         |-------|-------------|--------|\n\
         | `cargo-wrong-dir` | `cargo build/test/clippy/…` outside Rust project | prepend `cd` |\n\
         | `git-wrong-dir` | `git *` outside a repo | prepend `cd` |\n\
         | `go-wrong-dir` | `go build/test/…` outside Go module | prepend `cd` |\n\
         | `make-wrong-dir` | `make` without Makefile in cwd | prepend `cd` |\n\
         | `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside Node project | prepend `cd` |\n\
         | `python-wrong-dir` | `python/pytest/pip` outside Python project | prepend `cd` |\n\
         | `jj-translate` | `git *` in a jj-colocated repo | rewrite to `jj` |\n\
         | `asciinema-gif` | `asciinema rec` | rewrite to `precc gif` |\n\n\
         ## Hook invocation pattern\n\n\
         ```bash\n\
         echo '{{\"tool_input\":{{\"command\":\"CMD\"}}}}' | precc-hook\n\
         # use .hookSpecificOutput.updatedInput.command if non-empty, else original\n\
         ```\n\
         {end}\n",
        start = PRECC_START,
        end = PRECC_END,
        ver = env!("CARGO_PKG_VERSION"),
    )
}

fn write_claude_md() -> Result<()> {
    let home = std::env::var("HOME").context("$HOME not set")?;
    let claude_dir = std::path::PathBuf::from(home).join(".claude");
    std::fs::create_dir_all(&claude_dir)?;
    let path = claude_dir.join("CLAUDE.md");

    let existing = std::fs::read_to_string(&path).unwrap_or_default();
    let base = strip_precc_block_owned(&existing);

    if !existing.is_empty() && !base.trim().is_empty() && !existing.contains(PRECC_START) {
        std::fs::copy(&path, claude_dir.join("CLAUDE.md.precc-bak"))
            .context("failed to back up CLAUDE.md")?;
        println!("  Backed up existing ~/.claude/CLAUDE.md to CLAUDE.md.precc-bak");
    }

    let precc_block = generate_precc_block();
    let new_content = if base.trim().is_empty() {
        precc_block
    } else {
        format!("{}\n\n{}", base.trim_end(), precc_block)
    };

    std::fs::write(&path, &new_content)?;
    println!("  ~/.claude/CLAUDE.md — compact PRECC trigger index written");
    Ok(())
}

fn cmd_cache_hint() -> Result<()> {
    let block = generate_precc_block();
    let hint = serde_json::json!({
        "systemPrompt": [{
            "type": "text",
            "text": block,
            "cache_control": {"type": "ephemeral"}
        }]
    });
    println!("# Anthropic API prompt-caching hint for PRECC");
    println!("# Merge the following into ~/.claude/settings.json:");
    println!("{}", serde_json::to_string_pretty(&hint)?);
    Ok(())
}

#[cfg(test)]
mod cache_hint_tests {
    use super::*;

    #[test]
    fn test_strip_precc_block_removes_block() {
        let content = "line1\n\n<!-- PRECC:start -->\nsome content\n<!-- PRECC:end -->\n";
        assert_eq!(strip_precc_block_owned(content).trim(), "line1");
    }

    #[test]
    fn test_strip_precc_block_no_markers() {
        let content = "existing content\n";
        assert_eq!(strip_precc_block_owned(content), content);
    }

    #[test]
    fn test_generate_precc_block_contains_markers() {
        let block = generate_precc_block();
        assert!(block.contains("<!-- PRECC:start -->"));
        assert!(block.contains("<!-- PRECC:end -->"));
        assert!(block.contains("cargo-wrong-dir"));
        assert!(block.contains("precc-hook"));
    }
}

#[cfg(test)]
mod update_tests {
    use super::*;
    use serde_json::json;

    // ── update_normalise_version ─────────────────────────────────────────────

    #[test]
    fn normalise_bare_semver() {
        assert_eq!(update_normalise_version("0.2.0"), "v0.2.0");
    }

    #[test]
    fn normalise_already_prefixed() {
        assert_eq!(update_normalise_version("v0.2.0"), "v0.2.0");
    }

    #[test]
    fn normalise_empty_string() {
        assert_eq!(update_normalise_version(""), "v");
    }

    // ── update_target_triple ─────────────────────────────────────────────────

    #[test]
    fn target_triple_is_known() {
        // On any CI/test host we support, must return Some.
        assert!(
            update_target_triple().is_some(),
            "unknown platform: {}/{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        );
    }

    #[test]
    fn target_triple_contains_arch() {
        let triple = update_target_triple().unwrap();
        let arch = std::env::consts::ARCH;
        // "x86_64" or "aarch64" must appear verbatim in the triple
        assert!(
            triple.contains(arch),
            "triple {triple:?} does not contain arch {arch:?}"
        );
    }

    // ── update_pick_asset ────────────────────────────────────────────────────

    fn make_assets(names: &[&str]) -> Vec<serde_json::Value> {
        names
            .iter()
            .map(|n| {
                json!({
                    "name": n,
                    "browser_download_url": format!("https://github.com/example/releases/download/v1/{n}")
                })
            })
            .collect()
    }

    #[test]
    fn pick_asset_exact_triple() {
        let assets = make_assets(&[
            "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz",
            "precc-v0.1.1-aarch64-apple-darwin.tar.gz",
        ]);
        let (name, url) = update_pick_asset(&assets, "x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(name, "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz");
        assert!(url.ends_with(name));
    }

    #[test]
    fn pick_asset_tag_version_differs_from_asset_version() {
        // Deploy script may tag v0.1.0 but assets are named v0.1.1 — must still match.
        let assets = make_assets(&[
            "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz",
            "precc-v0.1.1-aarch64-unknown-linux-gnu.tar.gz",
        ]);
        let (name, _) = update_pick_asset(&assets, "aarch64-unknown-linux-gnu").unwrap();
        assert_eq!(name, "precc-v0.1.1-aarch64-unknown-linux-gnu.tar.gz");
    }

    #[test]
    fn pick_asset_ignores_non_targz() {
        let assets = make_assets(&[
            "precc-v0.1.0-x86_64-unknown-linux-gnu.zip", // wrong extension
            "precc-v0.1.0-x86_64-unknown-linux-gnu.tar.gz",
        ]);
        let (name, _) = update_pick_asset(&assets, "x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(name, "precc-v0.1.0-x86_64-unknown-linux-gnu.tar.gz");
    }

    #[test]
    fn pick_asset_returns_none_when_no_match() {
        let assets = make_assets(&["precc-v0.1.0-x86_64-apple-darwin.tar.gz"]);
        assert!(update_pick_asset(&assets, "x86_64-unknown-linux-gnu").is_none());
    }

    #[test]
    fn pick_asset_empty_list() {
        assert!(update_pick_asset(&[], "x86_64-unknown-linux-gnu").is_none());
    }

    #[test]
    fn pick_asset_prefers_first_match() {
        // If somehow two assets match (shouldn't happen but be deterministic).
        let assets = make_assets(&[
            "precc-v0.1.0-x86_64-unknown-linux-gnu.tar.gz",
            "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz",
        ]);
        let (name, _) = update_pick_asset(&assets, "x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(name, "precc-v0.1.0-x86_64-unknown-linux-gnu.tar.gz");
    }

    // ── inner dir extraction (asset name → dir) ──────────────────────────────

    #[test]
    fn inner_dir_strips_tar_gz() {
        let asset_name = "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz";
        assert_eq!(
            asset_name.trim_end_matches(".tar.gz"),
            "precc-v0.1.1-x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn inner_dir_no_double_strip() {
        // Should not strip ".gz" and ".tar" separately
        let asset_name = "precc-v0.1.1-x86_64-unknown-linux-gnu.tar.gz";
        let inner = asset_name.trim_end_matches(".tar.gz");
        assert!(!inner.ends_with(".tar"));
    }

    // ── version tag parsing from GitHub API (serde_json) ────────────────────

    #[test]
    fn parse_tag_name_from_release_json() {
        let release = json!({ "tag_name": "v0.2.0", "assets": [] });
        let tag = release["tag_name"].as_str().unwrap();
        assert_eq!(tag, "v0.2.0");
    }

    #[test]
    fn parse_tag_name_strips_v_for_comparison() {
        let release = json!({ "tag_name": "v0.2.0" });
        let tag = release["tag_name"].as_str().unwrap();
        assert_eq!(tag.trim_start_matches('v'), "0.2.0");
    }

    #[test]
    fn current_version_is_valid_semver_shape() {
        let v = env!("CARGO_PKG_VERSION");
        let parts: Vec<&str> = v.split('.').collect();
        assert_eq!(parts.len(), 3, "expected major.minor.patch, got {v:?}");
        for p in parts {
            assert!(p.parse::<u32>().is_ok(), "non-numeric part {p:?} in {v:?}");
        }
    }

    // ── urlencoding_simple ───────────────────────────────────────────────────

    #[test]
    fn urlencoding_passthrough_safe_chars() {
        assert_eq!(
            urlencoding_simple("/update/0.1.4/linux/x86_64"),
            "/update/0.1.4/linux/x86_64"
        );
    }

    #[test]
    fn urlencoding_encodes_space() {
        assert_eq!(urlencoding_simple("hello world"), "hello%20world");
    }

    // ── fire_update_ping ────────────────────────────────────────────────────

    #[test]
    fn ping_suppressed_by_env() {
        std::env::set_var("PRECC_NO_TELEMETRY", "1");
        // Should return immediately without spawning anything (no panic)
        fire_update_ping("0.1.4");
        std::env::remove_var("PRECC_NO_TELEMETRY");
    }
}

#[cfg(test)]
mod report_tests {
    use super::*;

    #[test]
    fn generate_report_text_contains_header() {
        // generate_report_text reads from DB files which may not exist in test,
        // but it should still produce a valid header without panicking.
        let text = generate_report_text().unwrap();
        assert!(text.contains("PRECC Monthly Usage Report"));
        assert!(text.contains("Edition:"));
    }

    #[test]
    fn generate_report_text_shows_edition() {
        let text = generate_report_text().unwrap();
        // Should contain one of the known tier names
        let has_tier = text.contains("Free")
            || text.contains("Pro")
            || text.contains("Team")
            || text.contains("Enterprise");
        assert!(has_tier, "report should contain a tier name");
    }
}
