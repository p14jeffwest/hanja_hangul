//! 독음 후보 보기: `cargo run --release --example peek -- 樂 樂園`
fn main() {
    let c = hanja_hangul::Converter::global();
    for w in std::env::args().skip(1) {
        println!("{w}: {:?}", c.readings(&w, true, 8));
    }
}
