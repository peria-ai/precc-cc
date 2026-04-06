# Instalación

## Instalación rápida (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Esto descarga el binario de la última versión para su plataforma, verifica la suma de comprobación SHA256 y lo coloca en `~/.local/bin/`.

Después de la instalación, inicialice PRECC:

```bash
precc init
```

`precc init` registra el hook PreToolUse con Claude Code, crea los directorios de datos e inicializa la base de datos de habilidades.

## Opciones de instalación

### Verificación SHA256

Por defecto, el instalador verifica la suma de comprobación del binario contra la suma SHA256 publicada. Para omitir la verificación (no recomendado):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Prefijo de instalación personalizado

Instalar en una ubicación personalizada:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Herramientas complementarias (--extras)

PRECC incluye herramientas complementarias opcionales. Instálelas con `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Esto instala:

| Herramienta | Propósito |
|------|---------|
| **RTK** | Kit de reescritura de comandos |
| **lean-ctx** | Compresión de contexto para CLAUDE.md y archivos de prompt |
| **nushell** | Shell estructurado para pipelines avanzados |
| **cocoindex-code** | Indexación de código para una resolución de contexto más rápida |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Luego inicialice:

```powershell
precc init
```

## Instalación manual

1. Descargue el binario de lanzamiento para su plataforma desde [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verifique la suma de comprobación SHA256 contra el archivo `.sha256` del lanzamiento.
3. Coloque el binario en un directorio de su `PATH` (p. ej., `~/.local/bin/`).
4. Ejecute `precc init`.

## Actualización

```bash
precc update
```

Forzar la actualización a una versión específica:

```bash
precc update --force --version 0.3.0
```

Habilitar actualizaciones automáticas:

```bash
precc update --auto
```

## Verificación de la instalación

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Si `precc` no se encuentra, asegúrese de que `~/.local/bin` esté en su `PATH`.
