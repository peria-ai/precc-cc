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

    #[test]
    fn inject_frontmatter_to_plain_prompt() {
        let prompt = "Find all test files";
        let result = inject_hooks_frontmatter(prompt);
        assert!(result.starts_with("---"));
        assert!(result.contains("precc-hook"));
        assert!(result.ends_with("Find all test files"));
    }

    #[test]
    fn no_double_injection() {
        let prompt = "Find all test files";
        let first = inject_hooks_frontmatter(prompt);
        let second = inject_hooks_frontmatter(&first);
        assert_eq!(first, second);
    }

    #[test]
    fn has_frontmatter_detection() {
        assert!(!has_hooks_frontmatter("plain prompt"));
        assert!(!has_hooks_frontmatter("---\ntitle: foo\n---\nprompt"));
        assert!(has_hooks_frontmatter(
            "---\nhooks:\n  PreToolUse:\n    - matcher: Bash\n      hooks:\n        - type: command\n          command: precc-hook\n---\nprompt"
        ));
    }

    #[test]
    fn preserves_existing_non_precc_frontmatter() {
        let prompt = "---\ntitle: My Agent\n---\nDo something";
        let result = inject_hooks_frontmatter(prompt);
        // Should prepend PRECC frontmatter before the existing one
        assert!(result.starts_with("---"));
        assert!(result.contains("precc-hook"));
        assert!(result.contains("Do something"));
    }
}
