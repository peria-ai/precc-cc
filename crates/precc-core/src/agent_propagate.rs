//! Agent tool hook propagation: inject PRECC hooks into subagent prompts
//! so that subagents inherit the same Bash/Read/Grep optimizations.

/// YAML frontmatter block that configures PRECC hooks for subagents.
const HOOKS_FRONTMATTER: &str = "\
---
hooks:
  PreToolUse:
    - matcher: Bash
      hooks:
        - type: command
          command: precc-hook
          timeout: 5
    - matcher: Read
      hooks:
        - type: command
          command: precc-hook
          timeout: 3
    - matcher: Grep
      hooks:
        - type: command
          command: precc-hook
          timeout: 3
---
";

/// Check if a prompt already contains hooks frontmatter (to avoid double-injection).
pub fn has_hooks_frontmatter(prompt: &str) -> bool {
    let trimmed = prompt.trim_start();
    if !trimmed.starts_with("---") {
        return false;
    }
    // Check if the frontmatter contains hook-related keywords
    if let Some(end) = trimmed[3..].find("---") {
        let frontmatter = &trimmed[3..3 + end];
        frontmatter.contains("precc-hook")
    } else {
        false
    }
}

/// Inject hooks frontmatter into a subagent prompt.
/// Returns the modified prompt with PRECC hooks prepended.
/// If hooks are already present, returns the original prompt unchanged.
pub fn inject_hooks_frontmatter(prompt: &str) -> String {
    if has_hooks_frontmatter(prompt) {
        return prompt.to_string();
    }

    format!("{}{}", HOOKS_FRONTMATTER, prompt)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // inject_hooks_frontmatter
    // =========================================================================

    #[test]
    fn inject_frontmatter_to_plain_prompt() {
        let prompt = "Find all test files";
        let result = inject_hooks_frontmatter(prompt);
        assert!(result.starts_with("---"));
        assert!(result.contains("precc-hook"));
        assert!(result.ends_with("Find all test files"));
    }

    #[test]
    fn inject_frontmatter_empty_prompt() {
        let result = inject_hooks_frontmatter("");
        assert!(result.starts_with("---"));
        assert!(result.contains("precc-hook"));
    }

    #[test]
    fn inject_frontmatter_multiline_prompt() {
        let prompt = "Search for all test files\nAlso check coverage\nReport results";
        let result = inject_hooks_frontmatter(prompt);
        assert!(result.contains("precc-hook"));
        assert!(result.contains("Search for all test files"));
        assert!(result.contains("Also check coverage"));
        assert!(result.contains("Report results"));
    }

    #[test]
    fn inject_frontmatter_contains_all_matchers() {
        let result = inject_hooks_frontmatter("test prompt");
        assert!(result.contains("matcher: Bash"));
        assert!(result.contains("matcher: Read"));
        assert!(result.contains("matcher: Grep"));
    }

    #[test]
    fn inject_frontmatter_contains_correct_timeouts() {
        let result = inject_hooks_frontmatter("test prompt");
        // Bash gets timeout: 5, Read/Grep get timeout: 3
        assert!(result.contains("timeout: 5"));
        assert!(result.contains("timeout: 3"));
    }

    #[test]
    fn inject_frontmatter_valid_yaml_structure() {
        let result = inject_hooks_frontmatter("test");
        // Should start and end frontmatter with ---
        assert!(result.starts_with("---\n"));
        // Should have closing --- before the prompt
        let parts: Vec<&str> = result.splitn(3, "---").collect();
        assert!(
            parts.len() >= 3,
            "frontmatter should have opening and closing ---"
        );
    }

    // =========================================================================
    // no_double_injection
    // =========================================================================

    #[test]
    fn no_double_injection() {
        let prompt = "Find all test files";
        let first = inject_hooks_frontmatter(prompt);
        let second = inject_hooks_frontmatter(&first);
        assert_eq!(first, second);
    }

    #[test]
    fn no_double_injection_triple() {
        let prompt = "Do something";
        let first = inject_hooks_frontmatter(prompt);
        let second = inject_hooks_frontmatter(&first);
        let third = inject_hooks_frontmatter(&second);
        assert_eq!(first, second);
        assert_eq!(second, third);
    }

    // =========================================================================
    // has_hooks_frontmatter
    // =========================================================================

    #[test]
    fn has_frontmatter_detection() {
        assert!(!has_hooks_frontmatter("plain prompt"));
        assert!(!has_hooks_frontmatter("---\ntitle: foo\n---\nprompt"));
        assert!(has_hooks_frontmatter(
            "---\nhooks:\n  PreToolUse:\n    - matcher: Bash\n      hooks:\n        - type: command\n          command: precc-hook\n---\nprompt"
        ));
    }

    #[test]
    fn has_frontmatter_empty_string() {
        assert!(!has_hooks_frontmatter(""));
    }

    #[test]
    fn has_frontmatter_just_dashes() {
        assert!(!has_hooks_frontmatter("---"));
        assert!(!has_hooks_frontmatter("------"));
    }

    #[test]
    fn has_frontmatter_no_closing() {
        // Unclosed frontmatter should not match
        assert!(!has_hooks_frontmatter("---\ncommand: precc-hook\n"));
    }

    #[test]
    fn has_frontmatter_with_leading_whitespace() {
        // Trim leading whitespace before checking
        assert!(has_hooks_frontmatter(
            "  ---\ncommand: precc-hook\n---\nprompt"
        ));
    }

    #[test]
    fn has_frontmatter_non_precc_hooks() {
        // Frontmatter with hooks but not precc-hook should not match
        assert!(!has_hooks_frontmatter(
            "---\nhooks:\n  PreToolUse:\n    - matcher: Bash\n      hooks:\n        - type: command\n          command: other-hook\n---\nprompt"
        ));
    }

    #[test]
    fn has_frontmatter_round_trip() {
        // Injected frontmatter should be detected
        let injected = inject_hooks_frontmatter("test prompt");
        assert!(has_hooks_frontmatter(&injected));
    }

    // =========================================================================
    // preserves_existing_content
    // =========================================================================

    #[test]
    fn preserves_existing_non_precc_frontmatter() {
        let prompt = "---\ntitle: My Agent\n---\nDo something";
        let result = inject_hooks_frontmatter(prompt);
        // Should prepend PRECC frontmatter before the existing one
        assert!(result.starts_with("---"));
        assert!(result.contains("precc-hook"));
        assert!(result.contains("Do something"));
    }

    #[test]
    fn preserves_prompt_with_special_characters() {
        let prompt = "Search for patterns: fn main() { println!(\"hello\"); }";
        let result = inject_hooks_frontmatter(prompt);
        assert!(result.contains(prompt));
    }

    #[test]
    fn preserves_prompt_with_yaml_like_content() {
        let prompt = "key: value\nlist:\n  - item1\n  - item2";
        let result = inject_hooks_frontmatter(prompt);
        assert!(result.contains(prompt));
    }
}
