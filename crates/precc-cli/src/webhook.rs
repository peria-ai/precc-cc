//! Stripe webhook server for automatic license key delivery.
//!
//! Listens for `checkout.session.completed` events from Stripe,
//! generates an email-bound PRECC Pro license key, and emails it
//! to the customer via local sendmail.
//!
//! # Usage
//! ```bash
//! precc webhook serve --port 8090 --stripe-secret whsec_XXXXX
//! ```
//!
//! # Configuration
//! The webhook signing secret can be provided via:
//! - `--stripe-secret` CLI flag
//! - `PRECC_STRIPE_WEBHOOK_SECRET` environment variable
//!
//! # Nginx reverse proxy
//! ```nginx
//! location /webhook/stripe {
//!     proxy_pass http://127.0.0.1:8090/webhook/stripe;
//! }
//! ```

use anyhow::{bail, Result};
use hmac::{Hmac, Mac};
use precc_core::license;
use sha2::Sha256;

use crate::mail;
use std::io::Read as _;

type HmacSha256 = Hmac<Sha256>;

/// Default port for the webhook server.
const DEFAULT_PORT: u16 = 8090;

/// Run the webhook HTTP server.
pub fn serve(port: Option<u16>, stripe_secret: Option<String>) -> Result<()> {
    let port = port.unwrap_or(DEFAULT_PORT);
    let secret = stripe_secret
        .or_else(|| std::env::var("PRECC_STRIPE_WEBHOOK_SECRET").ok())
        .unwrap_or_default();

    if secret.is_empty() {
        bail!(
            "Stripe webhook signing secret required.\n\
             Provide via --stripe-secret or PRECC_STRIPE_WEBHOOK_SECRET env var.\n\
             Get it from: https://dashboard.stripe.com/webhooks"
        );
    }

    let addr = format!("0.0.0.0:{port}");
    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to bind {addr}: {e}"))?;

    eprintln!("PRECC webhook server listening on {addr}");
    eprintln!("  Endpoints:");
    eprintln!("    POST /webhook/stripe     — Stripe license delivery");
    eprintln!("    POST /api/telemetry/v1   — Anonymous usage telemetry");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().to_string();

        if method == "POST" && url == "/api/telemetry/v1" {
            match handle_telemetry(&mut request) {
                Ok(()) => {
                    let resp = tiny_http::Response::from_string("OK").with_status_code(200);
                    let _ = request.respond(resp);
                }
                Err(e) => {
                    eprintln!("[TELEMETRY ERR] {e}");
                    let resp =
                        tiny_http::Response::from_string("Bad request").with_status_code(400);
                    let _ = request.respond(resp);
                }
            }
            continue;
        }

        if method != "POST" || url != "/webhook/stripe" {
            let resp = tiny_http::Response::from_string("Not found").with_status_code(404);
            let _ = request.respond(resp);
            continue;
        }

        match handle_stripe_webhook(&mut request, &secret) {
            Ok(msg) => {
                eprintln!("[OK] {msg}");
                let resp = tiny_http::Response::from_string("OK").with_status_code(200);
                let _ = request.respond(resp);
            }
            Err(e) => {
                eprintln!("[ERR] {e}");
                let resp = tiny_http::Response::from_string(format!("{e}")).with_status_code(400);
                let _ = request.respond(resp);
            }
        }
    }

    Ok(())
}

/// Handle a telemetry POST — append JSON-lines to ~/.local/share/precc/telemetry.jsonl
fn handle_telemetry(request: &mut tiny_http::Request) -> Result<()> {
    use std::io::Write;

    let mut body = String::new();
    request
        .as_reader()
        .read_to_string(&mut body)
        .map_err(|e| anyhow::anyhow!("Failed to read telemetry body: {e}"))?;

    // Validate it's valid JSON
    let value: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| anyhow::anyhow!("Invalid JSON: {e}"))?;

    // Add server-side timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut record = value;
    record["_received_at"] = serde_json::json!(now);
    record["_remote_ip"] = serde_json::json!(request
        .remote_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_default());

    // Append to telemetry log
    let data_dir = precc_core::db::data_dir()?;
    let path = data_dir.join("telemetry.jsonl");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let line = serde_json::to_string(&record)?;
    writeln!(file, "{line}")?;

    eprintln!(
        "[TELEMETRY] v={} os={} tier={}",
        record["precc_version"].as_str().unwrap_or("?"),
        record["os"].as_str().unwrap_or("?"),
        record["tier"].as_str().unwrap_or("?"),
    );

    Ok(())
}

