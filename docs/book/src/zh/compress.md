# 压缩

`precc compress` 缩小 CLAUDE.md 和其他上下文文件，以减少 Claude Code 加载时的 token 使用量。这是 Pro 功能。

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

## 试运行

预览将要更改的内容而不修改文件：

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## 还原

原始文件会自动备份。要恢复它们：

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## 压缩了什么

压缩器应用多种转换：

- 删除冗余空白和空行
- 缩短冗长的措辞同时保留含义
- 压缩表格和列表
- 去除注释和装饰性格式
- 保留所有代码块、路径和技术标识符

压缩后的输出仍然是人类可读的——它不是压缩化或混淆的。

## 针对特定文件

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
