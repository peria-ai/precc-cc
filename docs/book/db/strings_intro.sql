-- Introduction chapter strings
INSERT OR REPLACE INTO strings (key, en, es, de, zh, pl) VALUES
('intro_title', 'Introduction', 'Introducción', 'Einführung', '简介', 'Wprowadzenie'),
('intro_what_is', 'What is PRECC?', '¿Qué es PRECC?', 'Was ist PRECC?', '什么是PRECC？', 'Czym jest PRECC?'),
('intro_what_is_body',
    'is a Rust tool that intercepts Claude Code bash commands via the official PreToolUse hook mechanism. It fixes errors *before they happen*, saving tokens and eliminating retry loops.',
    'es una herramienta Rust que intercepta los comandos bash de Claude Code a través del mecanismo oficial de hooks PreToolUse. Corrige errores *antes de que ocurran*, ahorrando tokens y eliminando bucles de reintento.',
    'ist ein Rust-Tool, das Claude Code Bash-Befehle über den offiziellen PreToolUse-Hook-Mechanismus abfängt. Es behebt Fehler *bevor sie auftreten*, spart Token und eliminiert Wiederholungsschleifen.',
    '是一个Rust工具，通过官方的PreToolUse钩子机制拦截Claude Code的bash命令。它在错误*发生之前*修复它们，节省token并消除重试循环。',
    'to narzędzie Rust, które przechwytuje polecenia bash Claude Code za pomocą oficjalnego mechanizmu hooków PreToolUse. Naprawia błędy *zanim się pojawią*, oszczędzając tokeny i eliminując pętle ponawiania.'),
