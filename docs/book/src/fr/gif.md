# Enregistrement GIF

`precc gif` crée des enregistrements GIF animés de sessions de terminal à partir de scripts bash. C'est une fonctionnalité Pro.

## Utilisation de base

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Le premier argument est un script bash contenant les commandes à exécuter. Le second argument est la durée maximale d'enregistrement.

## Format du script

Le script est un fichier bash standard :

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulation d'entrée

Pour les commandes interactives, fournissez les valeurs d'entrée comme arguments supplémentaires :

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Chaque argument supplémentaire est fourni comme une ligne stdin lorsque le script demande une entrée.

## Options de sortie

Le fichier de sortie est nommé d'après le script par défaut (`script.gif`). Le GIF utilise un thème de terminal sombre avec des dimensions standard 80x24.

## Pourquoi GIF au lieu d'asciinema ?

La compétence intégrée `asciinema-gif` réécrit automatiquement `asciinema rec` en `precc gif`. Les fichiers GIF sont plus portables -- ils s'affichent en ligne dans les README GitHub, Slack et les e-mails sans nécessiter de lecteur.
