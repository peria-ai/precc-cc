# Inicio rápido

Pon PRECC en marcha en 5 minutos.

## Paso 1: Instalar

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Paso 2: Inicializar

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Paso 3: Verificar que el hook esté activo

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

## Paso 4: Usar Claude Code normalmente

Abre Claude Code y trabaja como siempre. PRECC se ejecuta silenciosamente en segundo plano. Cuando Claude emite un comando que fallaría, PRECC lo corrige antes de la ejecución.

### Ejemplo: Cargo Build en directorio incorrecto

Supón que tu proyecto está en `~/projects/myapp/` y Claude ejecuta:

```
cargo build
```

desde `~/projects/` (un nivel demasiado alto, sin `Cargo.toml` allí).

**Sin PRECC:** Claude recibe el error `could not find Cargo.toml in /home/user/projects or any parent directory`, lo lee, razona y reintenta con `cd myapp && cargo build`. Costo: ~2.000 tokens desperdiciados.

**Con PRECC:** El hook detecta el `Cargo.toml` faltante, lo encuentra en `myapp/` y reescribe el comando a:

```
cd /home/user/projects/myapp && cargo build
```

Claude nunca ve un error. Cero tokens desperdiciados.

## Paso 5: Comprueba tus ahorros

Después de una sesión, mira cuántos tokens ahorró PRECC:

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

## Próximos pasos

- [Habilidades](skills.md) -- Ver todas las habilidades disponibles y cómo crear las tuyas.
- [Pipeline del Hook](hook-pipeline.md) -- Entiende qué sucede bajo el capó.
- [Ahorros](savings.md) -- Análisis detallado de ahorro de tokens.
