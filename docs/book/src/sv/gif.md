# GIF-inspelning

`precc gif` skapar animerade GIF-inspelningar av terminalsessioner från bash-skript. Detta är en Pro-funktion.

## Grundläggande användning

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Det första argumentet är ett bash-skript med kommandona som ska köras. Det andra argumentet är den maximala inspelningslängden.

## Skriptformat

Skriptet är en standard bash-fil:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Inmatningssimulering

För interaktiva kommandon, ange inmatningsvärden som ytterligare argument:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Varje ytterligare argument matas som en rad stdin när skriptet frågar efter inmatning.

## Utdataalternativ

Utdatafilen namnges efter skriptet som standard (`script.gif`). GIF-filen använder ett mörkt terminaltema med standard 80x24-dimensioner.

## Varför GIF istället för asciinema?

Den inbyggda färdigheten `asciinema-gif` skriver automatiskt om `asciinema rec` till `precc gif`. GIF-filer är mer portabla -- de visas inline i GitHub READMEs, Slack och e-post utan att kräva en spelare.