/// Handle a single Stripe webhook request.
fn handle_stripe_webhook(request: &mut tiny_http::Request, secret: &str) -> Result<String> {
    // Read body
    let mut body = String::new();
    request
        .as_reader()
        .read_to_string(&mut body)
        .map_err(|e| anyhow::anyhow!("Failed to read request body: {e}"))?;

    // Get Stripe-Signature header
    let sig_header = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Stripe-Signature" || h.field.as_str() == "stripe-signature")
        .map(|h| h.value.as_str().to_string())
        .ok_or_else(|| anyhow::anyhow!("Missing Stripe-Signature header"))?;

    // Verify signature
    verify_stripe_signature(&body, &sig_header, secret)?;

    // Parse event
    let event: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| anyhow::anyhow!("Invalid JSON: {e}"))?;

    let event_type = event["type"].as_str().unwrap_or("");

    match event_type {
        "checkout.session.completed" => handle_checkout_completed(&event),
        _ => {
            // Acknowledge but ignore other event types
            Ok(format!("Ignored event type: {event_type}"))
        }
    }
}

/// Handle a checkout.session.completed event.
fn handle_checkout_completed(event: &serde_json::Value) -> Result<String> {
    let session = &event["data"]["object"];

    let email = session["customer_details"]["email"]
        .as_str()
        .or_else(|| session["customer_email"].as_str())
        .ok_or_else(|| anyhow::anyhow!("No customer email in checkout session"))?
        .trim()
        .to_lowercase();

    let session_id = session["id"].as_str().unwrap_or("unknown");

    // Determine expiry from metadata (set by stripe-setup.sh).
    // Metadata contains relative days (e.g. 180, 365).
    let duration_days: u32 = session["metadata"]["expiry_days"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30); // default to 30-day

    // Convert relative days to absolute days-since-epoch for the key.
    let now_days = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86400;
    let expiry_absolute = (now_days as u32) + duration_days;

    // Generate email-bound Pro license key
    let key = license::generate_for_email(&email, expiry_absolute, 1); // edition_flags=1 = Pro

    // Send license key to customer
    send_license_email(&email, &key, duration_days)?;

    // Schedule a reminder email for the last day before expiry
    schedule_expiry_reminder(&email, &key, duration_days)?;

    Ok(format!(
        "License sent to {email} (session={session_id}, duration={duration_days}d, expires=day {expiry_absolute})"
    ))
}

/// Verify the Stripe webhook signature (v1 scheme).
fn verify_stripe_signature(payload: &str, sig_header: &str, secret: &str) -> Result<()> {
    // Parse header: t=TIMESTAMP,v1=SIGNATURE
    let mut timestamp = "";
    let mut signature = "";

    for part in sig_header.split(',') {
        let part = part.trim();
        if let Some(t) = part.strip_prefix("t=") {
            timestamp = t;
        } else if let Some(v) = part.strip_prefix("v1=") {
            signature = v;
        }
    }

    if timestamp.is_empty() || signature.is_empty() {
        bail!("Invalid Stripe-Signature header format");
    }

    // Compute expected signature: HMAC-SHA256(secret, "TIMESTAMP.PAYLOAD")
    let signed_payload = format!("{timestamp}.{payload}");
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| anyhow::anyhow!("Invalid webhook secret"))?;
    mac.update(signed_payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    // Constant-time comparison
    if !constant_time_eq(expected.as_bytes(), signature.as_bytes()) {
        bail!("Stripe signature verification failed");
    }

    // Optional: check timestamp is recent (within 5 minutes)
    if let Ok(ts) = timestamp.parse::<i64>() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        if (now - ts).abs() > 300 {
            bail!("Stripe webhook timestamp too old (replay attack?)");
        }
    }

    Ok(())
}

