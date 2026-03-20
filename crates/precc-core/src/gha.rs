//! GitHub Actions log analysis — fetch failed workflow logs and diagnose failures.
//!
//! Given a GitHub Actions URL, fetches the logs for the failed job(s) and produces
//! a structured diagnosis: is it a flaky test, a breaking commit, or a config issue?

use anyhow::{bail, Context, Result};

/// Parsed GitHub Actions URL components.
#[derive(Debug, Clone)]
pub struct GhaUrl {
    pub owner: String,
    pub repo: String,
    pub run_id: u64,
}

/// Diagnosis of a GitHub Actions failure.
#[derive(Debug)]
pub struct GhaDiagnosis {
    /// The workflow run URL
    pub url: String,
    /// Run status (e.g., "failure", "success")
    pub status: String,
    /// Run conclusion
    pub conclusion: String,
    /// Workflow name
    pub workflow_name: String,
    /// Branch
    pub branch: String,
    /// Failed job names
    pub failed_jobs: Vec<String>,
    /// Extracted error lines from logs
    pub error_lines: Vec<String>,
    /// Classification: "flaky", "breaking", "config", or "unknown"
    pub classification: String,
    /// Human-readable summary
    pub summary: String,
}

/// Parse a GitHub Actions URL into its components.
///
/// Accepts formats:
/// - `https://github.com/owner/repo/actions/runs/12345`
/// - `https://github.com/owner/repo/actions/runs/12345/job/67890`
/// - `owner/repo/12345` (shorthand)
pub fn parse_url(url: &str) -> Result<GhaUrl> {
    // Try full URL first
    if url.contains("github.com") {
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
        // Find "actions" / "runs" / <id>
        if let Some(pos) = parts.iter().position(|&p| p == "runs") {
            if pos + 1 < parts.len() {
                let run_id: u64 = parts[pos + 1].parse().context("invalid run ID in URL")?;
                // owner is 2 before "actions", repo is 1 before
                let actions_pos = parts
                    .iter()
                    .position(|&p| p == "actions")
                    .context("missing 'actions' in URL")?;
                if actions_pos >= 2 {
                    return Ok(GhaUrl {
                        owner: parts[actions_pos - 2].to_string(),
                        repo: parts[actions_pos - 1].to_string(),
                        run_id,
                    });
                }
            }
        }
        bail!("could not parse GitHub Actions URL: {}", url);
    }

    // Try shorthand: owner/repo/run_id
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() == 3 {
        let run_id: u64 = parts[2].parse().context("invalid run ID")?;
        return Ok(GhaUrl {
            owner: parts[0].to_string(),
            repo: parts[1].to_string(),
            run_id,
        });
    }

    bail!(
        "unrecognized format — use https://github.com/owner/repo/actions/runs/ID or owner/repo/ID"
    )
}

/// Fetch workflow run metadata via the GitHub API.
/// Returns the run JSON as a serde_json::Value.
fn fetch_run(gha: &GhaUrl) -> Result<serde_json::Value> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs/{}",
        gha.owner, gha.repo, gha.run_id
    );
    let resp = ureq::get(&url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "precc-cli")
        .call()
        .context("failed to fetch workflow run")?;
    let body: serde_json::Value = resp.into_json()?;
    Ok(body)
}

/// Fetch the jobs for a workflow run.
fn fetch_jobs(gha: &GhaUrl) -> Result<serde_json::Value> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs/{}/jobs",
        gha.owner, gha.repo, gha.run_id
    );
    let resp = ureq::get(&url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "precc-cli")
        .call()
        .context("failed to fetch jobs")?;
    let body: serde_json::Value = resp.into_json()?;
    Ok(body)
}

/// Fetch the log for a specific job (returns plain text).
fn fetch_job_log(gha: &GhaUrl, job_id: u64) -> Result<String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/jobs/{}/logs",
        gha.owner, gha.repo, job_id
    );
    let resp = ureq::get(&url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "precc-cli")
        .call()
        .context("failed to fetch job logs")?;
    let body = resp.into_string()?;
    Ok(body)
}

/// Extract error-relevant lines from a log.
fn extract_errors(log: &str) -> Vec<String> {
    let error_patterns = [
        "error",
        "Error",
        "ERROR",
        "FAILED",
        "FAIL",
        "panic",
        "PANIC",
        "fatal",
        "Fatal",
        "AssertionError",
        "assert",
        "exit code",
        "exited with",
        "timed out",
        "timeout",
        "denied",
        "permission",
    ];

    let mut errors = Vec::new();
    for line in log.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.len() < 10 {
            continue;
        }
        if error_patterns.iter().any(|p| trimmed.contains(p)) {
            // Skip noisy lines
            if trimmed.contains("##[debug]")
                || trimmed.contains("::debug::")
                || trimmed.starts_with("Downloading")
            {
                continue;
            }
            errors.push(trimmed.to_string());
            if errors.len() >= 50 {
                break; // cap at 50 error lines
            }
        }
    }
    errors
}

/// Classify a failure based on error patterns.
fn classify(errors: &[String], conclusion: &str) -> String {
    if conclusion == "success" {
        return "success".to_string();
    }

    let text = errors.join("\n").to_lowercase();

    // Flaky indicators
    if text.contains("timed out")
        || text.contains("timeout")
        || text.contains("flaky")
        || text.contains("intermittent")
        || text.contains("connection reset")
        || text.contains("socket hang up")
    {
        return "flaky".to_string();
    }

    // Config indicators
    if text.contains("permission denied")
        || text.contains("secret")
        || text.contains("credentials")
        || text.contains("not found") && text.contains("action")
        || text.contains("invalid workflow")
    {
        return "config".to_string();
    }

    // Breaking commit indicators (compilation/test failures)
    if text.contains("error[")
        || text.contains("failed to compile")
        || text.contains("test failed")
        || text.contains("assertion")
        || text.contains("cargo test")
    {
        return "breaking".to_string();
    }

    "unknown".to_string()
}

