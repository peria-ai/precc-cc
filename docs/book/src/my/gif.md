# GIF ရိုက်ကူးခြင်း

`precc gif` သည် bash script များမှ terminal session များ၏ animated GIF ရိုက်ကူးမှုများ ဖန်တီးသည်။ ၎င်းသည် Pro အင်္ဂါရပ်ဖြစ်သည်။

## အခြေခံ အသုံးပြုပုံ

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

ပထမ argument သည် run ရမည့် command များပါဝင်သော bash script ဖြစ်သည်။ ဒုတိယ argument သည် အများဆုံး ရိုက်ကူးမှု အရှည်ဖြစ်သည်။

## Script ပုံစံ

Script သည် standard bash ဖိုင်ဖြစ်သည်:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Input Simulation

Interactive command များအတွက်၊ input တန်ဖိုးများကို argument အပိုများအဖြစ် ပေးပါ:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Script က input တောင်းသောအခါ argument အပိုတစ်ခုစီကို stdin စာကြောင်းတစ်ကြောင်းအဖြစ် ထည့်သွင်းသည်။

## Output ရွေးချယ်စရာများ

Output ဖိုင်သည် default အားဖြင့် script အမည်အတိုင်း သတ်မှတ်သည် (`script.gif`)။ GIF သည် standard 80x24 အတိုင်းအတာဖြင့် dark terminal theme ကို အသုံးပြုသည်။

## ဘာကြောင့် asciinema အစား GIF သုံးသလဲ?

Built-in skill `asciinema-gif` သည် `asciinema rec` ကို `precc gif` သို့ အလိုအလျောက် ပြန်ရေးသည်။ GIF ဖိုင်များသည် ပိုမို portable ဖြစ်သည် -- GitHub README, Slack နှင့် email တွင် player မလိုဘဲ inline ပြသနိုင်သည်။
