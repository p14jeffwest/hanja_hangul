//! 속도 측정: `cargo run --release --example bench -- 파일`
fn main() {
    let path = std::env::args().nth(1).expect("파일");
    let text = std::fs::read_to_string(path).unwrap();
    let t0 = std::time::Instant::now();
    let conv = hanja_hangul::Converter::new();
    let init = t0.elapsed();
    let t1 = std::time::Instant::now();
    let out = conv.convert(&text);
    let dt = t1.elapsed();
    let n = text.chars().count();
    let h = text.chars().filter(|&c| hanja_hangul::is_hanja(c)).count();
    eprintln!("표 만들기 {init:?}; {n} 자 (한자 {h}) 변환 {dt:?} = {:.1} 백만 자/초; 출력 {} 바이트",
              n as f64 / dt.as_secs_f64() / 1e6, out.len());
}
