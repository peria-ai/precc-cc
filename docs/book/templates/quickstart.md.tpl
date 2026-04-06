# {{i18n:qs_title}}

{{i18n:qs_subtitle}}

## {{i18n:qs_step1_title}}

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## {{i18n:qs_step2_title}}

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## {{i18n:qs_step3_title}}

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## {{i18n:qs_step4_title}}

{{i18n:qs_step4_body}}

### {{i18n:qs_example_title}}

{{i18n:qs_example_setup}}

```
cargo build
```

{{i18n:qs_example_context}}

{{i18n:qs_without_precc}}

{{i18n:qs_with_precc}}

```
cd /home/user/projects/myapp && cargo build
```

{{i18n:qs_example_result}}

## {{i18n:qs_step5_title}}

{{i18n:qs_step5_body}}

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## {{i18n:qs_next_steps_title}}

- [{{i18n:qs_link_skills}}](skills.md) -- {{i18n:qs_link_skills_desc}}
- [{{i18n:qs_link_pipeline}}](hook-pipeline.md) -- {{i18n:qs_link_pipeline_desc}}
- [{{i18n:qs_link_savings}}](savings.md) -- {{i18n:qs_link_savings_desc}}
