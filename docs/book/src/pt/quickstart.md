# Início rápido

Coloque o PRECC em funcionamento em 5 minutos.

## Passo 1: Instalar

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Passo 2: Inicializar

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Passo 3: Verificar se o hook está ativo

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

## Passo 4: Usar o Claude Code normalmente

Abra o Claude Code e trabalhe normalmente. O PRECC roda silenciosamente em segundo plano. Quando o Claude emite um comando que falharia, o PRECC o corrige antes da execução.

### Exemplo: Cargo Build no diretório errado

Suponha que seu projeto está em `~/projects/myapp/` e o Claude emite:

```
cargo build
```

a partir de `~/projects/` (um nível acima, sem `Cargo.toml` lá).

**Sem PRECC:** Claude recebe o erro `could not find Cargo.toml in /home/user/projects or any parent directory`, lê, raciocina e tenta novamente com `cd myapp && cargo build`. Custo: ~2.000 tokens desperdiçados.

**Com PRECC:** O hook detecta o `Cargo.toml` ausente, encontra-o em `myapp/` e reescreve o comando para:

```
cd /home/user/projects/myapp && cargo build
```

Claude nunca vê um erro. Zero tokens desperdiçados.

## Passo 5: Verifique suas economias

Após uma sessão, veja quantos tokens o PRECC economizou:

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

## Próximos passos

- [Habilidades](skills.md) -- Veja todas as habilidades disponíveis e como criar as suas.
- [Pipeline do Hook](hook-pipeline.md) -- Entenda o que acontece por baixo dos panos.
- [Economias](savings.md) -- Análise detalhada de economia de tokens.
