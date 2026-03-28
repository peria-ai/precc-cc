//! License key validation for PRECC.
//!
//! # Key format
//! `PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX`
//! where each `X` block is uppercase hex. The key encodes:
//!   - 4 bytes: machine fingerprint (first 4 bytes of SHA-256(hostname+username))
//!   - 4 bytes: expiry as days-since-epoch (big-endian u32), 0 = no expiry
//!   - 4 bytes: edition flags (bit 0 = pro, bit 1 = team, bit 2 = enterprise)
//!   - 4 bytes: HMAC-SHA256 truncated MAC (first 4 bytes)
//!
//! The HMAC key is the build-time secret `PRECC_LICENSE_SECRET` (env at build time),
//! falling back to a compiled-in default for open builds.
//!
//! # Storage
//! Active key stored in `~/.local/share/precc/license.key` (plain text, one line).

use anyhow::{bail, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::path::PathBuf;
use std::sync::OnceLock;

type HmacSha256 = Hmac<Sha256>;

// Build-time secret injected via PRECC_LICENSE_SECRET env var.
// Falls back to a public default so open/unprotected builds still compile.
const LICENSE_SECRET: &str = match option_env!("PRECC_LICENSE_SECRET") {
    Some(s) => s,
    None => "precc-open-build-secret-v1",
};

/// Parsed license key contents.
#[derive(Debug, Clone)]
pub struct License {
    /// First 4 bytes of SHA-256(hostname+username) — used to bind key to machine.
    pub machine_tag: [u8; 4],
    /// Days since Unix epoch at expiry, or 0 for no expiry.
    pub expiry_days: u32,
    /// Edition flags: bit 0 = pro, bit 1 = team, bit 2 = enterprise.
    pub edition_flags: u32,
    /// Whether this license is machine-bound (machine_tag != [0,0,0,0]).
    pub machine_bound: bool,
}

impl License {
    pub fn is_pro(&self) -> bool {
        self.edition_flags & 1 != 0
    }
    pub fn is_team(&self) -> bool {
        self.edition_flags & 2 != 0
    }
    pub fn is_enterprise(&self) -> bool {
        self.edition_flags & 4 != 0
    }
    pub fn is_expired(&self) -> bool {
        if self.expiry_days == 0 {
            return false;
        }
        let now_days = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            / 86400;
        now_days > self.expiry_days as u64
    }
    pub fn edition_name(&self) -> &'static str {
        if self.is_enterprise() {
            "Enterprise"
        } else if self.is_team() {
            "Team"
        } else if self.is_pro() {
            "Pro"
        } else {
            "Community"
        }
    }
}

/// Subscription tier — determined once per process from the stored license key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    Free,
    Pro,
    Team,
    Enterprise,
}

impl Tier {
    pub fn is_paid(self) -> bool {
        self != Tier::Free
    }

    /// Human-readable name for error messages.
    pub fn name(self) -> &'static str {
        match self {
            Tier::Free => "Free",
            Tier::Pro => "Pro",
            Tier::Team => "Team",
            Tier::Enterprise => "Enterprise",
        }
    }
}

/// Return the active tier for this process.
///
/// Reads `~/.local/share/precc/license.key` exactly once and caches the
/// result in a `OnceLock` so the hook can call this on every command
/// with zero re-parsing cost after the first call.
///
/// Silently falls back to `Tier::Free` on any error (missing file,
/// invalid key, expired key, wrong machine) — the hook must never block.
pub fn tier() -> Tier {
    static TIER: OnceLock<Tier> = OnceLock::new();
    *TIER.get_or_init(|| match load() {
        Ok(Some(lic)) if !lic.is_expired() => {
            if lic.is_enterprise() {
                Tier::Enterprise
            } else if lic.is_team() {
                Tier::Team
            } else if lic.is_pro() {
                Tier::Pro
            } else {
                Tier::Free
            }
        }
        _ => Tier::Free,
    })
}

/// Emit a consistent upgrade prompt to stderr and return an error.
/// Used by gated commands so every gate looks the same.
pub fn require_paid(feature: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "{feature} requires a PRECC Pro (or higher) license.\n\
         Activate:  precc license activate <gumroad-or-stripe-key>\n\
         Buy a key: https://github.com/peria-ai/precc-cc#pricing"
    )
}

