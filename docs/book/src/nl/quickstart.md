# Snelstart

PRECC in 5 minuten operationeel.

## Stap 1: Installeren

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Stap 2: Initialiseren

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Stap 3: Controleer of de hook actief is

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

## Stap 4: Gebruik Claude Code normaal

Open Claude Code en werk zoals gewoonlijk. PRECC draait stil op de achtergrond. Wanneer Claude een commando geeft dat zou mislukken, corrigeert PRECC het vóór uitvoering.

### Voorbeeld: Cargo Build in verkeerde map

Stel dat je project in `~/projects/myapp/` staat en Claude uitvoert:

```
cargo build
```

vanuit `~/projects/` (één niveau te hoog, geen `Cargo.toml` daar).

**Zonder PRECC:** Claude krijgt de fout `could not find Cargo.toml in /home/user/projects or any parent directory`, leest deze, redeneert erover en probeert opnieuw met `cd myapp && cargo build`. Kosten: ~2.000 tokens verspild.

**Met PRECC:** De hook detecteert het ontbrekende `Cargo.toml`, vindt het in `myapp/` en herschrijft het commando naar:

```
cd /home/user/projects/myapp && cargo build
```

Claude ziet nooit een fout. Nul tokens verspild.

## Stap 5: Controleer je besparingen

Bekijk na een sessie hoeveel tokens PRECC heeft bespaard:

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

## Volgende stappen

- [Vaardigheden](skills.md) -- Bekijk alle beschikbare vaardigheden en hoe je je eigen kunt maken.
- [Hook Pipeline](hook-pipeline.md) -- Begrijp wat er onder de motorkap gebeurt.
- [Besparingen](savings.md) -- Gedetailleerde analyse van tokenbesparingen.
