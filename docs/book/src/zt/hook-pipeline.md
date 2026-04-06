# 鉤子管道

`precc-hook` 二進制文件是PRECC的核心。它位於Claude Code和shell之間，在5毫秒內處理每個bash命令。

## Claude Code如何調用鉤子

Claude Code支持PreToolUse鉤子——可以在執行前檢查和修改工具輸入的外部程序。當Claude即將運行bash命令時，它通過stdin將JSON發送給 `precc-hook` 並從stdout讀取響應。

## 管道階段

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## 示例：JSON輸入和輸出

### 輸入（來自Claude Code）

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC檢測到當前目錄沒有 `Cargo.toml`，但 `./myapp/Cargo.toml` 存在。

### 輸出（到Claude Code）

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

如果不需要修改，`updatedInput.command` 爲空，Claude Code使用原始命令。

## 階段詳情

### 階段1：解析JSON

從stdin讀取完整的JSON對象。提取 `tool_input.command`。如果解析失敗，鉤子立即退出，Claude Code使用原始命令（fail-open設計）。

### 階段2：技能匹配

查詢SQLite啓發式數據庫，尋找觸發模式與命令匹配的技能。技能按優先級順序檢查。內置TOML技能和挖掘的技能都會被評估。

### 階段3：目錄修正

對於構建命令（`cargo`、`go`、`make`、`npm`、`python` 等），檢查預期的項目文件是否存在於當前目錄中。如果不存在，掃描附近目錄尋找最近匹配並添加 `cd <dir> &&` 前綴。

目錄掃描使用緩存的文件系統索引，TTL爲5秒，以保持高速。

### 階段4：GDB檢查

如果命令可能產生崩潰（例如運行調試二進制文件），PRECC可以建議或注入GDB包裝器來捕獲結構化的調試輸出，而不是原始崩潰日誌。

### 階段5：RTK重寫

應用RTK（重寫工具包）規則，縮短冗長命令、抑制嘈雜輸出或重構命令以提高token效率。

### 階段6：輸出JSON

將修改後的命令序列化回JSON並寫入stdout。如果沒有更改，輸出信號Claude Code使用原始命令。

## 性能

整個管道在5毫秒（p99）內完成。關鍵優化：

- SQLite使用WAL模式實現無鎖併發讀取
- 預編譯的正則表達式模式用於技能匹配
- 緩存的文件系統掃描（5秒TTL）
- 熱路徑中無網絡調用
- Fail-open：任何錯誤都回退到原始命令

## 手動測試鉤子

你可以直接調用鉤子：

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