/// Analyze a GitHub Actions workflow run and produce a diagnosis.
pub fn analyze(url_str: &str) -> Result<GhaDiagnosis> {
    let gha = parse_url(url_str)?;

    // Fetch run metadata
    let run = fetch_run(&gha)?;
    let status = run["status"].as_str().unwrap_or("unknown").to_string();
    let conclusion = run["conclusion"].as_str().unwrap_or("unknown").to_string();
    let workflow_name = run["name"].as_str().unwrap_or("unknown").to_string();
    let branch = run["head_branch"].as_str().unwrap_or("unknown").to_string();

    // Fetch jobs
    let jobs_json = fetch_jobs(&gha)?;
    let jobs = jobs_json["jobs"].as_array();

    let mut failed_jobs = Vec::new();
    let mut all_errors = Vec::new();

    if let Some(jobs) = jobs {
        for job in jobs {
            let job_conclusion = job["conclusion"].as_str().unwrap_or("");
            if job_conclusion == "failure" {
                let job_name = job["name"].as_str().unwrap_or("unnamed").to_string();
                failed_jobs.push(job_name);

                // Try to fetch logs for this job
                if let Some(job_id) = job["id"].as_u64() {
                    if let Ok(log) = fetch_job_log(&gha, job_id) {
                        let mut errors = extract_errors(&log);
                        all_errors.append(&mut errors);
                    }
                }
            }
        }
    }

    let classification = classify(&all_errors, &conclusion);

    let summary = if conclusion == "success" {
        format!(
            "Workflow '{}' on branch '{}' succeeded.",
            workflow_name, branch
        )
    } else {
        let job_list = if failed_jobs.is_empty() {
            "no specific jobs identified".to_string()
        } else {
            failed_jobs.join(", ")
        };
        let class_desc = match classification.as_str() {
            "flaky" => "likely a flaky test or transient network issue",
            "config" => "likely a configuration or permissions issue",
            "breaking" => "likely a breaking commit (compilation or test failure)",
            _ => "unable to determine root cause from logs alone",
        };
        format!(
            "Workflow '{}' on branch '{}' failed. Failed jobs: {}. Classification: {} — {}. Found {} error lines in logs.",
            workflow_name, branch, job_list, classification, class_desc, all_errors.len()
        )
    };

    Ok(GhaDiagnosis {
        url: url_str.to_string(),
        status,
        conclusion,
        workflow_name,
        branch,
        failed_jobs,
        error_lines: all_errors,
        classification,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_url() {
        let gha = parse_url("https://github.com/yijunyu/precc-cc/actions/runs/12345").unwrap();
        assert_eq!(gha.owner, "yijunyu");
        assert_eq!(gha.repo, "precc-cc");
        assert_eq!(gha.run_id, 12345);
    }

    #[test]
    fn parse_url_with_job() {
        let gha = parse_url("https://github.com/owner/repo/actions/runs/999/job/111").unwrap();
        assert_eq!(gha.owner, "owner");
        assert_eq!(gha.repo, "repo");
        assert_eq!(gha.run_id, 999);
    }

    #[test]
    fn parse_shorthand() {
        let gha = parse_url("owner/repo/42").unwrap();
        assert_eq!(gha.owner, "owner");
        assert_eq!(gha.repo, "repo");
        assert_eq!(gha.run_id, 42);
    }

    #[test]
    fn parse_invalid_url() {
        assert!(parse_url("not-a-url").is_err());
    }

    #[test]
    fn parse_url_trailing_slash() {
        let gha = parse_url("https://github.com/a/b/actions/runs/1/").unwrap();
        assert_eq!(gha.run_id, 1);
    }

    #[test]
    fn extract_errors_basic() {
        let log = "step 1: ok\nERROR: something broke\nstep 2: ok\nFAILED test_foo\n";
        let errors = extract_errors(log);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("ERROR"));
        assert!(errors[1].contains("FAILED"));
    }

    #[test]
    fn extract_errors_skips_debug() {
        let log = "##[debug] Error in debug\nReal error: compilation failed\n";
        let errors = extract_errors(log);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("compilation failed"));
    }

    #[test]
    fn extract_errors_caps_at_50() {
        let log: String = (0..100)
            .map(|i| format!("ERROR: failure number {}\n", i))
            .collect();
        let errors = extract_errors(&log);
        assert_eq!(errors.len(), 50);
    }

    #[test]
    fn classify_flaky() {
        let errors = vec!["connection reset by peer".to_string()];
        assert_eq!(classify(&errors, "failure"), "flaky");
    }

    #[test]
    fn classify_config() {
        let errors = vec!["Error: permission denied accessing secrets".to_string()];
        assert_eq!(classify(&errors, "failure"), "config");
    }

    #[test]
    fn classify_breaking() {
        let errors = vec!["error[E0308]: mismatched types".to_string()];
        assert_eq!(classify(&errors, "failure"), "breaking");
    }

    #[test]
    fn classify_success() {
        let errors = vec![];
        assert_eq!(classify(&errors, "success"), "success");
    }

    #[test]
    fn classify_unknown() {
        let errors = vec!["something happened".to_string()];
        assert_eq!(classify(&errors, "failure"), "unknown");
    }
}
