# Report

`precc report` genera una dashboard analitica che riassume l'attività PRECC e il risparmio di token.

## Generazione di un report

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

## Invio di un report via email

Invia il report a un indirizzo email (richiede configurazione mail, vedi [Email](email.md)):

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

L'indirizzo destinatario viene letto da `~/.config/precc/mail.toml`. Puoi anche usare `precc mail report EMAIL` per inviare a un indirizzo specifico.

## Dati del report

I report vengono generati dal database PRECC locale in `~/.local/share/precc/history.db`. Nessun dato lascia la tua macchina a meno che tu non invii esplicitamente il report via email.
