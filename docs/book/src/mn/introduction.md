# Танилцуулга

## PRECC гэж юу вэ?

PRECC (Claude Code-ийн урьдчилсан алдаа засах) нь албан ёсны PreToolUse hook механизмаар Claude Code bash тушаалуудыг таслан зогсоодог Rust хэрэгсэл юм.

Нийгэмлэгийн хэрэглэгчдэд үнэгүй.

## Асуудал

Claude Code урьдчилан сэргийлэх боломжтой алдаанд их хэмжээний токен үрдэг:

- **Сан алдаа** -- `Cargo.toml` байхгүй сан дотор `cargo build` ажиллуулах.
- **Дахин оролдлогын давталт** -- Амжилтгүй тушаал их хэмжээний гаралт үүсгэдэг.
- **Их хэмжээний гаралт** -- `find`, `ls -R` зэрэг тушаалууд мянга мянган мөр үүсгэдэг.

## Дөрвөн тулгуур

### Контекст засвар (cd-prepend)

`cargo build` буюу `npm test` зэрэг тушаал буруу сан дотор ажиллаж байгааг илрүүлж, ажиллуулахын өмнө `cd /зөв/зам &&` нэмдэг.

### GDB дебаг

Segfault болон эвдрэлийг гүнзгий дебаг хийхэд GDB холбох боломжийг илрүүлдэг.

### Сешн олборлолт

Claude Code-ийн сешн логоос алдаа-засварын хос олдог. Ижил алдаа давтагдахад автоматаар хэрэгжүүлдэг.

### Автоматжуулалтын ур чадвар

Тушаалын хэв маягийг таньж, дахин бичих ур чадварын сан. TOML файл эсвэл SQLite мөрөөр тодорхойлогддог.

## Хэрхэн ажилладаг (30 секундын хувилбар)

1. Claude Code bash тушаал ажиллуулах гэж байна.
2. PreToolUse hook тушаалыг JSON хэлбэрээр `precc-hook`-д илгээнэ.
3. `precc-hook` тушаалыг 3 миллисекундээс бага хугацаанд боловсруулна.
4. Засварласан тушаал JSON хэлбэрээр буцаана.
5. Claude Code засварласан тушаалыг ажиллуулна.

Claude хэзээ ч алдаа харахгүй.

### Дасан зохицох шахалт

Тушаал шахсны дараа амжилтгүй болвол PRECC дараагийн оролдлогод шахалтыг автоматаар алгасаж, Claude дебаг хийхэд бүрэн гаралтыг өгдөг.

## Шууд ашиглалтын статистик

Current version <span data-stat="current_version">--</span>:

| Хэмжүүр | Утга |
|---|---|
| Hook дуудлага | <span data-stat="total_invocations">--</span> |
| Хэмнэсэн токен | <span data-stat="total_tokens_saved">--</span> |
| Хэмнэлтийн харьцаа | <span data-stat="saving_pct">--</span>% |
| RTK дахин бичилт | <span data-stat="rtk_rewrites">--</span> |
| CD засвар | <span data-stat="cd_prepends">--</span> |
| Hook хоцролт | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Unique users | <span data-stat="unique_users">--</span> |

### Savings by Release

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Unique users</th><th>Hook дуудлага</th><th>Хэмнэсэн токен</th><th>Хэмнэлтийн харьцаа</th></tr></thead>
<tbody><tr><td colspan="5"><em>Loading...</em></td></tr></tbody>
</table>

<small>Тоонууд тооцоолол юм. Урьдчилан сэргийлсэн алдаа бүр бүрэн дахин оролдлогын мөчлөгөөс зайлсхийдэг. Эдгээр тоонууд нэргүй телеметрээс автоматаар шинэчлэгддэг.</small>

## Холбоосууд

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Вэбсайт: [https://peria.ai](https://peria.ai)
- Баримт бичиг: [https://precc.cc](https://precc.cc)
