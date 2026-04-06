# التنقيب

يقوم PRECC بتنقيب سجلات جلسات Claude Code لتعلم أنماط الخطأ والإصلاح. عندما يرى نفس الخطأ مرة أخرى، يطبق الإصلاح تلقائيًا.

## استيعاب سجلات الجلسات

### استيعاب ملف واحد

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### استيعاب جميع السجلات

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### فرض إعادة الاستيعاب

لإعادة معالجة الملفات التي تم استيعابها بالفعل:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## كيف يعمل التنقيب

1. يقرأ PRECC ملف سجل JSONL للجلسة.
2. يحدد أزواج الأوامر حيث فشل الأمر الأول وكان الثاني إعادة محاولة مصححة.
3. يستخرج النمط (ما الخطأ) والإصلاح (ما فعله Claude بشكل مختلف).
4. تُخزن الأنماط في `~/.local/share/precc/history.db`.
5. عندما يصل نمط إلى عتبة الثقة، يصبح مهارة مُستخرجة في `heuristics.db`.

### مثال على نمط

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## خدمة precc-learner

يعمل خادم `precc-learner` في الخلفية ويراقب سجلات الجلسات الجديدة تلقائيًا:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

يستخدم الخادم إشعارات نظام الملفات (inotify على Linux، FSEvents على macOS) فيتفاعل فورًا عند انتهاء الجلسة.

## من الأنماط إلى المهارات

تتخرج الأنماط المستخرجة إلى مهارات عندما تستوفي هذه المعايير:

- شوهدت 3 مرات على الأقل عبر الجلسات
- نمط إصلاح متسق (نفس نوع التصحيح في كل مرة)
- لم يتم اكتشاف إيجابيات كاذبة

يمكنك مراجعة مرشحي المهارات باستخدام:

```bash
$ precc skills advise
```

راجع [Skills](skills.md) لتفاصيل إدارة المهارات.

## تخزين البيانات

- **أزواج الخطأ-الإصلاح**: `~/.local/share/precc/history.db`
- **المهارات المتخرجة**: `~/.local/share/precc/heuristics.db`

كلاهما قاعدتا بيانات SQLite في وضع WAL للوصول المتزامن الآمن.