/// Parse and validate a license key string.
///
/// Returns `Ok(License)` if the key is structurally valid and the MAC matches.
/// Does NOT check machine binding or expiry — call `validate()` for full check.
pub fn parse(key: &str) -> Result<License> {
    let key = key.trim();
    // Strip optional "PRECC-" prefix
    let hex_part = key.strip_prefix("PRECC-").unwrap_or(key);
    // Remove dashes
    let hex = hex_part.replace('-', "");
    if hex.len() != 32 {
        bail!("Invalid license key length (expected 32 hex chars after stripping dashes)");
    }
    let bytes = hex::decode(&hex).map_err(|_| anyhow::anyhow!("Invalid hex in license key"))?;
    // bytes[0..4]  = machine_tag
    // bytes[4..8]  = expiry_days (big-endian u32)
    // bytes[8..12] = edition_flags (big-endian u32)
    // bytes[12..16] = MAC (first 4 bytes of HMAC-SHA256)
    let machine_tag: [u8; 4] = bytes[0..4].try_into().unwrap();
    let expiry_days = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
    let edition_flags = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    let embedded_mac: [u8; 4] = bytes[12..16].try_into().unwrap();

    // Verify MAC over first 12 bytes
    let mut mac =
        HmacSha256::new_from_slice(LICENSE_SECRET.as_bytes()).expect("HMAC accepts any key size");
    mac.update(&bytes[0..12]);
    let full_mac = mac.finalize().into_bytes();
    if full_mac[0..4] != embedded_mac {
        bail!("License key MAC verification failed — key is invalid or tampered");
    }

    let machine_bound = machine_tag != [0u8; 4];
    Ok(License {
        machine_tag,
        expiry_days,
        edition_flags,
        machine_bound,
    })
}

/// Full validation: parse + check machine binding + check expiry.
pub fn validate(key: &str) -> Result<License> {
    let lic = parse(key)?;
    if lic.machine_bound {
        let fp = machine_fingerprint();
        if lic.machine_tag != fp {
            bail!(
                "License key is bound to a different machine (tag {:02x}{:02x}{:02x}{:02x})",
                lic.machine_tag[0],
                lic.machine_tag[1],
                lic.machine_tag[2],
                lic.machine_tag[3]
            );
        }
    }
    if lic.is_expired() {
        bail!("License key has expired");
    }
    Ok(lic)
}

/// Activate: validate key and write to license file.
pub fn activate(key: &str) -> Result<License> {
    let lic = validate(key)?;
    let path = license_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, format!("{}\n", key.trim()))?;
    Ok(lic)
}

/// Read the stored license key and validate it.
/// Returns `Ok(None)` if no license is stored (community/open mode).
///
/// Supports two formats:
/// - Single line: `key` (machine-bound validation)
/// - Two lines: `key\nemail` (email-bound validation)
pub fn load() -> Result<Option<License>> {
    let path = match license_path() {
        Ok(p) => p,
        Err(_) => return Ok(None),
    };
    if !path.exists() {
        return Ok(None);
    }
    let raw = std::fs::read_to_string(&path)?;
    let mut lines = raw.lines();
    let key = match lines.next() {
        Some(k) if !k.trim().is_empty() => k.trim(),
        _ => return Ok(None),
    };
    let email = lines.next().map(|e| e.trim()).filter(|e| !e.is_empty());

    // Gumroad keys: stored as "GR:<key>" — trusted after first online verification
    if is_gumroad_key(key) {
        return Ok(Some(License {
            machine_tag: [0; 4],
            expiry_days: 0,
            edition_flags: 1, // Pro
            machine_bound: false,
        }));
    }

    // Stripe keys: stored as "ST:<key>" — trusted after first online verification
    if is_stored_stripe_key(key) {
        return Ok(Some(License {
            machine_tag: [0; 4],
            expiry_days: 0,
            edition_flags: 1, // Pro
            machine_bound: false,
        }));
    }

    // GitHub Sponsors keys: stored as "GH:<username>" — trusted after first online verification
    if is_stored_github_key(key) {
        return Ok(Some(License {
            machine_tag: [0; 4],
            expiry_days: 0,
            edition_flags: 1, // Pro
            machine_bound: false,
        }));
    }

    // PRECC native keys
    let lic = parse(key)?;
    if lic.is_expired() {
        bail!("License key has expired");
    }

    if lic.machine_bound {
        if let Some(email) = email {
            // Email-based: check email fingerprint
            let expected = email_fingerprint(email);
            if lic.machine_tag != expected {
                bail!("License key does not match stored email");
            }
        } else {
            // Machine-based: check machine fingerprint
            let fp = machine_fingerprint();
            if lic.machine_tag != fp {
                bail!("License key is bound to a different machine");
            }
        }
    }

    Ok(Some(lic))
}

