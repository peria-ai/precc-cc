# Telemetri

PRECC, aracı geliştirmeye yardımcı olmak için isteğe bağlı anonim telemetriyi destekler. Açıkça onay vermediğiniz sürece hiçbir veri toplanmaz.

## Katılma

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Ayrılma

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Durum kontrolü

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Gönderilecek verilerin önizlemesi

Katılmadan önce hangi verilerin toplanacağını tam olarak görebilirsiniz:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Toplanan veriler

- PRECC sürümü, işletim sistemi ve mimari
- Toplu sayımlar: yakalanan komutlar, etkinleştirilen beceriler, kullanılan sütunlar
- Ortalama hook gecikmesi
- Oturum sayısı

## Toplanmayan veriler

- Komut metni veya argüman yok
- Dosya yolları veya dizin adları yok
- Proje adları veya depo URL'leri yok
- Kişisel tanımlayıcı bilgi (PII) yok
- IP adresleri yok (sunucu bunları kaydetmez)

## Ortam değişkeni geçersiz kılma

Komut çalıştırmadan telemetriyi devre dışı bırakmak için (CI veya paylaşılan ortamlarda kullanışlı):

```bash
export PRECC_NO_TELEMETRY=1
```

Bu, onay ayarının önüne geçer.

## Veri hedefi

Telemetri verileri HTTPS üzerinden `https://telemetry.peria.ai/v1/precc` adresine gönderilir. Veriler yalnızca kullanım kalıplarını anlamak ve geliştirmeyi önceliklendirmek için kullanılır.
