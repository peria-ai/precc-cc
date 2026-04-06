# {{i18n:install_title}}

## {{i18n:install_quick_title}}

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

{{i18n:install_quick_body}}

{{i18n:install_after_init}}

```bash
precc init
```

{{i18n:install_init_desc}}

## {{i18n:install_options_title}}

### {{i18n:install_sha256_title}}

{{i18n:install_sha256_body}}

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### {{i18n:install_prefix_title}}

{{i18n:install_prefix_body}}

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### {{i18n:install_extras_title}}

{{i18n:install_extras_body}}

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

{{i18n:install_extras_installs}}

| {{i18n:install_tool_col}} | {{i18n:install_purpose_col}} |
|------|---------|
| **RTK** | {{i18n:install_rtk_desc}} |
| **lean-ctx** | {{i18n:install_leanctx_desc}} |
| **nushell** | {{i18n:install_nushell_desc}} |
| **cocoindex-code** | {{i18n:install_cocoindex_desc}} |

## {{i18n:install_windows_title}}

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

{{i18n:install_windows_then}}

```powershell
precc init
```

## {{i18n:install_manual_title}}

1. {{i18n:install_manual_step1}}
2. {{i18n:install_manual_step2}}
3. {{i18n:install_manual_step3}}
4. {{i18n:install_manual_step4}}

## {{i18n:install_updating_title}}

```bash
precc update
```

{{i18n:install_update_force}}

```bash
precc update --force --version 0.3.0
```

{{i18n:install_update_auto}}

```bash
precc update --auto
```

## {{i18n:install_verify_title}}

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

{{i18n:install_verify_path_note}}
