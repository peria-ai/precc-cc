# 자주 묻는 질문

## PRECC는 안전한가요?

네. PRECC는 Claude Code의 공식 PreToolUse 훅 메커니즘을 사용합니다 -- Anthropic이 정확히 이 목적을 위해 설계한 확장 포인트입니다. 훅은:

- 완전히 오프라인으로 실행 (핫 패스에서 네트워크 호출 없음)
- 5밀리초 이내에 완료
- 페일 오픈: 문제가 발생하면 원래 명령이 수정 없이 실행
- 명령만 수정하고 직접 실행하지 않음
- 로컬 SQLite 데이터베이스에 데이터 저장

## PRECC는 다른 AI 코딩 도구와 호환되나요?

PRECC는 Claude Code 전용으로 설계되었습니다. Claude Code가 제공하는 PreToolUse 훅 프로토콜에 의존합니다. Cursor, Copilot, Windsurf 또는 다른 AI 코딩 도구와는 호환되지 않습니다.

## 텔레메트리는 어떤 데이터를 전송하나요?

텔레메트리는 옵트인 방식입니다. 활성화하면 다음을 전송합니다:

- PRECC 버전, OS 및 아키텍처
- 집계 카운트 (가로챈 명령, 활성화된 스킬)
- 평균 훅 지연 시간

명령 텍스트, 파일 경로, 프로젝트 이름 또는 개인 식별 정보를 전송하지 **않습니다**. 옵트인 전에 `precc telemetry preview`로 정확한 페이로드를 미리 볼 수 있습니다. 자세한 내용은 [텔레메트리](telemetry.md)를 참조하세요.

## PRECC를 어떻게 제거하나요?

??faq_uninstall_a_intro??

1. 훅 등록 제거:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. 바이너리 제거:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. 데이터 제거 (선택):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## 라이선스가 만료되었습니다. 어떻게 되나요?

PRECC는 Community 티어로 돌아갑니다. 모든 핵심 기능은 계속 작동합니다:

- 기본 스킬은 활성 상태 유지
- 훅 파이프라인은 정상 작동
- `precc savings`는 요약 보기 표시
- `precc ingest`와 세션 마이닝 작동

Pro 기능은 갱신할 때까지 사용할 수 없습니다:

- `precc savings --all` (상세 분석)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- 이메일 보고서

## 훅이 실행되지 않는 것 같습니다. 어떻게 디버그하나요?

??faq_debug_a_intro??

1. 훅이 등록되어 있는지 확인:
   ```bash
   precc init
   ```

2. 훅을 수동으로 테스트:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. 바이너리가 PATH에 있는지 확인:
   ```bash
   which precc-hook
   ```

4. `~/.claude/settings.json`에서 Claude Code 훅 설정을 확인하세요.

## PRECC가 Claude Code를 느리게 하나요?

아닙니다. 훅은 5밀리초(p99) 이내에 완료됩니다. 이는 Claude가 추론하고 응답을 생성하는 시간에 비해 감지할 수 없는 수준입니다.

## CI/CD에서 PRECC를 사용할 수 있나요?

PRECC는 대화형 Claude Code 세션을 위해 설계되었습니다. CI/CD에서는 연결할 Claude Code 인스턴스가 없습니다. 그러나 `precc gha`는 모든 환경에서 실패한 GitHub Actions 실행을 분석할 수 있습니다.

## 마이닝된 스킬은 기본 스킬과 어떻게 다른가요?

기본 스킬은 PRECC와 함께 제공되며 일반적인 잘못된 디렉터리 패턴을 다룹니다. 마이닝된 스킬은 특정 세션 로그에서 학습됩니다. 둘 다 SQLite에 저장되며 훅 파이프라인에서 동일하게 평가됩니다.

## 팀과 스킬을 공유할 수 있나요?

네. `precc skills export NAME`으로 스킬을 TOML로 내보내고 파일을 공유할 수 있습니다. 팀원들은 `skills/` 디렉터리에 넣거나 휴리스틱 데이터베이스에 가져올 수 있습니다.