/// Constant-time byte comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Schedule a reminder email to be sent on the last day before license expiry.
///
/// Writes a reminder file to `~/.local/share/precc/reminders/` that the
/// `precc webhook check-reminders` cron job picks up daily.
fn schedule_expiry_reminder(email: &str, key: &str, duration_days: u32) -> Result<()> {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Send reminder 1 day before expiry
    let remind_at_secs = now_secs + ((duration_days as u64).saturating_sub(1)) * 86400;

    let reminders_dir = if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home).join(".local/share/precc/reminders")
    } else {
        return Ok(()); // Can't schedule without HOME
    };
    std::fs::create_dir_all(&reminders_dir)?;

    let reminder = serde_json::json!({
        "email": email,
        "key": key,
        "duration_days": duration_days,
        "remind_at": remind_at_secs,
        "created_at": now_secs,
    });

    let filename = format!("{}-{}.json", remind_at_secs, email.replace('@', "_at_"));
    std::fs::write(
        reminders_dir.join(&filename),
        serde_json::to_string_pretty(&reminder)?,
    )?;

    eprintln!("  Reminder scheduled for {filename}");
    Ok(())
}

/// Check all pending reminders and send emails for any that are due.
/// Called by `precc webhook check-reminders` (intended for daily cron).
pub fn check_reminders() -> Result<u32> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    let reminders_dir = std::path::PathBuf::from(home).join(".local/share/precc/reminders");

    if !reminders_dir.exists() {
        return Ok(0);
    }

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut sent = 0u32;

    for entry in std::fs::read_dir(&reminders_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let content = std::fs::read_to_string(&path)?;
        let reminder: serde_json::Value = serde_json::from_str(&content)?;

        let remind_at = reminder["remind_at"].as_u64().unwrap_or(0);
        if now_secs < remind_at {
            continue; // Not due yet
        }

        let email = reminder["email"].as_str().unwrap_or("");
        let duration_days = reminder["duration_days"].as_u64().unwrap_or(0) as u32;

        if !email.is_empty() {
            match send_expiry_reminder_email(email, duration_days) {
                Ok(()) => {
                    eprintln!("  Sent expiry reminder to {email}");
                    sent += 1;
                }
                Err(e) => {
                    eprintln!("  Failed to send reminder to {email}: {e}");
                }
            }
        }

        // Remove the reminder file whether sent or not (avoid retrying forever)
        let _ = std::fs::remove_file(&path);
    }

    Ok(sent)
}

/// Send a license expiry reminder email.
fn send_expiry_reminder_email(to: &str, duration_days: u32) -> Result<()> {
    let plan = if duration_days >= 365 {
        "12-month"
    } else {
        "6-month"
    };

    let body = format!(
        "Hi,\n\
         \n\
         Your PRECC Pro license ({plan} plan) expires tomorrow.\n\
         \n\
         To renew, visit: https://github.com/sponsors/yijunyu\n\
         \n\
         After payment, you'll receive a new license key automatically.\n\
         \n\
         If you've already renewed, you can ignore this email.\n\
         \n\
         — PRECC Team\n\
         https://github.com/peria-ai/precc-cc\n"
    );

    mail::send_mail(to, "Your PRECC Pro license expires tomorrow", &body, &[])
}

