//! Geofence — IP-based region detection and compliance guard (Pro feature).
//!
//! Detects whether the user's egress IP originates from a blocked region
//! (e.g., jurisdictions where Anthropic API access may result in account bans).
//! When a blocked region is detected, the hook denies the command and suggests
//! alternative LLM providers accessible in that region.
//!
//! # Architecture
//!
//! - **Cache file**: `~/.local/share/precc/geofence.cache` (JSON, refreshed with TTL)
//! - **Refresh**: Called by `precc geofence refresh` or `precc init` — never in the hook hot path.
//! - **Hook check**: Reads the cache file (<1ms), returns cached verdict.
//!
//! # Pro feature
//!
//! Geofence checking is gated behind a Pro (or higher) license. Free-tier users
//! are silently skipped (no warning, no block).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cache TTL: 10 minutes (in seconds).
const CACHE_TTL_SECS: u64 = 600;

/// Countries where Anthropic API access is restricted or may lead to bans.
/// ISO 3166-1 alpha-2 codes.
const BLOCKED_COUNTRIES: &[&str] = &[
    "CN", // China
    "CU", // Cuba
    "IR", // Iran
    "KP", // North Korea
    "SY", // Syria
    "RU", // Russia
];

/// Alternative LLM providers accessible from blocked regions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeLlm {
    pub name: &'static str,
    pub provider: &'static str,
    pub api_url: &'static str,
    pub notes: &'static str,
}

/// Known alternatives for users in blocked regions.
pub const ALTERNATIVES: &[AlternativeLlm] = &[
    AlternativeLlm {
        name: "DeepSeek V3",
        provider: "DeepSeek",
        api_url: "https://api.deepseek.com",
        notes: "OpenAI-compatible API, strong coding performance",
    },
    AlternativeLlm {
        name: "Qwen-Max",
        provider: "Alibaba Cloud",
        api_url: "https://dashscope.aliyuncs.com",
        notes: "Qwen series, multilingual, tool-use capable",
    },
    AlternativeLlm {
        name: "GLM-4",
        provider: "Zhipu AI",
        api_url: "https://open.bigmodel.cn",
        notes: "ChatGLM series, strong Chinese language support",
    },
    AlternativeLlm {
        name: "Yi-Large",
        provider: "01.AI",
        api_url: "https://api.lingyiwanwu.com",
        notes: "Bilingual, good reasoning capabilities",
    },
    AlternativeLlm {
        name: "Doubao",
        provider: "ByteDance (Volcano Engine)",
        api_url: "https://ark.cn-beijing.volces.com",
        notes: "Doubao/Skylark series, integrated with Volcano ecosystem",
    },
];

/// Cached geofence lookup result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeofenceCache {
    /// User's detected egress IP address.
    pub ip: String,
    /// ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// Human-readable country name.
    pub country_name: String,
    /// Whether this IP is in a blocked region.
    pub blocked: bool,
    /// Unix timestamp when this cache entry was written.
    pub timestamp: u64,
}

/// Result of a geofence check in the hook hot path.
#[derive(Debug, Clone)]
pub enum GeofenceVerdict {
    /// IP is in an allowed region (or no cache / feature disabled).
    Allow,
    /// IP is in a blocked region — includes details for the deny message.
    Blocked(GeofenceCache),
    /// Cache is stale or missing — allow but suggest refresh.
    Stale,
}

/// Path to the geofence cache file.
pub fn cache_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("precc")
        .join("geofence.cache"))
}

