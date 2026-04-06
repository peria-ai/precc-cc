# GitHub Actions ခွဲခြမ်းစိတ်ဖြာခြင်း

`precc gha` သည် ပျက်ကွက်သော GitHub Actions run များကို ခွဲခြမ်းစိတ်ဖြာပြီး ပြုပြင်ချက်များ အကြံပြုသည်။ ၎င်းသည် Pro အင်္ဂါရပ်ဖြစ်သည်။

## အသုံးပြုပုံ

ပျက်ကွက်သော GitHub Actions run ၏ URL ကို ပေးပါ:

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## ဘာလုပ်သလဲ

1. GitHub Actions run URL ကို parse လုပ်ပြီး owner, repo နှင့် run ID ကို ထုတ်ယူသည်။
2. GitHub API မှတစ်ဆင့် run log များကို ရယူသည် (`GITHUB_TOKEN` သတ်မှတ်ထားလျှင် အသုံးပြုသည်၊ မဟုတ်ပါက public access)။
3. ပျက်ကွက်သော step ကို ဖော်ထုတ်ပြီး သက်ဆိုင်ရာ error line များကို ထုတ်ယူသည်။
4. Error ကို ခွဲခြမ်းစိတ်ဖြာပြီး အသုံးများသော CI failure pattern များကို အခြေခံ၍ ပြုပြင်ချက် အကြံပြုသည်။

## ပံ့ပိုးသော failure pattern များ

- ပျောက်ဆုံးနေသော service container များ (databases, Redis စသည်)
- မမှန်ကန်သော runner OS သို့မဟုတ် architecture
- ပျောက်ဆုံးနေသော environment variables သို့မဟုတ် secrets
- Dependency installation ပျက်ကွက်ခြင်းများ
- Test timeout များ
- Permission error များ
- Cache miss များကြောင့် build နှေးခြင်း
