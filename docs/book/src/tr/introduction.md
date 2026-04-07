# Introduction

## What is PRECC?

PRECC (Claude Code için Tahminsel Hata Düzeltme) is a Rust tool that intercepts Claude Code bash commands via the official PreToolUse hook mechanism. It fixes errors *before they happen*, saving tokens and eliminating retry loops.

Topluluk kullanıcıları için ücretsiz.

## The Problem

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## The Four Pillars

### Bağlam Düzeltme (cd-prepend)

`cargo build` veya `npm test` gibi komutların yanlış dizinde çalıştığını algılar ve yürütmeden önce `cd /doğru/yol &&` ekler.

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

Bir komut sıkıştırmadan sonra başarısız olursa, PRECC bir sonraki denemede sıkıştırmayı otomatik olarak atlar ve Claude hata ayıklama için tam sıkıştırılmamış çıktıyı alır.

## Live Usage Statistics

Güncel sürüm <span data-stat="current_version">--</span>:

| Metric | Value |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Kullanıcılar | <span data-stat="unique_users">--</span> |

### Ölçülen Tasarruf (Gerçek Veri)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metric</th><th>Value</th></tr></thead>
<tbody>
<tr><td>Orijinal çıkış tokenleri (PRECC olmadan)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Gerçek çıkış tokenleri (PRECC ile)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Tokens saved</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Saving ratio</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Gerçek ölçümler</td><td><span data-measured="ground_truth_count">--</span> ölçüm</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### Yeniden Yazma Türüne Göre

<table id="rewrite-type-table">
<thead><tr><th>Tür</th><th>Sayı</th><th>Ort. Tasarruf %</th><th>Tokens saved</th></tr></thead>
<tbody><tr><td colspan="4"><em>Yükleniyor...</em></td></tr></tbody>
</table>
</div>

### Sürüme göre tasarruf

<table id="version-breakdown" style="display:none">
<thead><tr><th>Sürüm</th><th>Kullanıcılar</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Yükleniyor...</em></td></tr></tbody>
</table>

<small>These numbers update automatically from anonymized telemetry.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
