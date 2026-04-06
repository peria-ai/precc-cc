# Minería

PRECC mina los registros de sesión de Claude Code para aprender patrones de fallo-corrección. Cuando ve el mismo error de nuevo, aplica la corrección automáticamente.

## Ingesta de registros de sesión

### Ingestar un solo archivo

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingestar todos los registros

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Forzar reingesta

Para reprocesar archivos que ya fueron ingestados:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Cómo funciona la minería

1. PRECC lee el archivo de registro JSONL de la sesión.
2. Identifica pares de comandos donde el primero falló y el segundo fue un reintento corregido.
3. Extrae el patrón (qué salió mal) y la corrección (qué hizo Claude de manera diferente).
4. Los patrones se almacenan en `~/.local/share/precc/history.db`.
5. Cuando un patrón alcanza un umbral de confianza (visto varias veces), se convierte en una habilidad minada en `heuristics.db`.

### Ejemplo de patrón

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## El daemon precc-learner

El daemon `precc-learner` se ejecuta en segundo plano y vigila automáticamente los nuevos registros de sesión:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

El daemon usa notificaciones del sistema de archivos (inotify en Linux, FSEvents en macOS) por lo que reacciona inmediatamente cuando termina una sesión.

## De patrones a habilidades

Los patrones minados se gradúan a habilidades cuando cumplen estos criterios:

- Vistos al menos 3 veces en diferentes sesiones
- Patrón de corrección consistente (mismo tipo de corrección cada vez)
- Sin falsos positivos detectados

Puedes revisar los candidatos a habilidades con:

```bash
$ precc skills advise
```

Consulte [Skills](skills.md) para detalles sobre la gestión de habilidades.

## Almacenamiento de datos

- **Pares de fallo-corrección**: `~/.local/share/precc/history.db`
- **Habilidades graduadas**: `~/.local/share/precc/heuristics.db`

Ambas son bases de datos SQLite en modo WAL para acceso concurrente seguro.