// =============================================================================
// Gumroad license verification
// =============================================================================

/// The Gumroad product ID for PRECC Pro.
/// Set via PRECC_GUMROAD_PRODUCT_ID env at build time, or empty for open builds.
const GUMROAD_PRODUCT_ID: &str = match option_env!("PRECC_GUMROAD_PRODUCT_ID") {
    Some(s) => s,
    None => "",
};

/// Verify a Gumroad license key online and activate if valid.
///
/// Calls `https://api.gumroad.com/v2/licenses/verify` to check the key.
/// On success, stores the Gumroad key locally so subsequent loads don't
/// need network access (offline-friendly after first activation).
pub fn activate_gumroad(key: &str) -> Result<License> {
    let key = key.trim();

    if GUMROAD_PRODUCT_ID.is_empty() {
        bail!("Gumroad product ID not configured in this build");
    }

    // Call Gumroad license verification API
    let resp = ureq::post("https://api.gumroad.com/v2/licenses/verify")
        .send_form(&[
            ("product_id", GUMROAD_PRODUCT_ID),
            ("license_key", key),
            ("increment_uses_count", "true"),
        ])
        .map_err(|e| anyhow::anyhow!("Gumroad verification failed: {e}"))?;

    let body: serde_json::Value = resp
        .into_json()
        .map_err(|e| anyhow::anyhow!("Failed to parse Gumroad response: {e}"))?;

    let success = body["success"].as_bool().unwrap_or(false);
    if !success {
        let msg = body["message"]
            .as_str()
            .unwrap_or("Unknown error from Gumroad");
        bail!("Gumroad license verification failed: {msg}");
    }

    // Check for refund/dispute/chargeback
    let purchase = &body["purchase"];
    if purchase["refunded"].as_bool().unwrap_or(false)
        || purchase["disputed"].as_bool().unwrap_or(false)
        || purchase["chargebacked"].as_bool().unwrap_or(false)
    {
        bail!("This license has been refunded or disputed");
    }

    // Valid! Store as a Gumroad key (prefix with "GR:" to distinguish from PRECC keys)
    let email = purchase["email"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_lowercase();

    let path = license_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, format!("GR:{key}\n{email}\n"))?;

    Ok(License {
        machine_tag: [0; 4],
        expiry_days: 0,
        edition_flags: 1, // Pro
        machine_bound: false,
    })
}

/// Check if a stored key is a Gumroad key (prefixed with "GR:").
fn is_gumroad_key(key: &str) -> bool {
    key.starts_with("GR:")
}

// =============================================================================
// Stripe license verification
// =============================================================================

/// Stripe secret key, set via PRECC_STRIPE_SECRET_KEY env at build time.
const STRIPE_SECRET_KEY: &str = match option_env!("PRECC_STRIPE_SECRET_KEY") {
    Some(s) => s,
    None => "",
};

/// Check if a key looks like a Stripe key (cs_*, sub_*, pi_*).
pub fn is_stripe_key(key: &str) -> bool {
    let k = key.trim();
    k.starts_with("cs_") || k.starts_with("sub_") || k.starts_with("pi_")
}

/// Check if a stored key is a Stripe key (prefixed with "ST:").
fn is_stored_stripe_key(key: &str) -> bool {
    key.starts_with("ST:")
}

