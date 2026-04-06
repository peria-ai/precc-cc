# 节省

PRECC追踪每次拦截的估计token节省。使用 `precc savings` 查看PRECC阻止了多少浪费。

## 快速摘要

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

## 详细分类（Pro）

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

## 如何估算节省

每种修正类型都有基于没有PRECC时会发生什么的估计token成本：

| 修正类型 | 估计节省 | 原因 |
|----------------|-----------------|-----------|
| cd prepend | ~500 tokens | 错误输出 + Claude推理 + 重试 |
| 技能激活 | ~400 tokens | 错误输出 + Claude推理 + 重试 |
| RTK rewrite | ~250 tokens | Claude需要阅读的冗长输出 |
| Lean-ctx wrap | ~600 tokens | 大文件内容被压缩 |
| 挖掘预防 | ~500 tokens | 已知的失败模式被避免 |

这些是保守估计。实际节省通常更高，因为Claude对错误的推理可能很冗长。

## 累计节省

节省数据在PRECC数据库中跨会话持久化。随着时间推移，您可以跟踪总体影响：

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Lifetime savings: <span data-stat="total_tokens_saved">142,389</span> tokens across <span data-stat="total_sessions">47</span> sessions
```
