# {{i18n:tel_title}}

{{i18n:tel_intro}}

## {{i18n:tel_opt_in_title}}

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## {{i18n:tel_opt_out_title}}

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## {{i18n:tel_check_status_title}}

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## {{i18n:tel_preview_title}}

{{i18n:tel_preview_body}}

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

## {{i18n:tel_collected_title}}

{{i18n:tel_collected_body}}

## {{i18n:tel_not_collected_title}}

{{i18n:tel_not_collected_body}}

## {{i18n:tel_env_override_title}}

{{i18n:tel_env_override_body}}

```bash
export PRECC_NO_TELEMETRY=1
```

{{i18n:tel_env_override_note}}

## {{i18n:tel_destination_title}}

{{i18n:tel_destination_body}}
