# Grabación GIF

`precc gif` crea grabaciones GIF animadas de sesiones de terminal a partir de scripts bash. Esta es una función Pro.

## Uso básico

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

El primer argumento es un script bash que contiene los comandos a ejecutar. El segundo argumento es la duración máxima de la grabación.

## Formato del script

El script es un archivo bash estándar:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulación de entrada

Para comandos interactivos, proporcione valores de entrada como argumentos adicionales:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Cada argumento adicional se proporciona como una línea de stdin cuando el script solicita entrada.

## Opciones de salida

El archivo de salida se nombra según el script por defecto (`script.gif`). El GIF usa un tema de terminal oscuro con dimensiones estándar 80x24.

## ¿Por qué GIF en lugar de asciinema?

La habilidad integrada `asciinema-gif` reescribe automáticamente `asciinema rec` a `precc gif`. Los archivos GIF son más portátiles: se muestran en línea en READMEs de GitHub, Slack y correo electrónico sin necesidad de un reproductor.
