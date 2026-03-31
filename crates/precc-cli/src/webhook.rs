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
use std::io::Read as _;

type HmacSha256 = Hmac<Sha256>;

/// Default port for the webhook server.
const DEFAULT_PORT: u16 = 8090;

/// From address for license key emails.
const FROM_EMAIL: &str = "support@peria.ai";

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
    eprintln!("  Endpoint: POST /webhook/stripe");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().to_string();

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

    let message = format!(
        "From: PRECC <{FROM_EMAIL}>\n\
         To: {to}\n\
         Subject: Your PRECC Pro license expires tomorrow\n\
         Content-Type: text/plain; charset=utf-8\n\
         \n\
         {body}"
    );

    let mut child = std::process::Command::new("sendmail")
        .args(["-t", "-f", FROM_EMAIL])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to run sendmail: {e}"))?;

    use std::io::Write;
    child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("No stdin on sendmail"))?
        .write_all(message.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to sendmail: {e}"))?;

    let status = child.wait()?;
    if !status.success() {
        bail!("sendmail exited with status {}", status);
    }

    Ok(())
}

/// Send the license key to the customer via local sendmail.
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

    let message = format!(
        "From: PRECC <{FROM_EMAIL}>\n\
         To: {to}\n\
         Subject: Your PRECC Pro License Key\n\
         Content-Type: text/plain; charset=utf-8\n\
         \n\
         {body}"
    );

    // Use local sendmail (Postfix)
    let mut child = std::process::Command::new("sendmail")
        .args(["-t", "-f", FROM_EMAIL])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to run sendmail: {e}"))?;

    use std::io::Write;
    child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("No stdin on sendmail"))?
        .write_all(message.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to sendmail: {e}"))?;

    let status = child.wait()?;
    if !status.success() {
        bail!("sendmail exited with status {}", status);
    }

    Ok(())
}
