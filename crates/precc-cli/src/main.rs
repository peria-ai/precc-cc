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
use precc_core::{db, gdb, license, metrics, mining, rtk, skills};
#[allow(unused_imports)] // needed for writeln! on impl Write params
use std::io::Write;

mod gif;
mod mail;

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
    Report,
    /// Estimate token savings from PRECC over RTK alone
    Savings,
    /// Setup hook and databases
    Init,
    /// Convert a bash script to an animated GIF at a target duration
    Gif {
        /// Bash script to animate
        script: String,
        /// Target GIF length, e.g. "30s" or "2m"
        length: String,
        /// Expected user inputs (quoted strings, piped to script stdin)
        inputs: Vec<String>,
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
    /// Update PRECC binaries to the latest release
    Update {
        /// Force update even if already on the latest version
        #[arg(long)]
        force: bool,
        /// Install a specific version (e.g. v0.2.0) instead of latest
        #[arg(long)]
        version: Option<String>,
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
enum LicenseAction {
    /// Activate a license key
    Activate {
        /// License key (format: PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX)
        key: String,
    },
    /// Show current license status
    Status,
    /// Deactivate (remove) the stored license key
    Deactivate,
    /// Show this machine's fingerprint (for generating machine-bound keys)
    Fingerprint,
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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
        Some(Commands::Report) => cmd_report(),
        Some(Commands::Savings) => cmd_savings(),
        Some(Commands::Gif {
            script,
            length,
            inputs,
        }) => gif::cmd_gif(script, length, inputs),
        Some(Commands::License { action }) => cmd_license(action),
        Some(Commands::Mail { action }) => cmd_mail(action),
        Some(Commands::Update { force, version }) => cmd_update(force, version),
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
    println!("Init complete.");

    Ok(())
}

// =============================================================================
// precc ingest
// =============================================================================

fn cmd_ingest(file: Option<String>, all: bool, force: bool) -> Result<()> {
    let data_dir = db::data_dir()?;
    let conn = db::open_history(&data_dir)?;

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

fn cmd_savings() -> Result<()> {
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

    // ---- Grand total ---------------------------------------------------
    let grand_total = rtk_tokens + precc_over_rtk;
    let precc_pct = if grand_total > 0.0 {
        precc_over_rtk / grand_total * 100.0
    } else {
        0.0
    };

    println!("Summary");
    println!("-------");
    println!("  RTK baseline          : {:>8.0} tokens", rtk_tokens);
    println!("  PRECC additional gain : {:>8.0} tokens", precc_over_rtk);
    println!("  Grand total saved     : {:>8.0} tokens", grand_total);
    if grand_total > 0.0 {
        println!("  PRECC share of savings: {:>7.1}%", precc_pct);
    }
    println!();
    println!("Note: figures are estimates based on conservative medians per event.");
    println!(
        "      RTK ~{:.0} tok/rewrite (weighted avg), CD-miss ~{:.0} tok, skill ~{:.0} tok, pattern ~{:.0} tok.",
        model.rtk_per_rewrite_avg,
        model.precc_per_cd_prepend,
        model.precc_per_skill_activation,
        model.precc_per_mined_occurrence,
    );

    Ok(())
}

// =============================================================================
// License
// =============================================================================

fn cmd_license(action: LicenseAction) -> Result<()> {
    match action {
        LicenseAction::Activate { key } => {
            let lic = license::activate(&key)?;
            println!("License activated successfully.");
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
    }
}

// =============================================================================
// Mail
// =============================================================================

fn cmd_mail(action: MailAction) -> Result<()> {
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
// Update
// =============================================================================

/// Self-update PRECC binaries to the latest (or specified) GitHub release.
fn cmd_update(force: bool, requested_version: Option<String>) -> Result<()> {
    use std::io::Write;
    use std::process::Command;

    const REPO: &str = "yijunyu/precc-cc";
    const CURRENT: &str = env!("CARGO_PKG_VERSION");

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
    Ok(())
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
}
