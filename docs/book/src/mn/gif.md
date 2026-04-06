# GIF бичлэг

`precc gif` нь bash скриптээс терминал сешний анимацитай GIF бичлэг үүсгэнэ. Энэ нь Pro боломж юм.

## Үндсэн хэрэглээ

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Эхний аргумент нь ажиллуулах командуудыг агуулсан bash скрипт юм. Хоёр дахь аргумент нь бичлэгийн дээд хэмжээ юм.

## Скриптийн формат

Скрипт нь стандарт bash файл юм:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Оролтын симуляц

Интерактив командын хувьд оролтын утгуудыг нэмэлт аргумент болгон өгнө:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Скрипт оролт хүсэх үед нэмэлт аргумент тус бүрийг stdin мөр болгон өгнө.

## Гаралтын сонголтууд

Гаралтын файл нь анхдагчаар скриптийн нэрээр нэрлэгдэнэ (`script.gif`). GIF нь стандарт 80x24 хэмжээтэй харанхуй терминал загварыг ашигладаг.

## Яагаад asciinema биш GIF вэ?

Суурилуулсан `asciinema-gif` чадвар нь `asciinema rec`-ийг автоматаар `precc gif` болгон дахин бичнэ. GIF файлууд илүү зөөврийн -- GitHub README, Slack, имэйлд тоглуулагч шаардахгүйгээр inline харагдана.
