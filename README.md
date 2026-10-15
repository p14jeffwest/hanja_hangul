# hanja_hangul

한자가 섞인 한국어 글의 한자를 한글 독음으로 바꾸는 Rust 라이브러리와 명령줄 도구.
의존 크레이트가 없고 자료는 모두 내장되어 있다.

```
金대통령은 新女性 運動과 經濟理論을 美術評論家에게 說明했다. 1995年度 困難, 綠茶와 茶房.
→ 김대통령은 신여성 운동과 경제이론을 미술평론가에게 설명했다. 1995년도 곤란, 녹차와 다방.
```

## 정확도와 속도

**사전 표제어.** 국립국어원 《표준국어대사전》·《우리말샘》의 순한자 표제어 493,043개 가운데 10%를 떼어 두고,
나머지로 예외표를 만든 뒤 떼어 둔 표제어를 읽혀 잰 값(표제어 독음과 같으면 정답).

| 변환기 | 떼어 둔 표제어 49,971 | 그중 표준국어대사전 표제어 |
|---|---|---|
| 0.1.3 (글자별 + 두음법칙) | 93.43% | 94.88% |
| **0.2.0** | **98.52%** | **98.23%** |

예외표는 전체 표제어로 만들어 싣으므로 **사전에 있는 말은 99.99% 맞는다**. 위 98.5%는 사전에 없는 말을 어림한 값이다.

**신문.** 동아일보 1996~2000년·조선일보 2003~2004년 기사의 괄호 병기(`김종필(金鍾泌)`, `對北(대북)`) 62,037종,
출현 259,680회를 정답으로 잰 값(출현 가중). 절반쯤이 인명이다.

| 변환기 | 전체 | 2자어 | 성+이름 3자 | 그 밖 |
|---|---|---|---|---|
| 0.1.3 | 97.2% | 97.7% | 97.7% | 91.2% |
| **0.2.0** | **98.8%** | **98.8%** | **98.9%** | **98.7%** |

0.1.3 은 연쇄 중간에도 두음을 적용하는 동작 때문에 이름(金龍煥 김용환)은 우연히 맞고 보통 낱말(分裂 분렬)에서 틀린다.

- 틀리는 것: 이름 둘째 음절(尹鍾龍 윤종용 / 具滋龍 구자룡 — 사람마다 다름), 정치 약칭(親盧 친노), 사전 밖 다독음(樂 악/락, 車 차/거), 일본·중국 인명(森喜朗 → 삼희랑), 한자 숫자(二〇一六年).
- 속도: 한자가 절반인 글 558만 자를 0.54초(초당 약 1,000만 자, 단일 스레드). 표 만들기 10~20 ms. 실행 파일 1.2 MB.

## 쓰는 법

### 라이브러리

```toml
[dependencies]
hanja_hangul = "0.2"
```

```rust
use hanja_hangul::{Converter, Options, Style};

let conv = Converter::global();                     // 처음 부를 때 한 번 표를 만든다
assert_eq!(conv.convert("樂園에서 音樂을"), "낙원에서 음악을");

// 한글(漢字) 꼴, 성씨 규칙 끄기
let c = Converter::new().with_options(Options { style: Style::HangulHanja, surname: false });
assert_eq!(c.convert("經濟"), "경제(經濟)");

// 내 단어 더하기 (내장 표보다 우선)
let mut c = Converter::new();
c.add_word("金鍾泌", "김종필");

// 형태소 분석기용: 한 구간의 독음과 후보들 ([0] 은 언제나 reading 과 같다)
assert_eq!(conv.reading("樂園", true).as_deref(), Some("낙원"));
let cands = conv.readings("樂", true, 8);           // ["악", "낙", "요"] (첫머리라 락 → 낙)
```

### 명령줄

```
cargo install hanja_hangul
hanja2hangul 입력.txt -o 출력.txt
echo "經濟 發展" | hanja2hangul --style hanja-hangul      # 經濟(경제) 發展(발전)
hanja2hangul --no-surname < 입력.txt
```

## 어떻게 읽는가

글을 한자 연쇄(한자가 이어진 구간)로 나누고, 연쇄마다 왼쪽부터:

1. **예외표 최장 일치** — 글자별로 읽으면 틀리는 단어(客車 객차, 困難 곤란, 樂園 낙원, 便所 변소 …).
2. **글자별 대표 독음** — 표에 없으면 글자 하나씩.
3. **두음법칙** — 연쇄 첫머리의 첫 음절에(女子 여자). 숫자나 한글 바로 뒤는 첫머리가 아니다(1995年度 1995년도, 色깔論 색깔론). 연쇄 중간이라도 두음이 일어나는 단어가 시작되면 적용하고(新女性 신여성), 앞 글자와 이미 한 단어를 이루면 막는다(評論家 평론가).
4. **렬·률** — 모음·ㄴ 받침 뒤에서 열·율(分裂 분열, 比率 비율).
5. **不** — ㄷ·ㅈ 앞에서 부, 그 밖에는 불(不當 부당, 不法 불법).
6. **인명** — 金·沈·葉·契·單 으로 시작하는 연쇄가 보통 낱말(金額 금액, 沈默 침묵)이 아니면 성씨 독음(金泳三 김영삼, 金대통령 김대통령). 李·盧·柳·林 은 두음법칙이 맡는다. 흔한 성 한자 + 두 글자 이름이면 이름 첫 음절에도 두음(李龍俊 이용준, 金令培 김영배, 權魯甲 권노갑) — 사전 3자어(全羅道 전라도)나 앞 두 글자가 사전 2자어(權力化)인 것은 빼고. 연쇄 전체가 예외표에 있으면 예외표가 이긴다(金龍煥 김용환).
7. **괄호 병기** — `윤종용(尹鍾龍)`, `林昌烈(임창열)` 처럼 글쓴이가 단 한글이 그 한자의 독음으로 설명되면 그 한글로 읽는다(이름 끝 글자는 사람마다 달라 규칙으로 못 맞힌다: 임창열 / 최병렬). `도쿄(東京)`, `히로시(三塚博)` 처럼 설명되지 않으면 외국어 표기로 보고 한자를 그대로 둔다. `Options::gloss` 로 끈다.
8. **호환 한자**(U+F900~)는 통합 한자로 접어 읽는다. 표에 없는 한자는 그대로 둔다.
9. 손 표의 병기 — 문맥 없이 못 고르는 말은 둘 다 적는다: 契丹 → 거란(글안), 茶禮 → 차례/다례.

