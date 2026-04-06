# {{i18n:faq_title}}

## {{i18n:faq_safe_q}}

{{i18n:faq_safe_a}}

## {{i18n:faq_other_tools_q}}

{{i18n:faq_other_tools_a}}

## {{i18n:faq_telemetry_q}}

{{i18n:faq_telemetry_a}}

## {{i18n:faq_uninstall_q}}

{{i18n:faq_uninstall_a_intro}}

1. {{i18n:faq_uninstall_step1}}
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. {{i18n:faq_uninstall_step2}}
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. {{i18n:faq_uninstall_step3}}
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## {{i18n:faq_license_q}}

{{i18n:faq_license_a_community}}

{{i18n:faq_license_a_pro}}

## {{i18n:faq_debug_q}}

{{i18n:faq_debug_a_intro}}

1. {{i18n:faq_debug_step1}}
   ```bash
   precc init
   ```

2. {{i18n:faq_debug_step2}}
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. {{i18n:faq_debug_step3}}
   ```bash
   which precc-hook
   ```

4. {{i18n:faq_debug_step4}}

## {{i18n:faq_slow_q}}

{{i18n:faq_slow_a}}

## {{i18n:faq_cicd_q}}

{{i18n:faq_cicd_a}}

## {{i18n:faq_mined_q}}

{{i18n:faq_mined_a}}

## {{i18n:faq_share_q}}

{{i18n:faq_share_a}}
