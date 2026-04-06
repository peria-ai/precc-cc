# Guida rapida

Inizia con PRECC in 5 minuti.

## Passo 1: Installazione

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Passo 2: Inizializzazione

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Passo 3: Verifica che l'hook sia attivo

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

## Passo 4: Usa Claude Code normalmente

Apri Claude Code e lavora come al solito. PRECC viene eseguito silenziosamente in background. Quando Claude esegue un comando che fallirebbe, PRECC lo corregge prima dell'esecuzione.

### Esempio: build Cargo nella directory errata

Supponiamo che il tuo progetto sia in `~/projects/myapp/` e Claude esegua:

```
cargo build
```

da `~/projects/` (un livello troppo in alto, nessun `Cargo.toml` presente).

**Senza PRECC:** Claude ottiene l'errore `could not find Cargo.toml in /home/user/projects or any parent directory`, lo legge, ragiona e ritenta con `cd myapp && cargo build`. Costo: ~2.000 token sprecati.

**Con PRECC:** L'hook rileva il `Cargo.toml` mancante, lo trova in `myapp/` e riscrive il comando in:

```
cd /home/user/projects/myapp && cargo build
```

Claude non vede mai un errore. Zero token sprecati.

## Passo 5: Controlla i tuoi risparmi

Dopo una sessione, vedi quanti token PRECC ha risparmiato:

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

## Prossimi passi

- [Skill](skills.md) -- Vedi tutte le skill disponibili e come creare le tue.
- [Hook Pipeline](hook-pipeline.md) -- Comprendi cosa succede sotto il cofano.
- [Risparmi](savings.md) -- Analisi dettagliata del risparmio di token.
