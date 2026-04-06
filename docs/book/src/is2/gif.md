# GIF-upptaka

`precc gif` býr til hreyfimynda-GIF upptökur af skjástöðvalotum úr bash-skriftum. Þetta er Pro-eiginleiki.

## Grunnnotkun

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Fyrsta viðfangið er bash-skrift með skipunum til að keyra. Annað viðfangið er hámarkslengd upptöku.

## Skriftasnið

Skriftan er venjuleg bash-skrá:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Inntaksherming

Fyrir gagnvirkar skipanir, gefðu inntaksgildi sem viðbótarviðföng:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Hvert viðbótarviðfang er gefið sem stdin-lína þegar skriftan biður um innslátt.

## Úttaksvalkostir

Úttaksskráin er sjálfgefið nefnd eftir skriftinni (`script.gif`). GIF notar dökkt skjástöðvuþema með stöðluðum 80x24 stærðum.

## Hvers vegna GIF í stað asciinema?

Innbyggða `asciinema-gif` þekkingin endurskrifar sjálfkrafa `asciinema rec` í `precc gif`. GIF-skrár eru flytjanlegri -- þær birtast beint í GitHub READMEs, Slack og tölvupósti án þess að þurfa spilara.
