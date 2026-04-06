# 텔레메트리

PRECC는 도구 개선을 위한 선택적 익명 텔레메트리를 지원합니다. 명시적으로 동의하지 않는 한 데이터가 수집되지 않습니다.

## 옵트인

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## 옵트아웃

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## 상태 확인

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## 전송될 데이터 미리보기

옵트인하기 전에 수집될 데이터를 정확히 확인할 수 있습니다:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## 수집되는 데이터

- PRECC 버전, OS 및 아키텍처
- 집계 카운트: 가로챈 명령, 활성화된 스킬, 사용된 필라
- 평균 훅 지연 시간
- 세션 수

## 수집되지 않는 데이터

- 명령 텍스트나 인수 없음
- 파일 경로나 디렉터리 이름 없음
- 프로젝트 이름이나 저장소 URL 없음
- 개인 식별 정보(PII) 없음
- IP 주소 없음 (서버에서 기록하지 않음)

## 환경 변수 재정의

명령을 실행하지 않고 텔레메트리를 비활성화하려면 (CI 또는 공유 환경에서 유용):

```bash
export PRECC_NO_TELEMETRY=1
```

이것은 동의 설정보다 우선합니다.

## 데이터 전송 대상

텔레메트리 데이터는 HTTPS를 통해 `https://telemetry.peria.ai/v1/precc`로 전송됩니다. 데이터는 사용 패턴을 이해하고 개발 우선순위를 정하는 데만 사용됩니다.