/// Send the license key to the customer via SMTP (configured in mail.toml).
fn send_license_email(to: &str, key: &str, expiry_days: u32) -> Result<()> {
    let duration = if expiry_days >= 365 {
        "12 months".to_string()
    } else {
        format!("{expiry_days} days")
    };

    let body = format!(
        "Thank you for purchasing PRECC Pro!\n\
         \n\
         Your license key:\n\
         \n\
         {key}\n\
         \n\
         To activate, run:\n\
         \n\
         precc license activate {key} --email {to}\n\
         \n\
         Duration: {duration}\n\
         Edition: Pro\n\
         \n\
         If you have any questions, reply to this email.\n\
         \n\
         — PRECC Team\n\
         https://github.com/peria-ai/precc-cc\n"
    );

    mail::send_mail(to, "Your PRECC Pro License Key", &body, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_time_eq_identical() {
        assert!(constant_time_eq(b"hello", b"hello"));
    }

    #[test]
    fn constant_time_eq_different() {
        assert!(!constant_time_eq(b"hello", b"world"));
    }

    #[test]
    fn constant_time_eq_different_length() {
        assert!(!constant_time_eq(b"hello", b"hi"));
    }

    #[test]
    fn constant_time_eq_empty() {
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn constant_time_eq_one_bit_diff() {
        assert!(!constant_time_eq(b"\x00", b"\x01"));
    }

    #[test]
    fn verify_signature_valid() {
        let secret = "whsec_test_secret";
        let payload = r#"{"type":"checkout.session.completed"}"#;
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let signed = format!("{ts}.{payload}");
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed.as_bytes());
        let sig = hex::encode(mac.finalize().into_bytes());
        let header = format!("t={ts},v1={sig}");

        assert!(verify_stripe_signature(payload, &header, secret).is_ok());
    }

    #[test]
    fn verify_signature_wrong_secret() {
        let payload = "test";
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let signed = format!("{ts}.{payload}");
        let mut mac = HmacSha256::new_from_slice(b"wrong_secret").unwrap();
        mac.update(signed.as_bytes());
        let sig = hex::encode(mac.finalize().into_bytes());
        let header = format!("t={ts},v1={sig}");

        assert!(verify_stripe_signature(payload, &header, "correct_secret").is_err());
    }

    #[test]
    fn verify_signature_missing_fields() {
        assert!(verify_stripe_signature("body", "garbage", "secret").is_err());
    }

    #[test]
    fn verify_signature_replay_attack() {
        let secret = "whsec_test";
        let old_ts = 1000000;
        let payload = "test";
        let signed = format!("{old_ts}.{payload}");
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed.as_bytes());
        let sig = hex::encode(mac.finalize().into_bytes());
        let header = format!("t={old_ts},v1={sig}");

        let result = verify_stripe_signature(payload, &header, secret);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too old"));
    }

    #[test]
    fn reminder_json_format() {
        let dir = tempfile::tempdir().unwrap();
        let reminders_dir = dir.path().join("reminders");
        std::fs::create_dir_all(&reminders_dir).unwrap();

        let reminder = serde_json::json!({
            "email": "test@example.com",
            "key": "PRECC-TEST",
            "duration_days": 180,
            "remind_at": 0,
            "created_at": 0,
        });
        let path = reminders_dir.join("0-test_at_example.com.json");
        std::fs::write(&path, serde_json::to_string_pretty(&reminder).unwrap()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(content["email"], "test@example.com");
        assert_eq!(content["duration_days"], 180);
    }

    #[test]
    fn expiry_absolute_calculation() {
        let duration_days: u32 = 365;
        let now_days = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 86400;
        let expiry_absolute = (now_days as u32) + duration_days;

        let key = license::generate_for_email("test@example.com", expiry_absolute, 1);
        let lic = license::parse(&key).unwrap();
        assert!(!lic.is_expired());
        assert!(lic.days_remaining().unwrap() >= 364);
    }

    #[test]
    fn expiry_default_30_days() {
        let event = serde_json::json!({
            "data": {
                "object": {
                    "customer_details": {"email": "user@test.com"},
                    "metadata": {}
                }
            }
        });
        let session = &event["data"]["object"];
        let duration_days: u32 = session["metadata"]["expiry_days"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        assert_eq!(duration_days, 30);
    }

    #[test]
    fn email_extraction_customer_details() {
        let event = serde_json::json!({
            "data": {
                "object": {
                    "customer_details": {"email": "User@Example.COM"},
                    "metadata": {"expiry_days": "180"}
                }
            }
        });
        let session = &event["data"]["object"];
        let email = session["customer_details"]["email"]
            .as_str()
            .or_else(|| session["customer_email"].as_str())
            .unwrap()
            .trim()
            .to_lowercase();
        assert_eq!(email, "user@example.com");
    }

    #[test]
    fn email_extraction_fallback_customer_email() {
        let event = serde_json::json!({
            "data": {
                "object": {
                    "customer_email": "fallback@test.com",
                    "metadata": {}
                }
            }
        });
        let session = &event["data"]["object"];
        let email = session["customer_details"]["email"]
            .as_str()
            .or_else(|| session["customer_email"].as_str())
            .unwrap()
            .trim()
            .to_lowercase();
        assert_eq!(email, "fallback@test.com");
    }

    #[test]
    fn duration_formatting() {
        let format_duration = |days: u32| -> String {
            if days >= 365 {
                "12 months".to_string()
            } else {
                format!("{days} days")
            }
        };
        assert_eq!(format_duration(365), "12 months");
        assert_eq!(format_duration(180), "180 days");
        assert_eq!(format_duration(30), "30 days");
    }
}
