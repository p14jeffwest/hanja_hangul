//! # hanja_hangul — 한자를 한글로
//!
//! 국한문 혼용 글의 한자를 한글 독음으로 바꾼다. 의존 크레이트가 없고, 자료는 모두 내장이다.
//!
//! ```
//! use hanja_hangul::Converter;
//!
//! let conv = Converter::global();
//! assert_eq!(conv.convert("金대통령은 新女性 運動과 經濟理論을 말했다"),
//!            "김대통령은 신여성 운동과 경제이론을 말했다");
//! assert_eq!(conv.convert("樂園에서 音樂과 快樂을"), "낙원에서 음악과 쾌락을");
//! ```
//!
//! ## 어떻게 읽는가
//!
//! 글을 한자 연쇄(한자가 이어진 구간) 단위로 읽는다. 연쇄마다 왼쪽부터:
//!
//! 1. **예외표 최장 일치** — 글자별로 읽으면 틀리는 단어(客車 객차, 困難 곤란, 樂園 낙원 …)는
//!    표에 적힌 독음을 쓴다. 표는 손으로 검토한 `hanja_word.csv` 와, 국립국어원
//!    《표준국어대사전》·《우리말샘》 표제어에서 "글자별로 읽으면 틀리는 것만" 뽑은
//!    `data/words_nikl.csv` 다.
//! 2. **글자별 대표 독음** — 표에 없으면 글자 하나씩 대표 독음으로.
//! 3. **두음법칙** — 연쇄 첫머리(앞이 공백·문장부호·글 처음)의 첫 음절에. 숫자나 한글 바로 뒤는
//!    첫머리가 아니다(1995年度 1995년도, 色깔論 색깔론). 연쇄 중간이라도 두음이 일어나는
//!    단어가 시작되면 적용한다(新女性 신여성) — 단 앞 글자와 이미 한 단어를 이루면 막는다
//!    (評論家 평론가).
//! 4. **렬·률 규칙** — 모음이나 ㄴ 받침 뒤의 렬·률은 열·율(分裂 분열, 比率 비율).
//! 5. **不** — ㄷ·ㅈ 앞에서 부, 그 밖에는 불(不當 부당, 不法 불법).
//! 6. **茶** — 대표 독음은 차(綠茶 녹차, 茶 한 잔). 茶房 다방·茶道 다도처럼 앞자리 '다' 는 예외표가 받는다.
//! 7. **인명** — 金·沈·葉·契·單 으로 시작하는 연쇄가 보통 낱말(金額 금액)이 아니면 성씨 독음
//!    (金泳三 김영삼, 金대통령 김대통령). 흔한 성 한자 + 두 글자 이름이면 이름 첫 음절에 두음
//!    (李龍俊 이용준, 金令培 김영배). 연쇄 전체가 예외표에 있으면 예외표가 이긴다.
//!    [`Options::surname`] 으로 끈다.
//! 8. **괄호 병기** — `윤종용(尹鍾龍)`·`林昌烈(임창열)` 처럼 글쓴이가 단 한글이 그 한자의 독음으로 설명되면
//!    그 한글로 읽고, `도쿄(東京)`·`히로시(三塚博)` 처럼 설명되지 않으면 외국어 표기로 보고 한자를 그대로 둔다. [`Options::gloss`] 로 끈다.
//!
//! 사전 표제어로 잰 정확도와 자료 출처는 README 를 보라.

mod tables;

use std::borrow::Cow;
use std::sync::OnceLock;

use tables::Tables;

/// 출력 꼴
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Style {
    /// 한글만: 經濟 → 경제
    #[default]
    Hangul,
    /// 한글(漢字): 經濟 → 경제(經濟)
    HangulHanja,
    /// 漢字(한글): 經濟 → 經濟(경제)
    HanjaHangul,
}

/// 변환 선택 사항
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    /// 출력 꼴
    pub style: Style,
    /// 인명 규칙: 성씨(金泳三 김영삼)와 이름 첫 음절 두음(李龍俊 이용준). 끄면 金 은 늘 금.
    pub surname: bool,
    /// 괄호 병기 문맥: `윤종용(尹鍾龍)`·`林昌烈(임창열)` 처럼 글쓴이가 단 한글이 그 한자의 독음으로
    /// 설명되면 그 한글로 읽고, `도쿄(東京)`·`히로시(三塚博)` 처럼 설명되지 않으면(외국어 표기) 한자를 그대로 둔다.
    pub gloss: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { style: Style::Hangul, surname: true, gloss: true }
    }
}

