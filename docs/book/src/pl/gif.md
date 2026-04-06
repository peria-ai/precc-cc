# Nagrywanie GIF

`precc gif` tworzy animowane nagrania GIF sesji terminala ze skryptów bash. To funkcja Pro.

## Podstawowe użycie

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Pierwszy argument to skrypt bash zawierający polecenia do uruchomienia. Drugi argument to maksymalna długość nagrania.

## Format skryptu

Skrypt to standardowy plik bash:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Symulacja danych wejściowych

W przypadku poleceń interaktywnych podaj wartości wejściowe jako dodatkowe argumenty:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Każdy dodatkowy argument jest podawany jako linia stdin, gdy skrypt prosi o dane wejściowe.

## Opcje wyjścia

Plik wyjściowy jest domyślnie nazwany po skrypcie (`script.gif`). GIF używa ciemnego motywu terminala o standardowych wymiarach 80x24.

## Dlaczego GIF zamiast asciinema?

Wbudowana umiejętność `asciinema-gif` automatycznie zamienia `asciinema rec` na `precc gif`. Pliki GIF są bardziej przenośne -- wyświetlają się inline w README GitHub, Slack i e-mailu bez potrzeby odtwarzacza.
