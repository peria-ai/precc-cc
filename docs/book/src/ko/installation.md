# 설치

## 빠른 설치 (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

이 명령은 플랫폼에 맞는 최신 릴리스 바이너리를 다운로드하고, SHA256 체크섬을 확인한 후, `~/.local/bin/`에 배치합니다.

설치 후 PRECC를 초기화하세요:

```bash
precc init
```

`precc init`은 Claude Code에 PreToolUse 훅을 등록하고, 데이터 디렉토리를 생성하며, 스킬 데이터베이스를 초기화합니다.

## 설치 옵션

### SHA256 검증

기본적으로 설치 프로그램은 게시된 SHA256 합계와 바이너리 체크섬을 검증합니다. 검증을 건너뛰려면(권장하지 않음):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### 사용자 지정 설치 경로

사용자 지정 위치에 설치:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### 동반 도구 (--extras)

PRECC에는 선택적 동반 도구가 포함되어 있습니다. `--extras`로 설치하세요:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

다음을 설치합니다:

| 도구 | 용도 |
|------|---------|
| **RTK** | 명령어 재작성 도구 모음 |
| **lean-ctx** | CLAUDE.md 및 프롬프트 파일용 컨텍스트 압축 |
| **nushell** | 고급 파이프라인을 위한 구조화된 셸 |
| **cocoindex-code** | 더 빠른 컨텍스트 해결을 위한 코드 인덱싱 |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

그런 다음 초기화하세요:

```powershell
precc init
```

## 수동 설치

1. 플랫폼에 맞는 릴리스 바이너리를 [GitHub Releases](https://github.com/peria-ai/precc-cc/releases)에서 다운로드하세요.
2. 릴리스의 `.sha256` 파일과 SHA256 체크섬을 확인하세요.
3. 바이너리를 `PATH`에 있는 디렉토리(예: `~/.local/bin/`)에 배치하세요.
4. `precc init`을 실행하세요.

## 업데이트

```bash
precc update
```

특정 버전으로 강제 업데이트:

```bash
precc update --force --version 0.3.0
```

자동 업데이트 활성화:

```bash
precc update --auto
```

## 설치 확인

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

`precc`를 찾을 수 없는 경우 `~/.local/bin`이 `PATH`에 포함되어 있는지 확인하세요.
