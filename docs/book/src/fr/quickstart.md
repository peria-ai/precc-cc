# Démarrage rapide

Lancez PRECC en 5 minutes.

## Étape 1 : Installation

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Étape 2 : Initialisation

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Étape 3 : Vérifier que le hook est actif

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

## Étape 4 : Utiliser Claude Code normalement

Ouvrez Claude Code et travaillez normalement. PRECC s'exécute silencieusement en arrière-plan. Quand Claude émet une commande qui échouerait, PRECC la corrige avant l'exécution.

### Exemple : Cargo Build dans le mauvais répertoire

Supposons que votre projet est dans `~/projects/myapp/` et Claude exécute :

```
cargo build
```

depuis `~/projects/` (un niveau trop haut, pas de `Cargo.toml` là).

**Sans PRECC :** Claude reçoit l'erreur `could not find Cargo.toml in /home/user/projects or any parent directory`, la lit, raisonne, puis réessaie avec `cd myapp && cargo build`. Coût : ~2 000 tokens gaspillés.

**Avec PRECC :** Le hook détecte le `Cargo.toml` manquant, le trouve dans `myapp/` et réécrit la commande en :

```
cd /home/user/projects/myapp && cargo build
```

Claude ne voit jamais d'erreur. Zéro token gaspillé.

## Étape 5 : Vérifier vos économies

Après une session, voyez combien de tokens PRECC a économisé :

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

## Étapes suivantes

- [Compétences](skills.md) -- Voir toutes les compétences disponibles et comment créer les vôtres.
- [Pipeline du hook](hook-pipeline.md) -- Comprenez ce qui se passe sous le capot.
- [Économies](savings.md) -- Analyse détaillée des économies de tokens.