/// Verify a Stripe checkout/subscription/payment-intent and activate if valid.
///
/// Calls Stripe API to retrieve the object and check its status.
/// On success, stores the key locally prefixed with "ST:" so subsequent loads
/// don't need network access.
pub fn activate_stripe(key: &str) -> Result<License> {
    let key = key.trim();

    if STRIPE_SECRET_KEY.is_empty() {
        bail!("Stripe secret key not configured in this build");
    }

    // Determine the Stripe API endpoint based on key prefix
    let url = if key.starts_with("cs_") {
        format!("https://api.stripe.com/v1/checkout/sessions/{}", key)
    } else if key.starts_with("sub_") {
        format!("https://api.stripe.com/v1/subscriptions/{}", key)
    } else if key.starts_with("pi_") {
        format!("https://api.stripe.com/v1/payment_intents/{}", key)
    } else {
        bail!("Unrecognized Stripe key format: {key}");
    };

    let resp = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", STRIPE_SECRET_KEY))
        .call()
        .map_err(|e| anyhow::anyhow!("Stripe verification failed: {e}"))?;

    let body: serde_json::Value = resp
        .into_json()
        .map_err(|e| anyhow::anyhow!("Failed to parse Stripe response: {e}"))?;

    // Check payment status based on object type
    let status = body["status"].as_str().unwrap_or("");
    let valid = if key.starts_with("cs_") {
        status == "complete"
    } else if key.starts_with("sub_") {
        status == "active" || status == "trialing"
    } else {
        // pi_*
        status == "succeeded"
    };

    if !valid {
        bail!("Stripe payment not valid (status: {status})");
    }

    // Store as a Stripe key
    let path = license_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, format!("ST:{key}\n"))?;

    Ok(License {
        machine_tag: [0; 4],
        expiry_days: 0,
        edition_flags: 1, // Pro
        machine_bound: false,
    })
}

// =============================================================================
// GitHub Sponsors license verification
// =============================================================================

/// The GitHub username of the sponsored maintainer.
const GITHUB_SPONSOR_LOGIN: &str = "yijunyu";

/// Check if a key looks like a GitHub Sponsors flag.
pub fn is_github_key(key: &str) -> bool {
    key.trim().starts_with("--github")
}

/// Check if a stored key is a GitHub Sponsors key (prefixed with "GH:").
fn is_stored_github_key(key: &str) -> bool {
    key.starts_with("GH:")
}

/// Verify GitHub sponsorship and activate if the user is an active sponsor.
///
/// Uses `GITHUB_TOKEN` env var or falls back to the `gh` CLI token.
/// Queries GitHub GraphQL API to check if the authenticated user sponsors
/// the maintainer. On success, stores `GH:<username>` locally.
pub fn activate_github() -> Result<License> {
    let token = resolve_github_token()?;

    // Query the authenticated user's login
    let user_resp = ureq::post("https://api.github.com/graphql")
        .set("Authorization", &format!("Bearer {token}"))
        .set("User-Agent", "precc")
        .send_json(serde_json::json!({
            "query": "{ viewer { login } }"
        }))
        .map_err(|e| anyhow::anyhow!("GitHub API request failed: {e}"))?;

    let user_body: serde_json::Value = user_resp
        .into_json()
        .map_err(|e| anyhow::anyhow!("Failed to parse GitHub response: {e}"))?;

    let username = user_body["data"]["viewer"]["login"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Could not determine GitHub username from token"))?
        .to_string();

    // Check if the authenticated user sponsors the maintainer
    let query = format!(
        r#"{{ user(login: "{GITHUB_SPONSOR_LOGIN}") {{ isSponsoredBy(accountLogin: "{username}") }} }}"#
    );

    let sponsor_resp = ureq::post("https://api.github.com/graphql")
        .set("Authorization", &format!("Bearer {token}"))
        .set("User-Agent", "precc")
        .send_json(serde_json::json!({ "query": query }))
        .map_err(|e| anyhow::anyhow!("GitHub sponsorship check failed: {e}"))?;

    let sponsor_body: serde_json::Value = sponsor_resp
        .into_json()
        .map_err(|e| anyhow::anyhow!("Failed to parse GitHub sponsorship response: {e}"))?;

    let is_sponsored = sponsor_body["data"]["user"]["isSponsoredBy"]
        .as_bool()
        .unwrap_or(false);

    if !is_sponsored {
        bail!(
            "GitHub user @{username} is not currently sponsoring @{GITHUB_SPONSOR_LOGIN}.\n\
             Sponsor at: https://github.com/sponsors/{GITHUB_SPONSOR_LOGIN}"
        );
    }

    // Valid sponsor — store locally
    let path = license_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, format!("GH:{username}\n"))?;

    Ok(License {
        machine_tag: [0; 4],
        expiry_days: 0,
        edition_flags: 1, // Pro
        machine_bound: false,
    })
}

