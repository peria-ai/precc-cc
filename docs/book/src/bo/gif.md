# GIF Recording

`precc gif` creates animated GIF recordings of terminal sessions from bash scripts. This is a Pro feature.

## Basic Usage

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

The first argument is a bash script containing the commands to run. The second argument is the maximum recording length.

## Script Format

The script is a standard bash file:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Input Simulation

For interactive commands, provide input values as additional arguments:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Each additional argument is fed as a line of stdin when the script prompts for input.

## Output Options

The output file is named after the script by default (`script.gif`). The GIF uses a dark terminal theme with standard 80x24 dimensions.

## Why GIF Instead of asciinema?

The `asciinema-gif` built-in skill automatically rewrites `asciinema rec` to `precc gif`. GIF files are more portable -- they display inline in GitHub READMEs, Slack, and email without requiring a player.
