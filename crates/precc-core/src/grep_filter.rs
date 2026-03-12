//! Grep tool optimization: auto head_limit, auto type filter, and LSP hints.

use serde_json::Value;
use std::path::Path;

/// Default head_limit to inject when none is set and output_mode is "content".
const DEFAULT_HEAD_LIMIT: u64 = 50;

/// Suggest a head_limit if none is set and output_mode is "content".
pub fn suggest_head_limit(grep_input: &Value) -> Option<u64> {
    // Only inject for content mode (which can produce huge output)
    let output_mode = grep_input
        .get("output_mode")
        .and_then(|v| v.as_str())
        .unwrap_or("files_with_matches");

    if output_mode != "content" {
        return None;
    }

    // Don't override an existing head_limit
    if grep_input.get("head_limit").is_some() {
        return None;
    }

    Some(DEFAULT_HEAD_LIMIT)
}

/// Detect the project type based on marker files in the given directory.
/// Returns the ripgrep type string (e.g., "rust", "go", "py", "ts").
pub fn detect_project_type(cwd: &str) -> Option<&'static str> {
    let cwd_path = Path::new(cwd);

    if cwd_path.join("Cargo.toml").exists() {
        return Some("rust");
    }
    if cwd_path.join("go.mod").exists() {
        return Some("go");
    }
    if cwd_path.join("pyproject.toml").exists() || cwd_path.join("setup.py").exists() {
        return Some("py");
    }
    if cwd_path.join("tsconfig.json").exists() {
        return Some("ts");
    }
    if cwd_path.join("package.json").exists() {
        return Some("js");
    }

    None
}

/// Suggest a type filter if none is set and we can detect the project type.
/// Only suggests if no `type` or `glob` filter is already set.
pub fn suggest_type_filter(grep_input: &Value, cwd: &str) -> Option<&'static str> {
    // Don't override existing filters
    if grep_input.get("type").is_some() || grep_input.get("glob").is_some() {
        return None;
    }

    detect_project_type(cwd)
}

