# GIF kaydı

`precc gif` bash betiklerinden terminal oturumlarının animasyonlu GIF kayıtlarını oluşturur. Bu bir Pro özelliğidir.

## Temel kullanım

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

İlk argüman çalıştırılacak komutları içeren bir bash betiğidir. İkinci argüman maksimum kayıt süresidir.

## Betik formatı

Betik standart bir bash dosyasıdır:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Girdi simülasyonu

Etkileşimli komutlar için giriş değerlerini ek argümanlar olarak sağlayın:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Her ek argüman, betik girdi istediğinde bir stdin satırı olarak beslenir.

## Çıktı seçenekleri

Çıktı dosyası varsayılan olarak betiğin adını alır (`script.gif`). GIF, standart 80x24 boyutlarında koyu bir terminal teması kullanır.

## Neden asciinema yerine GIF?

Yerleşik `asciinema-gif` becerisi `asciinema rec` komutunu otomatik olarak `precc gif` olarak yeniden yazar. GIF dosyaları daha taşınabilirdir -- GitHub README'lerinde, Slack'te ve e-postada bir oynatıcı gerektirmeden satır içi görüntülenirler.
