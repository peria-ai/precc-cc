# مجوز

PRECC دو سطح ارائه می‌دهد: Community (رایگان) و Pro.

## سطح Community (رایگان)

سطح Community شامل:

- تمام مهارت‌های داخلی (اصلاح دایرکتوری اشتباه، ترجمه jj و غیره)
- خط لوله hook با پشتیبانی کامل Pillar 1 و Pillar 4
- خلاصه پایه `precc savings`
- استخراج جلسه با `precc ingest`
- استفاده محلی نامحدود

## سطح Pro

Pro ویژگی‌های اضافی را باز می‌کند:

- **جزئیات صرفه‌جویی** -- `precc savings --all` با تحلیل هر دستور
- **ضبط GIF** -- `precc gif` برای ایجاد GIF متحرک ترمینال
- **انطباق geofence IP** -- برای محیط‌های تحت نظارت
- **گزارش‌های ایمیلی** -- `precc mail report` برای ارسال تحلیل‌ها
- **تحلیل GitHub Actions** -- `precc gha` برای رفع اشکال گردش کار ناموفق
- **فشرده‌سازی زمینه** -- `precc compress` برای بهینه‌سازی CLAUDE.md
- **پشتیبانی اولویت‌دار**

## فعال‌سازی مجوز

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## بررسی وضعیت مجوز

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## فعال‌سازی GitHub Sponsors

اگر PRECC را از طریق GitHub Sponsors حمایت می‌کنید، مجوز شما به‌طور خودکار از طریق ایمیل GitHub فعال می‌شود. نیازی به کلید نیست -- فقط مطمئن شوید ایمیل حامی مطابقت دارد:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## اثر انگشت دستگاه

هر مجوز به اثر انگشت دستگاه متصل است. خود را ببینید با:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

اگر نیاز به انتقال مجوز به ماشین جدید دارید، ابتدا غیرفعال کنید:

```bash
precc license deactivate
```

سپس در ماشین جدید فعال کنید.

## مجوز منقضی شده؟

وقتی مجوز Pro منقضی شود، PRECC به سطح Community برمی‌گردد. تمام مهارت‌های داخلی و عملکردهای اصلی به کار خود ادامه می‌دهند. فقط ویژگی‌های خاص Pro غیرقابل دسترس می‌شوند. برای جزئیات بیشتر [سوالات متداول](faq.md) را ببینید.
