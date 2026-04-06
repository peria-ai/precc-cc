# Introduction

## What is PRECC?

PRECC (Correction prédictive des erreurs pour Claude Code) is a Rust tool that intercepts Claude Code bash commands via the official PreToolUse hook mechanism. It fixes errors *before they happen*, saving tokens and eliminating retry loops.

Gratuit pour les utilisateurs communautaires.

## The Problem

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## The Four Pillars

### Correction de contexte (cd-prepend)

Détecte quand des commandes comme `cargo build` ou `npm test` sont exécutées dans le mauvais répertoire et ajoute `cd /bon/chemin &&` avant l'exécution.

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

Si une commande échoue après compression, PRECC ignore automatiquement la compression lors de la prochaine tentative pour que Claude obtienne la sortie complète non compressée pour le débogage.

## Live Usage Statistics

Version actuelle <span data-stat="current_version">--</span>:

| Metric | Value |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Utilisateurs | <span data-stat="unique_users">--</span> |

### Économies par version

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Utilisateurs</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Chargement...</em></td></tr></tbody>
</table>

<small>Les chiffres sont des estimations. Chaque échec évité épargne un cycle complet de nouvelle tentative : sortie d'erreur, raisonnement du modèle et commande de nouvelle tentative. These numbers update automatically from anonymized telemetry.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
