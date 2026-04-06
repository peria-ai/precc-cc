# {{i18n:gif_title}}

{{i18n:gif_intro}}

## {{i18n:gif_basic_title}}

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

{{i18n:gif_basic_body}}

## {{i18n:gif_script_title}}

{{i18n:gif_script_body}}

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## {{i18n:gif_input_title}}

{{i18n:gif_input_body}}

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

{{i18n:gif_input_detail}}

## {{i18n:gif_output_title}}

{{i18n:gif_output_body}}

## {{i18n:gif_why_title}}

{{i18n:gif_why_body}}