('intro_problem_title', 'The Problem', 'El problema', 'Das Problem', '问题', 'Problem'),
('intro_problem_body',
    'Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.',
    'Claude Code desperdicia tokens significativos en errores prevenibles:

- **Errores de directorio** -- Ejecutar `cargo build` en un directorio padre sin `Cargo.toml`, y reintentar después de leer el error.
- **Bucles de reintento** -- Un comando fallido produce salida verbose, Claude la lee, razona y reintenta. Cada ciclo quema cientos de tokens.
- **Salida verbose** -- Comandos como `find` o `ls -R` generan miles de líneas que Claude debe procesar.',
    'Claude Code verschwendet erhebliche Token durch vermeidbare Fehler:

- **Falsche Verzeichnisse** -- `cargo build` in einem übergeordneten Verzeichnis ohne `Cargo.toml` ausführen und nach dem Lesen des Fehlers erneut versuchen.
- **Wiederholungsschleifen** -- Ein fehlgeschlagener Befehl erzeugt ausführliche Ausgabe, Claude liest sie, denkt darüber nach und versucht es erneut.
- **Ausführliche Ausgabe** -- Befehle wie `find` oder `ls -R` erzeugen tausende Zeilen, die Claude verarbeiten muss.',
    'Claude Code在可预防的错误上浪费大量token：

- **目录错误** -- 在没有 `Cargo.toml` 的父目录中运行 `cargo build`，然后在读取错误后重试。
- **重试循环** -- 失败的命令产生冗长的输出，Claude读取、推理并重试。每个循环消耗数百个token。
- **冗长输出** -- `find` 或 `ls -R` 等命令输出数千行，Claude必须处理这些内容。',
    'Claude Code marnuje znaczną liczbę tokenów na możliwe do uniknięcia błędy:

- **Błędy katalogu** -- Uruchomienie `cargo build` w katalogu nadrzędnym bez `Cargo.toml`, a następnie ponowna próba po przeczytaniu błędu.
- **Pętle ponawiania** -- Nieudane polecenie generuje szczegółowe wyjście, Claude je czyta, analizuje i ponawia próbę.
- **Szczegółowe wyjście** -- Polecenia takie jak `find` lub `ls -R` generują tysiące linii, które Claude musi przetworzyć.'),
('intro_pillars_title', 'The Four Pillars', 'Los cuatro pilares', 'Die vier Säulen', '四大支柱', 'Cztery filary'),
('pillar2_name', 'GDB Debugging', 'Depuración GDB', 'GDB-Debugging', 'GDB调试', 'Debugowanie GDB'),
('pillar2_desc',
    'Detects opportunities to attach GDB for deeper debugging of segfaults and crashes, providing structured debug information instead of raw core dumps.',
    'Detecta oportunidades para adjuntar GDB para una depuración más profunda de segfaults y crashes, proporcionando información de depuración estructurada.',
    'Erkennt Möglichkeiten, GDB für tieferes Debugging von Segfaults und Abstürzen anzuhängen und liefert strukturierte Debug-Informationen.',
    '检测附加GDB进行更深入调试的机会，提供结构化的调试信息而不是原始的核心转储。',
    'Wykrywa możliwości podpięcia GDB do głębszego debugowania segfaultów i awarii, dostarczając ustrukturyzowane informacje debugowania.'),
('pillar3_name', 'Session Mining', 'Minería de sesiones', 'Session-Mining', '会话挖掘', 'Eksploracja sesji'),
('pillar3_desc',
    'Mines Claude Code session logs for failure-fix pairs. When the same mistake recurs, PRECC already knows the fix and applies it automatically.',
    'Mina los registros de sesión de Claude Code en busca de pares fallo-corrección. Cuando el mismo error recurre, PRECC ya conoce la corrección y la aplica automáticamente.',
    'Analysiert Claude Code-Sitzungsprotokolle nach Fehler-Fix-Paaren. Bei wiederkehrenden Fehlern kennt PRECC die Lösung bereits und wendet sie automatisch an.',
    '挖掘Claude Code会话日志中的失败-修复对。当同样的错误再次发生时，PRECC已经知道修复方法并自动应用。',
    'Przeszukuje logi sesji Claude Code w poszukiwaniu par awaria-naprawa. Gdy ten sam błąd się powtarza, PRECC już zna poprawkę i stosuje ją automatycznie.'),
('pillar4_name', 'Automation Skills', 'Habilidades de automatización', 'Automatisierungsskills', '自动化技能', 'Umiejętności automatyzacji'),
('pillar4_desc',
    'A library of built-in and mined skills that match command patterns and rewrite them. Skills are defined as TOML files or SQLite rows, making them easy to inspect, edit, and share.',
    'Una biblioteca de habilidades integradas y minadas que coinciden con patrones de comandos y los reescriben. Las habilidades se definen como archivos TOML o filas SQLite.',
    'Eine Bibliothek eingebauter und geminter Skills, die Befehlsmuster erkennen und umschreiben. Skills werden als TOML-Dateien oder SQLite-Zeilen definiert.',
    '内置和挖掘技能库，匹配命令模式并重写它们。技能定义为TOML文件或SQLite行，便于检查、编辑和共享。',
    'Biblioteka wbudowanych i wydobytych umiejętności dopasowujących wzorce poleceń i je przepisujących. Umiejętności definiowane są jako pliki TOML lub wiersze SQLite.'),
('intro_how_it_works_title', 'How It Works (30-Second Version)', 'Cómo funciona (versión de 30 segundos)', 'So funktioniert es (30-Sekunden-Version)', '工作原理（30秒版本）', 'Jak to działa (wersja 30-sekundowa)'),
('intro_how_it_works_body',
    '1. Claude Code is about to run a bash command.
2. The PreToolUse hook sends the command to `precc-hook` as JSON on stdin.
3. `precc-hook` runs the command through the pipeline (skills, directory correction, compression) in under 3 milliseconds.
4. The corrected command is returned as JSON on stdout.
5. Claude Code executes the corrected command instead.

Claude never sees the error. No tokens wasted.',
    '1. Claude Code está a punto de ejecutar un comando bash.
2. El hook PreToolUse envía el comando a `precc-hook` como JSON por stdin.
3. `precc-hook` ejecuta el comando a través del pipeline (habilidades, corrección de directorio, compresión) en menos de 3 milisegundos.
4. El comando corregido se devuelve como JSON por stdout.
5. Claude Code ejecuta el comando corregido.

Claude nunca ve el error. Sin tokens desperdiciados.',
    '1. Claude Code ist im Begriff, einen Bash-Befehl auszuführen.
2. Der PreToolUse-Hook sendet den Befehl als JSON an `precc-hook` über stdin.
3. `precc-hook` verarbeitet den Befehl durch die Pipeline (Skills, Verzeichniskorrektur, Komprimierung) in unter 3 Millisekunden.
4. Der korrigierte Befehl wird als JSON auf stdout zurückgegeben.
5. Claude Code führt den korrigierten Befehl aus.

Claude sieht den Fehler nie. Keine Token verschwendet.',
    '1. Claude Code即将运行一个bash命令。
2. PreToolUse钩子将命令作为JSON通过stdin发送给 `precc-hook`。
3. `precc-hook` 在3毫秒内通过管道（技能、目录修正、压缩）处理命令。
4. 修正后的命令作为JSON通过stdout返回。
5. Claude Code执行修正后的命令。

Claude永远看不到错误。没有token浪费。',
    '1. Claude Code zamierza uruchomić polecenie bash.
2. Hook PreToolUse wysyła polecenie do `precc-hook` jako JSON na stdin.
3. `precc-hook` przetwarza polecenie przez potok (umiejętności, korekta katalogu, kompresja) w mniej niż 3 milisekundy.
4. Poprawione polecenie jest zwracane jako JSON na stdout.
5. Claude Code wykonuje poprawione polecenie.

Claude nigdy nie widzi błędu. Żadne tokeny nie są marnowane.'),
('intro_adaptive_title', '### Adaptive Compression', '### Compresión adaptativa', '### Adaptive Komprimierung', '### 自适应压缩', '### Kompresja adaptacyjna'),
('intro_live_stats_title', 'Live Usage Statistics', 'Estadísticas de uso en vivo', 'Live-Nutzungsstatistiken', '实时使用统计', 'Statystyki użycia na żywo'),
('stat_label', 'Metric', 'Métrica', 'Metrik', '指标', 'Metryka'),
('stat_value', 'Value', 'Valor', 'Wert', '值', 'Wartość'),
('stat_invocations', 'Hook invocations', 'Invocaciones del hook', 'Hook-Aufrufe', '钩子调用次数', 'Wywołania hooka'),
('stat_tokens_saved', 'Tokens saved', 'Tokens ahorrados', 'Gesparte Token', '节省的token', 'Zaoszczędzone tokeny'),
('stat_saving_pct', 'Saving ratio', 'Ratio de ahorro', 'Sparquote', '节省比率', 'Współczynnik oszczędności'),
('stat_rtk_rewrites', 'RTK rewrites', 'Reescrituras RTK', 'RTK-Umschreibungen', 'RTK重写', 'Przepisania RTK'),
('stat_cd_prepends', 'CD corrections', 'Correcciones CD', 'CD-Korrekturen', 'CD修正', 'Korekty CD'),
('stat_latency', 'Hook latency', 'Latencia del hook', 'Hook-Latenz', '钩子延迟', 'Opóźnienie hooka'),
('stats_live_note',
    'These numbers update automatically from anonymized telemetry.',
    'Estos números se actualizan automáticamente desde telemetría anonimizada.',
    'Diese Zahlen werden automatisch aus anonymisierter Telemetrie aktualisiert.',
    '这些数字会从匿名遥测数据自动更新。',
    'Te liczby aktualizują się automatycznie z zanonimizowanej telemetrii.'),
('intro_links_title', 'Links', 'Enlaces', 'Links', '链接', 'Linki'),
('website_label', 'Website', 'Sitio web', 'Webseite', '网站', 'Strona'),
('docs_label', 'Documentation', 'Documentación', 'Dokumentation', '文档', 'Dokumentacja');
