# 라이선스

PRECC는 두 가지 티어를 제공합니다: Community(무료)와 Pro.

## Community 티어 (무료)

Community 티어에는 다음이 포함됩니다:

- 모든 내장 스킬 (잘못된 디렉토리 수정, jj 변환 등)
- Pillar 1 및 Pillar 4를 완전히 지원하는 Hook 파이프라인
- 기본 `precc savings` 요약
- `precc ingest`를 사용한 세션 마이닝
- 무제한 로컬 사용

## Pro 티어

Pro는 추가 기능을 잠금 해제합니다:

- **상세 절약 분석** -- `precc savings --all` 명령별 분석 포함
- **GIF 녹화** -- `precc gif` 애니메이션 터미널 GIF 생성용
- **IP 지오펜스 규정 준수** -- 규제 환경용
- **이메일 보고서** -- `precc mail report` 분석 전송용
- **GitHub Actions 분석** -- `precc gha` 실패한 워크플로 디버깅용
- **컨텍스트 압축** -- `precc compress` CLAUDE.md 최적화용
- **우선 지원**

## 라이선스 활성화

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## 라이선스 상태 확인

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors 활성화

GitHub Sponsors를 통해 PRECC를 후원하면 GitHub 이메일을 통해 라이선스가 자동으로 활성화됩니다. 키가 필요하지 않습니다 -- 후원자 이메일이 일치하는지 확인하세요:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## 디바이스 지문

각 라이선스는 디바이스 지문에 연결됩니다. 다음으로 확인하세요:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

라이선스를 새 머신으로 이전해야 하는 경우 먼저 비활성화하세요:

```bash
precc license deactivate
```

그런 다음 새 머신에서 활성화하세요.

## 라이선스가 만료되었나요?

Pro 라이선스가 만료되면 PRECC는 Community 티어로 되돌아갑니다. 모든 내장 스킬과 핵심 기능은 계속 작동합니다. Pro 전용 기능만 사용할 수 없게 됩니다. 자세한 내용은 [FAQ](faq.md)를 참조하세요.
