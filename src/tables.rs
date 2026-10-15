//! 내장 자료를 읽어 조회표를 만든다.
//!
//! 자료는 모두 `include_str!` 로 바이너리에 들어간다(파일을 따로 배포할 필요 없음).
//! 형식은 사람이 고칠 수 있는 CSV·한 줄 한 항목 텍스트이고, `#` 로 시작하는 줄은 주석이다.
//!
//! | 표 | 파일 | 뜻 |
//! |---|---|---|
//! | 글자표 | `hanja_char.csv` | 한자 → 대표 독음 (Jeff 원본; 통합 한자 행이 대표, 호환 한자 행은 다른 독음) |
//! | 대표 덮어쓰기 | `data/primary_override.csv` | 사전 근거로 바꾼 대표 독음 29자 |
//! | 글자표 확장 | `data/chars_ext.csv` | 원본에 없던 글자(확장 B 이상) 35자 |
//! | 다른 독음 | `data/alts_nikl.csv` | 글자별 다른 독음 (madi 후보용) |
//! | 두음 | `dueum.csv` | 녀→여, 력→역 … 52행 |
//! | 손 예외표 | `hanja_word.csv` | 손으로 검토한 불규칙 단어 (사전보다 우선) |
//! | 사전 예외표 | `data/words_nikl.csv` | 글자별로 읽으면 틀리는 사전 표제어 |
//! | 합성어 두음 | `data/dueum_words.txt` | 연쇄 중간에서 시작하면 두음을 적용할 단어 |
//! | 두음 막기 | `data/dueum_block.txt` | 앞 글자와 이 짝을 이루면 합성어 두음을 막는 2자어 |
//! | 성씨 막기 | `data/surname_block.txt` | 金·沈·葉·契·單 으로 시작하는 보통 낱말 |
//! | 이름 막기 | `data/name_block.txt` | 성+이름 꼴 3자 사전어(全羅道 전라도) — 이름 첫 음절 두음을 적용하지 않음 |
//! | 호환 한자 | `data/compat.csv` | 호환 한자 → 통합 한자 (Unicode NFC) |

use std::collections::{HashMap, HashSet};

const HANJA_CHAR: &str = include_str!("../hanja_char.csv");
const DUEUM: &str = include_str!("../dueum.csv");
const HAND_WORDS: &str = include_str!("../hanja_word.csv");
const COMPAT: &str = include_str!("../data/compat.csv");
const PRIMARY_OVERRIDE: &str = include_str!("../data/primary_override.csv");
const CHARS_EXT: &str = include_str!("../data/chars_ext.csv");
const ALTS: &str = include_str!("../data/alts_nikl.csv");
#[cfg(feature = "nikl")]
const NIKL_WORDS: &str = include_str!("../data/words_nikl.csv");
#[cfg(feature = "nikl")]
const DUEUM_WORDS: &str = include_str!("../data/dueum_words.txt");
#[cfg(feature = "nikl")]
const DUEUM_BLOCK: &str = include_str!("../data/dueum_block.txt");
#[cfg(feature = "nikl")]
const SURNAME_BLOCK: &str = include_str!("../data/surname_block.txt");
#[cfg(feature = "nikl")]
const NAME_BLOCK: &str = include_str!("../data/name_block.txt");

/// 변환에 쓰는 표 전부. `Converter` 가 하나를 가진다.
#[derive(Debug, Default, Clone)]
pub(crate) struct Tables {
    /// 호환 한자 → 통합 한자
    pub compat: HashMap<char, char>,
    /// 한자 → 대표 독음 (한 음절)
    pub prim: HashMap<char, char>,
    /// 한자 → 다른 독음들 (대표 제외)
    pub alts: HashMap<char, Vec<char>>,
    /// 두음법칙: 첫머리에서 바뀌는 음절 (녀→여)
    pub dueum: HashMap<char, char>,
    /// 예외표: 한자열 → 독음 (손 표가 사전 표를 덮는다)
    pub words: HashMap<Vec<char>, String>,
    /// 예외표 키의 최대 길이 (최장 일치 상한)
    pub max_word: usize,
    /// 합성어 안 두음 단어
    pub dwords: HashSet<Vec<char>>,
    pub dmax: usize,
    /// 합성어 두음을 막는 2자어
    pub dblock: HashSet<[char; 2]>,
    /// 성씨로 읽지 않는 金·沈·葉·契·單 시작 낱말
    pub surname_block: HashSet<Vec<char>>,
    /// 성+이름 꼴 3자 사전어
    pub name_block: HashSet<Vec<char>>,
}

/// 주석·빈 줄·BOM 을 걷어 낸 줄들
fn lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(|l| l.trim_start_matches('\u{FEFF}').trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
}

