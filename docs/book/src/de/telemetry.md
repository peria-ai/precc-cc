# Telemetrie

PRECC unterstützt optionale anonyme Telemetrie zur Verbesserung des Tools. Es werden keine Daten erfasst, es sei denn, Sie stimmen ausdrücklich zu.

## Aktivieren

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Deaktivieren

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Status prüfen

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Vorschau der zu sendenden Daten

Vor der Aktivierung können Sie genau sehen, welche Daten erfasst würden:

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

## Was erfasst wird

- PRECC-Version, Betriebssystem und Architektur
- Aggregierte Zähler: abgefangene Befehle, aktivierte Skills, verwendete Säulen
- Durchschnittliche Hook-Latenz
- Anzahl der Sitzungen

## Was NICHT erfasst wird

- Kein Befehlstext oder Argumente
- Keine Dateipfade oder Verzeichnisnamen
- Keine Projektnamen oder Repository-URLs
- Keine personenbezogenen Daten (PII)
- Keine IP-Adressen (der Server protokolliert sie nicht)

## Umgebungsvariable überschreiben

Um Telemetrie ohne Befehl zu deaktivieren (nützlich in CI oder gemeinsamen Umgebungen):

```bash
export PRECC_NO_TELEMETRY=1
```

Dies hat Vorrang vor der Einwilligungseinstellung.

## Datenziel

Telemetriedaten werden über HTTPS an `https://telemetry.peria.ai/v1/precc` gesendet. Die Daten werden ausschließlich verwendet, um Nutzungsmuster zu verstehen und die Entwicklung zu priorisieren.
