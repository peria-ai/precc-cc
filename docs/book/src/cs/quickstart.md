# Rychlý start

Zprovozněte PRECC za 5 minut.

## Krok 1: Instalace

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Krok 2: Inicializace

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Krok 3: Ověřte, že hook je aktivní

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

## Krok 4: Používejte Claude Code normálně

Otevřete Claude Code a pracujte jako obvykle. PRECC běží tiše na pozadí. Když Claude vydá příkaz, který by selhal, PRECC ho opraví před spuštěním.

### Příklad: Cargo build ve špatném adresáři

Předpokládejme, že váš projekt je v `~/projects/myapp/` a Claude vydá:

```
cargo build
```

z `~/projects/` (o úroveň výše, žádný `Cargo.toml` tam).

**Bez PRECC:** Claude dostane chybu `could not find Cargo.toml in /home/user/projects or any parent directory`, přečte ji, uvažuje a opakuje s `cd myapp && cargo build`. Cena: ~2 000 zbytečných tokenů.

**S PRECC:** Hook detekuje chybějící `Cargo.toml`, najde ho v `myapp/` a přepíše příkaz na:

```
cd /home/user/projects/myapp && cargo build
```

Claude nikdy nevidí chybu. Nula zbytečných tokenů.

## Krok 5: Zkontrolujte úspory

Po relaci se podívejte, kolik tokenů PRECC ušetřil:

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

## Další kroky

- [Dovednosti](skills.md) -- Zobrazit všechny dostupné dovednosti a jak vytvořit vlastní.
- [Hook Pipeline](hook-pipeline.md) -- Pochopte, co se děje pod kapotou.
- [Úspory](savings.md) -- Podrobná analýza úspor tokenů.
