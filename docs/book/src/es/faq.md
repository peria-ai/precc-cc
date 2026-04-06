# Preguntas frecuentes

## ¿Es seguro usar PRECC?

Sí. PRECC usa el mecanismo oficial de hooks PreToolUse de Claude Code -- el mismo punto de extensión que Anthropic diseñó exactamente para este propósito. El hook:

- Se ejecuta completamente sin conexión (sin llamadas de red en la ruta crítica)
- Se completa en menos de 5 milisegundos
- Es fail-open: si algo sale mal, el comando original se ejecuta sin modificar
- Solo modifica comandos, nunca los ejecuta por sí mismo
- Almacena datos localmente en bases de datos SQLite

## ¿PRECC funciona con otras herramientas de codificación IA?

PRECC está diseñado específicamente para Claude Code. Depende del protocolo de hooks PreToolUse que proporciona Claude Code. No funciona con Cursor, Copilot, Windsurf u otras herramientas de codificación IA.

## ¿Qué datos envía la telemetría?

La telemetría es solo por suscripción. Cuando está habilitada, envía:

- Versión de PRECC, SO y arquitectura
- Conteos agregados (comandos interceptados, habilidades activadas)
- Latencia promedio del hook

**No** envía texto de comandos, rutas de archivos, nombres de proyectos ni información personal identificable. Puede previsualizar la carga exacta con `precc telemetry preview` antes de suscribirse. Vea [Telemetría](telemetry.md) para más detalles.

## ¿Cómo desinstalo PRECC?

??faq_uninstall_a_intro??

1. Eliminar el registro del hook:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Eliminar el binario:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Eliminar datos (opcional):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Mi licencia expiró. ¿Qué sucede?

PRECC vuelve al nivel Community. Toda la funcionalidad principal sigue funcionando:

- Las habilidades integradas permanecen activas
- El pipeline del hook funciona normalmente
- `precc savings` muestra la vista resumida
- `precc ingest` y la minería de sesiones funcionan

Las funciones Pro dejan de estar disponibles hasta que renueve:

- `precc savings --all` (desglose detallado)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Informes por correo electrónico

## El hook no parece estar ejecutándose. ¿Cómo depuro?

??faq_debug_a_intro??

1. Verifique que el hook esté registrado:
   ```bash
   precc init
   ```

2. Pruebe el hook manualmente:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Verifique que el binario esté en su PATH:
   ```bash
   which precc-hook
   ```

4. Verifique la configuración del hook de Claude Code en `~/.claude/settings.json`.

## ¿PRECC ralentiza Claude Code?

No. El hook se completa en menos de 5 milisegundos (p99). Esto es imperceptible comparado con el tiempo que Claude dedica a razonar y generar respuestas.

## ¿Puedo usar PRECC en CI/CD?

PRECC está diseñado para sesiones interactivas de Claude Code. En CI/CD, no hay una instancia de Claude Code a la que engancharse. Sin embargo, `precc gha` puede analizar ejecuciones fallidas de GitHub Actions desde cualquier entorno.

## ¿En qué se diferencian las habilidades minadas de las integradas?

Las habilidades integradas vienen con PRECC y cubren patrones comunes de directorio incorrecto. Las habilidades minadas se aprenden de sus registros de sesión específicos -- capturan patrones únicos de su flujo de trabajo. Ambas se almacenan en SQLite y se evalúan de forma idéntica por el pipeline del hook.

## ¿Puedo compartir habilidades con mi equipo?

Sí. Exporte cualquier habilidad a TOML con `precc skills export NAME` y comparta el archivo. Los miembros del equipo pueden colocarlo en su directorio `skills/` o importarlo a su base de datos de heurísticas.
