//! hanja2hangul — 한자 섞인 글을 한글로
//!
//! ```text
//! hanja2hangul [입력파일] [-o 출력파일] [--style hangul|hangul-hanja|hanja-hangul] [--no-surname]
//! ```
//! 입력파일이 없으면 표준 입력, 출력파일이 없으면 표준 출력.

use std::io::{self, Read, Write};
use std::process::ExitCode;

use hanja_hangul::{Converter, Options, Style};

const USAGE: &str = "사용법: hanja2hangul [입력파일] [-o 출력파일] [--style hangul|hangul-hanja|hanja-hangul] [--no-surname]";

fn main() -> ExitCode {
    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut opts = Options::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--output" => match args.next() {
                Some(p) => output = Some(p),
                None => return fail("-o 뒤에 출력 파일 이름이 필요합니다"),
            },
            "--style" => {
                opts.style = match args.next().as_deref() {
                    Some("hangul") => Style::Hangul,
                    Some("hangul-hanja") => Style::HangulHanja,
                    Some("hanja-hangul") => Style::HanjaHangul,
                    _ => return fail("--style 은 hangul, hangul-hanja, hanja-hangul 중 하나"),
                }
            }
            "--no-surname" => opts.surname = false,
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            _ if a.starts_with('-') => return fail(&format!("모르는 선택 사항: {a}")),
            _ => input = Some(a),
        }
    }

    let mut text = String::new();
    let read = match &input {
        Some(p) => std::fs::read_to_string(p).map(|s| text = s),
        None => io::stdin().read_to_string(&mut text).map(|_| ()),
    };
    if let Err(e) = read {
        return fail(&format!("입력을 읽지 못했습니다: {e}"));
    }

    let conv = Converter::new().with_options(opts);
    let out = conv.convert(&text);
    let written = match &output {
        Some(p) => std::fs::write(p, out.as_bytes()),
        None => io::stdout().lock().write_all(out.as_bytes()),
    };
    if let Err(e) = written {
        return fail(&format!("출력을 쓰지 못했습니다: {e}"));
    }
    ExitCode::SUCCESS
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("{msg}\n{USAGE}");
    ExitCode::FAILURE
}