/// Resolve a GitHub token from GITHUB_TOKEN env or the `gh` CLI config.
fn resolve_github_token() -> Result<String> {
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    let output = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !token.is_empty() {
                return Ok(token);
            }
        }
    }

    bail!(
        "No GitHub token found. Either:\n\
         - Set GITHUB_TOKEN environment variable, or\n\
         - Install and login with GitHub CLI: gh auth login"
    )
}

/// Deactivate: remove the stored license key.
pub fn deactivate() -> Result<()> {
    let path = license_path()?;
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

/// Generate a new license key for the given parameters.
/// Only callable with the LICENSE_SECRET known at build time.
pub fn generate(machine_tag: [u8; 4], expiry_days: u32, edition_flags: u32) -> String {
    let mut payload = [0u8; 12];
    payload[0..4].copy_from_slice(&machine_tag);
    payload[4..8].copy_from_slice(&expiry_days.to_be_bytes());
    payload[8..12].copy_from_slice(&edition_flags.to_be_bytes());

    let mut mac =
        HmacSha256::new_from_slice(LICENSE_SECRET.as_bytes()).expect("HMAC accepts any key size");
    mac.update(&payload);
    let full_mac = mac.finalize().into_bytes();

    let mut key_bytes = [0u8; 16];
    key_bytes[0..12].copy_from_slice(&payload);
    key_bytes[12..16].copy_from_slice(&full_mac[0..4]);

    let hex = hex::encode(key_bytes).to_uppercase();
    format!(
        "PRECC-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..16],
        &hex[16..24],
        &hex[24..32]
    )
}

/// Compute the 4-byte fingerprint from an email address.
pub fn email_fingerprint(email: &str) -> [u8; 4] {
    use sha2::Digest;
    let normalized = email.trim().to_lowercase();
    let hash = sha2::Sha256::digest(normalized.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

/// Generate a license key bound to an email address.
pub fn generate_for_email(email: &str, expiry_days: u32, edition_flags: u32) -> String {
    let tag = email_fingerprint(email);
    generate(tag, expiry_days, edition_flags)
}

/// Activate a license key using email-based binding.
/// Validates the key's MAC and checks that the machine_tag matches SHA256(email).
/// Stores both key and email in the license file.
pub fn activate_with_email(key: &str, email: &str) -> Result<License> {
    let lic = parse(key)?;
    if lic.is_expired() {
        bail!("License key has expired");
    }
    let expected_tag = email_fingerprint(email);
    if lic.machine_tag != expected_tag {
        bail!("License key does not match this email address");
    }
    let path = license_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        &path,
        format!("{}\n{}\n", key.trim(), email.trim().to_lowercase()),
    )?;
    Ok(lic)
}

/// Compute the 4-byte machine fingerprint from hostname + username.
pub fn machine_fingerprint() -> [u8; 4] {
    use sha2::Digest;
    let hostname = hostname();
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let input = format!("{}\x00{}", hostname, username);
    let hash = sha2::Sha256::digest(input.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

fn hostname() -> String {
    // Try reading /etc/hostname (Linux/macOS)
    if let Ok(h) = std::fs::read_to_string("/etc/hostname") {
        let h = h.trim().to_string();
        if !h.is_empty() {
            return h;
        }
    }
    // Fall back to HOSTNAME env var or "unknown"
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}

fn license_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("precc")
        .join("license.key"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_parse_community_key() {
        // machine_tag=[0,0,0,0] → not machine-bound, no expiry, community edition
        let key = generate([0, 0, 0, 0], 0, 0);
        assert!(key.starts_with("PRECC-"));
        let lic = parse(&key).expect("parse should succeed");
        assert!(!lic.machine_bound);
        assert_eq!(lic.expiry_days, 0);
        assert!(!lic.is_expired());
        assert_eq!(lic.edition_name(), "Community");
    }

    #[test]
    fn generate_and_parse_pro_key() {
        let key = generate([0, 0, 0, 0], 0, 1); // edition_flags=1 → Pro
        let lic = parse(&key).expect("parse should succeed");
        assert!(lic.is_pro());
        assert!(!lic.is_team());
        assert_eq!(lic.edition_name(), "Pro");
    }

    #[test]
    fn generate_and_parse_machine_bound_key() {
        let fp = machine_fingerprint();
        let key = generate(fp, 0, 1);
        let lic = validate(&key).expect("validate should succeed on same machine");
        assert!(lic.machine_bound);
        assert!(lic.is_pro());
    }

    #[test]
    fn tampered_key_fails_mac() {
        let key = generate([0, 0, 0, 0], 0, 1);
        // Flip one hex char
        let mut chars: Vec<char> = key.chars().collect();
        let idx = chars.len() - 2;
        chars[idx] = if chars[idx] == '0' { '1' } else { '0' };
        let tampered: String = chars.into_iter().collect();
        assert!(parse(&tampered).is_err());
    }

    #[test]
    fn expired_key_fails_validate() {
        // expiry_days = 1 (Jan 2, 1970) → definitely expired
        let key = generate([0, 0, 0, 0], 1, 1);
        let result = validate(&key);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn email_fingerprint_deterministic() {
        let fp1 = email_fingerprint("User@Example.COM");
        let fp2 = email_fingerprint("user@example.com");
        assert_eq!(fp1, fp2); // case-insensitive
    }

    #[test]
    fn email_fingerprint_differs_for_different_emails() {
        let fp1 = email_fingerprint("alice@example.com");
        let fp2 = email_fingerprint("bob@example.com");
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn generate_for_email_roundtrip() {
        let email = "buyer@example.com";
        let key = generate_for_email(email, 0, 1);
        let lic = parse(&key).expect("parse should succeed");
        assert!(lic.is_pro());
        assert_eq!(lic.machine_tag, email_fingerprint(email));
    }

    #[test]
    fn activate_with_email_wrong_email_fails() {
        let key = generate_for_email("correct@example.com", 0, 1);
        let result = activate_with_email(&key, "wrong@example.com");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("does not match this email"));
    }

    #[test]
    fn generate_roundtrip_format() {
        let key = generate([0xDE, 0xAD, 0xBE, 0xEF], 0, 0);
        // Format: PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "PRECC");
        for part in &parts[1..] {
            assert_eq!(part.len(), 8);
        }
    }

    // =========================================================================
    // Stripe key detection
    // =========================================================================

    #[test]
    fn is_stripe_key_checkout_session() {
        assert!(is_stripe_key("cs_test_abc123"));
        assert!(is_stripe_key("cs_live_xyz"));
    }

    #[test]
    fn is_stripe_key_subscription() {
        assert!(is_stripe_key("sub_1234567890"));
    }

    #[test]
    fn is_stripe_key_payment_intent() {
        assert!(is_stripe_key("pi_3abc"));
    }

    #[test]
    fn is_stripe_key_rejects_other() {
        assert!(!is_stripe_key("PRECC-DEADBEEF-00000000-00000001-AABBCCDD"));
        assert!(!is_stripe_key("random_key"));
        assert!(!is_stripe_key(""));
    }

    #[test]
    fn is_stored_stripe_key_works() {
        assert!(is_stored_stripe_key("ST:cs_test_abc"));
        assert!(!is_stored_stripe_key("GR:abc"));
        assert!(!is_stored_stripe_key("cs_test_abc"));
    }

    #[test]
    fn is_gumroad_key_works() {
        assert!(is_gumroad_key("GR:abc123"));
        assert!(!is_gumroad_key("ST:abc"));
        assert!(!is_gumroad_key("abc"));
    }

    // =========================================================================
    // GitHub Sponsors key detection
    // =========================================================================

    #[test]
    fn is_stored_github_key_works() {
        assert!(is_stored_github_key("GH:octocat"));
        assert!(!is_stored_github_key("GR:abc"));
        assert!(!is_stored_github_key("ST:abc"));
        assert!(!is_stored_github_key("octocat"));
    }

    #[test]
    fn is_github_key_works() {
        assert!(is_github_key("--github"));
        assert!(!is_github_key("GH:abc"));
        assert!(!is_github_key("random"));
    }
}
