//! Build script for precc-core.
//!
//! Reads PRECC_LICENSE_SECRET from (in priority order):
//! 1. The PRECC_LICENSE_SECRET environment variable (existing behavior)
//! 2. ~/.config/precc/build-secret (local file, never committed)
//!
//! This means you set the secret once in a file and every `cargo build`
//! picks it up automatically — no need to prefix every build command.

fn main() {
    // Don't re-run on every build — only when the secret file changes
    let secret_file = dirs();
    if let Some(ref path) = secret_file {
        println!("cargo:rerun-if-changed={}", path);
    }
    println!("cargo:rerun-if-env-changed=PRECC_LICENSE_SECRET");
    println!("cargo:rerun-if-env-changed=PRECC_STRIPE_SECRET_KEY");

    // Read build-secret file (contains key=value lines or a bare license secret)
    let file_contents = secret_file
        .as_ref()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .unwrap_or_default();

    // PRECC_LICENSE_SECRET
    if std::env::var("PRECC_LICENSE_SECRET").is_err() {
        // Try key=value format first, then fall back to bare value (first line)
        let val = extract_key(&file_contents, "PRECC_LICENSE_SECRET")
            .or_else(|| file_contents.lines().next().map(|l| l.trim().to_string()))
            .unwrap_or_default();
        if !val.is_empty() {
            println!("cargo:rustc-env=PRECC_LICENSE_SECRET={}", val);
        }
    }

    // PRECC_STRIPE_SECRET_KEY
    if std::env::var("PRECC_STRIPE_SECRET_KEY").is_err() {
        if let Some(val) = extract_key(&file_contents, "PRECC_STRIPE_SECRET_KEY") {
            println!("cargo:rustc-env=PRECC_STRIPE_SECRET_KEY={}", val);
        }
    }
}

/// Extract a value from `KEY=VALUE` lines in a config string.
fn extract_key(contents: &str, key: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix(key) {
            if let Some(val) = rest.strip_prefix('=') {
                let val = val.trim();
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
    }
    None
}

fn dirs() -> Option<String> {
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.config/precc/build-secret", h))
}
