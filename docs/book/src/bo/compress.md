# Compress

`precc compress` shrinks CLAUDE.md and other context files to reduce token usage when Claude Code loads them. This is a Pro feature.

## Basic Usage

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## Dry Run

Preview what would change without modifying files:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Reverting

Originals are backed up automatically. To restore them:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## What Gets Compressed

The compressor applies several transformations:

- Removes redundant whitespace and blank lines
- Shortens verbose phrasing while preserving meaning
- Condenses tables and lists
- Strips comments and decorative formatting
- Preserves all code blocks, paths, and technical identifiers

The compressed output is still human-readable -- it is not minified or obfuscated.

## Targeting Specific Files

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
