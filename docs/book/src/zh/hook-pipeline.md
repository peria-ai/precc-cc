# 钩子管道

`precc-hook` 二进制文件是PRECC的核心。它位于Claude Code和shell之间，在5毫秒内处理每个bash命令。

## Claude Code如何调用钩子

Claude Code支持PreToolUse钩子——可以在执行前检查和修改工具输入的外部程序。当Claude即将运行bash命令时，它通过stdin将JSON发送给 `precc-hook` 并从stdout读取响应。

## 管道阶段

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

## 示例：JSON输入和输出

### 输入（来自Claude Code）

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC检测到当前目录没有 `Cargo.toml`，但 `./myapp/Cargo.toml` 存在。

### 输出（到Claude Code）

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

如果不需要修改，`updatedInput.command` 为空，Claude Code使用原始命令。

## 阶段详情

### 阶段1：解析JSON

从stdin读取完整的JSON对象。提取 `tool_input.command`。如果解析失败，钩子立即退出，Claude Code使用原始命令（fail-open设计）。

### 阶段2：技能匹配

查询SQLite启发式数据库，寻找触发模式与命令匹配的技能。技能按优先级顺序检查。内置TOML技能和挖掘的技能都会被评估。

### 阶段3：目录修正

对于构建命令（`cargo`、`go`、`make`、`npm`、`python` 等），检查预期的项目文件是否存在于当前目录中。如果不存在，扫描附近目录寻找最近匹配并添加 `cd <dir> &&` 前缀。

目录扫描使用缓存的文件系统索引，TTL为5秒，以保持高速。

### 阶段4：GDB检查

如果命令可能产生崩溃（例如运行调试二进制文件），PRECC可以建议或注入GDB包装器来捕获结构化的调试输出，而不是原始崩溃日志。

### 阶段5：RTK重写

应用RTK（重写工具包）规则，缩短冗长命令、抑制嘈杂输出或重构命令以提高token效率。

### 阶段6：输出JSON

将修改后的命令序列化回JSON并写入stdout。如果没有更改，输出信号Claude Code使用原始命令。

## 性能

整个管道在5毫秒（p99）内完成。关键优化：

- SQLite使用WAL模式实现无锁并发读取
- 预编译的正则表达式模式用于技能匹配
- 缓存的文件系统扫描（5秒TTL）
- 热路径中无网络调用
- Fail-open：任何错误都回退到原始命令

## 手动测试钩子

你可以直接调用钩子：

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
