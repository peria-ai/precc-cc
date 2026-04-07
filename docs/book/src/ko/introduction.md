# Introduction

## What is PRECC?

PRECC (Claude Code를 위한 예측 오류 수정) is a Rust tool that intercepts Claude Code bash commands via the official PreToolUse hook mechanism. It fixes errors *before they happen*, saving tokens and eliminating retry loops.

커뮤니티 사용자에게 무료.

## The Problem

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## The Four Pillars

### 컨텍스트 수정 (cd-prepend)

`cargo build`나 `npm test` 같은 명령이 잘못된 디렉토리에서 실행될 때를 감지하고 실행 전에 `cd /올바른/경로 &&`를 추가합니다.

### GDB Debugging

Detects opportunities to attach GDB for deeper debugging of segfaults and crashes, providing structured debug information instead of raw core dumps.

### Session Mining

Mines Claude Code session logs for failure-fix pairs. When the same mistake recurs, PRECC already knows the fix and applies it automatically.

### Automation Skills

A library of built-in and mined skills that match command patterns and rewrite them. Skills are defined as TOML files or SQLite rows, making them easy to inspect, edit, and share.

## How It Works (30-Second Version)

1. Claude Code is about to run a bash command.
2. The PreToolUse hook sends the command to `precc-hook` as JSON on stdin.
3. `precc-hook` runs the command through the pipeline (skills, directory correction, compression) in under 3 milliseconds.
4. The corrected command is returned as JSON on stdout.
5. Claude Code executes the corrected command instead.

Claude never sees the error. No tokens wasted.

### Adaptive Compression

명령이 압축 후 실패하면 PRECC가 다음 재시도에서 자동으로 압축을 건너뛰어 Claude가 디버깅을 위한 전체 비압축 출력을 받습니다.

## Live Usage Statistics

현재 버전 <span data-stat="current_version">--</span>:

| Metric | Value |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| 고유 사용자 | <span data-stat="unique_users">--</span> |

### 실측 절약 (실제 데이터)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metric</th><th>Value</th></tr></thead>
<tbody>
<tr><td>원본 출력 토큰 (PRECC 없이)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>실제 출력 토큰 (PRECC 포함)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Tokens saved</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Saving ratio</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>실측 데이터</td><td><span data-measured="ground_truth_count">--</span> 회 측정</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### 재작성 유형별

<table id="rewrite-type-table">
<thead><tr><th>유형</th><th>횟수</th><th>평균 절약 %</th><th>Tokens saved</th></tr></thead>
<tbody><tr><td colspan="4"><em>로딩 중...</em></td></tr></tbody>
</table>
</div>

### 릴리스별 절약

<table id="version-breakdown" style="display:none">
<thead><tr><th>버전</th><th>고유 사용자</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>로딩 중...</em></td></tr></tbody>
</table>

<small>These numbers update automatically from anonymized telemetry.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
