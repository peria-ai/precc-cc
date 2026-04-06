# Schnellstart

PRECC in 5 Minuten zum Laufen bringen.

## Schritt 1: Installation

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Schritt 2: Initialisierung

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Schritt 3: Überprüfen, ob der Hook aktiv ist

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

## Schritt 4: Claude Code normal verwenden

Öffne Claude Code und arbeite wie gewohnt. PRECC läuft still im Hintergrund. Wenn Claude einen Befehl ausgibt, der fehlschlagen würde, korrigiert PRECC ihn vor der Ausführung.

### Beispiel: Cargo Build im falschen Verzeichnis

Angenommen, dein Projekt ist unter `~/projects/myapp/` und Claude gibt aus:

```
cargo build
```

von `~/projects/` aus (eine Ebene zu hoch, kein `Cargo.toml` dort).

**Ohne PRECC:** Claude erhält den Fehler `could not find Cargo.toml in /home/user/projects or any parent directory`, liest ihn, denkt darüber nach und versucht es mit `cd myapp && cargo build` erneut. Kosten: ~2.000 Token verschwendet.

**Mit PRECC:** Der Hook erkennt das fehlende `Cargo.toml`, findet es in `myapp/` und schreibt den Befehl um zu:

```
cd /home/user/projects/myapp && cargo build
```

Claude sieht nie einen Fehler. Null Token verschwendet.

## Schritt 5: Ersparnisse überprüfen

Überprüfe nach einer Sitzung, wie viele Token PRECC gespart hat:

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

## Nächste Schritte

- [Skills](skills.md) -- Alle verfügbaren Skills und wie du eigene erstellen kannst.
- [Hook-Pipeline](hook-pipeline.md) -- Verstehe, was unter der Haube passiert.
- [Ersparnisse](savings.md) -- Detaillierte Analyse der Token-Einsparungen.
