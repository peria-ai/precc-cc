# Introduction

## What is PRECC?

PRECC (Sửa lỗi dự đoán cho Claude Code) is a Rust tool that intercepts Claude Code bash commands via the official PreToolUse hook mechanism. It fixes errors *before they happen*, saving tokens and eliminating retry loops.

Miễn phí cho người dùng cộng đồng.

## The Problem

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## The Four Pillars

### Sửa ngữ cảnh (cd-prepend)

Phát hiện khi các lệnh như `cargo build` hoặc `npm test` chạy trong thư mục sai và thêm `cd /đường/dẫn/đúng &&` trước khi thực thi.

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

Nếu một lệnh thất bại sau khi nén, PRECC tự động bỏ qua nén ở lần thử tiếp theo để Claude nhận được đầu ra đầy đủ không nén để gỡ lỗi.

## Live Usage Statistics

Phiên bản hiện tại <span data-stat="current_version">--</span>:

| Metric | Value |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Người dùng | <span data-stat="unique_users">--</span> |

### Tiết kiệm đo được (dữ liệu thực)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metric</th><th>Value</th></tr></thead>
<tbody>
<tr><td>Token đầu ra gốc (không có PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Token đầu ra thực tế (có PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Tokens saved</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Saving ratio</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Đo lường thực tế</td><td><span data-measured="ground_truth_count">--</span> lần đo</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### Theo loại viết lại

<table id="rewrite-type-table">
<thead><tr><th>Loại</th><th>Số lần</th><th>TB tiết kiệm %</th><th>Tokens saved</th></tr></thead>
<tbody><tr><td colspan="4"><em>Đang tải...</em></td></tr></tbody>
</table>
</div>

### Tiết kiệm theo phiên bản

<table id="version-breakdown" style="display:none">
<thead><tr><th>Phiên bản</th><th>Người dùng</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Đang tải...</em></td></tr></tbody>
</table>

<small>These numbers update automatically from anonymized telemetry.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
