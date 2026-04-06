# 許可證

PRECC提供兩個層級：Community（免費）和Pro。

## Community層（免費）

Community層包括：

- 所有內置技能（錯誤目錄修正、jj翻譯等）
- 支持完整Pillar 1和Pillar 4的Hook管道
- 基本的 `precc savings` 摘要
- 使用 `precc ingest` 進行會話挖掘
- 無限本地使用

## Pro層

Pro解鎖額外功能：

- **詳細節省分析** -- `precc savings --all` 逐命令分析
- **GIF錄製** -- `precc gif` 用於創建終端動畫GIF
- **IP地理圍欄合規** -- 適用於受監管的環境
- **電子郵件報告** -- `precc mail report` 發送分析報告
- **GitHub Actions分析** -- `precc gha` 用於調試失敗的工作流
- **上下文壓縮** -- `precc compress` 用於CLAUDE.md優化
- **優先支持**

## 激活許可證

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## 檢查許可證狀態

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors激活

如果您通過GitHub Sponsors贊助PRECC，您的許可證將通過您的GitHub郵箱自動激活。無需密鑰——只需確保您的贊助者郵箱匹配：

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## 設備指紋

每個許可證都綁定到設備指紋。使用以下命令查看：

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

如果需要將許可證轉移到新機器，請先停用：

```bash
precc license deactivate
```

然後在新機器上激活。

## 許可證過期？

當Pro許可證到期時，PRECC會恢復到Community層。所有內置技能和核心功能繼續工作。只有Pro特有功能變爲不可用。詳情請參閱[FAQ](faq.md)。
