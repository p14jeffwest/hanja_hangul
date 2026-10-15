//! 고정 사례. 규칙이나 자료를 바꿔도 이것들은 그대로여야 한다.
use hanja_hangul::{contains_hanja, is_hanja, Converter, Options, Style};
use std::borrow::Cow;

fn conv(s: &str) -> String {
    Converter::global().convert(s).into_owned()
}

#[test]
fn irregular_words() {
    // 손 예외표·사전 ▽ (음이 변한 한자)
    assert_eq!(conv("客車, 六月, 庫間, 女子, 十月, 困難, 木瓜"), "객차, 유월, 곳간, 여자, 시월, 곤란, 모과");
    assert_eq!(conv("差備 布施 喝食 喝火 魋結 椎結"), "채비 보시 할식 할화 추결 추결");
}

#[test]
fn polyphonic_by_dictionary() {
    assert_eq!(conv("樂園 音樂 快樂"), "낙원 음악 쾌락");
    assert_eq!(conv("便所 遊說 北京 行列"), "변소 유세 북경 행렬");
    // 독음이 둘인 표제어는 신문 괄호 병기에서 쓰인 쪽 (不實 부실; 불실 도 표제어)
    assert_eq!(conv("不實 雲龍"), "부실 운룡");
}

#[test]
fn dueum() {
    assert_eq!(conv("龍"), "용");
    assert_eq!(conv("勞動 歷史 來日"), "노동 역사 내일");
    // 합성어 안 두음과 그 막기
    assert_eq!(conv("新女性 經濟理論"), "신여성 경제이론");
    assert_eq!(conv("美術評論家 管理體制"), "미술평론가 관리체제");
    // 숫자·한글 바로 뒤는 첫머리가 아니다
    assert_eq!(conv("1995年度 色깔論"), "1995년도 색깔론");
}

#[test]
fn ryul_and_bu() {
    assert_eq!(conv("分裂 比率 先烈 自律 法律"), "분열 비율 선열 자율 법률");
    assert_eq!(conv("不當 不正 不法 不信"), "부당 부정 불법 불신");
}

#[test]
fn tea() {
    assert_eq!(conv("茶 한 잔, 綠茶, 紅茶, 茶房, 茶道"), "차 한 잔, 녹차, 홍차, 다방, 다도");
}

#[test]
fn surnames() {
    assert_eq!(conv("金泳三 金大中 金正日 沈大平 金浦空港"), "김영삼 김대중 김정일 심대평 김포공항");
    assert_eq!(conv("金대통령 金총리 沈淸傳"), "김대통령 김총리 심청전");
    assert_eq!(conv("金額 金融市場 金曜日 沈默 葉書 契約 單獨"), "금액 금융시장 금요일 침묵 엽서 계약 단독");
    // 이름 첫 음절 두음, 예외표 우선, 사전 3자어·2자어 막기
    assert_eq!(conv("李龍俊 金令培 權魯甲 韓利憲 金龍洙 金龍煥 金鍾泌"), "이용준 김영배 권노갑 한이헌 김용수 김용환 김종필");
    assert_eq!(conv("全羅道 元老院 權力化 文理大"), "전라도 원로원 권력화 문리대");
    let off = Converter::new().with_options(Options { surname: false, ..Options::default() });
    assert_eq!(off.convert("金泳三"), "금영삼");
}

#[test]
fn gloss() {
    // 손 표의 괄호 병기: convert 는 그대로, reading 은 괄호 앞, readings 는 둘 다
    let c = Converter::global();
    assert_eq!(c.convert("契丹"), "거란(글안)");
    assert_eq!(c.reading("契丹", true).as_deref(), Some("거란"));
    assert_eq!(c.readings("契丹", true, 4), vec!["거란".to_string(), "글안".to_string()]);
}

#[test]
fn readings_contract() {
    // [0] 은 언제나 reading 과 같다 (madi 가 이 순서에 기댄다)
    let c = Converter::global();
    for w in ["樂園", "金額", "音樂", "北京", "行列", "女子", "不當", "車庫"] {
        let r = c.readings(w, true, 16);
        assert_eq!(Some(&r[0]), c.reading(w, true).as_ref(), "{w}");
    }
    assert!(c.readings("樂", true, 16).len() > 1);
}

#[test]
fn styles_and_passthrough() {
    let c = Converter::new().with_options(Options { style: Style::HangulHanja, surname: true });
    assert_eq!(c.convert("經濟 발전"), "경제(經濟) 발전");
    let c = Converter::new().with_options(Options { style: Style::HanjaHangul, surname: true });
    assert_eq!(c.convert("經濟 발전"), "經濟(경제) 발전");
    assert!(matches!(Converter::global().convert("한자 없음"), Cow::Borrowed(_)));
}

#[test]
fn compat_and_extension_b() {
    // 호환 한자(U+F914 樂)도 통합 한자로 접어 읽는다
    assert_eq!(conv("\u{F914}園"), "낙원");
    // 확장 B 글자(사전 정렬로 넣은 것)
    assert_eq!(Converter::global().char_reading('𩿤'), Some('찰'));
    assert!(is_hanja('𩿤') && contains_hanja("a漢b") && !contains_hanja("한글"));
}

#[test]
fn unknown_char_kept() {
    // 글자표에 없는 한자는 그대로 두고 앞뒤만 읽는다 (U+30000 은 확장 G 의 첫 글자)
    assert_eq!(conv("經\u{30000}濟"), "경\u{30000}제");
}

#[test]
fn sample_news() {
    let s = "「宋寅壽기자」14일 농림해양수산위의 농협 국감에선 \n客車, 六月, 庫間, 女子\n金泳鎭(국민회의) 韓灝鮮(자민련) 權五乙의원(민주당)은";
    assert_eq!(conv(s), "「송인수기자」14일 농림해양수산위의 농협 국감에선 \n객차, 유월, 곳간, 여자\n김영진(국민회의) 한호선(자민련) 권오을의원(민주당)은");
}
