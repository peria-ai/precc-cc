# Beceriler

Beceriler, PRECC'nin komutları tespit etmek ve düzeltmek için kullandığı desen eşleme kurallarıdır. Yerleşik (TOML dosyaları olarak dağıtılan) veya oturum günlüklerinden çıkarılmış olabilirler.

## Yerleşik beceriler

| Beceri | Tetikleyici | Eylem |
|-------|-------------|--------|
| `cargo-wrong-dir` | Rust projesi dışında `cargo build/test/clippy` | En yakın `Cargo.toml` dizinine `cd` ekle |
| `git-wrong-dir` | git deposu dışında `git *` | En yakın `.git` dizinine `cd` ekle |
| `go-wrong-dir` | Go modülü dışında `go build/test` | En yakın `go.mod` dizinine `cd` ekle |
| `make-wrong-dir` | Geçerli dizinde Makefile olmadan `make` | En yakın Makefile dizinine `cd` ekle |
| `npm-wrong-dir` | Node projesi dışında `npm/npx/pnpm/yarn` | En yakın `package.json` dizinine `cd` ekle |
| `python-wrong-dir` | Python projesi dışında `python/pytest/pip` | En yakın Python projesine `cd` ekle |
| `jj-translate` | jj birlikte yerleşimli depoda `git *` | Eşdeğer `jj` komutuna yeniden yaz |
| `asciinema-gif` | `asciinema rec` | `precc gif` olarak yeniden yaz |

## Becerileri listeleme

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
  9 fix-pytest-path    mined     pytest with wrong test path
```

## Beceri ayrıntılarını göster

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## Bir beceriyi TOML'a dışa aktar

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## Bir beceriyi düzenle

```bash
$ precc skills edit cargo-wrong-dir
```

Bu, beceri tanımını `$EDITOR`'ınızda açar. Kaydettikten sonra beceri otomatik olarak yeniden yüklenir.

## Advise komutu

`precc skills advise` son oturumunuzu analiz eder ve tekrarlanan kalıplara dayalı yeni beceriler önerir:

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## Becerileri kümeleme

```bash
$ precc skills cluster
```

Gereksiz veya örtüşen kalıpları belirlemeye yardımcı olmak için benzer çıkarılmış becerileri gruplar.

## Çıkarılmış ve yerleşik beceriler

Yerleşik beceriler PRECC ile birlikte gelir ve `skills/builtin/*.toml` içinde tanımlanır. En yaygın yanlış dizin hatalarını kapsar.

Çıkarılmış beceriler, oturum günlüklerinizden `precc ingest` veya `precc-learner` arka plan servisi tarafından oluşturulur. `~/.local/share/precc/heuristics.db` içinde saklanır ve iş akışınıza özgüdür. Ayrıntılar için [Madencilik](mining.md) sayfasına bakın.
