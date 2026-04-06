# Introducción

## ¿Qué es PRECC?

PRECC (Corrección predictiva de errores para Claude Code) es una herramienta Rust que intercepta los comandos bash de Claude Code a través del mecanismo oficial de hooks PreToolUse. Corrige errores *antes de que ocurran*, ahorrando tokens y eliminando bucles de reintento.

Gratuito para usuarios de la comunidad.

## El problema

Claude Code desperdicia tokens significativos en errores prevenibles:

- **Errores de directorio** -- Ejecutar `cargo build` en un directorio padre sin `Cargo.toml`, y reintentar después de leer el error.
- **Bucles de reintento** -- Un comando fallido produce salida verbose, Claude la lee, razona y reintenta. Cada ciclo quema cientos de tokens.
- **Salida verbose** -- Comandos como `find` o `ls -R` generan miles de líneas que Claude debe procesar.

## Los cuatro pilares

### Corrección de contexto (cd-prepend)

Detecta cuando comandos como `cargo build` o `npm test` se ejecutan en el directorio incorrecto y antepone `cd /ruta/correcta &&` antes de la ejecución.

### Depuración GDB

Detecta oportunidades para adjuntar GDB para una depuración más profunda de segfaults y crashes, proporcionando información de depuración estructurada.

### Minería de sesiones

Mina los registros de sesión de Claude Code en busca de pares fallo-corrección. Cuando el mismo error recurre, PRECC ya conoce la corrección y la aplica automáticamente.

### Habilidades de automatización

Una biblioteca de habilidades integradas y minadas que coinciden con patrones de comandos y los reescriben. Las habilidades se definen como archivos TOML o filas SQLite.

## Cómo funciona (versión de 30 segundos)

1. Claude Code está a punto de ejecutar un comando bash.
2. El hook PreToolUse envía el comando a `precc-hook` como JSON por stdin.
3. `precc-hook` ejecuta el comando a través del pipeline (habilidades, corrección de directorio, compresión) en menos de 3 milisegundos.
4. El comando corregido se devuelve como JSON por stdout.
5. Claude Code ejecuta el comando corregido.

Claude nunca ve el error. Sin tokens desperdiciados.

### Compresión adaptativa

Si un comando falla después de la compresión, PRECC omite automáticamente la compresión en el reintento para que Claude obtenga la salida completa sin comprimir para depurar.

## Estadísticas de uso en vivo

Versión actual <span data-stat="current_version">--</span>:

| Métrica | Valor |
|---|---|
| Invocaciones del hook | <span data-stat="total_invocations">--</span> |
| Tokens ahorrados | <span data-stat="total_tokens_saved">--</span> |
| Ratio de ahorro | <span data-stat="saving_pct">--</span>% |
| Reescrituras RTK | <span data-stat="rtk_rewrites">--</span> |
| Correcciones CD | <span data-stat="cd_prepends">--</span> |
| Latencia del hook | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Usuarios | <span data-stat="unique_users">--</span> |

### Ahorro por versión

<table id="version-breakdown" style="display:none">
<thead><tr><th>Versión</th><th>Usuarios</th><th>Invocaciones del hook</th><th>Tokens ahorrados</th><th>Ratio de ahorro</th></tr></thead>
<tbody><tr><td colspan="5"><em>Cargando...</em></td></tr></tbody>
</table>

<small>Las cifras son estimaciones. Cada fallo prevenido evita un ciclo completo de reintento: salida de error, razonamiento del modelo y comando de reintento. Estos números se actualizan automáticamente desde telemetría anonimizada.</small>

## Enlaces

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Sitio web: [https://peria.ai](https://peria.ai)
- Documentación: [https://precc.cc](https://precc.cc)
