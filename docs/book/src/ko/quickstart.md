# 빠른 시작

5분 안에 PRECC를 실행하세요.

## 1단계: 설치

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## 2단계: 초기화

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## 3단계: 훅이 활성 상태인지 확인

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
```

## 4단계: Claude Code를 평소처럼 사용

Claude Code를 열고 평소처럼 작업하세요. PRECC는 백그라운드에서 조용히 실행됩니다. Claude가 실패할 명령을 내리면 PRECC가 실행 전에 수정합니다.

### 예시: 잘못된 디렉토리에서 Cargo Build

프로젝트가 `~/projects/myapp/`에 있고 Claude가 다음을 실행한다고 가정합니다:

```
cargo build
```

`~/projects/`에서 (한 단계 위, 거기에 `Cargo.toml` 없음).

**PRECC 없이:** Claude가 `could not find Cargo.toml in /home/user/projects or any parent directory` 오류를 받고, 읽고, 추론한 후 `cd myapp && cargo build`로 재시도합니다. 비용: ~2,000 토큰 낭비.

**PRECC 사용 시:** 훅이 누락된 `Cargo.toml`을 감지하고 `myapp/`에서 찾아 명령을 다음으로 재작성합니다:

```
cd /home/user/projects/myapp && cargo build
```

Claude는 오류를 절대 보지 않습니다. 낭비되는 토큰 제로.

## 5단계: 절약량 확인

세션 후 PRECC가 절약한 토큰 수를 확인하세요:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## 다음 단계

- [스킬](skills.md) -- 사용 가능한 모든 스킬과 직접 만드는 방법을 확인하세요.
- [훅 파이프라인](hook-pipeline.md) -- 내부에서 무슨 일이 일어나는지 이해하세요.
- [절약](savings.md) -- 상세한 토큰 절약 분석.