/// Check if a grep pattern looks like a symbol/definition lookup.
/// These are better served by LSP Go to Definition.
pub fn is_symbol_lookup(pattern: &str) -> bool {
    // Common definition patterns
    let definition_prefixes = [
        "^(pub )?(fn|struct|enum|trait|impl|type|const|static) ",
        "^pub (fn|struct|enum|trait|impl|type|const|static) ",
        "^fn ",
        "^struct ",
        "^enum ",
        "^trait ",
        "^impl ",
        "^class ",
        "^def ",
        "^func ",
        "^function ",
        "^interface ",
    ];

    let trimmed = pattern.trim();
    for prefix in &definition_prefixes {
        if trimmed.starts_with(prefix) {
            return true;
        }
    }

    // Also catch simple "fn foo_bar" patterns without the ^
    let simple_def = regex::Regex::new(
        r"^(pub\s+)?(fn|struct|enum|trait|impl|type|const|static|class|def|func|function|interface)\s+\w+$",
    );
    if let Ok(re) = simple_def {
        if re.is_match(trimmed) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // =========================================================================
    // suggest_head_limit
    // =========================================================================

    #[test]
    fn head_limit_injected_for_content_mode() {
        let input = json!({"pattern": "foo", "output_mode": "content"});
        assert_eq!(suggest_head_limit(&input), Some(50));
    }

    #[test]
    fn head_limit_not_injected_for_files_mode() {
        let input = json!({"pattern": "foo", "output_mode": "files_with_matches"});
        assert_eq!(suggest_head_limit(&input), None);
    }

    #[test]
    fn head_limit_not_injected_for_count_mode() {
        let input = json!({"pattern": "foo", "output_mode": "count"});
        assert_eq!(suggest_head_limit(&input), None);
    }

    #[test]
    fn head_limit_not_injected_if_already_set() {
        let input = json!({"pattern": "foo", "output_mode": "content", "head_limit": 20});
        assert_eq!(suggest_head_limit(&input), None);
    }

    #[test]
    fn head_limit_not_injected_if_zero() {
        // head_limit: 0 is "unlimited" but it IS explicitly set, so don't override
        let input = json!({"pattern": "foo", "output_mode": "content", "head_limit": 0});
        assert_eq!(suggest_head_limit(&input), None);
    }

    #[test]
    fn head_limit_not_injected_for_default_mode() {
        // default output_mode is "files_with_matches", not "content"
        let input = json!({"pattern": "foo"});
        assert_eq!(suggest_head_limit(&input), None);
    }

    #[test]
    fn head_limit_content_mode_no_other_fields() {
        let input = json!({"output_mode": "content"});
        assert_eq!(suggest_head_limit(&input), Some(50));
    }

    // =========================================================================
    // suggest_type_filter
    // =========================================================================

    #[test]
    fn type_filter_not_overridden() {
        let input = json!({"pattern": "foo", "type": "py"});
        assert_eq!(suggest_type_filter(&input, "/tmp"), None);
    }

    #[test]
    fn glob_filter_prevents_type_suggestion() {
        let input = json!({"pattern": "foo", "glob": "*.js"});
        assert_eq!(suggest_type_filter(&input, "/tmp"), None);
    }

    #[test]
    fn type_filter_suggested_for_rust_project() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("rust")
        );
    }

    #[test]
    fn type_filter_suggested_for_go_project() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("go.mod"), "module example.com/foo").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("go")
        );
    }

    #[test]
    fn type_filter_suggested_for_python_project_pyproject() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("pyproject.toml"), "[project]").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("py")
        );
    }

    #[test]
    fn type_filter_suggested_for_python_project_setup_py() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("setup.py"), "from setuptools import setup").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("py")
        );
    }

    #[test]
    fn type_filter_suggested_for_ts_project() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("tsconfig.json"), "{}").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("ts")
        );
    }

    #[test]
    fn type_filter_suggested_for_js_project() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("package.json"), "{}").unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            Some("js")
        );
    }

    #[test]
    fn type_filter_not_suggested_for_unknown_project() {
        let tmp = tempfile::tempdir().unwrap();
        let input = json!({"pattern": "foo"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            None
        );
    }

    #[test]
    fn type_filter_not_suggested_when_type_already_set() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        let input = json!({"pattern": "foo", "type": "js"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            None
        );
    }

    #[test]
    fn type_filter_not_suggested_when_glob_already_set() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        let input = json!({"pattern": "foo", "glob": "*.toml"});
        assert_eq!(
            suggest_type_filter(&input, tmp.path().to_str().unwrap()),
            None
        );
    }

    // =========================================================================
    // is_symbol_lookup
    // =========================================================================

    #[test]
    fn symbol_lookup_detected() {
        assert!(is_symbol_lookup("fn main"));
        assert!(is_symbol_lookup("pub fn process_event"));
        assert!(is_symbol_lookup("struct Pipeline"));
        assert!(is_symbol_lookup("class MyClass"));
        assert!(is_symbol_lookup("def process_data"));
        assert!(is_symbol_lookup("func handleRequest"));
    }

    #[test]
    fn symbol_lookup_rust_keywords() {
        assert!(is_symbol_lookup("enum Color"));
        assert!(is_symbol_lookup("trait Iterator"));
        assert!(is_symbol_lookup("impl Display"));
        assert!(is_symbol_lookup("type Result"));
        assert!(is_symbol_lookup("const MAX_SIZE"));
        assert!(is_symbol_lookup("static COUNTER"));
        assert!(is_symbol_lookup("pub struct Config"));
        assert!(is_symbol_lookup("pub enum Error"));
        assert!(is_symbol_lookup("pub trait Service"));
    }

    #[test]
    fn symbol_lookup_other_languages() {
        assert!(is_symbol_lookup("function handleClick"));
        assert!(is_symbol_lookup("interface ApiResponse"));
    }

    #[test]
    fn symbol_lookup_with_caret_prefix() {
        assert!(is_symbol_lookup("^fn main"));
        assert!(is_symbol_lookup("^struct Config"));
        assert!(is_symbol_lookup("^class Handler"));
        assert!(is_symbol_lookup("^def process"));
        // Regex alternation patterns from definition_prefixes list
        assert!(is_symbol_lookup(
            "^(pub )?(fn|struct|enum|trait|impl|type|const|static) foo"
        ));
        assert!(is_symbol_lookup(
            "^pub (fn|struct|enum|trait|impl|type|const|static) bar"
        ));
    }

    #[test]
    fn symbol_lookup_with_whitespace() {
        assert!(is_symbol_lookup("  fn main  "));
        assert!(is_symbol_lookup("\tstruct Pipeline\t"));
    }

    #[test]
    fn non_symbol_patterns_pass() {
        assert!(!is_symbol_lookup("TODO"));
        assert!(!is_symbol_lookup("error.*timeout"));
        assert!(!is_symbol_lookup("import foo"));
        assert!(!is_symbol_lookup("log.*Error"));
    }

    #[test]
    fn non_symbol_common_grep_patterns() {
        assert!(!is_symbol_lookup("FIXME"));
        assert!(!is_symbol_lookup("use std::io"));
        assert!(!is_symbol_lookup("println!"));
        assert!(!is_symbol_lookup("return Ok"));
        assert!(!is_symbol_lookup("// TODO"));
        assert!(!is_symbol_lookup("require\\("));
        assert!(!is_symbol_lookup("from .models import"));
    }

    #[test]
    fn non_symbol_empty_string() {
        assert!(!is_symbol_lookup(""));
    }

    #[test]
    fn non_symbol_regex_patterns() {
        assert!(!is_symbol_lookup(r"fn\s+\w+\("));
        assert!(!is_symbol_lookup(r"struct\s+\w+\s*\{"));
    }

    // =========================================================================
    // detect_project_type
    // =========================================================================

    #[test]
    fn detect_project_type_works() {
        let tmp = tempfile::tempdir().unwrap();
        let cwd = tmp.path().to_str().unwrap();

        // No marker files
        assert_eq!(detect_project_type(cwd), None);

        // Create Cargo.toml
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        assert_eq!(detect_project_type(cwd), Some("rust"));
    }

    #[test]
    fn detect_project_type_priority() {
        // Rust takes priority over other types if multiple marker files exist
        let tmp = tempfile::tempdir().unwrap();
        let cwd = tmp.path().to_str().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        std::fs::write(tmp.path().join("package.json"), "{}").unwrap();
        assert_eq!(detect_project_type(cwd), Some("rust"));
    }

    #[test]
    fn detect_project_type_nonexistent_dir() {
        assert_eq!(detect_project_type("/nonexistent/dir/abc123"), None);
    }

    #[test]
    fn detect_project_type_all_types() {
        // Test each project type individually
        let markers: Vec<(&str, &str, &str)> = vec![
            ("Cargo.toml", "[package]", "rust"),
            ("go.mod", "module foo", "go"),
            ("pyproject.toml", "[project]", "py"),
            ("tsconfig.json", "{}", "ts"),
            ("package.json", "{}", "js"),
        ];

        for (marker, content, expected) in markers {
            let tmp = tempfile::tempdir().unwrap();
            std::fs::write(tmp.path().join(marker), content).unwrap();
            assert_eq!(
                detect_project_type(tmp.path().to_str().unwrap()),
                Some(expected),
                "failed for marker {}",
                marker
            );
        }
    }
}
