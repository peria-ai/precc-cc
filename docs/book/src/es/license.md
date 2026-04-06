# Licencia

PRECC ofrece dos niveles: Community (gratuito) y Pro.

## Nivel Community (gratuito)

El nivel Community incluye:

- Todas las habilidades integradas (corrección de directorio, traducción jj, etc.)
- Pipeline de hooks con soporte completo de Pillar 1 y Pillar 4
- Resumen básico de `precc savings`
- Minería de sesiones con `precc ingest`
- Uso local ilimitado

## Nivel Pro

Pro desbloquea funciones adicionales:

- **Desglose detallado de ahorros** -- `precc savings --all` con análisis por comando
- **Grabación de GIF** -- `precc gif` para crear GIFs animados de terminal
- **Cumplimiento de geovalla IP** -- Para entornos regulados
- **Informes por correo** -- `precc mail report` para enviar analíticas
- **Análisis de GitHub Actions** -- `precc gha` para depuración de workflows fallidos
- **Compresión de contexto** -- `precc compress` para optimización de CLAUDE.md
- **Soporte prioritario**

## Activar una licencia

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Verificar el estado de la licencia

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Activación por GitHub Sponsors

Si patrocinas PRECC a través de GitHub Sponsors, tu licencia se activa automáticamente con tu correo de GitHub. No se necesita clave -- solo asegúrate de que tu correo de patrocinador coincida:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Huella del dispositivo

Cada licencia está vinculada a una huella de dispositivo. Consulta la tuya con:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Si necesitas transferir tu licencia a una máquina nueva, desactívala primero:

```bash
precc license deactivate
```

Luego activa en la nueva máquina.

## ¿Licencia expirada?

Cuando una licencia Pro expira, PRECC vuelve al nivel Community. Todas las habilidades integradas y la funcionalidad principal siguen funcionando. Solo las funciones específicas de Pro dejan de estar disponibles. Consulta las [FAQ](faq.md) para más detalles.
