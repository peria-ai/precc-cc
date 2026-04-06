# Telemetry

PRECC supports opt-in anonymous telemetry to help improve the tool. No data is collected unless you explicitly consent.

## Opting In

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Opting Out

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Checking Status

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Previewing What Would Be Sent

Before opting in, you can see exactly what data would be collected:

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

## What Is Collected

- PRECC version, OS, and architecture
- Aggregate counts: commands intercepted, skills activated, pillars used
- Average hook latency
- Session count

## What Is NOT Collected

- No command text or arguments
- No file paths or directory names
- No project names or repository URLs
- No personally identifiable information (PII)
- No IP addresses (the server does not log them)

## Environment Variable Override

To disable telemetry without running a command (useful in CI or shared environments):

```bash
export PRECC_NO_TELEMETRY=1
```

This takes precedence over the consent setting.

## Data Destination

Telemetry data is sent to `https://telemetry.peria.ai/v1/precc` over HTTPS. The data is used solely to understand usage patterns and prioritize development.
