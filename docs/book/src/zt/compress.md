# 壓縮

`precc compress` 縮小 CLAUDE.md 和其他上下文文件，以減少 Claude Code 加載時的 token 使用量。這是 Pro 功能。

## 基本用法

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## 試運行

預覽將要更改的內容而不修改文件：

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## 還原

原始文件會自動備份。要恢復它們：

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## 壓縮了什麼

壓縮器應用多種轉換：

- 刪除冗餘空白和空行
- 縮短冗長的措辭同時保留含義
- 壓縮表格和列表
- 去除註釋和裝飾性格式
- 保留所有代碼塊、路徑和技術標識符

壓縮後的輸出仍然是人類可讀的——它不是壓縮化或混淆的。

## 針對特定文件

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
