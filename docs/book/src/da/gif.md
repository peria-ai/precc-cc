# GIF-optagelse

`precc gif` opretter animerede GIF-optagelser af terminalsessioner fra bash-scripts. Dette er en Pro-funktion.

## Grundlæggende brug

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Det første argument er et bash-script med kommandoerne der skal køres. Det andet argument er den maksimale optagelængde.

## Script-format

Scriptet er en standard bash-fil:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Input-simulering

Til interaktive kommandoer, angiv inputværdier som yderligere argumenter:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Hvert yderligere argument leveres som en linje stdin, når scriptet beder om input.

## Output-muligheder

Outputfilen er som standard opkaldt efter scriptet (`script.gif`). GIF'en bruger et mørkt terminaltema med standard 80x24-dimensioner.

## Hvorfor GIF i stedet for asciinema?

Den indbyggede færdighed `asciinema-gif` omskriver automatisk `asciinema rec` til `precc gif`. GIF-filer er mere portable -- de vises inline i GitHub READMEs, Slack og e-mail uden at kræve en afspiller.
