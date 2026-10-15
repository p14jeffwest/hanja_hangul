//! 파이썬 기준 구현(tools/demo.py)과의 대조용: 표준 입력 한 줄마다 convert 결과를 한 줄씩.
//! `cargo run --release --example parity < keys.txt > rust.txt`
use std::io::{self, BufRead, Write};

fn main() {
    let conv = hanja_hangul::Converter::global();
    let stdin = io::stdin();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let t = std::time::Instant::now();
    let mut chars = 0usize;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        chars += line.chars().count();
        writeln!(out, "{}", conv.convert(&line)).unwrap();
    }
    out.flush().unwrap();
    eprintln!("{chars} chars in {:?}", t.elapsed());
}
