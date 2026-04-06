-- Template strings: shared text across languages
-- key: unique identifier used in templates as {{key}}
-- en/es/de/zh/pl: translated text per language
CREATE TABLE IF NOT EXISTS strings (
    key   TEXT PRIMARY KEY,
    en    TEXT NOT NULL,
    es    TEXT NOT NULL DEFAULT '',
    de    TEXT NOT NULL DEFAULT '',
    zh    TEXT NOT NULL DEFAULT '',
    pl    TEXT NOT NULL DEFAULT ''
);

-- Live stats: updated from telemetry, rendered client-side via JS
-- These are NOT baked into templates — they're served as /api/stats.json
CREATE TABLE IF NOT EXISTS stats (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    label      TEXT NOT NULL DEFAULT '',
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Common strings used across multiple chapters
INSERT OR REPLACE INTO strings (key, en, es, de, zh, pl) VALUES
('product_name', 'PRECC', 'PRECC', 'PRECC', 'PRECC', 'PRECC'),
('product_tagline',
    'Predictive Error Correction for Claude Code',
    'Corrección predictiva de errores para Claude Code',
    'Prädiktive Fehlerkorrektur für Claude Code',
    'Claude Code 预测性错误纠正',
    'Predykcyjna korekcja błędów dla Claude Code'),
('hook_latency_target',
    'under 3ms average latency',
    'latencia promedio inferior a 3ms',
    'durchschnittliche Latenz unter 3ms',
    '平均延迟低于3毫秒',
    'średnie opóźnienie poniżej 3ms'),
('fail_open_guarantee',
    'If the hook encounters any error, it exits 0 instantly — the original command runs unchanged. Claude Code is never blocked.',
    'Si el hook encuentra algún error, sale con 0 instantáneamente — el comando original se ejecuta sin cambios. Claude Code nunca se bloquea.',
    'Bei einem Fehler beendet sich der Hook sofort mit 0 — der ursprüngliche Befehl wird unverändert ausgeführt. Claude Code wird nie blockiert.',
    '如果钩子遇到任何错误，它会立即以0退出——原始命令不变地运行。Claude Code永远不会被阻塞。',
    'Jeśli hook napotka jakikolwiek błąd, kończy się natychmiast z kodem 0 — oryginalne polecenie wykonuje się bez zmian. Claude Code nigdy nie jest blokowany.'),
('install_cmd',
    'curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash',
    'curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash',
    'curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash',
    'curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash',
    'curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash'),
('free_forever',
    'Open source. Free forever.',
    'Código abierto. Gratis para siempre.',
    'Open Source. Für immer kostenlos.',
    '开源。永久免费。',
    'Otwarte źródło. Na zawsze za darmo.'),
('pillar1_name',
    'Context Fix (cd-prepend)',
    'Corrección de contexto (cd-prepend)',
    'Kontextkorrektur (cd-prepend)',
    '上下文修复 (cd-prepend)',
    'Korekta kontekstu (cd-prepend)'),
('pillar1_desc',
    'Detects when commands like `cargo build` or `npm test` run in the wrong directory and prepends `cd /correct/path &&` before execution.',
    'Detecta cuando comandos como `cargo build` o `npm test` se ejecutan en el directorio incorrecto y antepone `cd /ruta/correcta &&` antes de la ejecución.',
    'Erkennt, wenn Befehle wie `cargo build` oder `npm test` im falschen Verzeichnis ausgeführt werden, und stellt `cd /korrekter/pfad &&` voran.',
    '检测到 `cargo build` 或 `npm test` 等命令在错误的目录中运行时，在执行前添加 `cd /正确/路径 &&`。',
    'Wykrywa, gdy polecenia takie jak `cargo build` lub `npm test` są uruchamiane w złym katalogu i dodaje `cd /poprawna/ścieżka &&` przed wykonaniem.'),
('rtk_desc',
    'Wraps commands in [RTK](https://github.com/rtk-ai/rtk) to compress verbose CLI output by 60–90%. Instead of Claude reading hundreds of lines, it sees a compact summary.',
    'Envuelve comandos en [RTK](https://github.com/rtk-ai/rtk) para comprimir la salida CLI verbose en un 60–90%. En lugar de que Claude lea cientos de líneas, ve un resumen compacto.',
    'Umschließt Befehle mit [RTK](https://github.com/rtk-ai/rtk), um ausführliche CLI-Ausgaben um 60–90% zu komprimieren. Statt hunderte Zeilen zu lesen, sieht Claude eine kompakte Zusammenfassung.',
    '使用 [RTK](https://github.com/rtk-ai/rtk) 包装命令，将冗长的CLI输出压缩60-90%。Claude看到的是紧凑的摘要，而不是数百行输出。',
    'Opakowuje polecenia w [RTK](https://github.com/rtk-ai/rtk), aby skompresować szczegółowe wyjście CLI o 60–90%. Zamiast czytać setki linii, Claude widzi kompaktowe podsumowanie.'),
('adaptive_expand_desc',
    'If a command fails after compression, PRECC automatically skips compression on the retry so Claude gets the full uncompressed output to debug with.',
    'Si un comando falla después de la compresión, PRECC omite automáticamente la compresión en el reintento para que Claude obtenga la salida completa sin comprimir para depurar.',
    'Wenn ein Befehl nach der Komprimierung fehlschlägt, überspringt PRECC automatisch die Komprimierung beim erneuten Versuch, damit Claude die vollständige unkomprimierte Ausgabe zum Debuggen erhält.',
    '如果命令在压缩后失败，PRECC会自动在重试时跳过压缩，以便Claude获得完整的未压缩输出来调试。',
    'Jeśli polecenie zawiedzie po kompresji, PRECC automatycznie pomija kompresję przy ponownej próbie, aby Claude otrzymał pełne nieskompresowane wyjście do debugowania.'),
('token_model_note',
    'Figures are estimates. Each prevented failure avoids a full retry cycle: error output, model reasoning, and retry command.',
    'Las cifras son estimaciones. Cada fallo prevenido evita un ciclo completo de reintento: salida de error, razonamiento del modelo y comando de reintento.',
    'Die Zahlen sind Schätzungen. Jeder verhinderte Fehler vermeidet einen vollständigen Wiederholungszyklus: Fehlerausgabe, Modell-Reasoning und Wiederholungsbefehl.',
    '数字为估算值。每次预防的失败避免了完整的重试循环：错误输出、模型推理和重试命令。',
    'Liczby są szacunkowe. Każda zapobieżona awaria unika pełnego cyklu ponownej próby: wyjście błędu, rozumowanie modelu i ponowne polecenie.');

-- Seed stats with current values (these get updated by telemetry)
INSERT OR REPLACE INTO stats (key, value, label) VALUES
('total_tokens_saved',    '300350',  'Total tokens saved'),
('rtk_rewrites',          '748',     'RTK rewrites'),
('cd_prepends',           '42',      'CD prepends'),
('skill_activations',     '78',      'Skill activations'),
('mined_preventions',     '380',     'Mined preventions'),
('total_api_tokens',      '14080860','Total API tokens processed'),
('total_invocations',     '28404',   'Total hook invocations'),
('avg_latency_p50_ms',    '0.69',    'p50 hook latency (ms)'),
('report_count',          '7',       'Telemetry reports received'),
('saving_pct',            '2.1',     'Saving ratio (%)');
