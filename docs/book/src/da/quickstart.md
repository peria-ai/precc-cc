# Hurtigstart

Kom i gang med PRECC på 5 minutter.

## Trin 1: Installer

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Trin 2: Initialiser

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Trin 3: Verificer at hook er aktiv

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## Trin 4: Brug Claude Code normalt

Åbn Claude Code og arbejd som normalt. PRECC kører stille i baggrunden. Når Claude udfører en kommando der ville fejle, retter PRECC den før udførelse.

### Eksempel: Cargo-build i forkert mappe

Antag at dit projekt er i `~/projects/myapp/` og Claude udfører:

```
cargo build
```

fra `~/projects/` (et niveau for højt, ingen `Cargo.toml` der).

**Uden PRECC:** Claude får fejlen `could not find Cargo.toml in /home/user/projects or any parent directory`, læser den, ræsonnerer og prøver igen med `cd myapp && cargo build`. Pris: ~2.000 tokens spildt.

**Med PRECC:** Hook registrerer den manglende `Cargo.toml`, finder den i `myapp/` og omskriver kommandoen til:

```
cd /home/user/projects/myapp && cargo build
```

Claude ser aldrig en fejl. Nul tokens spildt.

## Trin 5: Tjek dine besparelser

Efter en session, se hvor mange tokens PRECC har sparet:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## Næste skridt

- [Færdigheder](skills.md) -- Se alle tilgængelige færdigheder og hvordan du opretter dine egne.
- [Hook Pipeline](hook-pipeline.md) -- Forstå hvad der sker under motorhjelmen.
- [Besparelser](savings.md) -- Detaljeret tokenbesparelsesanalyse.
