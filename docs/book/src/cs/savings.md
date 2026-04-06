# Úspory

PRECC sleduje odhadované úspory tokenů z každého zachycení. Použijte `precc savings` pro zobrazení, kolik plýtvání PRECC zabránil.

## Rychlý souhrn

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Breakdown:
  Pillar 1 (cd prepends):         <span data-stat="session_p1_tokens">3,204</span> tokens  (<span data-stat="session_p1_count">6</span> corrections)
  Pillar 4 (skill activations):   <span data-stat="session_p4_tokens">1,560</span> tokens  (<span data-stat="session_p4_count">4</span> activations)
  RTK rewrites:                   <span data-stat="session_rtk_tokens">2,749</span> tokens  (<span data-stat="session_rtk_count">11</span> rewrites)
  Lean-ctx wraps:                 <span data-stat="session_lean_tokens">1,228</span> tokens  (<span data-stat="session_lean_count">2</span> wraps)
```

## Podrobný rozpis (Pro)

```bash
$ precc savings --all
Session Token Savings (Detailed)
================================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Command-by-command:
  #  Time   Command                          Saving   Source
  1  09:12  cargo build                      534 tk   cd prepend (cargo-wrong-dir)
  2  09:14  cargo test                       534 tk   cd prepend (cargo-wrong-dir)
  3  09:15  git status                       412 tk   cd prepend (git-wrong-dir)
  4  09:18  npm install                      824 tk   cd prepend (npm-wrong-dir)
  5  09:22  find . -name "*.rs"              387 tk   RTK rewrite (output truncation)
  6  09:25  cat src/main.rs                  249 tk   RTK rewrite (lean-ctx wrap)
  7  09:31  cargo clippy                     534 tk   cd prepend (cargo-wrong-dir)
  ...

Pillar Breakdown:
  Pillar 1 (context resolution):   <span data-stat="session_p1_tokens">3,204</span> tokens  <span data-stat="session_p1_pct">36.6</span>%
  Pillar 2 (GDB debugging):            0 tokens   0.0%
  Pillar 3 (mined preventions):        0 tokens   0.0%
  Pillar 4 (automation skills):    <span data-stat="session_p4_tokens">1,560</span> tokens  <span data-stat="session_p4_pct">17.8</span>%
  RTK rewrites:                    <span data-stat="session_rtk_tokens">2,749</span> tokens  <span data-stat="session_rtk_pct">31.5</span>%
  Lean-ctx wraps:                  <span data-stat="session_lean_tokens">1,228</span> tokens  <span data-stat="session_lean_pct">14.1</span>%
```

## Jak se odhadují úspory

Každý typ korekce má odhadovanou cenu v tokenech na základě toho, co by se stalo bez PRECC:

| Typ korekce | Odhadované úspory | Odůvodnění |
|----------------|-----------------|-----------|
| cd prepend | ~500 tokens | Chybový výstup + Claudovo uvažování + opakování |
| Aktivace dovednosti | ~400 tokens | Chybový výstup + Claudovo uvažování + opakování |
| RTK rewrite | ~250 tokens | Rozvláčný výstup, který by Claude musel přečíst |
| Lean-ctx wrap | ~600 tokens | Obsah velkých souborů komprimován |
| Naučená prevence | ~500 tokens | Známý vzor chyby se vyhnul |

Jedná se o konzervativní odhady. Skutečné úspory jsou často vyšší, protože Claudovo uvažování o chybách může být rozvláčné.

## Kumulativní úspory

Úspory přetrvávají napříč relacemi v databázi PRECC. Postupem času můžete sledovat celkový dopad:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Lifetime savings: <span data-stat="total_tokens_saved">142,389</span> tokens across <span data-stat="total_sessions">47</span> sessions
```
