# GitHub Actions analizi

`precc gha` başarısız GitHub Actions çalışmalarını analiz eder ve düzeltmeler önerir. Bu bir Pro özelliğidir.

## Kullanım

Başarısız bir GitHub Actions çalışmasının URL'sini iletin:

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

## Ne yapar

1. GitHub Actions çalıştırma URL'sini ayrıştırarak sahip, depo ve çalıştırma kimliğini çıkarır.
2. GitHub API üzerinden çalıştırma günlüklerini alır (ayarlanmışsa `GITHUB_TOKEN` kullanır, aksi takdirde genel erişim).
3. Başarısız adımı belirler ve ilgili hata satırlarını çıkarır.
4. Hatayı analiz eder ve yaygın CI hata kalıplarına dayalı bir düzeltme önerir.

## Desteklenen hata kalıpları

- Eksik hizmet konteynerleri (veritabanları, Redis, vb.)
- Yanlış runner işletim sistemi veya mimarisi
- Eksik ortam değişkenleri veya gizli anahtarlar
- Bağımlılık kurulum hataları
- Test zaman aşımları
- İzin hataları
- Yavaş derlemelere neden olan önbellek kayıpları
