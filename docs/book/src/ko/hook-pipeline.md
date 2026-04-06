# 훅 파이프라인

`precc-hook` 바이너리는 PRECC의 핵심입니다. Claude Code와 셸 사이에 위치하여 모든 bash 명령을 5밀리초 이내에 처리합니다.

## Claude Code가 훅을 호출하는 방법

Claude Code는 PreToolUse 훅을 지원합니다 -- 실행 전에 도구 입력을 검사하고 수정할 수 있는 외부 프로그램입니다. Claude가 bash 명령을 실행하려 할 때, stdin으로 `precc-hook`에 JSON을 보내고 stdout에서 응답을 읽습니다.

## 파이프라인 단계

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## 예제: JSON 입력 및 출력

### 입력 (Claude Code에서)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC는 현재 디렉토리에 `Cargo.toml`이 없지만 `./myapp/Cargo.toml`이 존재함을 감지합니다.

### 출력 (Claude Code로)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

수정이 필요하지 않으면 `updatedInput.command`가 비어 있고 Claude Code는 원래 명령을 사용합니다.

## 단계 세부 정보

### 단계 1: JSON 파싱

stdin에서 전체 JSON 객체를 읽습니다. `tool_input.command`를 추출합니다. 파싱에 실패하면 훅이 즉시 종료되고 Claude Code는 원래 명령을 사용합니다(fail-open 설계).

### 단계 2: 스킬 매칭

SQLite 휴리스틱 데이터베이스에서 트리거 패턴이 명령과 일치하는 스킬을 쿼리합니다. 스킬은 우선순위 순서로 확인됩니다. 내장 TOML 스킬과 마이닝된 스킬 모두 평가됩니다.

### 단계 3: 디렉토리 수정

빌드 명령(`cargo`, `go`, `make`, `npm`, `python` 등)에 대해 예상 프로젝트 파일이 현재 디렉토리에 있는지 확인합니다. 없으면 인근 디렉토리를 스캔하여 가장 가까운 일치를 찾고 `cd <dir> &&`를 앞에 추가합니다.

디렉토리 스캔은 5초 TTL을 가진 캐시된 파일 시스템 인덱스를 사용하여 빠른 속도를 유지합니다.

### 단계 4: GDB 확인

명령이 크래시를 일으킬 가능성이 있는 경우(예: 디버그 바이너리 실행), PRECC는 원시 크래시 로그 대신 구조화된 디버그 출력을 캡처하기 위해 GDB 래퍼를 제안하거나 주입할 수 있습니다.

### 단계 5: RTK 재작성

장황한 명령을 단축하고, 노이즈가 많은 출력을 억제하거나, 토큰 효율성을 위해 명령을 재구성하는 RTK(Rewrite Toolkit) 규칙을 적용합니다.

### 단계 6: JSON 출력

수정된 명령을 JSON으로 직렬화하여 stdout에 씁니다. 변경 사항이 없으면 출력은 Claude Code에 원래 명령을 사용하도록 신호를 보냅니다.

## 성능

전체 파이프라인이 5밀리초(p99) 이내에 완료됩니다. 주요 최적화:

- 잠금 없는 동시 읽기를 위한 WAL 모드의 SQLite
- 스킬 매칭을 위한 사전 컴파일된 정규식 패턴
- 캐시된 파일 시스템 스캔(5초 TTL)
- 핫 경로에 네트워크 호출 없음
- Fail-open: 모든 오류는 원래 명령으로 넘어감

## 훅 수동 테스트

훅을 직접 호출할 수 있습니다:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
