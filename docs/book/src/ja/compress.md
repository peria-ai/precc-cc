# 圧縮

`precc compress` は CLAUDE.md やその他のコンテキストファイルを圧縮し、Claude Code がそれらを読み込む際のトークン使用量を削減します。これは Pro 機能です。

## 基本的な使い方

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

## ドライラン

ファイルを変更せずに変更内容をプレビュー：

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## 元に戻す

元のファイルは自動的にバックアップされます。復元するには：

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## 何が圧縮されるか

コンプレッサーはいくつかの変換を適用します：

- 冗長な空白と空行を削除
- 意味を保ちながら冗長な表現を短縮
- テーブルとリストを圧縮
- コメントと装飾的なフォーマットを除去
- すべてのコードブロック、パス、技術的識別子を保持

圧縮された出力はまだ人間が読める形式です——ミニファイや難読化はされていません。

## 特定のファイルを対象にする

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
