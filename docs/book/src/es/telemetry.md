# Telemetría

PRECC admite telemetría anónima opcional para ayudar a mejorar la herramienta. No se recopilan datos a menos que usted consienta explícitamente.

## Activar

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Desactivar

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Verificar estado

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Vista previa de lo que se enviaría

Antes de activar, puede ver exactamente qué datos se recopilarían:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Qué se recopila

- Versión de PRECC, SO y arquitectura
- Conteos agregados: comandos interceptados, habilidades activadas, pilares utilizados
- Latencia promedio del hook
- Cantidad de sesiones

## Qué NO se recopila

- Sin texto de comandos ni argumentos
- Sin rutas de archivos ni nombres de directorios
- Sin nombres de proyectos ni URLs de repositorios
- Sin información personal identificable (PII)
- Sin direcciones IP (el servidor no las registra)

## Anulación por variable de entorno

Para desactivar la telemetría sin ejecutar un comando (útil en CI o entornos compartidos):

```bash
export PRECC_NO_TELEMETRY=1
```

Esto tiene prioridad sobre la configuración de consentimiento.

## Destino de los datos

Los datos de telemetría se envían a `https://telemetry.peria.ai/v1/precc` mediante HTTPS. Los datos se utilizan únicamente para comprender patrones de uso y priorizar el desarrollo.
