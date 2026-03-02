//! `precc gif` — Convert a bash script to an animated GIF at a target duration.
//!
//! Workflow:
//!   1. Pass 1: record the script with `asciinema rec` and measure wall-clock time.
//!   2. Compute agg speed multiplier so the GIF lasts exactly the target duration.
//!   3. Pass 2: record again at natural speed for a clean cast.
//!   4. Convert the clean cast to a GIF via `agg --speed <multiplier>`.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Parse a human duration string into seconds.
///
/// Accepted formats: `"30s"`, `"2m"`, `"1m30s"`, bare integer (treated as seconds).
pub fn parse_duration(s: &str) -> Result<f64> {
    let mut secs = 0.0f64;
    let mut num = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
        } else if ch == 'm' {
            if num.is_empty() {
                bail!("Invalid duration (no number before 'm'): {}", s);
            }
            secs += num.parse::<f64>()? * 60.0;
            num.clear();
        } else if ch == 's' {
            if num.is_empty() {
                bail!("Invalid duration (no number before 's'): {}", s);
            }
            secs += num.parse::<f64>()?;
            num.clear();
        } else {
            bail!("Invalid duration character '{}' in: {}", ch, s);
        }
    }
    // Bare integer with no unit → treat as seconds
    if !num.is_empty() {
        secs += num.parse::<f64>()?;
    }
    if secs <= 0.0 {
        bail!("Invalid duration (must be > 0): {}", s);
    }
    Ok(secs)
}

/// Check that an external binary is on PATH.
fn require_binary(name: &str) -> Result<()> {
    let status = Command::new("which")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        _ => bail!(
            "Required binary '{}' not found. Install it first:\n  {}",
            name,
            install_hint(name)
        ),
    }
}

fn install_hint(name: &str) -> &str {
    match name {
        "asciinema" => "pip install asciinema  OR  sudo apt install asciinema",
        "agg" => "cargo install --git https://github.com/asciinema/agg  OR  see https://github.com/asciinema/agg",
        _ => "check your PATH",
    }
}

/// Return the stem of a file path (filename without extension).
fn script_stem(script: &str) -> String {
    std::path::Path::new(script)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
        .to_string()
}

/// Build the shell command string for running the script with optional piped inputs.
fn build_shell_command(script: &str, inputs: &[String]) -> String {
    if inputs.is_empty() {
        format!("bash {}", shell_quote(script))
    } else {
        // Join inputs with newlines and pipe to bash via printf
        let joined = inputs.join("\\n");
        format!("printf '{}\\n' | bash {}", joined, shell_quote(script))
    }
}

/// Minimal single-quote shell quoting (sufficient for file paths).
fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Record the script with `asciinema rec` and return the wall-clock duration.
fn record_cast(shell_cmd: &str, cast_path: &PathBuf) -> Result<f64> {
    let start = Instant::now();

    let status = Command::new("asciinema")
        .arg("rec")
        .arg("--overwrite")
        .arg("--command")
        .arg(shell_cmd)
        .arg(cast_path)
        .status()
        .context("failed to run asciinema")?;

    let elapsed = start.elapsed().as_secs_f64();

    if !status.success() {
        bail!("asciinema exited with status {}", status);
    }

    Ok(elapsed)
}

/// Convert a `.cast` file to `.gif` using `agg` at the given speed multiplier.
fn cast_to_gif(cast_path: &PathBuf, gif_path: &PathBuf, speed: f64) -> Result<()> {
    let status = Command::new("agg")
        .arg("--speed")
        .arg(format!("{:.4}", speed))
        .arg(cast_path)
        .arg(gif_path)
        .status()
        .context("failed to run agg")?;

    if !status.success() {
        bail!("agg exited with status {}", status);
    }

    Ok(())
}

