# Лиценз

PRECC хоёр түвшин санал болгодог: Community (үнэгүй) ба Pro.

## Community түвшин (үнэгүй)

Community түвшинд багтана:

- Бүх суулгагдсан ур чадвар (директорын засвар, jj орчуулга г.м.)
- Pillar 1, Pillar 4-ийг бүрэн дэмждэг Hook pipeline
- Үндсэн `precc savings` товч
- `precc ingest` ашиглан сессийн олборлолт
- Хязгааргүй орон нутгийн хэрэглээ

## Pro түвшин

Pro нэмэлт боломжуудыг нээдэг:

- **Хэмнэлтийн дэлгэрэнгүй задаргаа** -- `precc savings --all` тушаал тус бүрийн шинжилгээтэй
- **GIF бичлэг** -- `precc gif` Анимацитай терминалын GIF үүсгэхэд
- **IP geofence нийцэл** -- Зохицуулалттай орчинд зориулсан
- **Имэйл тайлан** -- `precc mail report` Аналитик илгээхэд
- **GitHub Actions шинжилгээ** -- `precc gha` Амжилтгүй workflow дибаг хийхэд
- **Контекст шахалт** -- `precc compress` CLAUDE.md оновчлолд
- **Тэргүүлэх дэмжлэг**

## Лиценз идэвхжүүлэх

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Лицензийн төлөв шалгах

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors идэвхжүүлэлт

Хэрэв та GitHub Sponsors-оор PRECC-ийг ивээн тэтгэвэл, таны лиценз GitHub имэйлээр автоматаар идэвхждэг. Түлхүүр шаардлагагүй -- зөвхөн ивээн тэтгэгчийн имэйл таарч байгааг шалгаарай:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Төхөөрөмжийн хурууны хээ

Лиценз бүр төхөөрөмжийн хурууны хээтэй холбоотой. Өөрийнхөө хурууны хээг харна уу:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Лицензээ шинэ машинд шилжүүлэх шаардлагатай бол эхлээд идэвхгүй болго:

```bash
precc license deactivate
```

Дараа нь шинэ машин дээр идэвхжүүлнэ үү.

## Лицензийн хугацаа дууссан уу?

Pro лицензийн хугацаа дуусахад PRECC Community түвшинд буцна. Бүх суулгагдсан ур чадвар болон үндсэн функцууд үргэлжлэн ажиллана. Зөвхөн Pro-д зориулсан боломжууд ашиглагдахгүй болно. Дэлгэрэнгүйг [FAQ](faq.md)-аас үзнэ үү.
