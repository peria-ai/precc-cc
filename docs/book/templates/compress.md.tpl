# {{i18n:cmp_title}}

{{i18n:cmp_intro}}

## {{i18n:cmp_basic_title}}

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

## {{i18n:cmp_dryrun_title}}

{{i18n:cmp_dryrun_body}}

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## {{i18n:cmp_revert_title}}

{{i18n:cmp_revert_body}}

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## {{i18n:cmp_what_title}}

{{i18n:cmp_what_body}}

{{i18n:cmp_readable_note}}

## {{i18n:cmp_target_title}}

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
