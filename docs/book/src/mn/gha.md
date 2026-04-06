# GitHub Actions шинжилгээ

`precc gha` нь амжилтгүй болсон GitHub Actions ажиллагааг шинжилж, засварыг санал болгоно. Энэ нь Pro боломж юм.

## Хэрэглээ

Амжилтгүй болсон GitHub Actions ажиллагааны URL-ийг дамжуулна:

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

## Юу хийдэг вэ

1. GitHub Actions ажиллагааны URL-ийг задлан шинжилж эзэмшигч, repo, ажиллагааны ID-г гаргаж авна.
2. GitHub API-аар дамжуулан ажиллагааны логуудыг авна (`GITHUB_TOKEN` тохируулсан бол ашиглана, үгүй бол нийтийн хандалт).
3. Амжилтгүй болсон алхмыг тодорхойлж, холбогдох алдааны мөрүүдийг гаргаж авна.
4. Алдааг шинжилж, түгээмэл CI алдааны хэв маягт үндэслэн засварыг санал болгоно.

## Дэмжигдсэн алдааны хэв маягууд

- Дутагдаж буй үйлчилгээний контейнерууд (мэдээллийн сан, Redis гэх мэт)
- Буруу runner OS эсвэл архитектур
- Дутагдаж буй орчны хувьсагчууд эсвэл нууцууд
- Хамаарлын суулгалтын алдаа
- Тестийн хугацаа хэтрэлт
- Зөвшөөрлийн алдаа
- Удаан build үүсгэж буй кэш алдаа
