# 快速入門

5分鐘內啓動PRECC。

## 步驟1：安裝

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## 步驟2：初始化

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## 步驟3：驗證Hook已激活

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

## 步驟4：正常使用Claude Code

打開Claude Code並照常工作。PRECC在後臺靜默運行。當Claude發出一個會失敗的命令時，PRECC會在執行前修正它。

### 示例：錯誤目錄的Cargo Build

假設你的項目在 `~/projects/myapp/`，Claude發出：

```
cargo build
```

從 `~/projects/`（高了一級，那裏沒有 `Cargo.toml`）。

**沒有PRECC：** Claude收到錯誤 `could not find Cargo.toml in /home/user/projects or any parent directory`，讀取、推理，然後用 `cd myapp && cargo build` 重試。代價：浪費約2,000個token。

**使用PRECC：** Hook檢測到缺失的 `Cargo.toml`，在 `myapp/` 中找到它，並將命令重寫爲：

```
cd /home/user/projects/myapp && cargo build
```

Claude永遠看不到錯誤。零token浪費。

## 步驟5：查看節省情況

會話結束後，查看PRECC節省了多少token：

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

## 後續步驟

- [技能](skills.md) -- 查看所有可用技能以及如何創建自己的技能。
- [Hook管道](hook-pipeline.md) -- 瞭解底層發生了什麼。
- [節省](savings.md) -- 詳細的token節省分析。
