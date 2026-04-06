# 遥测

PRECC支持可选的匿名遥测以帮助改进工具。除非您明确同意，否则不会收集任何数据。

## 选择加入

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## 选择退出

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## 检查状态

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## 预览将发送的数据

在选择加入之前，您可以查看将收集的确切数据：

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## 收集的数据

- PRECC版本、操作系统和架构
- 汇总计数：拦截的命令、激活的技能、使用的支柱
- 平均钩子延迟
- 会话数

## 不收集的数据

- 不收集命令文本或参数
- 不收集文件路径或目录名
- 不收集项目名称或仓库URL
- 不收集个人身份信息（PII）
- 不收集IP地址（服务器不记录它们）

## 环境变量覆盖

无需运行命令即可禁用遥测（适用于CI或共享环境）：

```bash
export PRECC_NO_TELEMETRY=1
```

这优先于同意设置。

## 数据目的地

遥测数据通过HTTPS发送到 `https://telemetry.peria.ai/v1/precc`。数据仅用于了解使用模式和确定开发优先级。
