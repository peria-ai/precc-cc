# Суулгах

## Хурдан суулгах (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Энэ нь таны платформд зориулсан хамгийн сүүлийн хувилбарыг татаж, SHA256 шалгах нийлбэрийг баталгаажуулж, `~/.local/bin/` руу байрлуулна.

Суулгасны дараа PRECC-ийг эхлүүлнэ:

```bash
precc init
```

`precc init` нь PreToolUse hook-ийг Claude Code-д бүртгэж, өгөгдлийн сангуудыг үүсгэж, ур чадварын мэдээллийн санг эхлүүлнэ.

## Суулгах сонголтууд

### SHA256 баталгаажуулалт

Анхдагч байдлаар суулгагч нь нийтлэгдсэн SHA256 нийлбэртэй харьцуулж шалгана. Шалгалтыг алгасахын тулд (зөвлөдөггүй):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Тохируулсан суулгах зам

Тохируулсан байршилд суулгах:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Нэмэлт хэрэгслүүд (--extras)

PRECC нь нэмэлт хэрэгслүүдтэй ирдэг. `--extras` ашиглан суулгана:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Энэ нь дараахыг суулгана:

| Хэрэгсэл | Зорилго |
|------|---------|
| **RTK** | Командыг дахин бичих хэрэгсэл |
| **lean-ctx** | CLAUDE.md болон prompt файлуудын контекст шахалт |
| **nushell** | Дэвшилтэт pipeline-д зориулсан бүтэцтэй shell |
| **cocoindex-code** | Контекстыг хурдан шийдвэрлэхэд зориулсан код индекслэл |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Дараа нь эхлүүлнэ:

```powershell
precc init
```

## Гараар суулгах

1. Платформдоо зориулсан хувилбарыг [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) хуудаснаас татаж авна.
2. SHA256 шалгах нийлбэрийг хувилбар дахь `.sha256` файлтай тулгаж шалгана.
3. Бинари файлыг `PATH` дээрх хавтаст байрлуулна (жнь `~/.local/bin/`).
4. `precc init` ажиллуулна.

## Шинэчлэх

```bash
precc update
```

Тодорхой хувилбар руу албадан шинэчлэх:

```bash
precc update --force --version 0.3.0
```

Автомат шинэчлэлтийг идэвхжүүлэх:

```bash
precc update --auto
```

## Суулгалтыг шалгах

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

`precc` олдохгүй бол `~/.local/bin` таны `PATH` дээр байгаа эсэхийг шалгана уу.
