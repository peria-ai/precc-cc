# Ур чадвар

Ур чадвар нь PRECC-ийн тушаалуудыг илрүүлж засахад ашигладаг хэв маягийн тааруулалтын дүрмүүд юм.

## Суурилуулсан ур чадвар

| Ур чадвар | Идэвхжүүлэгч | Үйлдэл |
|-------|-------------|--------|
| `cargo-wrong-dir` | Rust төслөөс гадуур `cargo build/test/clippy` | Хамгийн ойрын `Cargo.toml` хавтас руу `cd` нэмэх |
| `git-wrong-dir` | git repo-оос гадуур `git *` | Хамгийн ойрын `.git` хавтас руу `cd` нэмэх |
| `go-wrong-dir` | Go модулиас гадуур `go build/test` | Хамгийн ойрын `go.mod` хавтас руу `cd` нэмэх |
| `make-wrong-dir` | Одоогийн хавтаст Makefile байхгүй үед `make` | Хамгийн ойрын Makefile хавтас руу `cd` нэмэх |
| `npm-wrong-dir` | Node төслөөс гадуур `npm/npx/pnpm/yarn` | Хамгийн ойрын `package.json` хавтас руу `cd` нэмэх |
| `python-wrong-dir` | Python төслөөс гадуур `python/pytest/pip` | Хамгийн ойрын Python төсөл рүү `cd` нэмэх |
| `jj-translate` | jj-colocated repo дахь `git *` | Адил `jj` тушаал руу дахин бичих |
| `asciinema-gif` | `asciinema rec` | `precc gif` руу дахин бичих |

## Ур чадваруудын жагсаалт

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

## Ур чадварын дэлгэрэнгүйг харуулах

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

## Ур чадварыг TOML руу экспортлох

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

## Ур чадварыг засварлах

```bash
$ precc skills edit cargo-wrong-dir
```

Энэ нь `$EDITOR` дотор ур чадварын тодорхойлолтыг нээнэ. Хадгалсны дараа ур чадвар автоматаар дахин ачаалагдана.

## Advise тушаал

`precc skills advise` таны сүүлийн сессийг шинжилж, давтагдсан хэв маягт үндэслэн шинэ ур чадвар санал болгоно:

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

## Ур чадваруудыг бүлэглэх

```bash
$ precc skills cluster
```

Давхцаж буй хэв маягийг тодорхойлоход туслахын тулд ижил төстэй ур чадваруудыг бүлэглэнэ.

## Олборлосон ба суурилуулсан ур чадвар

Суурилуулсан ур чадвар нь PRECC-тэй хамт ирдэг бөгөөд `skills/builtin/*.toml`-д тодорхойлогдсон. Хамгийн түгээмэл буруу хавтасны алдааг хамардаг.

Олборлосон ур чадварыг `precc ingest` эсвэл `precc-learner` демон таны сессийн логоос үүсгэдэг. `~/.local/share/precc/heuristics.db`-д хадгалагддаг. Дэлгэрэнгүйг [Олборлолт](mining.md)-оос үзнэ үү.
