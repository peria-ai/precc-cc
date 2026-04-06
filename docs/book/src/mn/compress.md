# Шахалт

`precc compress` нь Claude Code ачаалах үед токен хэрэглээг бууруулахын тулд CLAUDE.md болон бусад контекст файлуудыг шахдаг. Энэ бол Pro функц юм.

## Үндсэн хэрэглээ

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

## Туршилтын ажиллуулалт

Файлуудыг өөрчлөхгүйгээр юу өөрчлөгдөхийг урьдчилан харах:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Буцаах

Эх файлууд автоматаар нөөцлөгддөг. Сэргээхийн тулд:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Юу шахагдах вэ

Шахагч хэд хэдэн хувиргалт хэрэглэнэ:

- Шаардлагагүй хоосон зай, хоосон мөрүүдийг хасна
- Утгыг хадгалж дэлгэрэнгүй үг хэллэгийг товчилно
- Хүснэгт, жагсаалтыг нягтруулна
- Тайлбар, чимэглэлийн форматыг хасна
- Бүх кодын блок, зам, техникийн тодорхойлогчийг хадгална

Шахсан гаралт хүний уншигдахуйц хэвээр -- жижиглэсэн эсвэл бүдгэрүүлээгүй.

## Тодорхой файлуудыг чиглүүлэх

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
