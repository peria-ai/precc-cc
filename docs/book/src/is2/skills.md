# Þekking

Þekking er mynstursamsvörunarreglur sem PRECC notar til að greina og leiðrétta skipanir. Þær geta verið innbyggðar (sendar sem TOML-skrár) eða lærðar úr lotunarskrám.

## Innbyggð þekking

| Þekking | Ræsir á | Aðgerð |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` utan Rust-verkefnis | Bæta `cd` framan á í næstu `Cargo.toml`-möppu |
| `git-wrong-dir` | `git *` utan git-geymslu | Bæta `cd` framan á í næstu `.git`-möppu |
| `go-wrong-dir` | `go build/test` utan Go-einingar | Bæta `cd` framan á í næstu `go.mod`-möppu |
| `make-wrong-dir` | `make` án Makefile í cwd | Bæta `cd` framan á í næstu Makefile-möppu |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` utan Node-verkefnis | Bæta `cd` framan á í næstu `package.json`-möppu |
| `python-wrong-dir` | `python/pytest/pip` utan Python-verkefnis | Bæta `cd` framan á í næsta Python-verkefni |
| `jj-translate` | `git *` í jj-colocated geymslu | Endurskrifa í samsvarandi `jj`-skipun |
| `asciinema-gif` | `asciinema rec` | Endurskrifa í `precc gif` |

## Listi yfir þekkingu

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

## Þekkingarsmáatriði

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

## Útflutningur þekkingar í TOML

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

## Breyting á þekkingu

```bash
$ precc skills edit cargo-wrong-dir
```

Þetta opnar þekkingarskilgreiningu í `$EDITOR`. Eftir vistun er þekkingin endurhlaðin sjálfkrafa.

## Advise-skipunin

`precc skills advise` greinir nýlega lotu þína og stingur upp á nýrri þekkingu byggðri á endurteknum mynstrum:

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

## Flokkun þekkingar

```bash
$ precc skills cluster
```

Flokkar svipaða lærða þekkingu saman til að auðkenna óþarfa eða skarast mynstur.

## Lærð vs. innbyggð þekking

Innbyggð þekking fylgir PRECC og er skilgreind í `skills/builtin/*.toml`. Hún nær yfir algengustu röng-möppu villurnar.

Lærð þekking er búin til af `precc ingest` eða `precc-learner`-þjóninum úr lotunarskránum þínum. Hún er geymd í `~/.local/share/precc/heuristics.db` og er sértæk fyrir þitt verkflæði. Sjá [Mining](mining.md) fyrir nánari upplýsingar.