/// `a,b,...` 의 앞 두 칸
fn pair(line: &str) -> Option<(&str, &str)> {
    let mut it = line.splitn(3, ',');
    let a = it.next()?.trim();
    let b = it.next()?.trim();
    if a.is_empty() || b.is_empty() {
        None
    } else {
        Some((a, b))
    }
}

fn first(s: &str) -> Option<char> {
    s.chars().next()
}

impl Tables {
    /// 내장 자료로 표를 만든다. `nikl` 기능(기본 켬)을 끄면 사전 파생 표 넷은 비어 있다.
    pub fn builtin() -> Self {
        let mut t = Tables::default();

        // 1. 호환 한자 → 통합 한자
        for l in lines(COMPAT) {
            if let Some((a, b)) = pair(l) {
                if let (Some(a), Some(b)) = (first(a), first(b)) {
                    t.compat.insert(a, b);
                }
            }
        }

        // 2. 글자표: 통합 한자 행 = 대표. 호환 한자 행은 통합 한자로 접어서
        //    대표가 이미 있으면 다른 독음으로, 없으면 대표로 (madi hanja.rs 와 같은 규칙)
        let mut compat_rows = Vec::new();
        for l in lines(HANJA_CHAR) {
            let Some((k, v)) = pair(l) else { continue };
            let (Some(k), Some(v)) = (first(k), first(v)) else { continue };
            if is_compat(k) {
                compat_rows.push((t.fold(k), v));
            } else {
                t.prim.insert(k, v);
            }
        }
        for (k, v) in compat_rows {
            match t.prim.get(&k) {
                Some(&p) if p != v => push_unique(t.alts.entry(k).or_default(), v),
                Some(_) => {}
                None => {
                    t.prim.insert(k, v);
                }
            }
        }

        // 3. 대표 덮어쓰기 · 글자표 확장
        for l in lines(PRIMARY_OVERRIDE).chain(lines(CHARS_EXT)) {
            if let Some((k, v)) = pair(l) {
                if let (Some(k), Some(v)) = (first(k), first(v)) {
                    t.prim.insert(k, v);
                }
            }
        }
        // 不 은 대표 '불', ㄷ·ㅈ 앞에서 '부' (규칙은 lib.rs)
        t.prim.insert('不', '불');

        // 4. 다른 독음: 사전 정렬로 확인된 것을 더한다
        for l in lines(ALTS) {
            if let Some((k, v)) = pair(l) {
                let Some(k) = first(k) else { continue };
                for a in v.split_whitespace().filter_map(first) {
                    push_unique(t.alts.entry(k).or_default(), a);
                }
            }
        }
        for (k, a) in t.alts.iter_mut() {
            if let Some(p) = t.prim.get(k) {
                a.retain(|c| c != p);
            }
        }
        t.alts.retain(|_, a| !a.is_empty());

        // 5. 두음
        for l in lines(DUEUM) {
            if let Some((k, v)) = pair(l) {
                if let (Some(k), Some(v)) = (first(k), first(v)) {
                    t.dueum.insert(k, v);
                }
            }
        }

        // 6. 예외표: 사전 표를 먼저, 손 표가 덮는다
        #[cfg(feature = "nikl")]
        for l in lines(NIKL_WORDS) {
            if let Some((k, v)) = pair(l) {
                let k = t.fold_str(k);
                t.words.insert(k, v.to_string());
            }
        }
        for l in lines(HAND_WORDS) {
            if let Some((k, v)) = pair(l) {
                let k = t.fold_str(k);
                t.words.insert(k, v.to_string());
            }
        }
        t.max_word = t.words.keys().map(Vec::len).max().unwrap_or(0);

        // 7. 합성어 두음 · 두음 막기 · 성씨 막기
        #[cfg(feature = "nikl")]
        {
            for l in lines(DUEUM_WORDS) {
                t.dwords.insert(l.chars().collect());
            }
            t.dmax = t.dwords.iter().map(Vec::len).max().unwrap_or(0);
            for l in lines(DUEUM_BLOCK) {
                let v: Vec<char> = l.chars().collect();
                if v.len() == 2 {
                    t.dblock.insert([v[0], v[1]]);
                }
            }
            for l in lines(SURNAME_BLOCK) {
                t.surname_block.insert(l.chars().collect());
            }
            for l in lines(NAME_BLOCK) {
                t.name_block.insert(l.chars().collect());
            }
        }
        t
    }

    /// 호환 한자를 통합 한자로 (그 밖의 글자는 그대로)
    pub fn fold(&self, c: char) -> char {
        *self.compat.get(&c).unwrap_or(&c)
    }

    pub fn fold_str(&self, s: &str) -> Vec<char> {
        s.chars().map(|c| self.fold(c)).collect()
    }
}

fn is_compat(c: char) -> bool {
    matches!(c as u32, 0xF900..=0xFAFF | 0x2F800..=0x2FA1F)
}

fn push_unique(v: &mut Vec<char>, c: char) {
    if !v.contains(&c) {
        v.push(c);
    }
}
