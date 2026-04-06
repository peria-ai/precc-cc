# Registrazione GIF

`precc gif` crea registrazioni GIF animate di sessioni terminale da script bash. Questa è una funzionalità Pro.

## Uso base

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Il primo argomento è uno script bash contenente i comandi da eseguire. Il secondo argomento è la durata massima della registrazione.

## Formato script

Lo script è un file bash standard:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulazione input

Per comandi interattivi, fornisci i valori di input come argomenti aggiuntivi:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Ogni argomento aggiuntivo viene fornito come riga di stdin quando lo script richiede un input.

## Opzioni di output

Il file di output prende il nome dallo script per impostazione predefinita (`script.gif`). La GIF usa un tema terminale scuro con dimensioni standard 80x24.

## Perché GIF invece di asciinema?

La skill integrata `asciinema-gif` riscrive automaticamente `asciinema rec` in `precc gif`. I file GIF sono più portabili -- si visualizzano inline nei README di GitHub, Slack e nelle email senza richiedere un player.
