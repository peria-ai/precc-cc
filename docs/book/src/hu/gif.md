# GIF-felvétel

A `precc gif` animált GIF-felvételeket készít terminál-munkamenetekről bash szkriptekből. Ez egy Pro funkció.

## Alapvető használat

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Az első argumentum egy bash szkript a futtatandó parancsokkal. A második argumentum a maximális felvételi hossz.

## Szkript formátum

A szkript egy szabványos bash fájl:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Bemenet szimuláció

Interaktív parancsoknál adja meg a bemeneti értékeket további argumentumként:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Minden további argumentum stdin-sorként kerül beadásra, amikor a szkript bemenetet kér.

## Kimeneti beállítások

A kimeneti fájl alapértelmezés szerint a szkriptről kap nevet (`script.gif`). A GIF sötét terminál-témát használ szabványos 80x24-es méretekkel.

## Miért GIF az asciinema helyett?

Az `asciinema-gif` beépített készség automatikusan átírja az `asciinema rec`-et `precc gif`-re. A GIF-fájlok hordozhatóbbak -- megjelennek a GitHub README-kban, Slackben és e-mailben lejátszó nélkül.