/// 한자 → 한글 변환기. 표를 한 번 만들어 두고 여러 번 쓴다.
#[derive(Debug, Clone)]
pub struct Converter {
    t: Tables,
    opts: Options,
}

/// 이름 첫 음절 두음 규칙을 쓰는 흔한 성 한자
const NAME_SURNAMES: &str = "金李朴崔鄭姜趙尹張林韓吳徐申權黃安宋柳洪全高文孫梁裵白曺許劉南沈盧河丁成郭車兪具禹朱任羅辛閔陳池嚴蔡元千方孔玄咸卞楊廉邊呂秋魯都蘇愼石宣薛馬吉周延表明魏潘王琴玉陸印孟諸卓秦慶睦太奇殷片龍芮邢偰鞠陰甘";

/// 성씨 독음 (두음법칙으로 해결되지 않는 다독음 성씨만; 李·盧·柳·林 은 두음법칙이 맡는다)
const SURNAMES: [(char, char); 5] = [('金', '김'), ('沈', '심'), ('葉', '섭'), ('契', '글'), ('單', '선')];

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter {
    /// 내장 자료로 새 변환기를 만든다 (수십 ms). 여러 번 쓸 거면 [`Converter::global`].
    pub fn new() -> Self {
        Converter { t: Tables::builtin(), opts: Options::default() }
    }

    /// 프로그램 전체가 함께 쓰는 기본 변환기 (처음 부를 때 한 번 만든다)
    pub fn global() -> &'static Converter {
        static G: OnceLock<Converter> = OnceLock::new();
        G.get_or_init(Converter::new)
    }

    /// 선택 사항을 바꾼 변환기
    pub fn with_options(mut self, opts: Options) -> Self {
        self.opts = opts;
        self
    }

    pub fn options(&self) -> Options {
        self.opts
    }

    /// 예외 단어를 더한다 (내장 표보다 우선). 독음에 `거란(글안)` 이나 `차례/다례` 처럼 병기하면
    /// [`convert`](Self::convert) 는 그대로 내고, [`reading`](Self::reading) 은 앞쪽을,
    /// [`readings`](Self::readings) 는 둘 다 낸다.
    pub fn add_word(&mut self, hanja: &str, hangul: &str) {
        let k = self.t.fold_str(hanja);
        self.t.max_word = self.t.max_word.max(k.len());
        self.t.words.insert(k, hangul.to_string());
    }

    /// 글자 하나의 대표 독음
    pub fn char_reading(&self, c: char) -> Option<char> {
        self.t.prim.get(&self.t.fold(c)).copied()
    }

    /// 글 전체를 바꾼다. 한자가 없으면 빌려 온 그대로 돌려준다.
    pub fn convert<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if !contains_hanja(text) {
            return Cow::Borrowed(text);
        }
        let cs: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len() * 3 / 2);
        let mut i = 0;
        while i < cs.len() {
            if !is_hanja(cs[i]) {
                out.push(cs[i]);
                i += 1;
                continue;
            }
            let mut j = i;
            while j < cs.len() && is_hanja(cs[j]) {
                j += 1;
            }
            // 연쇄 첫머리: 앞이 숫자나 한글 음절이면 아니다
            let word_start = i == 0 || !(cs[i - 1].is_ascii_digit() || is_syllable(cs[i - 1]));
            if !(self.opts.gloss && self.convert_glossed(&cs, i, j, &mut out)) {
                self.convert_run(&cs[i..j], word_start, &mut out);
            }
            i = j;
        }
        Cow::Owned(out)
    }

    /// 괄호 병기 문맥이면 처리하고 true. `한글(漢字)` 또는 `漢字(한글)` 에서 한글이 한자와 글자 수가 같을 때:
    /// 한글이 글자마다 그 한자의 독음(대표·다른 독음·두음 꼴)이면 그 한글을 독음으로 쓰고,
    /// `한글(漢字)` 인데 독음으로 설명되지 않으면(도쿄(東京), 히로시(三塚博)) 한자를 그대로 둔다.
    fn convert_glossed(&self, cs: &[char], i: usize, j: usize, out: &mut String) -> bool {
        let n = j - i;
        let closes = |k: usize| k == cs.len() || matches!(cs[k], ')' | '·' | ',' | '/');
        // 한글(漢字): 연쇄 앞이 '(' 이고 그 앞 n 글자가 한글 음절
        let before = (i > n && cs[i - 1] == '(' && closes(j) && cs[i - 1 - n..i - 1].iter().all(|&c| is_syllable(c)))
            .then(|| &cs[i - 1 - n..i - 1]);
        // 漢字(한글): 연쇄 뒤가 '(' 이고 그다음 n 글자가 한글 음절
        let after = (j + 1 + n <= cs.len() && j < cs.len() && cs[j] == '(' && closes(j + 1 + n)
            && cs[j + 1..j + 1 + n].iter().all(|&c| is_syllable(c)))
            .then(|| &cs[j + 1..j + 1 + n]);
        let Some(given) = before.or(after) else { return false };
        let run = &cs[i..j];
        if self.explains(run, given) {
            let reading: String = given.iter().collect();
            let original: String = run.iter().collect();
            self.emit(&reading, &original, out);
            true
        } else if before.is_some() {
            out.extend(run.iter());      // 외국어 표기의 한자: 그대로
            true
        } else {
            false
        }
    }

    /// 한글 `given` 이 글자마다 한자 `run` 의 독음으로 설명되는가
    fn explains(&self, run: &[char], given: &[char]) -> bool {
        run.len() == given.len()
            && run.iter().zip(given).all(|(&h, &g)| {
                let h = self.t.fold(h);
                let Some(&p) = self.t.prim.get(&h) else { return false };
                let alts = self.t.alts.get(&h).map(Vec::as_slice).unwrap_or(&[]);
                std::iter::once(p).chain(alts.iter().copied()).any(|r| r == g || self.du(r) == g)
            })
    }

    fn emit(&self, reading: &str, original: &str, out: &mut String) {
        match self.opts.style {
            Style::Hangul => out.push_str(reading),
            Style::HangulHanja => {
                out.push_str(reading);
                out.push('(');
                out.push_str(original);
                out.push(')');
            }
            Style::HanjaHangul => {
                out.push_str(original);
                out.push('(');
                out.push_str(reading);
                out.push(')');
            }
        }
    }

    /// 한자 연쇄 하나. 글자표에 없는 글자가 있으면 그 글자는 그대로 두고 앞뒤를 따로 읽는다.
    fn convert_run(&self, run: &[char], word_start: bool, out: &mut String) {
        let folded: Vec<char> = run.iter().map(|&c| self.t.fold(c)).collect();
        let mut s = 0;
        let mut ws = word_start;
        while s < folded.len() {
            if !self.t.prim.contains_key(&folded[s]) {
                out.push(run[s]);
                s += 1;
                ws = false;
                continue;
            }
            let mut e = s;
            while e < folded.len() && self.t.prim.contains_key(&folded[e]) {
                e += 1;
            }
            let reading = self.read_with_surname(&folded[s..e], ws, true).unwrap_or_default();
            let original: String = run[s..e].iter().collect();
            self.emit(&reading, &original, out);
            s = e;
            ws = false;
        }
    }

    /// 인명 규칙을 얹은 읽기. 순서: 예외표 → 성씨 → 이름 첫 음절 두음
    fn read_with_surname(&self, span: &[char], word_start: bool, gloss: bool) -> Option<String> {
        let mut r = self.read(span, word_start, gloss)?;
        if !self.opts.surname {
            return Some(r);
        }
        // 연쇄 전체가 예외표에 있으면 예외표가 이긴다 (사전 인명 표제어 金龍煥 김용환)
        let exact = self.t.words.contains_key(span);
        let mut surname = false;
        if !exact {
            if let Some(s) = self.surname_reading(span, gloss) {
                r = s;
                surname = true;
            }
        }
        // 이름 첫 음절 두음: 흔한 성 한자 + 두 글자 이름 (李龍俊 이용준, 金令培 김영배, 權魯甲 권노갑).
        // 사전 3자어(全羅道)이거나 앞 두 글자가 사전 2자어(權力化 의 權力)면 하지 않는다.
        // 단 성씨 규칙이 이미 인명으로 봤으면 2자어 막기는 건너뛴다 (金龍洙: 金龍 금룡 은 낱말이지만 인명).
        if span.len() == 3
            && !exact
            && !self.t.name_block.is_empty()
            && NAME_SURNAMES.contains(span[0])
            && !self.t.name_block.contains(span)
            && (surname || !self.t.dblock.contains(&[span[0], span[1]]))
        {
            let mut cs: Vec<char> = r.chars().collect();
            if cs.len() == 3 {
                cs[1] = self.du(cs[1]);
                r = cs.into_iter().collect();
            }
        }
        Some(r)
    }

    /// 金·沈·葉·契·單 으로 시작하는 연쇄를 성씨로 읽을지.
    /// 연쇄가 2~3자면 연쇄 전체가, 4자 이상이면 앞 2~4자가 보통 낱말 표(surname_block)에 있으면 성씨가 아니다.
    /// 한 글자(뒤에 한글이 붙는 金대통령)는 金·沈 만.
    fn surname_reading(&self, span: &[char], gloss: bool) -> Option<String> {
        // 보통 낱말 표가 없으면(nikl 기능 끔) 金額 까지 김액 이 되므로 규칙을 쓰지 않는다
        if self.t.surname_block.is_empty() {
            return None;
        }
        let &(c, sur) = SURNAMES.iter().find(|(c, _)| *c == span[0])?;
        if span.len() == 1 && c != '金' && c != '沈' {
            return None;
        }
        let blocked = if span.len() <= 3 {
            self.t.surname_block.contains(span)
        } else {
            (2..=4).any(|l| self.t.surname_block.contains(&span[..l]))
        };
        if blocked {
            return None;
        }
        let mut r = String::new();
        r.push(sur);
        if span.len() > 1 {
            r.push_str(&self.read(&span[1..], false, gloss)?);
        }
        Some(r)
    }

    /// 한자열(통합 한자로 접힌 것)의 독음. `word_start` 면 첫머리 두음.
    /// 글자표에 없는 글자가 있으면 `None`. 성씨 규칙은 쓰지 않는다.
    pub fn reading(&self, hanja: &str, word_start: bool) -> Option<String> {
        let span = self.t.fold_str(hanja);
        self.read(&span, word_start, false)
    }

    /// 독음 후보들: `[0]` 은 언제나 [`reading`](Self::reading) 과 같은 대표 독음이고,
    /// 뒤로 다독음 글자 조합(樂 낙·락·요 …)과 괄호 병기(거란(글안) → 글안)를 `limit` 개까지 붙인다.
    /// 형태소 분석기가 후보마다 사전을 다시 찾아 고르는 데 쓴다 (madi).
    pub fn readings(&self, hanja: &str, word_start: bool, limit: usize) -> Vec<String> {
        let span = self.t.fold_str(hanja);
        let Some(pieces) = self.segment(&span, word_start) else { return Vec::new() };
        // 조각마다 선택지: [0] = 기본
        let opts: Vec<Vec<Vec<char>>> = pieces
            .iter()
            .map(|p| match p.src {
                Some(h) => {
                    let mut v = vec![p.text.clone()];
                    if let Some(alts) = self.t.alts.get(&h) {
                        for &a in alts {
                            let a = if p.dueum { self.du(a) } else { a };
                            if !v.iter().any(|x| x[0] == a) {
                                v.push(vec![a]);
                            }
                        }
                    }
                    v
                }
                None => {
                    let (main, gl) = split_gloss(&p.text);
                    let mut v = vec![main];
                    if let Some(g) = gl {
                        v.push(g);
                    }
                    v
                }
            })
            .collect();
        let mut out: Vec<String> = Vec::new();
        let mut idx = vec![0usize; opts.len()];
        loop {
            let chosen: Vec<Piece> = pieces
                .iter()
                .zip(&idx)
                .zip(&opts)
                .map(|((p, &k), o)| Piece { text: o[k].clone(), src: p.src, dueum: p.dueum })
                .collect();
            let r = self.finish(&chosen, word_start);
            if !out.contains(&r) {
                out.push(r);
                if out.len() >= limit.max(1) {
                    break;
                }
            }
            // 다음 조합 (뒤 자리부터 올림)
            let mut k = opts.len();
            loop {
                if k == 0 {
                    return out;
                }
                k -= 1;
                idx[k] += 1;
                if idx[k] < opts[k].len() {
                    break;
                }
                idx[k] = 0;
            }
        }
        out
    }

    fn du(&self, s: char) -> char {
        *self.t.dueum.get(&s).unwrap_or(&s)
    }

    /// 읽기 = 조각 나누기 + 후처리
    fn read(&self, span: &[char], word_start: bool, gloss: bool) -> Option<String> {
        let mut pieces = self.segment(span, word_start)?;
        if !gloss {
            for p in pieces.iter_mut().filter(|p| p.src.is_none()) {
                p.text = split_gloss(&p.text).0;
            }
        }
        Some(self.finish(&pieces, word_start))
    }

    /// 연쇄를 조각으로: 예외표 단어 조각 / 글자 조각 (두음까지 적용)
    fn segment(&self, span: &[char], word_start: bool) -> Option<Vec<Piece>> {
        let t = &self.t;
        let n = span.len();
        let mut out = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            // 1. 예외표 최장 일치
            let mut hit = None;
            for l in (1..=t.max_word.min(n - i)).rev() {
                if let Some(v) = t.words.get(&span[i..i + l]) {
                    hit = Some((l, v));
                    break;
                }
            }
            if let Some((l, v)) = hit {
                let mut text: Vec<char> = v.chars().collect();
                if i == 0 && word_start {
                    if let Some(f) = text.first_mut() {
                        *f = self.du(*f);
                    }
                }
                out.push(Piece { text, src: None, dueum: false });
                i += l;
                continue;
            }
            // 2. 글자별 대표 독음
            let c = span[i];
            let mut r = *t.prim.get(&c)?;
            let mut dueum = false;
            if i == 0 && word_start {
                dueum = true;
            } else if i >= 1 && !t.dwords.is_empty() && !t.dblock.contains(&[span[i - 1], c]) {
                // 합성어 안 두음: 여기서 두음 단어가 시작하는가
                dueum = (2..=t.dmax.min(n - i)).rev().any(|l| t.dwords.contains(&span[i..i + l]));
            }
            if dueum {
                r = self.du(r);
            }
            out.push(Piece { text: vec![r], src: Some(c), dueum });
            i += 1;
        }
        Some(out)
    }

    /// 후처리: 글자 조각에만 不·렬/률 규칙
    fn finish(&self, pieces: &[Piece], word_start: bool) -> String {
        // 음절 단위로 펼친다: (음절, 원 한자 — 글자 조각일 때만)
        let mut syl: Vec<(char, Option<char>)> = Vec::new();
        for p in pieces {
            match p.src {
                Some(h) => syl.push((p.text[0], Some(h))),
                None => syl.extend(p.text.iter().map(|&c| (c, None))),
            }
        }
        let mut res: Vec<char> = syl.iter().map(|x| x.0).collect();
        for k in 0..syl.len() {
            let (s, Some(h)) = syl[k] else { continue };
            if h == '不' {
                let next_is_dj = res.get(k + 1).and_then(|&c| onset(c)).is_some_and(|o| o == ONSET_D || o == ONSET_J);
                res[k] = if next_is_dj { '부' } else { '불' };
                if k == 0 && word_start {
                    res[k] = self.du(res[k]);
                }
            }
            if k > 0 && (s == '렬' || s == '률') && matches!(coda(res[k - 1]), Some(0) | Some(4)) {
                res[k] = if s == '렬' { '열' } else { '율' };
            }
        }
        res.into_iter().collect()
    }
}

