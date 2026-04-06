# Kurulum

## Hızlı Kurulum (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Bu, platformunuz için en son sürüm ikili dosyasını indirir, SHA256 sağlama toplamını doğrular ve `~/.local/bin/` dizinine yerleştirir.

Kurulumdan sonra PRECC'yi başlatın:

```bash
precc init
```

`precc init` PreToolUse hook'unu Claude Code'a kaydeder, veri dizinlerini oluşturur ve beceri veritabanını başlatır.

## Kurulum Seçenekleri

### SHA256 Doğrulama

Varsayılan olarak, yükleyici ikili dosyanın sağlama toplamını yayımlanan SHA256 toplamıyla doğrular. Doğrulamayı atlamak için (önerilmez):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Özel Kurulum Öneki

Özel bir konuma kurulum:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Yardımcı Araçlar (--extras)

PRECC isteğe bağlı yardımcı araçlarla birlikte gelir. Bunları `--extras` ile kurun:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Bu şunları kurar:

| Araç | Amaç |
|------|---------|
| **RTK** | Komut yeniden yazma araç seti |
| **lean-ctx** | CLAUDE.md ve prompt dosyaları için bağlam sıkıştırma |
| **nushell** | Gelişmiş boru hatları için yapılandırılmış kabuk |
| **cocoindex-code** | Daha hızlı bağlam çözümleme için kod indeksleme |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Ardından başlatın:

```powershell
precc init
```

## Manuel Kurulum

1. Platformunuz için sürüm ikili dosyasını [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) adresinden indirin.
2. SHA256 sağlama toplamını sürümdeki `.sha256` dosyasıyla doğrulayın.
3. İkili dosyayı `PATH` üzerindeki bir dizine yerleştirin (örn. `~/.local/bin/`).
4. `precc init` komutunu çalıştırın.

## Güncelleme

```bash
precc update
```

Belirli bir sürüme zorla güncelleme:

```bash
precc update --force --version 0.3.0
```

Otomatik güncellemeleri etkinleştir:

```bash
precc update --auto
```

## Kurulumu Doğrulama

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

`precc` bulunamazsa, `~/.local/bin` dizininin `PATH` üzerinde olduğundan emin olun.