/// Read the cached geofence result (hot path, <1ms).
///
/// Returns `Allow` if:
/// - Cache file doesn't exist
/// - Cache is corrupted (fail-open)
/// - Cached result says IP is not blocked
///
/// Returns `Blocked` if cache says IP is in a blocked region and cache is fresh.
/// Returns `Stale` if cache exists but is expired (> TTL).
pub fn check_cached() -> GeofenceVerdict {
    let path = match cache_path() {
        Ok(p) => p,
        Err(_) => return GeofenceVerdict::Allow,
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return GeofenceVerdict::Allow, // No cache — fail-open
    };

    let cache: GeofenceCache = match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(_) => return GeofenceVerdict::Allow, // Corrupted — fail-open
    };

    // Check TTL
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if now.saturating_sub(cache.timestamp) > CACHE_TTL_SECS {
        return GeofenceVerdict::Stale;
    }

    if cache.blocked {
        GeofenceVerdict::Blocked(cache)
    } else {
        GeofenceVerdict::Allow
    }
}

/// Refresh the geofence cache by probing an external IP geolocation API.
///
/// This is NOT called in the hook hot path. It's invoked by:
/// - `precc geofence refresh`
/// - `precc init`
/// - The background miner daemon (periodic refresh)
///
/// Uses ip-api.com (free, no API key required, 45 req/min limit).
pub fn refresh_cache() -> Result<GeofenceCache> {
    let resp: serde_json::Value =
        ureq::get("http://ip-api.com/json/?fields=query,countryCode,country,status,message")
            .call()
            .map_err(|e| anyhow::anyhow!("IP geolocation lookup failed: {e}"))?
            .into_json()
            .map_err(|e| anyhow::anyhow!("Failed to parse geolocation response: {e}"))?;

    let status = resp["status"].as_str().unwrap_or("");
    if status != "success" {
        let msg = resp["message"].as_str().unwrap_or("unknown error");
        anyhow::bail!("IP geolocation API error: {msg}");
    }

    let ip = resp["query"].as_str().unwrap_or("").to_string();
    let country_code = resp["countryCode"].as_str().unwrap_or("").to_string();
    let country_name = resp["country"].as_str().unwrap_or("").to_string();
    let blocked = BLOCKED_COUNTRIES.contains(&country_code.as_str());

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cache = GeofenceCache {
        ip,
        country_code,
        country_name,
        blocked,
        timestamp: now,
    };

    // Write cache file
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&cache)?;
    std::fs::write(&path, json)?;

    Ok(cache)
}

/// Check if a given country code is blocked.
pub fn is_blocked_country(country_code: &str) -> bool {
    BLOCKED_COUNTRIES.contains(&country_code.to_uppercase().as_str())
}

/// Format a user-facing deny message when a blocked region is detected.
pub fn format_deny_message(cache: &GeofenceCache) -> String {
    let mut msg = format!(
        "PRECC geofence: your IP ({}) resolves to {} ({}), \
         which is a region where Anthropic API access may result in account suspension.\n\n\
         To protect your account, this request has been blocked.\n\n\
         Alternative LLM providers accessible in your region:\n",
        cache.ip, cache.country_name, cache.country_code
    );

    for alt in ALTERNATIVES {
        msg.push_str(&format!(
            "  - {} ({}) — {}\n    API: {}\n",
            alt.name, alt.provider, alt.notes, alt.api_url
        ));
    }

    msg.push_str(
        "\nTo reconfigure PRECC to use an alternative provider, run:\n  \
         precc geofence configure --provider <name>\n\n\
         To re-check your IP (e.g., after connecting to a VPN):\n  \
         precc geofence refresh\n\n\
         To override this check (at your own risk):\n  \
         export PRECC_GEOFENCE_OVERRIDE=1",
    );

    msg
}

/// Format a short stderr warning for stale cache.
pub fn format_stale_warning() -> String {
    "[precc] Geofence cache is stale — run `precc geofence refresh` to update".to_string()
}

