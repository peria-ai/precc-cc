# 소개

## PRECC란?

PRECC (Claude Code를 위한 예측 오류 수정) 는 공식 PreToolUse 훅 메커니즘을 통해 Claude Code bash 명령을 가로채는 Rust 도구입니다. 오류가 *발생하기 전에* 수정하여 토큰을 절약하고 재시도 루프를 제거합니다.

커뮤니티 사용자에게 무료.

## 문제점

Claude Code는 예방 가능한 실수에 상당한 토큰을 낭비합니다:

- **디렉토리 오류** -- `Cargo.toml`이 없는 상위 디렉토리에서 `cargo build` 실행 후 재시도.
- **재시도 루프** -- 실패한 명령이 장황한 출력을 생성.
- **장황한 출력** -- `find`나 `ls -R` 같은 명령이 수천 줄을 출력.

## 네 가지 기둥

### 컨텍스트 수정 (cd-prepend)

`cargo build`나 `npm test` 같은 명령이 잘못된 디렉토리에서 실행될 때를 감지하고 실행 전에 `cd /올바른/경로 &&`를 추가합니다.

### GDB 디버깅

세그폴트와 크래시의 심층 디버깅을 위해 GDB를 연결할 기회를 감지합니다.

### 세션 마이닝

Claude Code 세션 로그에서 실패-수정 쌍을 분석합니다. 같은 실수가 반복되면 자동으로 적용합니다.

### 자동화 스킬

명령 패턴을 매칭하고 다시 작성하는 스킬 라이브러리.

## 작동 방식 (30초 버전)

1. Claude Code가 bash 명령을 실행하려 합니다.
2. PreToolUse 훅이 명령을 JSON으로 `precc-hook`에 전송합니다.
3. `precc-hook`이 3밀리초 미만으로 명령을 처리합니다.
4. 수정된 명령이 JSON으로 반환됩니다.
5. Claude Code가 수정된 명령을 실행합니다.

Claude는 오류를 보지 못합니다. 토큰 낭비 없음.

### 적응형 압축

명령이 압축 후 실패하면 PRECC가 다음 재시도에서 자동으로 압축을 건너뛰어 Claude가 디버깅을 위한 전체 비압축 출력을 받습니다.

## 실시간 사용 통계

현재 버전 <span data-stat="current_version">--</span>:

| 지표 | 값 |
|---|---|
| 훅 호출 | <span data-stat="total_invocations">--</span> |
| 절약된 토큰 | <span data-stat="total_tokens_saved">--</span> |
| 절약 비율 | <span data-stat="saving_pct">--</span>% |
| RTK 재작성 | <span data-stat="rtk_rewrites">--</span> |
| CD 수정 | <span data-stat="cd_prepends">--</span> |
| 훅 지연 | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| 고유 사용자 | <span data-stat="unique_users">--</span> |

### 릴리스별 절약

<table id="version-breakdown" style="display:none">
<thead><tr><th>버전</th><th>고유 사용자</th><th>훅 호출</th><th>절약된 토큰</th><th>절약 비율</th></tr></thead>
<tbody><tr><td colspan="5"><em>로딩 중...</em></td></tr></tbody>
</table>

<small>수치는 추정치입니다. 예방된 각 실패는 전체 재시도 사이클을 회피합니다: 오류 출력, 모델 추론, 재시도 명령. 이 수치는 익명화된 원격 측정에서 자동으로 업데이트됩니다.</small>

## 링크

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- 웹사이트: [https://peria.ai](https://peria.ai)
- 문서: [https://precc.cc](https://precc.cc)
