# Телеметр

PRECC нь хэрэгслийг сайжруулахад туслах нэргүй телеметрийг дэмждэг. Таны зөвшөөрөлгүйгээр ямар ч мэдээлэл цуглуулахгүй.

## Идэвхжүүлэх

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Идэвхгүй болгох

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Төлөв шалгах

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Илгээх мэдээллийн урьдчилсан харагдац

Идэвхжүүлэхээс өмнө ямар мэдээлэл цуглуулахыг яг харж болно:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Цуглуулагдах мэдээлэл

- PRECC хувилбар, үйлдлийн систем, архитектур
- Нэгтгэсэн тоо: таслан зогсоосон командууд, идэвхжүүлсэн чадварууд, ашигласан тулгуурууд
- Дундаж hook хоцрогдол
- Сессийн тоо

## Цуглуулагдахгүй мэдээлэл

- Командын текст, аргумент байхгүй
- Файлын зам, директорын нэр байхгүй
- Төслийн нэр, репозиторийн URL байхгүй
- Хувийн мэдээлэл (PII) байхгүй
- IP хаяг байхгүй (сервер бүртгэдэггүй)

## Орчны хувьсагчаар дарах

Команд ажиллуулахгүйгээр телеметрийг идэвхгүй болгохын тулд (CI болон хуваалцсан орчинд хэрэгтэй):

```bash
export PRECC_NO_TELEMETRY=1
```

Энэ нь зөвшөөрлийн тохиргооноос давуу эрхтэй.

## Мэдээллийн очих газар

Телеметрийн мэдээлэл HTTPS-ээр `https://telemetry.peria.ai/v1/precc` руу илгээгддэг. Мэдээллийг зөвхөн хэрэглээний хэв маягийг ойлгох, хөгжүүлэлтийг эрэмбэлэхэд ашигладаг.
