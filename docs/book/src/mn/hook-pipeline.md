# Hook Pipeline

`precc-hook` хоёртын файл нь PRECC-ийн цөм юм. Claude Code болон shell-ийн хооронд байрлаж, bash команд бүрийг 5 миллисекундын дотор боловсруулна.

## Claude Code Hook-ийг хэрхэн дуудах вэ

Claude Code нь PreToolUse hook-уудыг дэмждэг -- гүйцэтгэхээс өмнө хэрэгслийн оролтыг шалгаж, өөрчилж чадах гадаад программууд. Claude bash команд ажиллуулах гэж байгаа үед stdin-ээр `precc-hook`-д JSON илгээж, stdout-аас хариуг уншина.

## Pipeline-ийн үе шатууд

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

## Жишээ: JSON оролт ба гаралт

### Оролт (Claude Code-оос)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC одоогийн хавтаст `Cargo.toml` байхгүйг, гэхдээ `./myapp/Cargo.toml` байгааг илрүүлнэ.

### Гаралт (Claude Code руу)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Өөрчлөлт шаардлагагүй бол `updatedInput.command` хоосон байх ба Claude Code анхны командыг ашиглана.

## Үе шатны дэлгэрэнгүй

### Үе шат 1: JSON задлах

stdin-ээс бүтэн JSON объектыг уншина. `tool_input.command`-ыг ялгаж авна. Задлал амжилтгүй болвол hook шууд гарч, Claude Code анхны командыг ашиглана (fail-open загвар).

### Үе шат 2: Skill тааруулалт

Команд дээр trigger хэв маяг таарах skill-үүдийг SQLite heuristics мэдээллийн сангаас хайна. Skill-үүдийг эрэмбээр шалгана.

### Үе шат 3: Хавтас засвар

Build командуудын хувьд (`cargo`, `go`, `make`, `npm`, `python` гэх мэт) одоогийн хавтаст хүлээгдэж буй төслийн файл байгаа эсэхийг шалгана. Байхгүй бол ойролцоох хавтсуудаас хамгийн ойр тааралтыг хайж `cd <dir> &&` нэмнэ.

Хавтас сканнер нь хурдыг хадгалахын тулд 5 секундын TTL-тэй кэшлэгдсэн файлын системийн индекс ашигладаг.

### Үе шат 4: GDB шалгалт

Хэрэв команд нь гэмтэл үүсгэх магадлалтай бол (жишээ нь debug binary ажиллуулах), PRECC нь бүдүүлэг гэмтлийн бүртгэлийн оронд бүтэцлэгдсэн debug гаралтыг авахын тулд GDB wrapper санал болгож болно.

### Үе шат 5: RTK дахин бичих

Урт командуудыг богиносгох, шуугиантай гаралтыг дарах, эсвэл токен үр ашгийн тулд командуудыг дахин бүтэцлэх RTK (Rewrite Toolkit) дүрмүүдийг хэрэглэнэ.

### Үе шат 6: JSON гаргах

Өөрчлөгдсөн командыг JSON руу буцааж цуваачилж stdout руу бичнэ. Өөрчлөлт хийгээгүй бол гаралт нь Claude Code-д анхны командыг ашиглахыг дохионо.

## Гүйцэтгэл

Бүх pipeline 5 миллисекундын дотор (p99) дуусна. Гол оновчлолууд:

- Түгжээгүй зэрэгцээ уншилтын төлөө WAL горимтой SQLite
- Skill тааруулалтад зориулсан урьдчилан хөрвүүлсэн regex загварууд
- Кэшлэгдсэн файлын системийн сканнер (5 секундын TTL)
- Халуун замд сүлжээний дуудлага байхгүй
- Fail-open: аливаа алдаа нь анхны команд руу шилжинэ

## Hook-ийг гараар шалгах

Hook-ийг шууд дуудаж болно:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
