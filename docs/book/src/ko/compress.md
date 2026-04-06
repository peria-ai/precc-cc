# 압축

`precc compress`는 Claude Code가 로드할 때 토큰 사용량을 줄이기 위해 CLAUDE.md 및 기타 컨텍스트 파일을 축소합니다. Pro 기능입니다.

## 기본 사용법

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## 드라이 런

파일을 수정하지 않고 변경될 내용 미리보기:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## 되돌리기

원본은 자동으로 백업됩니다. 복원하려면:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## 무엇이 압축되는가

압축기는 여러 변환을 적용합니다:

- 불필요한 공백과 빈 줄 제거
- 의미를 유지하면서 장황한 표현 단축
- 테이블과 목록 압축
- 주석과 장식적 서식 제거
- 모든 코드 블록, 경로, 기술 식별자 보존

압축된 출력은 여전히 사람이 읽을 수 있습니다 -- 축소화되거나 난독화되지 않았습니다.

## 특정 파일 대상 지정

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