## 자료

| 파일 | 내용 | 출처 |
|---|---|---|
| `hanja_char.csv` | 한자 27,848자 → 독음 | 지은이 |
| `dueum.csv` | 두음법칙 52행 | 지은이 |
| `hanja_word.csv` | 손으로 검토한 불규칙 단어 135행 (내장 표 가운데 가장 우선) | 지은이 |
| `data/words_nikl.csv` | 예외표 9,536행 — 사전 표제어 가운데 글자별로 읽으면 틀리는 것만 | 국립국어원 사전 파생 |
| `data/dueum_words.txt` | 합성어 안 두음 단어 17,292 | 국립국어원 사전 파생 |
| `data/dueum_block.txt` | 합성어 두음을 막는 2자어 7,523 | 국립국어원 사전 파생 |
| `data/surname_block.txt` | 성씨로 읽지 않는 金·沈·葉·契·單 시작 낱말 2,815 | 국립국어원 사전 파생 |
| `data/name_block.txt` | 성+이름 꼴 3자 사전어 564 (이름 두음을 적용하지 않음) | 국립국어원 사전 파생 |
| `data/primary_override.csv` | 사전 표제어 집계로 바꾼 대표 독음 28자 | 국립국어원 사전 파생 |
| `data/chars_ext.csv`, `data/alts_nikl.csv` | 확장 B 이상 글자 35자, 글자별 다른 독음 | 국립국어원 사전 파생 |
| `data/compat.csv` | 호환 한자 → 통합 한자 | Unicode |

`data/` 의 사전 파생 표는 `tools/` 의 스크립트로 다시 만들 수 있다
(`extract_nikl.py` → `build_words.py`; 입력은 국립국어원 사이트에서 받는 사전 전체 내려받기 자료).

## 라이선스

- 코드와 지은이 자료(`hanja_char.csv`, `dueum.csv`, `hanja_word.csv`): MIT (`LICENSE`).
- `data/` 의 국립국어원 사전 파생 표: **CC BY-SA 2.0 KR**. 출처 —
  국립국어원 《표준국어대사전》(https://stdict.korean.go.kr), 《우리말샘》(https://opendict.korean.go.kr).
  이 표들을 고쳐 다시 배포하면 같은 조건(CC BY-SA)을 따른다.
- 사전 파생 표를 싣고 싶지 않으면 `default-features = false` 로 쓴다(지은이 자료만; 정확도는 0.1.3 수준).

## 0.1 에서 옮기기

0.1 의 `load_dictionary`, `load_dictionary_from_file`, `chi2kor_str`, `chi2kor_file` 은 없어졌고 `tokio` 의존도 없어졌다.

| 0.1 | 0.2 |
|---|---|
| `let (c, d, w) = load_dictionary()?; chi2kor_str(s, &c, &d, &w)` | `Converter::global().convert(s)` |
| `chi2kor_file(입력, 출력, …).await` | `std::fs::write(출력, conv.convert(&std::fs::read_to_string(입력)?).as_bytes())` 또는 `hanja2hangul 입력 -o 출력` |
| `hanja_word.csv` 에 단어 더하기 | `conv.add_word("金鍾泌", "김종필")` |

---

## English

`hanja_hangul` converts hanja (Chinese characters) in Korean text into hangul readings.
No dependencies; all tables are embedded. It reads each run of hanja left to right using
a longest-match exception table (only the dictionary headwords that per-character reading
gets wrong), per-character default readings, the initial sound law (두음법칙, including
inside compounds), the 렬/률 and 不 rules, and a surname rule for 金·沈·葉·契·單.

On 49,971 held-out headwords of the NIKL Standard Korean Dictionary and Urimalsaem it reads
98.5% correctly (0.1.3: 93.4%); words in the dictionaries are read correctly 99.99% of the time.
On 259,680 hanja–hangul parenthetical glosses from Korean newspapers (about half personal names)
it reads 98.8% correctly.
About 10 million characters per second on one core.

Code: MIT. Tables in `data/` are derived from the NIKL Standard Korean Dictionary and
Urimalsaem and are licensed CC BY-SA 2.0 KR; disable the default `nikl` feature to leave them out.
