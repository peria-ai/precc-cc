# Nahrávání GIF

`precc gif` vytváří animované GIF nahrávky terminálových relací z bash skriptů. Toto je funkce Pro.

## Základní použití

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

První argument je bash skript obsahující příkazy ke spuštění. Druhý argument je maximální délka nahrávky.

## Formát skriptu

Skript je standardní bash soubor:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulace vstupu

Pro interaktivní příkazy zadejte vstupní hodnoty jako další argumenty:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Každý další argument je podán jako řádek stdin, když skript žádá o vstup.

## Možnosti výstupu

Výstupní soubor je pojmenován podle skriptu (`script.gif`). GIF používá tmavý terminálový motiv se standardními rozměry 80x24.

## Proč GIF místo asciinema?

Vestavěná dovednost `asciinema-gif` automaticky přepisuje `asciinema rec` na `precc gif`. GIF soubory jsou přenosnější -- zobrazují se inline v GitHub README, Slack a e-mailu bez nutnosti přehrávače.
