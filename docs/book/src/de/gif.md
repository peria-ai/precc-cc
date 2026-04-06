# GIF-Aufnahme

`precc gif` erstellt animierte GIF-Aufnahmen von Terminal-Sitzungen aus Bash-Skripten. Dies ist eine Pro-Funktion.

## Grundlegende Verwendung

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Das erste Argument ist ein Bash-Skript mit den auszuführenden Befehlen. Das zweite Argument ist die maximale Aufnahmedauer.

## Skriptformat

Das Skript ist eine Standard-Bash-Datei:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Eingabesimulation

Für interaktive Befehle geben Sie Eingabewerte als zusätzliche Argumente an:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Jedes zusätzliche Argument wird als stdin-Zeile übergeben, wenn das Skript nach Eingabe fragt.

## Ausgabeoptionen

Die Ausgabedatei wird standardmäßig nach dem Skript benannt (`script.gif`). Das GIF verwendet ein dunkles Terminal-Theme mit Standard-80x24-Dimensionen.

## Warum GIF statt asciinema?

Der eingebaute Skill `asciinema-gif` schreibt `asciinema rec` automatisch in `precc gif` um. GIF-Dateien sind portabler -- sie werden inline in GitHub-READMEs, Slack und E-Mails angezeigt, ohne einen Player zu benötigen.
