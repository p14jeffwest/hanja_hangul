use hanja_hangul as h2h;
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. test with load_dictionary which uses the hardcoded dictionary.
    let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
    h2h::chi2kor_file("sample.txt", "sample_hangul1.txt", &char_dic, &dueum_dic, &word_dic).await?;

    //2. test with load_dictionary_from_file which loads the dictionary from a file.
    let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary_from_file(
        Path::new("hanja_char.csv"),
        Path::new("dueum.csv"),
        Path::new("hanja_word.csv")
    )?;
    h2h::chi2kor_file("sample.txt", "sample_hangul2.txt", &char_dic, &dueum_dic, &word_dic).await?;
    Ok(())
}