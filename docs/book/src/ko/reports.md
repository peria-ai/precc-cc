# 보고서

`precc report`는 PRECC 활동과 토큰 절감을 요약하는 분석 대시보드를 생성합니다.

## 보고서 생성

```bash
$ precc report
PRECC Report -- 2026-04-03
==========================

Sessions analyzed: 12
Commands intercepted: 87
Total token savings: 42,389

Top skills by activation:
  1. cargo-wrong-dir     34 activations   17,204 tokens saved
  2. npm-wrong-dir       18 activations    9,360 tokens saved
  3. git-wrong-dir       12 activations    4,944 tokens saved
  4. RTK rewrite         15 activations    3,750 tokens saved
  5. python-wrong-dir     8 activations    4,131 tokens saved

Savings by pillar:
  Pillar 1 (context resolution):  28,639 tokens  67.6%
  Pillar 4 (automation skills):    7,000 tokens  16.5%
  RTK rewrites:                    3,750 tokens   8.8%
  Lean-ctx wraps:                  3,000 tokens   7.1%

Recent corrections:
  2026-04-03 09:12  cargo build -> cd myapp && cargo build
  2026-04-03 09:18  npm test -> cd frontend && npm test
  2026-04-03 10:05  git status -> cd repo && git status
  ...
```

## 보고서 이메일 전송

이메일 주소로 보고서 전송 (메일 설정 필요, [Email](email.md) 참조):

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

수신자 주소는 `~/.config/precc/mail.toml`에서 읽힙니다. `precc mail report EMAIL`을 사용하여 특정 주소로 보낼 수도 있습니다.

## 보고서 데이터

보고서는 `~/.local/share/precc/history.db`의 로컬 PRECC 데이터베이스에서 생성됩니다. 보고서를 명시적으로 이메일로 보내지 않는 한 데이터는 머신을 떠나지 않습니다.
