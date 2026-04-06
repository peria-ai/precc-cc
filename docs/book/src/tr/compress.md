# Sıkıştırma

`precc compress`, Claude Code yüklediğinde token kullanımını azaltmak için CLAUDE.md ve diğer bağlam dosyalarını küçültür. Bu bir Pro özelliğidir.

## Temel kullanım

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

## Deneme çalıştırma

Dosyaları değiştirmeden nelerin değişeceğini önizleyin:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Geri alma

Orijinaller otomatik olarak yedeklenir. Geri yüklemek için:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Ne sıkıştırılır

Sıkıştırıcı birkaç dönüşüm uygular:

- Gereksiz boşlukları ve boş satırları kaldırır
- Anlamı koruyarak ayrıntılı ifadeleri kısaltır
- Tabloları ve listeleri yoğunlaştırır
- Yorumları ve dekoratif biçimlendirmeyi kaldırır
- Tüm kod bloklarını, yolları ve teknik tanımlayıcıları korur

Sıkıştırılmış çıktı hâlâ okunabilirdir -- küçültülmemiş veya gizlenmemiştir.

## Belirli dosyaları hedefleme

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
