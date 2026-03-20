#!/usr/bin/env node
/**
 * precc-ts-compress.js — Context file compressor for Claude Code
 *
 * Adapted from token-saver (MIT-0, by RubenAQuispe).
 * Strips filler words and verbose phrasing from CLAUDE.md and other
 * always-loaded context files, reducing token usage on every API call.
 *
 * Usage:
 *   node precc-ts-compress.js [--dry-run] [--revert] [dir]
 *
 * Targets: CLAUDE.md, .claude/memory/*.md, .claude/settings.json docs
 * Logs savings to ~/.precc/ts-metrics.jsonl
 */

const fs = require('fs');
const path = require('path');

// ── Compression patterns (from token-saver, adapted) ────────────────────────
const REPLACEMENTS = [
    // Filler removal
    [/\bplease\b/gi, ''],
    [/\bkindly\b/gi, ''],
    [/\bjust\b/gi, ''],
    [/\bsimply\b/gi, ''],
    [/\bbasically\b/gi, ''],
    [/\bactually\b/gi, ''],
    [/\bIn order to\b/gi, 'To'],
    [/\bdue to the fact that\b/gi, 'because'],
    [/\bat this point in time\b/gi, 'now'],
    [/\bin the event that\b/gi, 'if'],
    [/\bfor the purpose of\b/gi, 'to'],
    [/\bwith regard to\b/gi, 're:'],
    [/\bin terms of\b/gi, 're:'],
    [/\bIt is important to note that\b/gi, 'Note:'],
    [/\bIt should be noted that\b/gi, 'Note:'],
    [/\bAs mentioned (earlier|previously|above)\b/gi, ''],
    [/\bAs you (may |might )?know\b/gi, ''],

    // Action patterns
    [/\bBefore doing anything else\b/gi, 'First'],

    // Flow notation
    [/\bIf (.+?), then (.+?)\./g, '$1 → $2'],
    [/\bFirst,? (.+?)[,.]? [Tt]hen,? (.+?)[,.]? [Ff]inally,? (.+?)\./g, '$1 → $2 → $3'],
    [/\bstep (\d+)/gi, '$1)'],

    // Common phrases
    [/\byou should\b/gi, ''],
    [/\bmake sure (to |that )?/gi, 'ensure '],
    [/\bkeep in mind (that )?/gi, 'note: '],
    [/\bfor example/gi, 'e.g.'],
    [/\bsuch as\b/gi, 'e.g.'],
    [/\betc\.?\b/gi, '...'],
    [/\band so on\b/gi, '...'],
    [/\band others?\b/gi, '...'],
    [/\bincluding but not limited to\b/gi, 'incl.'],
    [/\bin other words\b/gi, 'i.e.'],
    [/\bthat is to say\b/gi, 'i.e.'],

    // Whitespace cleanup
    [/  +/g, ' '],
    [/\n +/g, '\n'],
    [/ +\n/g, '\n'],
    [/\n{3,}/g, '\n\n'],
];

// ── File discovery ──────────────────────────────────────────────────────────
function discoverFiles(projectDir) {
    const files = [];

    // CLAUDE.md at project root
    const claudeMd = path.join(projectDir, 'CLAUDE.md');
    if (fs.existsSync(claudeMd)) files.push(claudeMd);

    // .claude/CLAUDE.md
    const dotClaudeMd = path.join(projectDir, '.claude', 'CLAUDE.md');
    if (fs.existsSync(dotClaudeMd)) files.push(dotClaudeMd);

    // Memory files
    const memDir = path.join(projectDir, '.claude', 'memory');
    if (fs.existsSync(memDir)) {
        for (const entry of fs.readdirSync(memDir)) {
            if (entry.endsWith('.md') && !entry.endsWith('.backup')) {
                files.push(path.join(memDir, entry));
            }
        }
    }

    // Nested CLAUDE.md in subdirectories (1 level deep)
    try {
        for (const entry of fs.readdirSync(projectDir)) {
            if (entry.startsWith('.')) continue;
            const sub = path.join(projectDir, entry, 'CLAUDE.md');
            if (fs.existsSync(sub)) files.push(sub);
        }
    } catch (e) { /* skip */ }

    return files;
}

