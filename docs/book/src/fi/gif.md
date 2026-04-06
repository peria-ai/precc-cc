# GIF-tallennus

`precc gif` luo animoituja GIF-tallenteita terminaali-istunnoista bash-skripteistä. Tämä on Pro-ominaisuus.

## Peruskäyttö

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Ensimmäinen argumentti on bash-skripti suoritettavine komentoineen. Toinen argumentti on tallennuksen enimmäispituus.

## Skriptimuoto

Skripti on tavallinen bash-tiedosto:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Syötteen simulointi

Interaktiivisille komennoille anna syötearvot lisäargumentteina:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Jokainen lisäargumentti syötetään stdin-rivinä, kun skripti pyytää syötettä.

## Tulostusvaihtoehdot

Tulostiedosto nimetään oletuksena skriptin mukaan (`script.gif`). GIF käyttää tummaa terminaaliteemaa vakiokokoisena 80x24.

## Miksi GIF ascineman sijaan?

Sisäänrakennettu `asciinema-gif`-taito kirjoittaa automaattisesti `asciinema rec` uudelleen muotoon `precc gif`. GIF-tiedostot ovat siirrettävämpiä -- ne näkyvät inline GitHubin README-tiedostoissa, Slackissa ja sähköpostissa ilman soitinta.
