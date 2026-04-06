# 快速入门

5分钟内启动PRECC。

## 步骤1：安装

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## 步骤2：初始化

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## 步骤3：验证Hook已激活

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

## 步骤4：正常使用Claude Code

打开Claude Code并照常工作。PRECC在后台静默运行。当Claude发出一个会失败的命令时，PRECC会在执行前修正它。

### 示例：错误目录的Cargo Build

假设你的项目在 `~/projects/myapp/`，Claude发出：

```
cargo build
```

从 `~/projects/`（高了一级，那里没有 `Cargo.toml`）。

**没有PRECC：** Claude收到错误 `could not find Cargo.toml in /home/user/projects or any parent directory`，读取、推理，然后用 `cd myapp && cargo build` 重试。代价：浪费约2,000个token。

**使用PRECC：** Hook检测到缺失的 `Cargo.toml`，在 `myapp/` 中找到它，并将命令重写为：

```
cd /home/user/projects/myapp && cargo build
```

Claude永远看不到错误。零token浪费。

## 步骤5：查看节省情况

会话结束后，查看PRECC节省了多少token：

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

## 后续步骤

- [技能](skills.md) -- 查看所有可用技能以及如何创建自己的技能。
- [Hook管道](hook-pipeline.md) -- 了解底层发生了什么。
- [节省](savings.md) -- 详细的token节省分析。
