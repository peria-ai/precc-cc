# Giriş

## PRECC nedir?

PRECC (Claude Code için Tahminsel Hata Düzeltme) Claude Code bash komutlarını resmi PreToolUse kanca mekanizması aracılığıyla yakalayan bir Rust aracıdır. Hataları *olmadan önce* düzeltir.

Topluluk kullanıcıları için ücretsiz.

## Sorun

Claude Code önlenebilir hatalarda önemli miktarda token israf eder:

- **Dizin hataları** -- `Cargo.toml` olmayan dizinde `cargo build`
- **Yeniden deneme döngüleri** -- Başarısız komut ayrıntılı çıktı üretir
- **Ayrıntılı çıktı** -- `find` veya `ls -R` binlerce satır üretir

## Dört Sütun

### Bağlam Düzeltme (cd-prepend)

`cargo build` veya `npm test` gibi komutların yanlış dizinde çalıştığını algılar ve yürütmeden önce `cd /doğru/yol &&` ekler.

### GDB Hata Ayıklama

Segfault ve çökmelerin daha derin hata ayıklaması için GDB ekleme fırsatlarını algılar.

### Oturum Madenciliği

Hata-düzeltme çiftleri için Claude Code oturum günlüklerini analiz eder.

### Otomasyon Becerileri

Komut kalıplarını eşleştiren ve yeniden yazan beceriler kütüphanesi.

## Nasıl çalışır (30 saniyelik versiyon)

1. Claude Code bir bash komutu çalıştırmak üzere.
2. PreToolUse kancası komutu JSON olarak gönderir.
3. `precc-hook` komutu 3 milisaniyeden kısa sürede işler.
4. Düzeltilmiş komut JSON olarak döndürülür.
5. Claude Code düzeltilmiş komutu çalıştırır.

Claude hatayı asla görmez.

### Adaptif Sıkıştırma

Bir komut sıkıştırmadan sonra başarısız olursa, PRECC bir sonraki denemede sıkıştırmayı otomatik olarak atlar ve Claude hata ayıklama için tam sıkıştırılmamış çıktıyı alır.

## Canlı Kullanım İstatistikleri

Güncel sürüm <span data-stat="current_version">--</span>:

| Metrik | Değer |
|---|---|
| Hook çağrıları | <span data-stat="total_invocations">--</span> |
| Tasarruf edilen tokenler | <span data-stat="total_tokens_saved">--</span> |
| Tasarruf oranı | <span data-stat="saving_pct">--</span>% |
| RTK yeniden yazmaları | <span data-stat="rtk_rewrites">--</span> |
| CD düzeltmeleri | <span data-stat="cd_prepends">--</span> |
| Hook gecikmesi | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Kullanıcılar | <span data-stat="unique_users">--</span> |

### Ölçülen Tasarruf (Gerçek Veri)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metrik</th><th>Değer</th></tr></thead>
<tbody>
<tr><td>Orijinal çıkış tokenleri (PRECC olmadan)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Gerçek çıkış tokenleri (PRECC ile)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Tasarruf edilen tokenler</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Tasarruf oranı</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Gerçek ölçümler</td><td><span data-measured="ground_truth_count">--</span> ölçüm</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### Yeniden Yazma Türüne Göre

<table id="rewrite-type-table">
<thead><tr><th>Tür</th><th>Sayı</th><th>Ort. Tasarruf %</th><th>Tasarruf edilen tokenler</th></tr></thead>
<tbody><tr><td colspan="4"><em>Yükleniyor...</em></td></tr></tbody>
</table>
</div>

### Sürüme göre tasarruf

<table id="version-breakdown" style="display:none">
<thead><tr><th>Sürüm</th><th>Kullanıcılar</th><th>Hook çağrıları</th><th>Tasarruf edilen tokenler</th><th>Tasarruf oranı</th></tr></thead>
<tbody><tr><td colspan="5"><em>Yükleniyor...</em></td></tr></tbody>
</table>

<small>Bu rakamlar anonimleştirilmiş telemetriden otomatik olarak güncellenir.</small>

## Bağlantılar

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Web sitesi: [https://peria.ai](https://peria.ai)
- Belgeler: [https://precc.cc](https://precc.cc)
