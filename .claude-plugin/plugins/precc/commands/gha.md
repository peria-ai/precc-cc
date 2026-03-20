---
description: Analyze a GitHub Actions workflow failure — fetch logs, extract errors, classify root cause
---

Run `precc gha $ARGUMENTS` to diagnose a GitHub Actions workflow failure.

Accepts a GitHub Actions URL or shorthand:
- `precc gha https://github.com/owner/repo/actions/runs/12345`
- `precc gha owner/repo/12345`

Fetches failed job logs via the GitHub API, extracts error lines, and classifies the failure as:
- **flaky** — timeouts, connection resets, intermittent issues
- **breaking** — compilation errors, test failures
- **config** — permission denied, missing secrets, invalid workflow
- **unknown** — unable to determine from logs alone
