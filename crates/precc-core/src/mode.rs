//! Compression mode taxonomy: a single source of truth for the modes
//! that PRECC's pipeline can apply to a Bash command.

/// The compression mode actually applied to a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompressionMode {
    /// No compression — original command runs as-is.
    Basic,
    /// Diet stage: rule-based pipe filters / flag injection.
    Diet,
    /// Nushell wrap: structured/compact output via nu translation.
    Nushell,
    /// lean-ctx wrap: external LLM-aware compression.
    LeanCtx,
    /// RTK rewrite or jj translation.
    Rtk,
    /// Adaptive expand: compression intentionally skipped due to recent failure.
    AdaptiveExpand,
}

impl CompressionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CompressionMode::Basic => "basic",
            CompressionMode::Diet => "diet",
            CompressionMode::Nushell => "nushell",
            CompressionMode::LeanCtx => "lean-ctx",
            CompressionMode::Rtk => "rtk",
            CompressionMode::AdaptiveExpand => "adaptive-expand",
        }
    }

    pub fn from_str(s: &str) -> Option<CompressionMode> {
        match s {
            "basic" => Some(CompressionMode::Basic),
            "diet" => Some(CompressionMode::Diet),
            "nushell" => Some(CompressionMode::Nushell),
            "lean-ctx" => Some(CompressionMode::LeanCtx),
            "rtk" => Some(CompressionMode::Rtk),
            "adaptive-expand" => Some(CompressionMode::AdaptiveExpand),
            _ => None,
        }
    }

    /// Derive the compression mode from a list of pipeline reasons.
    /// Picks the most specific compression-related reason, ignoring meta tags
    /// like "skill:..." or "cd:...". Priority: nushell > lean-ctx > rtk > diet.
    pub fn from_pipeline_reasons(reasons: &[String]) -> CompressionMode {
        // Check in priority order (most specific first)
        for r in reasons {
            if r.starts_with("nushell-wrap") {
                return CompressionMode::Nushell;
            }
        }
        for r in reasons {
            if r.starts_with("lean-ctx-wrap") {
                return CompressionMode::LeanCtx;
            }
        }
        for r in reasons {
            if r.starts_with("rtk-rewrite") || r.starts_with("jj-translate") {
                return CompressionMode::Rtk;
            }
        }
        for r in reasons {
            if r == "diet" {
                return CompressionMode::Diet;
            }
        }
        for r in reasons {
            if r.starts_with("adaptive-expand") {
                return CompressionMode::AdaptiveExpand;
            }
        }
        CompressionMode::Basic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pipeline_reasons_picks_compression_mode() {
        let r = vec!["skill:cargo-fix (conf=0.9)".to_string(), "rtk-rewrite".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Rtk);

        let r = vec!["nushell-wrap".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Nushell);

        let r = vec!["diet".to_string(), "lean-ctx-wrap".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::LeanCtx);

        let r = vec!["jj-translate".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Rtk);

        let r = vec!["diet".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Diet);

        let r = vec!["cd:Cargo.toml (conf=0.9)".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Basic);

        let r: Vec<String> = vec![];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Basic);
    }

    #[test]
    fn nushell_takes_priority_over_rtk() {
        // If both somehow appear, nushell wins (it would have actually fired first)
        let r = vec!["rtk-rewrite".to_string(), "nushell-wrap".to_string()];
        assert_eq!(CompressionMode::from_pipeline_reasons(&r), CompressionMode::Nushell);
    }

    #[test]
    fn round_trip_str() {
        for m in [
            CompressionMode::Basic,
            CompressionMode::Diet,
            CompressionMode::Nushell,
            CompressionMode::LeanCtx,
            CompressionMode::Rtk,
            CompressionMode::AdaptiveExpand,
        ] {
            assert_eq!(CompressionMode::from_str(m.as_str()), Some(m));
        }
    }
}
