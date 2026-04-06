# GIF 녹화

`precc gif`는 bash 스크립트에서 터미널 세션의 애니메이션 GIF 녹화를 생성합니다. Pro 기능입니다.

## 기본 사용법

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

첫 번째 인수는 실행할 명령을 포함하는 bash 스크립트입니다. 두 번째 인수는 최대 녹화 시간입니다.

## 스크립트 형식

스크립트는 표준 bash 파일입니다:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## 입력 시뮬레이션

대화형 명령의 경우 입력 값을 추가 인수로 제공하세요:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

각 추가 인수는 스크립트가 입력을 요청할 때 stdin 줄로 전달됩니다.

## 출력 옵션

출력 파일은 기본적으로 스크립트 이름을 따릅니다(`script.gif`). GIF는 표준 80x24 크기의 어두운 터미널 테마를 사용합니다.

## 왜 asciinema 대신 GIF인가?

내장 스킬 `asciinema-gif`는 `asciinema rec`를 자동으로 `precc gif`로 다시 작성합니다. GIF 파일은 더 이식성이 높습니다 -- 플레이어 없이 GitHub README, Slack, 이메일에서 인라인으로 표시됩니다.
