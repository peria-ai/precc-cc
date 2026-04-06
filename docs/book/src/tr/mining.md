# Madencilik

PRECC, hata-düzeltme kalıplarını öğrenmek için Claude Code oturum günlüklerini analiz eder. Aynı hatayı tekrar gördüğünde düzeltmeyi otomatik olarak uygular.

## Oturum günlüklerini alma

### Tek bir dosyayı alma

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Tüm günlükleri alma

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Yeniden almayı zorla

Zaten alınmış dosyaları yeniden işlemek için:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Madencilik nasıl çalışır

1. PRECC oturum JSONL günlük dosyasını okur.
2. İlk komutun başarısız olduğu ve ikincisinin düzeltilmiş bir yeniden deneme olduğu komut çiftlerini belirler.
3. Kalıbı (neyin yanlış gittiği) ve düzeltmeyi (Claude'un ne yaptığı) çıkarır.
4. Kalıplar `~/.local/share/precc/history.db` içinde saklanır.
5. Bir kalıp güven eşiğine ulaştığında, `heuristics.db` içinde kazılmış bir beceri olur.

### Örnek kalıp

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner arka plan hizmeti

`precc-learner` arka plan hizmeti arka planda çalışır ve yeni oturum günlüklerini otomatik olarak izler:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Arka plan hizmeti dosya sistemi bildirimlerini (Linux'ta inotify, macOS'ta FSEvents) kullanır ve bir oturum sona erdiğinde hemen tepki verir.

## Kalıplardan becerilere

Kazılmış kalıplar bu kriterleri karşıladığında becerilere dönüşür:

- Oturumlar boyunca en az 3 kez görülmüş
- Tutarlı düzeltme kalıbı (her seferinde aynı tür düzeltme)
- Yanlış pozitif tespit edilmemiş

Beceri adaylarını şu komutla inceleyebilirsiniz:

```bash
$ precc skills advise
```

Becerileri yönetme ayrıntıları için [Skills](skills.md) bölümüne bakın.

## Veri depolama

- **Hata-düzeltme çiftleri**: `~/.local/share/precc/history.db`
- **Terfi etmiş beceriler**: `~/.local/share/precc/heuristics.db`

Her ikisi de güvenli eşzamanlı erişim için WAL modunda SQLite veritabanlarıdır.
