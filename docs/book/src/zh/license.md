# 许可证

PRECC提供两个层级：Community（免费）和Pro。

## Community层（免费）

Community层包括：

- 所有内置技能（错误目录修正、jj翻译等）
- 支持完整Pillar 1和Pillar 4的Hook管道
- 基本的 `precc savings` 摘要
- 使用 `precc ingest` 进行会话挖掘
- 无限本地使用

## Pro层

Pro解锁额外功能：

- **详细节省分析** -- `precc savings --all` 逐命令分析
- **GIF录制** -- `precc gif` 用于创建终端动画GIF
- **IP地理围栏合规** -- 适用于受监管的环境
- **电子邮件报告** -- `precc mail report` 发送分析报告
- **GitHub Actions分析** -- `precc gha` 用于调试失败的工作流
- **上下文压缩** -- `precc compress` 用于CLAUDE.md优化
- **优先支持**

## 激活许可证

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## 检查许可证状态

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors激活

如果您通过GitHub Sponsors赞助PRECC，您的许可证将通过您的GitHub邮箱自动激活。无需密钥——只需确保您的赞助者邮箱匹配：

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## 设备指纹

每个许可证都绑定到设备指纹。使用以下命令查看：

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

如果需要将许可证转移到新机器，请先停用：

```bash
precc license deactivate
```

然后在新机器上激活。

## 许可证过期？

当Pro许可证到期时，PRECC会恢复到Community层。所有内置技能和核心功能继续工作。只有Pro特有功能变为不可用。详情请参阅[FAQ](faq.md)。
