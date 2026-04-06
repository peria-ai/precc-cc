# Relatórios

`precc report` gera um painel analítico resumindo a atividade do PRECC e a economia de tokens.

## Gerando um relatório

```bash
$ precc report
PRECC Report -- 2026-04-03
==========================

Sessions analyzed: 12
Commands intercepted: 87
Total token savings: 42,389

Top skills by activation:
  1. cargo-wrong-dir     34 activations   17,204 tokens saved
  2. npm-wrong-dir       18 activations    9,360 tokens saved
  3. git-wrong-dir       12 activations    4,944 tokens saved
  4. RTK rewrite         15 activations    3,750 tokens saved
  5. python-wrong-dir     8 activations    4,131 tokens saved

Savings by pillar:
  Pillar 1 (context resolution):  28,639 tokens  67.6%
  Pillar 4 (automation skills):    7,000 tokens  16.5%
  RTK rewrites:                    3,750 tokens   8.8%
  Lean-ctx wraps:                  3,000 tokens   7.1%

Recent corrections:
  2026-04-03 09:12  cargo build -> cd myapp && cargo build
  2026-04-03 09:18  npm test -> cd frontend && npm test
  2026-04-03 10:05  git status -> cd repo && git status
  ...
```

## Enviando um relatório por e-mail

Envie o relatório para um endereço de e-mail (requer configuração de e-mail, veja [Email](email.md)):

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

O endereço do destinatário é lido de `~/.config/precc/mail.toml`. Você também pode usar `precc mail report EMAIL` para enviar para um endereço específico.

## Dados do relatório

Relatórios são gerados a partir do banco de dados PRECC local em `~/.local/share/precc/history.db`. Nenhum dado sai da sua máquina a menos que você envie o relatório por e-mail explicitamente.
