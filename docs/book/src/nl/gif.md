# GIF-opname

`precc gif` maakt geanimeerde GIF-opnames van terminalsessies vanuit bash-scripts. Dit is een Pro-functie.

## Basisgebruik

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Het eerste argument is een bash-script met de uit te voeren opdrachten. Het tweede argument is de maximale opnameduur.

## Scriptformaat

Het script is een standaard bash-bestand:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Invoersimulatie

Geef voor interactieve opdrachten invoerwaarden op als extra argumenten:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Elk extra argument wordt als een stdin-regel ingevoerd wanneer het script om invoer vraagt.

## Uitvoeropties

Het uitvoerbestand wordt standaard vernoemd naar het script (`script.gif`). De GIF gebruikt een donker terminalthema met standaard 80x24-afmetingen.

## Waarom GIF in plaats van asciinema?

De ingebouwde vaardigheid `asciinema-gif` herschrijft `asciinema rec` automatisch naar `precc gif`. GIF-bestanden zijn draagbaarder -- ze worden inline weergegeven in GitHub READMEs, Slack en e-mail zonder een speler nodig te hebben.
