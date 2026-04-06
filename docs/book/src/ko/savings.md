# 절약

PRECC는 매 인터셉션에서 추정 토큰 절약량을 추적합니다. `precc savings`를 사용하여 PRECC가 얼마나 낭비를 방지했는지 확인하세요.

## 빠른 요약

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Breakdown:
  Pillar 1 (cd prepends):         <span data-stat="session_p1_tokens">3,204</span> tokens  (<span data-stat="session_p1_count">6</span> corrections)
  Pillar 4 (skill activations):   <span data-stat="session_p4_tokens">1,560</span> tokens  (<span data-stat="session_p4_count">4</span> activations)
  RTK rewrites:                   <span data-stat="session_rtk_tokens">2,749</span> tokens  (<span data-stat="session_rtk_count">11</span> rewrites)
  Lean-ctx wraps:                 <span data-stat="session_lean_tokens">1,228</span> tokens  (<span data-stat="session_lean_count">2</span> wraps)
```

## 상세 분석 (Pro)

```bash
$ precc savings --all
Session Token Savings (Detailed)
================================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Command-by-command:
  #  Time   Command                          Saving   Source
  1  09:12  cargo build                      534 tk   cd prepend (cargo-wrong-dir)
  2  09:14  cargo test                       534 tk   cd prepend (cargo-wrong-dir)
  3  09:15  git status                       412 tk   cd prepend (git-wrong-dir)
  4  09:18  npm install                      824 tk   cd prepend (npm-wrong-dir)
  5  09:22  find . -name "*.rs"              387 tk   RTK rewrite (output truncation)
  6  09:25  cat src/main.rs                  249 tk   RTK rewrite (lean-ctx wrap)
  7  09:31  cargo clippy                     534 tk   cd prepend (cargo-wrong-dir)
  ...

Pillar Breakdown:
  Pillar 1 (context resolution):   <span data-stat="session_p1_tokens">3,204</span> tokens  <span data-stat="session_p1_pct">36.6</span>%
  Pillar 2 (GDB debugging):            0 tokens   0.0%
  Pillar 3 (mined preventions):        0 tokens   0.0%
  Pillar 4 (automation skills):    <span data-stat="session_p4_tokens">1,560</span> tokens  <span data-stat="session_p4_pct">17.8</span>%
  RTK rewrites:                    <span data-stat="session_rtk_tokens">2,749</span> tokens  <span data-stat="session_rtk_pct">31.5</span>%
  Lean-ctx wraps:                  <span data-stat="session_lean_tokens">1,228</span> tokens  <span data-stat="session_lean_pct">14.1</span>%
```

## 절약량 추정 방법

각 수정 유형에는 PRECC 없이 발생했을 상황을 기반으로 한 예상 토큰 비용이 있습니다:

| 수정 유형 | 예상 절약 | 근거 |
|----------------|-----------------|-----------|
| cd prepend | ~500 tokens | 오류 출력 + Claude 추론 + 재시도 |
| 스킬 활성화 | ~400 tokens | 오류 출력 + Claude 추론 + 재시도 |
| RTK rewrite | ~250 tokens | Claude가 읽어야 할 장황한 출력 |
| Lean-ctx wrap | ~600 tokens | 대용량 파일 내용 압축 |
| 마이닝된 예방 | ~500 tokens | 알려진 실패 패턴 회피 |

이것은 보수적인 추정치입니다. Claude의 오류에 대한 추론이 장황할 수 있으므로 실제 절약량은 종종 더 높습니다.

## 누적 절약

절약량은 PRECC 데이터베이스에서 세션 간에 유지됩니다. 시간이 지나면서 전체적인 영향을 추적할 수 있습니다:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Lifetime savings: <span data-stat="total_tokens_saved">142,389</span> tokens across <span data-stat="total_sessions">47</span> sessions
```