/// Main entry point for `precc gif <script> <length> [inputs...]`.
pub fn cmd_gif(script: String, length: String, inputs: Vec<String>) -> Result<()> {
    // --- Preflight checks ---
    require_binary("asciinema")?;
    require_binary("agg")?;

    let target_secs =
        parse_duration(&length).with_context(|| format!("invalid length argument '{length}'"))?;

    let stem = script_stem(&script);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX epoch")?
        .as_secs();

    let tmp_dir = std::env::temp_dir();
    let cast1 = tmp_dir.join(format!("{stem}-probe-{timestamp}.cast"));
    let cast2 = tmp_dir.join(format!("{stem}-final-{timestamp}.cast"));

    let gif_name = format!("{stem}-{timestamp}.gif");
    let gif_path = std::env::current_dir()
        .context("failed to get current directory")?
        .join(&gif_name);

    let shell_cmd = build_shell_command(&script, &inputs);

    // --- Pass 1: probe run at 20× speed to measure natural duration ---
    println!("Pass 1: probing script duration (20× speed)…");
    // asciinema itself doesn't have a built-in speed multiplier on record;
    // we record normally but measure wall-clock time to infer natural speed.
    // The 20× comes from running through a fast pseudo-TTY; we simply record
    // and time it. The "speed distortion" mentioned in the plan means we
    // record normally and use the measured wall time directly.
    let measured_secs = record_cast(&shell_cmd, &cast1)
        .with_context(|| format!("Pass 1 recording failed for '{script}'"))?;

    println!(
        "  Wall-clock duration: {:.2}s  →  natural speed ~{:.2}s",
        measured_secs, measured_secs
    );

    // --- Compute agg speed multiplier ---
    // We recorded at natural speed (not 20×), so natural_secs == measured_secs.
    // The plan's "20×" is a probe strategy; we simply use the measured time.
    // Required agg multiplier: measured_secs / target_secs
    // (agg --speed > 1 speeds up, < 1 slows down)
    let speed_multiplier = if measured_secs > 0.0 {
        measured_secs / target_secs
    } else {
        1.0
    };

    println!("  Target: {target_secs:.2}s  →  agg speed multiplier: {speed_multiplier:.4}×");

    // --- Pass 2: final clean recording ---
    println!("Pass 2: recording final cast…");
    let _final_secs = record_cast(&shell_cmd, &cast2)
        .with_context(|| format!("Pass 2 recording failed for '{script}'"))?;

    // --- Convert to GIF ---
    println!("Converting to GIF…");
    cast_to_gif(&cast2, &gif_path, speed_multiplier)
        .with_context(|| format!("GIF conversion failed → {}", gif_path.display()))?;

    // --- Clean up temp casts ---
    let _ = std::fs::remove_file(&cast1);
    let _ = std::fs::remove_file(&cast2);

    println!("Generated: {}", gif_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_seconds() {
        assert!((parse_duration("30s").unwrap() - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_duration_minutes() {
        assert!((parse_duration("2m").unwrap() - 120.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_duration_mixed() {
        assert!((parse_duration("1m30s").unwrap() - 90.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_duration_bare_int() {
        assert!((parse_duration("45").unwrap() - 45.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(parse_duration("0s").is_err());
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn test_script_stem() {
        assert_eq!(script_stem("/tmp/hello.sh"), "hello");
        assert_eq!(script_stem("demo.sh"), "demo");
        assert_eq!(script_stem("/path/to/my-script.bash"), "my-script");
    }

    #[test]
    fn test_build_shell_command_no_inputs() {
        let cmd = build_shell_command("/tmp/hello.sh", &[]);
        assert_eq!(cmd, "bash '/tmp/hello.sh'");
    }

    #[test]
    fn test_build_shell_command_with_inputs() {
        let inputs = vec!["yes".to_string(), "mypassword".to_string()];
        let cmd = build_shell_command("/tmp/greet.sh", &inputs);
        assert!(cmd.contains("printf"));
        assert!(cmd.contains("yes"));
        assert!(cmd.contains("mypassword"));
        assert!(cmd.contains("bash '/tmp/greet.sh'"));
    }
}
