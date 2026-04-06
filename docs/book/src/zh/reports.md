# 报告

`precc report` 生成一个分析仪表板，总结 PRECC 活动和 token 节省情况。

## 生成报告

```bash
$ precc report
PRECC Report -- 2026-04-03
==========================

Sessions analyzed: 12
Commands intercepted: 87
Total token savings: 42,389

Top skills by activation:
  1. cargo-wrong-dir     34 activations   17,204 tokens saved
  2. npm-wrong-dir       18 activations    9,360 tokens saved
  3. git-wrong-dir       12 activations    4,944 tokens saved
  4. RTK rewrite         15 activations    3,750 tokens saved
  5. python-wrong-dir     8 activations    4,131 tokens saved

Savings by pillar:
  Pillar 1 (context resolution):  28,639 tokens  67.6%
  Pillar 4 (automation skills):    7,000 tokens  16.5%
  RTK rewrites:                    3,750 tokens   8.8%
  Lean-ctx wraps:                  3,000 tokens   7.1%

Recent corrections:
  2026-04-03 09:12  cargo build -> cd myapp && cargo build
  2026-04-03 09:18  npm test -> cd frontend && npm test
  2026-04-03 10:05  git status -> cd repo && git status
  ...
```

## 通过电子邮件发送报告

将报告发送到电子邮件地址（需要邮件设置，见 [Email](email.md)）：

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

收件人地址从 `~/.config/precc/mail.toml` 读取。您也可以使用 `precc mail report EMAIL` 发送到特定地址。

## 报告数据

报告从本地 PRECC 数据库 `~/.local/share/precc/history.db` 生成。除非您明确通过电子邮件发送报告，否则没有数据离开您的机器。
