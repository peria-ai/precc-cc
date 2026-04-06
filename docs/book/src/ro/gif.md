# Înregistrare GIF

`precc gif` creează înregistrări GIF animate ale sesiunilor de terminal din scripturi bash. Aceasta este o funcție Pro.

## Utilizare de bază

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Primul argument este un script bash conținând comenzile de executat. Al doilea argument este lungimea maximă de înregistrare.

## Format script

Scriptul este un fișier bash standard:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulare intrare

Pentru comenzi interactive, furnizați valorile de intrare ca argumente suplimentare:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Fiecare argument suplimentar este furnizat ca o linie stdin când scriptul solicită intrare.

## Opțiuni de ieșire

Fișierul de ieșire este denumit implicit după script (`script.gif`). GIF-ul folosește o temă de terminal întunecată cu dimensiuni standard 80x24.

## De ce GIF în loc de asciinema?

Abilitatea integrată `asciinema-gif` rescrie automat `asciinema rec` în `precc gif`. Fișierele GIF sunt mai portabile -- se afișează inline în README-urile GitHub, Slack și e-mail fără a necesita un player.
