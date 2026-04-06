# 스킬

스킬은 PRECC가 명령어를 감지하고 수정하는 데 사용하는 패턴 매칭 규칙입니다. 내장(TOML 파일로 배포) 또는 세션 로그에서 마이닝될 수 있습니다.

## 내장 스킬

| 스킬 | 트리거 조건 | 동작 |
|-------|-------------|--------|
| `cargo-wrong-dir` | Rust 프로젝트 외부에서 `cargo build/test/clippy` | 가장 가까운 `Cargo.toml` 디렉토리로 `cd` 추가 |
| `git-wrong-dir` | git 저장소 외부에서 `git *` | 가장 가까운 `.git` 디렉토리로 `cd` 추가 |
| `go-wrong-dir` | Go 모듈 외부에서 `go build/test` | 가장 가까운 `go.mod` 디렉토리로 `cd` 추가 |
| `make-wrong-dir` | 현재 디렉토리에 Makefile 없이 `make` | 가장 가까운 Makefile 디렉토리로 `cd` 추가 |
| `npm-wrong-dir` | Node 프로젝트 외부에서 `npm/npx/pnpm/yarn` | 가장 가까운 `package.json` 디렉토리로 `cd` 추가 |
| `python-wrong-dir` | Python 프로젝트 외부에서 `python/pytest/pip` | 가장 가까운 Python 프로젝트로 `cd` 추가 |
| `jj-translate` | jj 공존 저장소에서 `git *` | 동등한 `jj` 명령어로 재작성 |
| `asciinema-gif` | `asciinema rec` | `precc gif`로 재작성 |

## 스킬 목록

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

## 스킬 세부 정보 표시

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

## 스킬을 TOML로 내보내기

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

## 스킬 편집

```bash
$ precc skills edit cargo-wrong-dir
```

이 명령은 `$EDITOR`에서 스킬 정의를 엽니다. 저장 후 스킬이 자동으로 다시 로드됩니다.

## Advise 명령어

`precc skills advise`는 최근 세션을 분석하고 반복 패턴을 기반으로 새로운 스킬을 제안합니다:

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

## 스킬 클러스터링

```bash
$ precc skills cluster
```

유사한 마이닝된 스킬을 그룹화하여 중복되거나 겹치는 패턴을 식별하는 데 도움을 줍니다.

## 마이닝 스킬 vs. 내장 스킬

내장 스킬은 PRECC와 함께 제공되며 `skills/builtin/*.toml`에 정의되어 있습니다. 가장 흔한 잘못된 디렉토리 실수를 다룹니다.

마이닝된 스킬은 세션 로그에서 `precc ingest` 또는 `precc-learner` 데몬에 의해 생성됩니다. `~/.local/share/precc/heuristics.db`에 저장되며 워크플로에 특화됩니다. 자세한 내용은 [마이닝](mining.md)을 참조하세요.
