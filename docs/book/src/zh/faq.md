# 常见问题

## PRECC安全吗？

是的。PRECC使用Claude Code官方的PreToolUse钩子机制——Anthropic专门为此目的设计的扩展点。该钩子：

- 完全离线运行（热路径中无网络调用）
- 在5毫秒内完成
- 是fail-open的：如果出现任何问题，原始命令将不受修改地运行
- 只修改命令，从不自己执行它们
- 将数据存储在本地SQLite数据库中

## PRECC能与其他AI编码工具一起使用吗？

PRECC专为Claude Code设计。它依赖于Claude Code提供的PreToolUse钩子协议。它不适用于Cursor、Copilot、Windsurf或其他AI编码工具。

## 遥测发送什么数据？

遥测仅在选择加入后启用。启用后发送：

- PRECC版本、操作系统和架构
- 汇总计数（拦截的命令、激活的技能）
- 平均钩子延迟

它**不**发送命令文本、文件路径、项目名称或任何个人身份信息。您可以在选择加入前使用 `precc telemetry preview` 预览确切的数据。详见[遥测](telemetry.md)。

## 如何卸载PRECC？

??faq_uninstall_a_intro??

1. 移除钩子注册：
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. 删除二进制文件：
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. 删除数据（可选）：
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## 我的许可证过期了。会发生什么？

PRECC恢复到社区版。所有核心功能继续正常工作：

- 内置技能保持活跃
- 钩子管道正常运行
- `precc savings` 显示摘要视图
- `precc ingest` 和会话挖掘正常工作

Pro功能在续订前不可用：

- `precc savings --all`（详细分类）
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- 电子邮件报告

## 钩子似乎没有运行。如何调试？

??faq_debug_a_intro??

1. 检查钩子是否已注册：
   ```bash
   precc init
   ```

2. 手动测试钩子：
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. 检查二进制文件是否在PATH中：
   ```bash
   which precc-hook
   ```

4. 检查 `~/.claude/settings.json` 中的Claude Code钩子配置。

## PRECC会减慢Claude Code吗？

不会。钩子在5毫秒内完成（p99）。与Claude推理和生成回复所花费的时间相比，这是不可察觉的。

## 我可以在CI/CD中使用PRECC吗？

PRECC是为交互式Claude Code会话设计的。在CI/CD中，没有Claude Code实例可以挂钩。但是，`precc gha` 可以从任何环境分析失败的GitHub Actions运行。

## 挖掘的技能与内置技能有何不同？

内置技能随PRECC提供，涵盖常见的错误目录模式。挖掘的技能从您的特定会话日志中学习——它们捕获您工作流程中独特的模式。两者都存储在SQLite中，并由钩子管道以相同方式评估。

## 我可以与团队共享技能吗？

可以。使用 `precc skills export NAME` 将任何技能导出为TOML并共享文件。团队成员可以将其放在 `skills/` 目录中或导入到他们的启发式数据库中。