/// 조각: 예외표 단어(`src = None`, 여러 음절) 또는 글자 하나(`src = Some(한자)`)
#[derive(Debug, Clone)]
struct Piece {
    text: Vec<char>,
    src: Option<char>,
    /// 글자 조각에 두음을 적용했는가 (다독음 후보에도 같게 적용)
    dueum: bool,
}

/// 손 표의 병기: `거란(글안)` → (`거란`, Some(`글안`)), `차례/다례` → (`차례`, Some(`다례`))
fn split_gloss(v: &[char]) -> (Vec<char>, Option<Vec<char>>) {
    match v.iter().position(|&c| c == '(' || c == '/') {
        Some(p) => {
            let g: Vec<char> = v[p + 1..].iter().copied().filter(|&c| c != ')').collect();
            (v[..p].to_vec(), if g.is_empty() { None } else { Some(g) })
        }
        None => (v.to_vec(), None),
    }
}

/// 한자인가: CJK 통합 한자(기본·확장 A~H), 호환 한자 (호환 한자 보충 U+2F800~ 은 확장 범위 안에 든다)
pub fn is_hanja(c: char) -> bool {
    matches!(c as u32,
        0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF | 0x20000..=0x3134F)
}

/// 글에 한자가 하나라도 있는가
pub fn contains_hanja(s: &str) -> bool {
    s.chars().any(is_hanja)
}

fn is_syllable(c: char) -> bool {
    ('\u{AC00}'..='\u{D7A3}').contains(&c)
}

const ONSET_D: u32 = 3; // ㄷ
const ONSET_J: u32 = 12; // ㅈ

/// 초성 번호 (ㄱ=0 … ㅎ=18)
fn onset(c: char) -> Option<u32> {
    is_syllable(c).then(|| (c as u32 - 0xAC00) / 588)
}

/// 받침 번호 (없음=0, ㄴ=4 …)
fn coda(c: char) -> Option<u32> {
    is_syllable(c).then(|| (c as u32 - 0xAC00) % 28)
}
