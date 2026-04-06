# Hook Pipeline

`precc-hook` ikili dosyası PRECC'nin çekirdeğidir. Claude Code ile kabuk arasında yer alır ve her bash komutunu 5 milisaniyenin altında işler.

## Claude Code Hook'u nasıl çağırır

Claude Code, PreToolUse hook'larını destekler -- yürütmeden önce araç girdilerini inceleyip değiştirebilen harici programlar. Claude bir bash komutu çalıştırmak üzereyken, stdin üzerinden `precc-hook`'a JSON gönderir ve stdout'dan yanıtı okur.

## Pipeline Aşamaları

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## Örnek: JSON Girdi ve Çıktı

### Girdi (Claude Code'dan)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC, geçerli dizinde `Cargo.toml` olmadığını ancak `./myapp/Cargo.toml` dosyasının var olduğunu tespit eder.

### Çıktı (Claude Code'a)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Değişiklik gerekmiyorsa, `updatedInput.command` boş olur ve Claude Code orijinal komutu kullanır.

## Aşama Detayları

### Aşama 1: JSON Ayrıştırma

Stdin'den tam JSON nesnesini okur. `tool_input.command` değerini çıkarır. Ayrıştırma başarısız olursa, hook hemen çıkar ve Claude Code orijinal komutu kullanır (fail-open tasarım).

### Aşama 2: Beceri Eşleştirme

Komutun tetikleme deseniyle eşleşen beceriler için SQLite buluşsal veritabanını sorgular. Beceriler öncelik sırasına göre kontrol edilir. Hem yerleşik TOML becerileri hem de çıkarılmış beceriler değerlendirilir.

### Aşama 3: Dizin Düzeltme

Build komutları (`cargo`, `go`, `make`, `npm`, `python` vb.) için beklenen proje dosyasının geçerli dizinde olup olmadığını kontrol eder. Yoksa, en yakın eşleşme için yakın dizinleri tarar ve başına `cd <dir> &&` ekler.

Dizin taraması, hızlı kalmak için 5 saniyelik TTL'ye sahip önbelleğe alınmış bir dosya sistemi dizini kullanır.

### Aşama 4: GDB Kontrolü

Komutun bir çökme üretmesi muhtemelse (ör. debug binary çalıştırma), PRECC ham çökme günlükleri yerine yapılandırılmış debug çıktısı yakalamak için GDB sarmalayıcıları önerebilir veya enjekte edebilir.

### Aşama 5: RTK Yeniden Yazma

Ayrıntılı komutları kısaltan, gürültülü çıktıyı bastıran veya token verimliliği için komutları yeniden yapılandıran RTK (Rewrite Toolkit) kurallarını uygular.

### Aşama 6: JSON Çıktısı

Değiştirilen komutu JSON'a geri serileştirir ve stdout'a yazar. Değişiklik yapılmadıysa, çıktı Claude Code'a orijinal komutu kullanmasını bildirir.

## Performans

Tüm pipeline 5 milisaniyenin altında (p99) tamamlanır. Temel optimizasyonlar:

- Kilitsiz eşzamanlı okumalar için WAL modunda SQLite
- Beceri eşleştirme için önceden derlenmiş regex kalıpları
- Önbelleğe alınmış dosya sistemi taramaları (5 saniyelik TTL)
- Sıcak yolda ağ çağrısı yok
- Fail-open: herhangi bir hata orijinal komuta geri döner

## Hook'u Manuel Test Etme

Hook'u doğrudan çağırabilirsiniz:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
