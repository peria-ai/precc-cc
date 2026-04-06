# Ghid rapid

Puneți PRECC în funcțiune în 5 minute.

## Pasul 1: Instalare

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Pasul 2: Inițializare

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Pasul 3: Verificați că hook-ul este activ

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

## Pasul 4: Folosiți Claude Code normal

Deschideți Claude Code și lucrați ca de obicei. PRECC rulează silențios în fundal. Când Claude emite o comandă care ar eșua, PRECC o corectează înainte de execuție.

### Exemplu: Build Cargo în directorul greșit

Presupuneți că proiectul dvs. este la `~/projects/myapp/` și Claude emite:

```
cargo build
```

din `~/projects/` (un nivel prea sus, fără `Cargo.toml` acolo).

**Fără PRECC:** Claude primește eroarea `could not find Cargo.toml in /home/user/projects or any parent directory`, o citește, raționează și reîncearcă cu `cd myapp && cargo build`. Cost: ~2.000 tokeni irosiți.

**Cu PRECC:** Hook-ul detectează `Cargo.toml` lipsă, îl găsește în `myapp/` și rescrie comanda în:

```
cd /home/user/projects/myapp && cargo build
```

Claude nu vede niciodată o eroare. Zero tokeni irosiți.

## Pasul 5: Verificați economiile

După o sesiune, vedeți câți tokeni a economisit PRECC:

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

## Pași următori

- [Abilități](skills.md) -- Vedeți toate abilitățile disponibile și cum să vă creați propriile.
- [Hook Pipeline](hook-pipeline.md) -- Înțelegeți ce se întâmplă în culise.
- [Economii](savings.md) -- Analiză detaliată a economiilor de tokeni.
