# 0.2 2단계 — Rust 구현 대조·속도 (2026-10-15)

## 파이썬 기준 구현과의 대조
- 입력: 국립국어원 사전 순한자 표제어 493,043개(한 줄에 하나) + `sample.txt`.
- 파이썬 `tools/demo.py` 와 Rust `examples/parity.rs` 의 `convert` 출력이 **493,214줄 모두 같다** (차이 0).
- 재현:
  ```
  python3 tools/dump_keys.py                         # out/all_keys.txt
  python3 tools/demo.py < keys.txt > py.txt
  cargo run --release --example parity < keys.txt > rs.txt
  diff py.txt rs.txt
  ```

## 속도 (작업 공간 Linux, 단일 스레드, release)
| | 558만 자(한자 284만) | 표 만들기 |
|---|---|---|
| 0.1.3 | 0.24 s (2,300만 자/s) | 3 ms |
| **0.2.0** | **0.54 s (1,000만 자/s)** | 12~19 ms |
| gukhanmun 0.3 (참고) | 50.9 s (11만 자/s) | — |

실행 파일 1.2 MB, crates.io 묶음 321 KB(압축).

## 시험
`tests/cases.rs` 12개 + 문서 예제 1개 통과. `--no-default-features`(사전 파생 표 없이) 빌드도 확인 — 이때 성씨 규칙은 꺼진다(보통 낱말 표가 없어 金額 이 김액 이 되므로).
