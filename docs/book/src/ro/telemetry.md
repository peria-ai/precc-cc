# Telemetrie

PRECC suportă telemetrie anonimă opt-in pentru a îmbunătăți instrumentul. Nicio dată nu este colectată decât dacă consimțiți explicit.

## Aderare

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Renunțare

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Verificare stare

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Previzualizarea datelor care ar fi trimise

Înainte de aderare, puteți vedea exact ce date ar fi colectate:

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

## Ce se colectează

- Versiunea PRECC, SO și arhitectura
- Numărători agregate: comenzi interceptate, abilități activate, piloni utilizați
- Latența medie a hook-ului
- Numărul de sesiuni

## Ce NU se colectează

- Fără text de comenzi sau argumente
- Fără căi de fișiere sau nume de directoare
- Fără nume de proiecte sau URL-uri de repository
- Fără informații personale identificabile (PII)
- Fără adrese IP (serverul nu le înregistrează)

## Suprascriere variabilă de mediu

Pentru a dezactiva telemetria fără a rula o comandă (util în CI sau medii partajate):

```bash
export PRECC_NO_TELEMETRY=1
```

Aceasta are prioritate față de setarea de consimțământ.

## Destinația datelor

Datele de telemetrie sunt trimise la `https://telemetry.peria.ai/v1/precc` prin HTTPS. Datele sunt folosite exclusiv pentru a înțelege tiparele de utilizare și a prioritiza dezvoltarea.
