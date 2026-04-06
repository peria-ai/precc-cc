# Pipeline del Hook

El binario `precc-hook` es el núcleo de PRECC. Se sitúa entre Claude Code y el shell, procesando cada comando bash en menos de 5 milisegundos.

## Cómo Claude Code invoca el Hook

Claude Code soporta hooks PreToolUse -- programas externos que pueden inspeccionar y modificar las entradas de herramientas antes de la ejecución. Cuando Claude está a punto de ejecutar un comando bash, envía JSON a `precc-hook` por stdin y lee la respuesta de stdout.

## Etapas del Pipeline

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

## Ejemplo: Entrada y salida JSON

### Entrada (desde Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC detecta que el directorio actual no tiene `Cargo.toml`, pero `./myapp/Cargo.toml` existe.

### Salida (a Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Si no se necesita modificación, `updatedInput.command` está vacío y Claude Code usa el comando original.

## Detalles de las etapas

### Etapa 1: Analizar JSON

Lee el objeto JSON completo desde stdin. Extrae `tool_input.command`. Si el análisis falla, el hook sale inmediatamente y Claude Code usa el comando original (diseño fail-open).

### Etapa 2: Coincidencia de habilidades

Consulta la base de datos heurística SQLite para habilidades cuyo patrón de activación coincida con el comando. Las habilidades se verifican en orden de prioridad. Se evalúan tanto las habilidades TOML integradas como las minadas.

### Etapa 3: Corrección de directorio

Para comandos de compilación (`cargo`, `go`, `make`, `npm`, `python`, etc.), verifica si el archivo de proyecto esperado existe en el directorio actual. Si no, escanea directorios cercanos buscando la coincidencia más cercana y antepone `cd <dir> &&`.

El escaneo de directorios usa un índice de sistema de archivos en caché con un TTL de 5 segundos para mantenerse rápido.

### Etapa 4: Verificación GDB

Si el comando probablemente producirá un fallo (p. ej., ejecutar un binario de depuración), PRECC puede sugerir o inyectar envoltorios GDB para capturar salida de depuración estructurada en lugar de registros de fallos sin procesar.

### Etapa 5: Reescritura RTK

Aplica reglas RTK (Rewrite Toolkit) que acortan comandos verbosos, suprimen salida ruidosa o reestructuran comandos para eficiencia de tokens.

### Etapa 6: Emitir JSON

Serializa el comando modificado de vuelta a JSON y lo escribe en stdout. Si no se realizaron cambios, la salida señala a Claude Code que use el comando original.

## Rendimiento

Todo el pipeline se completa en menos de 5 milisegundos (p99). Optimizaciones clave:

- SQLite en modo WAL para lecturas concurrentes sin bloqueo
- Patrones regex precompilados para coincidencia de habilidades
- Escaneos de sistema de archivos en caché (TTL de 5 segundos)
- Sin llamadas de red en la ruta crítica
- Fail-open: cualquier error pasa al comando original

## Probar el Hook manualmente

Puedes invocar el hook directamente:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
