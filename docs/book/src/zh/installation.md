# 安装

## 快速安装 (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

这会下载适用于您平台的最新版本二进制文件，验证SHA256校验和，并将其放置在 `~/.local/bin/` 中。

安装后，初始化PRECC：

```bash
precc init
```

`precc init` 在Claude Code中注册PreToolUse钩子，创建数据目录，并初始化技能数据库。

## 安装选项

### SHA256验证

默认情况下，安装程序会根据已发布的SHA256校验和验证二进制文件。要跳过验证（不推荐）：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### 自定义安装前缀

安装到自定义位置：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### 附加工具 (--extras)

PRECC附带可选的附加工具。使用 `--extras` 安装它们：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

这将安装：

| 工具 | 用途 |
|------|---------|
| **RTK** | 命令重写工具包 |
| **lean-ctx** | CLAUDE.md和提示文件的上下文压缩 |
| **nushell** | 用于高级管道的结构化Shell |
| **cocoindex-code** | 代码索引以加快上下文解析 |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

然后初始化：

```powershell
precc init
```

## 手动安装

1. 从 [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) 下载适用于您平台的发布二进制文件。
2. 根据版本中的 `.sha256` 文件验证SHA256校验和。
3. 将二进制文件放置在 `PATH` 中的目录中（例如 `~/.local/bin/`）。
4. 运行 `precc init`。

## 更新

```bash
precc update
```

强制更新到特定版本：

```bash
precc update --force --version 0.3.0
```

启用自动更新：

```bash
precc update --auto
```

## 验证安装

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

如果找不到 `precc`，请确保 `~/.local/bin` 在您的 `PATH` 中。