/// Check the PRECC_GEOFENCE_OVERRIDE env var.
pub fn is_overridden() -> bool {
    std::env::var("PRECC_GEOFENCE_OVERRIDE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Check whether a skill with the given `claude_interaction` level should be
/// blocked under the current geofence state.
///
/// - `claude_interaction`: 0 = no/unknown, 1 = direct, 2 = indirect
///
/// Returns `true` if the skill should be suppressed (i.e., user is in a blocked
/// region and the skill interacts with Claude).
pub fn should_block_skill(claude_interaction: i32) -> bool {
    if claude_interaction == 0 {
        return false; // Not Claude-interacting — always allowed
    }

    // Only enforce if there's a fresh blocked cache
    matches!(check_cached(), GeofenceVerdict::Blocked(_))
}

/// Delete the geofence cache file.
pub fn clear_cache() -> Result<()> {
    let path = cache_path()?;
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

/// Read the cache file without TTL check (for display in CLI).
pub fn read_cache() -> Result<Option<GeofenceCache>> {
    let path = cache_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)?;
    let cache: GeofenceCache = serde_json::from_str(&content)?;
    Ok(Some(cache))
}

/// Return the list of blocked country codes.
pub fn blocked_countries() -> &'static [&'static str] {
    BLOCKED_COUNTRIES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_country_detection() {
        assert!(is_blocked_country("CN"));
        assert!(is_blocked_country("cn")); // case-insensitive
        assert!(is_blocked_country("RU"));
        assert!(is_blocked_country("KP"));
        assert!(!is_blocked_country("US"));
        assert!(!is_blocked_country("GB"));
        assert!(!is_blocked_country("JP"));
        assert!(!is_blocked_country("DE"));
    }

    #[test]
    fn alternatives_non_empty() {
        assert!(!ALTERNATIVES.is_empty());
        for alt in ALTERNATIVES {
            assert!(!alt.name.is_empty());
            assert!(!alt.provider.is_empty());
            assert!(!alt.api_url.is_empty());
        }
    }

    #[test]
    fn deny_message_contains_essentials() {
        let cache = GeofenceCache {
            ip: "1.2.3.4".to_string(),
            country_code: "CN".to_string(),
            country_name: "China".to_string(),
            blocked: true,
            timestamp: 0,
        };
        let msg = format_deny_message(&cache);
        assert!(msg.contains("1.2.3.4"));
        assert!(msg.contains("China"));
        assert!(msg.contains("CN"));
        assert!(msg.contains("DeepSeek"));
        assert!(msg.contains("Qwen"));
        assert!(msg.contains("PRECC_GEOFENCE_OVERRIDE"));
        assert!(msg.contains("precc geofence refresh"));
    }

    #[test]
    fn override_env_check() {
        // Without env var set, should not be overridden
        // (This test may be flaky if env is set externally, but that's unlikely in CI)
        std::env::remove_var("PRECC_GEOFENCE_OVERRIDE");
        assert!(!is_overridden());
    }

    #[test]
    fn cache_path_uses_home() {
        // Just verify it doesn't panic
        if std::env::var("HOME").is_ok() {
            let path = cache_path().unwrap();
            assert!(path.to_string_lossy().contains("geofence.cache"));
            assert!(path.to_string_lossy().contains(".local/share/precc"));
        }
    }

    #[test]
    fn geofence_cache_roundtrip() {
        let cache = GeofenceCache {
            ip: "203.0.113.1".to_string(),
            country_code: "US".to_string(),
            country_name: "United States".to_string(),
            blocked: false,
            timestamp: 1711411200,
        };
        let json = serde_json::to_string(&cache).unwrap();
        let parsed: GeofenceCache = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ip, "203.0.113.1");
        assert_eq!(parsed.country_code, "US");
        assert!(!parsed.blocked);
    }

    #[test]
    fn check_cached_no_file_returns_allow() {
        // With no cache file, should fail-open
        let verdict = check_cached();
        assert!(matches!(
            verdict,
            GeofenceVerdict::Allow | GeofenceVerdict::Stale
        ));
    }

    #[test]
    fn stale_warning_non_empty() {
        let warning = format_stale_warning();
        assert!(warning.contains("stale"));
        assert!(warning.contains("precc geofence refresh"));
    }
}
