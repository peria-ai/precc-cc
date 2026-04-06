# Geofence

PRECC တွင် ထိန်းချုပ်ထားသော ပတ်ဝန်းကျင်များအတွက် IP geofence လိုက်နာမှု စစ်ဆေးခြင်း ပါဝင်သည်။ ၎င်းသည် Pro အင်္ဂါရပ်ဖြစ်သည်။

## ခြုံငုံသုံးသပ်ချက်

အချို့အဖွဲ့အစည်းများသည် ဖွံ့ဖြိုးရေးကိရိယာများ အတည်ပြုထားသော ပထဝီဝင်ဒေသများအတွင်းသာ အလုပ်လုပ်ရန် လိုအပ်သည်။ PRECC ၏ geofence feature သည် လက်ရှိစက်၏ IP လိပ်စာ ခွင့်ပြုထားသော ဒေသစာရင်းအတွင်း ရှိမရှိ စစ်ဆေးသည်။

## လိုက်နာမှု စစ်ဆေးခြင်း

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

စက်သည် ခွင့်ပြုထားသော ဒေသများ ပြင်ပတွင် ရှိပါက:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Geofence ဒေတာ ပြန်လည်ရယူခြင်း

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Geofence အချက်အလက် ကြည့်ရှုခြင်း

```bash
$ precc geofence info
Geofence Configuration
======================
Policy file:    ~/.config/precc/geofence.toml
Allowed regions: us-east-1, us-west-2, eu-west-1
Cache age:      2h 14m
Last check:     2026-04-03 09:12:00 UTC
Status:         COMPLIANT
```

## Cache ရှင်းလင်းခြင်း

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## ပြင်ဆင်သတ်မှတ်ခြင်း

Geofence မူဝါဒကို `~/.config/precc/geofence.toml` တွင် သတ်မှတ်ထားသည်:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

ခွင့်ပြုထားသော ဒေသများ ပြင်ပတွင် PRECC အလုပ်မလုပ်စေရန် `block_on_violation = true` ကို သတ်မှတ်ပါ။
