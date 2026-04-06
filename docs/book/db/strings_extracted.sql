-- Extracted English text for template keys

INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_title', 'cmd title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_subtitle', 'cmd subtitle');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_init_desc', 'Initialize PRECC and register the hook with Claude Code.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_ingest_desc', 'Mine session logs for failure-fix patterns.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_skills_desc', 'Manage automation skills.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_report_desc', 'Generate an analytics report.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_savings_desc', 'Show token savings.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_compress_desc', 'Compress context files to reduce token usage.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_license_desc', 'Manage your PRECC license.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_mail_desc', 'Email functionality.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_update_desc', 'Update PRECC to the latest version.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_telemetry_desc', 'Manage anonymous telemetry.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_geofence_desc', 'IP geofence compliance (Pro).');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_gif_desc', 'Record animated GIFs from bash scripts (Pro).');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_gha_desc', 'Analyze failed GitHub Actions runs (Pro).');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_cachehint_desc', 'Display cache hint information for the current project.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_trial_desc', 'Start a Pro trial.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmd_nushell_desc', 'Launch a Nushell session with PRECC integration.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_title', 'cmp title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_intro', 'cmp intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_basic_title', 'cmp basic title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_dryrun_title', 'cmp dryrun title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_dryrun_body', 'cmp dryrun body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_revert_title', '[precc] Files compressed. Use --revert to restore originals.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_revert_body', '[precc] Files compressed. Use --revert to restore originals.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_what_title', 'cmp what title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_what_body', 'cmp what body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_readable_note', 'cmp readable note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('cmp_target_title', 'cmp target title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_title', 'eml title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_intro', 'eml intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_setup_title', 'eml setup title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_config_title', 'eml config title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_config_body', 'eml config body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_config_edit', 'eml config edit');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_gmail_note', 'eml gmail note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_sending_reports_title', '[precc] Sending test email to you@gmail.com...
[precc] Test email sent successfully.');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_sending_files_title', 'eml sending files title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_ssh_relay_title', 'eml ssh relay title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_ssh_relay_body', 'eml ssh relay body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('eml_ssh_relay_auto', 'eml ssh relay auto');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_title', 'faq title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_safe_q', 'faq safe q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_safe_a', 'faq safe a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_other_tools_q', 'faq other tools q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_other_tools_a', 'faq other tools a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_telemetry_q', 'faq telemetry q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_telemetry_a', 'faq telemetry a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_uninstall_q', 'faq uninstall q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_uninstall_a_intro', 'faq uninstall a intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_uninstall_step1', 'faq uninstall step1');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_uninstall_step2', 'faq uninstall step2');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_uninstall_step3', 'faq uninstall step3');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_license_q', 'faq license q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_license_a_community', 'faq license a community');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_license_a_pro', 'faq license a pro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_q', 'faq debug q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_a_intro', 'faq debug a intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_step1', 'faq debug step1');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_step2', 'added it; removing it disables PRECC)
   ```');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_step3', 'faq debug step3');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_debug_step4', 'faq debug step4');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_slow_q', 'faq slow q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_slow_a', 'faq slow a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_cicd_q', 'faq cicd q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_cicd_a', 'faq cicd a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_mined_q', 'faq mined q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_mined_a', 'faq mined a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_share_q', 'faq share q');
INSERT OR IGNORE INTO strings (key, en) VALUES ('faq_share_a', 'faq share a');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_title', 'geo title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_intro', 'geo intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_overview_title', 'geo overview title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_overview_body', 'geo overview body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_check_title', 'geo check title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_check_noncompliant', 'geo check noncompliant');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_refresh_title', 'geo refresh title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_info_title', 'geo info title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_clear_title', 'geo clear title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_config_title', 'geo config title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_config_body', 'geo config body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('geo_config_block', 'geo config block');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_title', 'gha title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_intro', 'gha intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_usage_title', 'gha usage title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_usage_body', 'gha usage body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_what_title', 'gha what title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_what_body', 'gha what body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_patterns_title', 'gha patterns title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gha_patterns_body', 'gha patterns body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_title', 'gif title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_intro', 'gif intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_basic_title', 'gif basic title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_basic_body', 'gif basic body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_script_title', 'gif script title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_script_body', 'gif script body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_input_title', 'gif input title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_input_body', 'gif input body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_input_detail', 'gif input detail');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_output_title', 'gif output title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_output_body', 'gif output body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_why_title', 'gif why title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('gif_why_body', 'gif why body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_title', 'hp title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_intro', 'hp intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_invocation_title', 'hp invocation title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_invocation_body', 'hp invocation body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stages_title', 'hp stages title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_example_title', 'hp example title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_example_input', 'hp example input');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_example_explanation', '}
}');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_example_output', '}
}');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_no_modification_note', '}
  }
}');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage_details_title', '}
}');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage1_title', 'hp stage1 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage1_body', 'hp stage1 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage2_title', 'hp stage2 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage2_body', 'hp stage2 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage3_title', 'hp stage3 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage3_body', 'hp stage3 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage3_cache_note', 'hp stage3 cache note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage4_title', 'hp stage4 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage4_body', 'hp stage4 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage5_title', 'hp stage5 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage5_body', 'hp stage5 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage6_title', 'hp stage6 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_stage6_body', 'hp stage6 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_performance_title', 'hp performance title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_performance_intro', 'hp performance intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_perf_wal', 'hp perf wal');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_perf_regex', 'hp perf regex');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_perf_cache', 'hp perf cache');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_perf_no_network', 'hp perf no network');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_perf_failopen', 'hp perf failopen');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_testing_title', 'hp testing title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('hp_testing_body', 'hp testing body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_title', 'install title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_quick_title', 'install quick title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_quick_body', 'install quick body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_after_init', 'install after init');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_init_desc', 'install init desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_options_title', 'install options title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_sha256_title', 'install sha256 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_sha256_body', 'install sha256 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_prefix_title', 'install prefix title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_prefix_body', 'install prefix body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_extras_title', 'install extras title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_extras_body', 'install extras body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_extras_installs', 'install extras installs');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_tool_col', 'install tool col');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_purpose_col', 'install purpose col');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_rtk_desc', 'Command rewriting toolkit |
| **lean-ctx** | Context compression for CLAUDE.md and prompt files |
| **nushell** | Structured shell for advanced pipelines |
| **cocoindex-code** | Code indexing for faster context resolution |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_leanctx_desc', 'Context compression for CLAUDE.md and prompt files |
| **nushell** | Structured shell for advanced pipelines |
| **cocoindex-code** | Code indexing for faster context resolution |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_nushell_desc', 'Structured shell for advanced pipelines |
| **cocoindex-code** | Code indexing for faster context resolution |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_cocoindex_desc', 'Code indexing for faster context resolution |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_windows_title', 'install windows title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_windows_then', 'install windows then');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_manual_title', 'install manual title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_manual_step1', 'install manual step1');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_manual_step2', 'install manual step2');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_manual_step3', 'install manual step3');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_manual_step4', 'install manual step4');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_updating_title', 'install updating title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_update_force', 'install update force');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_update_auto', 'install update auto');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_verify_title', 'install verify title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('install_verify_path_note', 'install verify path note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_title', 'lic title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_intro', 'lic intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_title', 'lic community title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_includes', 'lic community includes');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_skills', 'lic community skills');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_pipeline', 'lic community pipeline');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_savings', 'lic community savings');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_mining', 'lic community mining');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_community_unlimited', 'lic community unlimited');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_title', 'lic pro title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_unlocks', 'lic pro unlocks');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_detailed', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_detailed_desc', 'lic pro detailed desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_gif', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_gif_desc', 'lic pro gif desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_geofence', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_geofence_desc', 'lic pro geofence desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_email', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_email_desc', 'lic pro email desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_gha', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_gha_desc', 'lic pro gha desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_compress', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_compress_desc', 'lic pro compress desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_pro_support', 'Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_activate_title', 'lic activate title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_status_title', 'lic status title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_sponsors_title', 'lic sponsors title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_sponsors_body', 'lic sponsors body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_fingerprint_title', 'lic fingerprint title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_fingerprint_body', 'lic fingerprint body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_transfer_body', 'lic transfer body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_transfer_then', 'lic transfer then');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_expired_title', 'lic expired title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('lic_expired_body', 'lic expired body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_title', 'min title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_intro', 'min intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_ingest_title', 'min ingest title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_ingest_single_title', 'min ingest single title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_ingest_all_title', 'min ingest all title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_force_title', 'min force title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_force_body', 'min force body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_how_title', 'min how title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_how_body', 'min how body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_example_title', 'min example title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_daemon_title', 'min daemon title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_daemon_intro', 'min daemon intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_daemon_notify', 'min daemon notify');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_graduation_title', 'min graduation title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_graduation_body', 'min graduation body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_graduation_review', 'min graduation review');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_graduation_see_skills', 'min graduation see skills');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_storage_title', 'min storage title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_storage_pairs', 'Failure-fix pairs**: `~/.local/share/precc/history.db`
- **Graduated skills**: `~/.local/share/precc/heuristics.db`');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_storage_skills', 'Failure-fix pairs**: `~/.local/share/precc/history.db`
- **Graduated skills**: `~/.local/share/precc/heuristics.db`');
INSERT OR IGNORE INTO strings (key, en) VALUES ('min_storage_note', 'min storage note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_title', 'qs title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_subtitle', 'qs subtitle');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step1_title', 'qs step1 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step2_title', 'qs step2 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step3_title', 'qs step3 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step4_title', 'qs step4 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step4_body', 'qs step4 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_example_title', 'qs example title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_example_setup', 'qs example setup');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_example_context', '/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_without_precc', '/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_with_precc', 'qs with precc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_example_result', 'qs example result');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step5_title', 'qs step5 title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_step5_body', 'qs step5 body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_next_steps_title', 'qs next steps title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_skills', 'qs link skills');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_skills_desc', 'qs link skills desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_pipeline', 'qs link pipeline');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_pipeline_desc', 'qs link pipeline desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_savings', 'qs link savings');
INSERT OR IGNORE INTO strings (key, en) VALUES ('qs_link_savings_desc', 'qs link savings desc');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_title', 'rpt title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_intro', 'rpt intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_generating_title', 'rpt generating title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_email_title', '...');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_email_body', '...');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_email_config', 'rpt email config');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_data_title', 'rpt data title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('rpt_data_body', 'rpt data body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_title', 'sav title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_intro', 'sav intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_quick_summary_title', 'sav quick summary title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_detailed_title', 'sav detailed title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_how_title', 'sav how title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_how_intro', 'sav how intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_col_type', 'sav col type');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_col_saving', 'sav col saving');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_col_reasoning', 'sav col reasoning');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_reason_cd', 'Error output + Claude reasoning + retry |
| Skill activation | ~400 tokens | Error output + Claude reasoning + retry |
| RTK rewrite | ~250 tokens | Verbose output that Claude would have to read |
| Lean-ctx wrap | ~600 tokens | Large file contents compressed |
| Mined prevention | ~500 tokens | Known failure pattern avoided |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_skill_activation', 'sav skill activation');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_reason_skill', 'sav reason skill');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_reason_rtk', 'Verbose output that Claude would have to read |
| Lean-ctx wrap | ~600 tokens | Large file contents compressed |
| Mined prevention | ~500 tokens | Known failure pattern avoided |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_reason_lean', 'Large file contents compressed |
| Mined prevention | ~500 tokens | Known failure pattern avoided |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_mined_prevention', 'sav mined prevention');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_reason_mined', 'sav reason mined');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_conservative_note', 'sav conservative note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_cumulative_title', 'sav cumulative title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sav_cumulative_body', 'sav cumulative body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_title', 'sk title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_intro', 'sk intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_builtin_title', 'sk builtin title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_col_skill', 'sk col skill');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_col_triggers', 'sk col triggers');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_col_action', 'sk col action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_cargo_trigger', '`cargo build/test/clippy` outside a Rust project | Prepend `cd` to the nearest `Cargo.toml` directory |
| `git-wrong-dir` | `git *` outside a git repo | Prepend `cd` to the nearest `.git` directory |
| `go-wrong-dir` | `go build/test` outside a Go module | Prepend `cd` to the nearest `go.mod` directory |
| `make-wrong-dir` | `make` without a Makefile in cwd | Prepend `cd` to the nearest Makefile directory |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_cargo_action', 'sk cargo action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_git_trigger', '`git *` outside a git repo | Prepend `cd` to the nearest `.git` directory |
| `go-wrong-dir` | `go build/test` outside a Go module | Prepend `cd` to the nearest `go.mod` directory |
| `make-wrong-dir` | `make` without a Makefile in cwd | Prepend `cd` to the nearest Makefile directory |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_git_action', 'sk git action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_go_trigger', '`go build/test` outside a Go module | Prepend `cd` to the nearest `go.mod` directory |
| `make-wrong-dir` | `make` without a Makefile in cwd | Prepend `cd` to the nearest Makefile directory |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_go_action', 'sk go action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_make_trigger', '`make` without a Makefile in cwd | Prepend `cd` to the nearest Makefile directory |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_make_action', 'sk make action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_npm_trigger', '`npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_npm_action', 'sk npm action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_python_trigger', '`python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_python_action', 'sk python action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_jj_trigger', '`git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_jj_action', 'sk jj action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_asciinema_trigger', '`asciinema rec` | Rewrite to `precc gif` |');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_asciinema_action', 'sk asciinema action');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_listing_title', 'sk listing title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_showing_title', 'sk showing title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_exporting_title', 'sk exporting title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_editing_title', 'sk editing title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_editing_body', 'sk editing body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_advise_title', 'sk advise title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_advise_body', 'sk advise body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_clustering_title', 'sk clustering title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_clustering_body', 'sk clustering body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_mined_vs_builtin_title', 'sk mined vs builtin title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('sk_mined_vs_builtin_body', 'sk mined vs builtin body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_title', 'tel title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_intro', 'tel intro');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_opt_in_title', 'tel opt in title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_opt_out_title', 'tel opt out title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_check_status_title', 'tel check status title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_preview_title', 'tel preview title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_preview_body', 'tel preview body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_collected_title', 'tel collected title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_collected_body', 'tel collected body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_not_collected_title', 'tel not collected title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_not_collected_body', 'tel not collected body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_env_override_title', 'tel env override title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_env_override_body', 'tel env override body');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_env_override_note', 'tel env override note');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_destination_title', 'tel destination title');
INSERT OR IGNORE INTO strings (key, en) VALUES ('tel_destination_body', 'tel destination body');
