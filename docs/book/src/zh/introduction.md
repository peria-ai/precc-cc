# 简介

## 什么是PRECC？

PRECC (Claude Code 预测性错误纠正) 是一个Rust工具，通过官方的PreToolUse钩子机制拦截Claude Code的bash命令。它在错误*发生之前*修复它们，节省token并消除重试循环。

对社区用户免费。

## 问题

Claude Code在可预防的错误上浪费大量token：

- **目录错误** -- 在没有 `Cargo.toml` 的父目录中运行 `cargo build`，然后在读取错误后重试。
- **重试循环** -- 失败的命令产生冗长的输出，Claude读取、推理并重试。每个循环消耗数百个token。
- **冗长输出** -- `find` 或 `ls -R` 等命令输出数千行，Claude必须处理这些内容。

## 四大支柱

### 上下文修复 (cd-prepend)

检测到 `cargo build` 或 `npm test` 等命令在错误的目录中运行时，在执行前添加 `cd /正确/路径 &&`。

### GDB调试

检测附加GDB进行更深入调试的机会，提供结构化的调试信息而不是原始的核心转储。

### 会话挖掘

挖掘Claude Code会话日志中的失败-修复对。当同样的错误再次发生时，PRECC已经知道修复方法并自动应用。

### 自动化技能

内置和挖掘技能库，匹配命令模式并重写它们。技能定义为TOML文件或SQLite行，便于检查、编辑和共享。

## 工作原理（30秒版本）

1. Claude Code即将运行一个bash命令。
2. PreToolUse钩子将命令作为JSON通过stdin发送给 `precc-hook`。
3. `precc-hook` 在3毫秒内通过管道（技能、目录修正、压缩）处理命令。
4. 修正后的命令作为JSON通过stdout返回。
5. Claude Code执行修正后的命令。

Claude永远看不到错误。没有token浪费。

### 自适应压缩

如果命令在压缩后失败，PRECC会自动在重试时跳过压缩，以便Claude获得完整的未压缩输出来调试。

## 实时使用统计

当前版本 <span data-stat="current_version">--</span>:

| 指标 | 值 |
|---|---|
| 钩子调用次数 | <span data-stat="total_invocations">--</span> |
| 节省的token | <span data-stat="total_tokens_saved">--</span> |
| 节省比率 | <span data-stat="saving_pct">--</span>% |
| RTK重写 | <span data-stat="rtk_rewrites">--</span> |
| CD修正 | <span data-stat="cd_prepends">--</span> |
| 钩子延迟 | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| 独立用户 | <span data-stat="unique_users">--</span> |

### 各版本节省情况

<table id="version-breakdown" style="display:none">
<thead><tr><th>版本</th><th>独立用户</th><th>钩子调用次数</th><th>节省的token</th><th>节省比率</th></tr></thead>
<tbody><tr><td colspan="5"><em>加载中...</em></td></tr></tbody>
</table>

<small>数字为估算值。每次预防的失败避免了完整的重试循环：错误输出、模型推理和重试命令。 这些数字会从匿名遥测数据自动更新。</small>

## 链接

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- 网站: [https://peria.ai](https://peria.ai)
- 文档: [https://precc.cc](https://precc.cc)
