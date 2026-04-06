# Олборлолт

PRECC нь Claude Code-ийн сессийн логуудаас алдаа-засварын загварыг сурдаг. Ижил алдааг дахин олбол засварыг автоматаар хэрэглэнэ.

## Сессийн логуудыг оруулах

### Нэг файл оруулах

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Бүх логуудыг оруулах

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Дахин оруулахыг шахах

Аль хэдийн оруулсан файлуудыг дахин боловсруулахын тулд:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Олборлолт хэрхэн ажилладаг

1. PRECC нь сессийн JSONL лог файлыг уншина.
2. Эхний команд амжилтгүй болж, хоёр дахь нь засварласан дахин оролдлого болсон командын хосуудыг тодорхойлно.
3. Загвар (юу буруу болсон) болон засвар (Claude юуг өөрөөр хийсэн)-ыг гаргаж авна.
4. Загварууд `~/.local/share/precc/history.db`-д хадгалагдана.
5. Загвар итгэлийн босгод хүрэхэд `heuristics.db`-д олборлосон ур чадвар болно.

### Загварын жишээ

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner демон

`precc-learner` демон нь ард дэвсгэрт ажиллаж, шинэ сессийн логуудыг автоматаар хянадаг:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Демон нь файлын системийн мэдэгдлүүдийг (Linux дээр inotify, macOS дээр FSEvents) ашигладаг тул сесс дуусахад шууд хариу үйлдэл хийнэ.

## Загвараас ур чадвар руу

Олборлосон загварууд дараах шалгуурыг хангасан тохиолдолд ур чадвар болно:

- Сессүүдийн туршид дор хаяж 3 удаа харагдсан
- Тогтмол засварын загвар (болгондоо ижил төрлийн засвар)
- Хуурамч эерэг илрээгүй

Ур чадварын нэр дэвшигчдийг дараахаар шалгаж болно:

```bash
$ precc skills advise
```

Ур чадварыг удирдах дэлгэрэнгүйг [Skills](skills.md)-ээс харна уу.

## Өгөгдлийн хадгалалт

- **Алдаа-засварын хосууд**: `~/.local/share/precc/history.db`
- **Дэвшүүлсэн ур чадварууд**: `~/.local/share/precc/heuristics.db`

Хоёулаа аюулгүй зэрэгцээ хандалтын төлөө WAL горимтой SQLite мэдээллийн сан юм.
