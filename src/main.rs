use hanja_hangul as h2h;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
    h2h::chi2kor_file("sample.txt", "sample_hangul.txt", &char_dic, &dueum_dic, &word_dic).await?;
    Ok(())
}