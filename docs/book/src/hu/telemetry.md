# Telemetria

A PRECC támogatja az opcionális anonim telemetriát az eszköz fejlesztése érdekében. Adatokat csak kifejezett hozzájárulás esetén gyűjtünk.

## Bekapcsolás

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Kikapcsolás

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Állapot ellenőrzése

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Az elküldendő adatok előnézete

A bekapcsolás előtt pontosan megtekintheti, milyen adatok kerülnének gyűjtésre:

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

## Mit gyűjtünk

- PRECC verzió, operációs rendszer és architektúra
- Összesített számok: elfogott parancsok, aktivált képességek, használt pillérek
- Átlagos hook késleltetés
- Munkamenetek száma

## Mit NEM gyűjtünk

- Nincs parancsszöveg vagy argumentum
- Nincsenek fájlútvonalak vagy könyvtárnevek
- Nincsenek projektnevek vagy tároló URL-ek
- Nincs személyazonosításra alkalmas információ (PII)
- Nincsenek IP-címek (a szerver nem naplózza őket)

## Környezeti változó felülírás

A telemetria letiltása parancs nélkül (hasznos CI vagy megosztott környezetekben):

```bash
export PRECC_NO_TELEMETRY=1
```

Ez felülírja a hozzájárulási beállítást.

## Adatcél

A telemetriai adatokat HTTPS-en keresztül küldjük a `https://telemetry.peria.ai/v1/precc` címre. Az adatokat kizárólag a használati minták megértésére és a fejlesztés priorizálására használjuk.
