# Telemetria

PRECC obsługuje opcjonalną anonimową telemetrię w celu ulepszenia narzędzia. Żadne dane nie są zbierane bez Twojej wyraźnej zgody.

## Włączenie

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Wyłączenie

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Sprawdzanie statusu

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Podgląd wysyłanych danych

Przed włączeniem możesz zobaczyć dokładnie, jakie dane byłyby zbierane:

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

## Co jest zbierane

- Wersja PRECC, system operacyjny i architektura
- Zagregowane liczby: przechwycone polecenia, aktywowane umiejętności, użyte filary
- Średnie opóźnienie hooka
- Liczba sesji

## Czego NIE zbieramy

- Brak tekstu poleceń lub argumentów
- Brak ścieżek plików lub nazw katalogów
- Brak nazw projektów lub adresów URL repozytoriów
- Brak danych osobowych (PII)
- Brak adresów IP (serwer ich nie rejestruje)

## Nadpisanie zmienną środowiskową

Aby wyłączyć telemetrię bez uruchamiania polecenia (przydatne w CI lub środowiskach współdzielonych):

```bash
export PRECC_NO_TELEMETRY=1
```

To ma pierwszeństwo nad ustawieniem zgody.

## Miejsce docelowe danych

Dane telemetryczne są wysyłane do `https://telemetry.peria.ai/v1/precc` przez HTTPS. Dane są wykorzystywane wyłącznie do zrozumienia wzorców użycia i priorytetyzacji rozwoju.
