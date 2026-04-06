# 報告

`precc report` 生成一個分析儀表板，總結 PRECC 活動和 token 節省情況。

## 生成報告

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

## 通過電子郵件發送報告

將報告發送到電子郵件地址（需要郵件設置，見 [Email](email.md)）：

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

收件人地址從 `~/.config/precc/mail.toml` 讀取。您也可以使用 `precc mail report EMAIL` 發送到特定地址。

## 報告數據

報告從本地 PRECC 數據庫 `~/.local/share/precc/history.db` 生成。除非您明確通過電子郵件發送報告，否則沒有數據離開您的機器。
