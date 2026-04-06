# 安裝

## 快速安裝 (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

這會下載適用於您平臺的最新版本二進制文件，驗證SHA256校驗和，並將其放置在 `~/.local/bin/` 中。

安裝後，初始化PRECC：

```bash
precc init
```

`precc init` 在Claude Code中註冊PreToolUse鉤子，創建數據目錄，並初始化技能數據庫。

## 安裝選項

### SHA256驗證

默認情況下，安裝程序會根據已發佈的SHA256校驗和驗證二進制文件。要跳過驗證（不推薦）：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### 自定義安裝前綴

安裝到自定義位置：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### 附加工具 (--extras)

PRECC附帶可選的附加工具。使用 `--extras` 安裝它們：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

這將安裝：

| 工具 | 用途 |
|------|---------|
| **RTK** | 命令重寫工具包 |
| **lean-ctx** | CLAUDE.md和提示文件的上下文壓縮 |
| **nushell** | 用於高級管道的結構化Shell |
| **cocoindex-code** | 代碼索引以加快上下文解析 |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

然後初始化：

```powershell
precc init
```

## 手動安裝

1. 從 [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) 下載適用於您平臺的發佈二進制文件。
2. 根據版本中的 `.sha256` 文件驗證SHA256校驗和。
3. 將二進制文件放置在 `PATH` 中的目錄中（例如 `~/.local/bin/`）。
4. 運行 `precc init`。

## 更新

```bash
precc update
```

強制更新到特定版本：

```bash
precc update --force --version 0.3.0
```

啓用自動更新：

```bash
precc update --auto
```

## 驗證安裝

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

如果找不到 `precc`，請確保 `~/.local/bin` 在您的 `PATH` 中。
