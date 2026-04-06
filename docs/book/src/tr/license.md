# Lisans

PRECC iki seviye sunar: Community (ücretsiz) ve Pro.

## Community seviyesi (ücretsiz)

Community seviyesi şunları içerir:

- Tüm yerleşik beceriler (dizin düzeltme, jj çevirisi vb.)
- Tam Pillar 1 ve Pillar 4 desteğiyle hook hattı
- Temel `precc savings` özeti
- `precc ingest` ile oturum madenciliği
- Sınırsız yerel kullanım

## Pro seviyesi

Pro ek özelliklerin kilidini açar:

- **Ayrıntılı tasarruf dökümü** -- `precc savings --all` komut bazlı analizle
- **GIF kaydı** -- `precc gif` animasyonlu terminal GIF'leri oluşturmak için
- **IP coğrafi sınır uyumluluğu** -- Düzenlenmiş ortamlar için
- **E-posta raporları** -- `precc mail report` analitik göndermek için
- **GitHub Actions analizi** -- `precc gha` başarısız iş akışı hata ayıklaması için
- **Bağlam sıkıştırma** -- `precc compress` CLAUDE.md optimizasyonu için
- **Öncelikli destek**

## Lisans etkinleştirme

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Lisans durumunu kontrol etme

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors etkinleştirme

PRECC'i GitHub Sponsors üzerinden sponsor olursanız, lisansınız GitHub e-postanız aracılığıyla otomatik olarak etkinleştirilir. Anahtar gerekmez -- sadece sponsor e-postanızın eşleştiğinden emin olun:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Cihaz parmak izi

Her lisans bir cihaz parmak izine bağlıdır. Kendinizinkini görüntüleyin:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Lisansınızı yeni bir makineye aktarmanız gerekiyorsa, önce devre dışı bırakın:

```bash
precc license deactivate
```

Ardından yeni makinede etkinleştirin.

## Lisans süresi doldu mu?

Bir Pro lisansı sona erdiğinde, PRECC Community seviyesine geri döner. Tüm yerleşik beceriler ve temel işlevler çalışmaya devam eder. Yalnızca Pro'ya özgü özellikler kullanılamaz hale gelir. Daha fazla ayrıntı için [SSS](faq.md) sayfasına bakın.