// ── Compression ─────────────────────────────────────────────────────────────
function compress(content) {
    let result = content;
    for (const [pattern, replacement] of REPLACEMENTS) {
        result = result.replace(pattern, replacement);
    }
    return result.replace(/  +/g, ' ').replace(/\n{3,}/g, '\n\n').trim() + '\n';
}

function estimateTokens(text) {
    return Math.round(text.length / 4);
}

// ── Metrics logging ─────────────────────────────────────────────────────────
const METRICS_FILE = path.join(process.env.HOME || '', '.precc', 'ts-metrics.jsonl');

function logMetrics(results) {
    const dir = path.dirname(METRICS_FILE);
    if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

    const entry = {
        ts: new Date().toISOString(),
        files: results.length,
        total_original_tokens: results.reduce((s, r) => s + r.originalTokens, 0),
        total_compressed_tokens: results.reduce((s, r) => s + r.compressedTokens, 0),
        total_saved_tokens: results.reduce((s, r) => s + r.savedTokens, 0),
        details: results.map(r => ({
            file: path.basename(r.file),
            saved: r.savedTokens,
            pct: r.pctSaved
        }))
    };
    fs.appendFileSync(METRICS_FILE, JSON.stringify(entry) + '\n');
}

// ── Main ────────────────────────────────────────────────────────────────────
function main() {
    const args = process.argv.slice(2);
    const dryRun = args.includes('--dry-run');
    const revert = args.includes('--revert');
    const dir = args.find(a => !a.startsWith('--')) || process.cwd();

    const files = discoverFiles(dir);

    if (files.length === 0) {
        console.log('No context files found (CLAUDE.md, .claude/memory/*.md).');
        process.exit(0);
    }

    if (revert) {
        let reverted = 0;
        for (const file of files) {
            const backup = file + '.backup';
            if (fs.existsSync(backup)) {
                fs.copyFileSync(backup, file);
                fs.unlinkSync(backup);
                console.log(`  Reverted: ${path.relative(dir, file)}`);
                reverted++;
            }
        }
        console.log(reverted ? `\nReverted ${reverted} file(s).` : 'No backups found.');
        process.exit(0);
    }

    console.log(`Compressing ${files.length} context file(s)...\n`);

    const results = [];
    for (const file of files) {
        const original = fs.readFileSync(file, 'utf8');
        const compressed = compress(original);
        const origTokens = estimateTokens(original);
        const compTokens = estimateTokens(compressed);
        const saved = origTokens - compTokens;
        const pct = origTokens > 0 ? Math.round((saved / origTokens) * 100) : 0;

        if (pct < 5) {
            console.log(`  ${path.relative(dir, file)}: already compact (${pct}% savings) — skipped`);
            continue;
        }

        const rel = path.relative(dir, file);
        console.log(`  ${rel}: ${origTokens} → ${compTokens} tokens (saved ${saved}, ${pct}%)`);

        if (!dryRun) {
            // Create backup if none exists
            const backup = file + '.backup';
            if (!fs.existsSync(backup)) {
                fs.copyFileSync(file, backup);
            }
            fs.writeFileSync(file, compressed);
        }

        results.push({
            file,
            originalTokens: origTokens,
            compressedTokens: compTokens,
            savedTokens: saved,
            pctSaved: pct
        });
    }

    if (results.length === 0) {
        console.log('\nAll files already compact. Nothing to do.');
        process.exit(0);
    }

    const totalSaved = results.reduce((s, r) => s + r.savedTokens, 0);
    const totalOrig = results.reduce((s, r) => s + r.originalTokens, 0);

    console.log(`\nTotal: ${totalSaved} tokens saved (${Math.round(totalSaved / totalOrig * 100)}%)`);

    if (dryRun) {
        console.log('(dry run — no files modified)');
    } else {
        logMetrics(results);
        console.log('Backups saved as *.backup. Revert with: precc-ts-compress.js --revert');
    }
}

main();
