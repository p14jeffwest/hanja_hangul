//! # Hanja to Hangul
//! 
//! hanja_hangul is a library that converts Chinese characters to Korean characters.
//! 
//! The library provides the following functions:
//! 
//! * load_dictionary - Load dictionary files and return the dictionary component as a tuple containing three HashMaps.
//! * chi2kor_file - Convert Chinese characters in the input file to Korean characters and write the result to the output file.
//! * chi2kor_str - Convert Chinese characters in the input string to Korean characters.
//! 
//! The dictionary files should be exist in the same directory as the executable file.
//! The dictionary files should be named as follows:
//! * hanja_char.csv - A CSV file that contains the mapping of Chinese characters to Korean characters.
//! * dueum.csv - A CSV file that contains the mapping of Korean characters to Korean characters according to the dueum law.
//! * hanja_word.csv - A CSV file that contains the mapping of Chinese words to Korean words.
//! 
//! # Examples
//! 
//! ```
//! use hanja_hangul as h2h;
//! 
//! use std::collections::HashMap;
//! use std::error::Error;
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
//!     h2h::chi2kor_file("123.txt", "", &char_dic, &dueum_dic, &word_dic).await?;
//!     Ok(())
//! }
//! ```
//! 
//! if the output_file is not supplied, the function will create a new file attached with "(한글화)" to the input file name.
//! ```
//! h2h::chi2kor_file("test.txt", "test_converted.txt", &char_dic, &dueum_dic, &word_dic).await?;
//! ```
//!
//! # Note
//! The function is an asynchronous function that uses the tokio library.
//! 
//! To use this library, add the following to your Cargo.toml file:
//! 
//! ```
//! [dependencies]
//! hanja_hangul = "0.1.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//! 
//! or, you can use the following command to add tokio to your project:
//! 
//! ```
//! cargo add tokio --features full
//! ```
//! 
//! # Note
//! In here, I'll explain how to convert Chinese characters to Korean characters in Korean.
//! 
//! hanja_hangul는 한글로 변환하는 라이브러리입니다.
//! 
//! 한자로된 문자는 한글로 변환할 수 있습니다. 따라서, 각 한자 별로 한글로 어떻게 읽으면 될지를 매핑하는 사전을 만들 수 있습니다.
//! 이렇게 만든 사전이 hanja_char.csv 파일입니다. 여기에는 한자와 한글로 변환된 문자가 콤마로 구분되어 저장되어 있습니다.
//! 
//! 그런데, 이와 같이 각각의 한자를 한글로 변환했을 때, 맞지 않는 경우가 있습니다. 
//! 예를 들어 '女子'라는 한자는 '녀자'로 변환되겠지만 실제로는 '여자'로 읽어야 합니다. 
//! 이러한 것을 '두음법칙'이라고 하는데, '녀, 뇨, 뉴, 니'가 단어 첫머리에 올 때, '여, 요, 유, 이'로 발음을 바꾸는 것을 말합니다.
//! 이러한 두음법칙이 적용되는 첫 글자를 매핑한 것이 dueum.csv 파일입니다.
//! 
//! 두음법칙만으로 해결되지 않는 경우도 있습니다. 
//! 대표적인 예가 '동음이자'입니다. '동음이자'란 한자가 여러 개의 음을 가질 때, 그 중에서 어떤 음으로 읽어야 할지를 말합니다.
//! 예를 들어 '車'라는 한자는 '차'로 읽을 수도 있고 '거'로 읽을 수도 있습니다. 'hanja_char.csv' 파일에는 '거'로 변환되게 되어 있습니다.
//! 따라서, '客車'라는 한자 단어에 대해서 'hanja_char.csv'에 의한 규칙으로는 '객거'로 변환되지만, 실제로는 '객차'로 읽어야 합니다.
//! 이러한 경우를 해결하기 위해 'hanja_word.csv' 파일에, '동음이자'의 경우 원래의 규칙과 다르게 변환되어야 하는 단어를 매핑해 놓았습니다.
//! 
//! 또한, 'hanja_word.csv' 파일에는 다른 불규칙한 변환이 필요한 단어들도 매핑되어 있습니다.
//! 예를 들어, '庫間'이라는 단어는 '고간'으로 변환되지만, 실제로는 '곳간'이라고 읽어야 합니다. 
//! 이러한 불규칙한 변환이 일어나는 단어들을 'hanja_word.csv' 파일에 매핑해 놓았습니다.
//! 
//! 'hanja_word.csv' 파일에, 불규칙하게 변환되는 단어들을 가능한 많이 넣어두긴 했는데, 모든 경우를 다 매핑했다고 할 수는 없습니다.
//! 만약 변환 결과가 맞지 않는 경우, 'hanja_word.csv' 파일에 새로운 매핑을 추가하면 됩니다. 
//! 
//! 아래 코드는 'hanja_hangul' 라이브러리를 사용하여 'sample.txt' 파일에 있는 한자를 한글로 변환한 후, 'sample_hangul.txt' 파일에 저장하는 코드입니다.
//! 
//! ```
//! use hanja_hangul as h2h;
//! use hanja_hangul as h2h;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
//!     h2h::chi2kor_file("sample.txt", "sample_hangul.txt", &char_dic, &dueum_dic, &word_dic).await?;
//!     Ok(())
//! }
//! ```
//! 
//! 원래 sample.txt에는 아래와 같이 한자가 포함되어 있습니다.
//! 
//! ```
//! 「宋寅壽기자」14일 농림해양수산위의 농협 국감에선 올해 추곡수매와 농산물시장 
//! 개방에 따른 대책 등이 주요 쟁점으로 대두됐다. 
//! 
//! 客車, 六月, 庫間, 女子
//! 
//! 金泳鎭(국민회의) 韓灝鮮(자민련) 權五乙의원(민주당)은 ......
//! ```
//! 
//! 이것을 chi2kor_file 함수를 사용하여 한글로 변환하면 아래와 같이 변환됩니다.
//! 
//! ```
//! 「송인수기자」14일 농림해양수산위의 농협 국감에선 올해 추곡수매와 농산물시장
//! 개방에 따른 대책 등이 주요 쟁점으로 대두됐다. 
//! 
//! 객차, 유월, 곳간, 여자
//! 
//! 김영진(국민회의) 한호선(자민련) 권오을의원(민주당)은 ......
//! ```
//! 
//! 일반 한자에 대한 한글 변환 및 두음법칙, 동음이자, 사이시옷 규칙에 의한 불규칙한 한글변환도 잘 되고 있음을 알 수 있습니다. 
//! 
//! # Note
//! 한자를 한글로 변환할 때 사용된 방법을 소개하겠습니다.
//! 
//! 각 문자에는 유니코드가 할당되어 있습니다. 한자에도 코드가 부여되어 있고, 한국, 중국, 일본에서 한자를 사용하다 보니, 굉장히 많은 코드가 한자에 부여되어 있습니다.
//! 이 라이브러리에서 사용하는 코드는 27,848개의 한자코드입니다. 모두 엑셀에서 한글로 변환 가능한 코드입니다.
//! 
//! * 한중일 통합 한자: 19968 ~ 40869 (20902개)
//! * 한중일 통합 한자 확장 A: 13312 ~ 19893 (6582개)
//! * 한중일 호환 한자1: 63744 ~ 64045 (302개)
//! * 한중일 호환 한자2: 64048 ~ 64109 (62개)
//! 
//! 이 코드를 이용하여 한자를 한글로 변환하는 것이 이 라이브러리의 핵심입니다.
//! 이 코드는 한자에 대한 유니코드에 대한 설명이 나와 있는 다른 자료와 코드 범위와 개수가 좀 다를 수 있습니다. 
//! 이 범위는 실제 엑셀에서 변환 가능한 범위를 기준으로 하였습니다. 기초 변환 자료를 엑셀로 만들어서 변환하다 보니, 이렇게 된 것입니다.
//! 
//! 1. 기본 변환
//! 
//! 기본 변환은, 위에서 설명한 27,848개의 한자코드에 대해 한글로 변환하는 것입니다. 
//! 
//! 변환 하려는 문장에서 문자 하나하나를 읽어서, 해당 문자가 한자이면, 한글로 변환합니다.
//! 
//! 이때 이용되는 것이 'hanja_char.csv' 파일입니다. 이 파일에는 한자와 한글로 변환된 문자가 콤마로 구분되어 저장되어 있습니다.
//! 
//! 2. 두음법칙
//! 
//! 두음법칙은 '녀, 뇨, 뉴, 니'가 단어 첫머리에 올 때, '여, 요, 유, 이'로 발음을 바꾸는 것을 말합니다.
//! 
//! 예를 들어, '녀자'는 '여자'로 읽어야 합니다.
//! 
//! 한자 단어를 한글로 변환 했을 때, 위 두음법칙에 해당하는 경우는 단어의 첫 글자를 두음법칙에 맞게 변환해야합니다. 
//! 
//! 여기서, 알고리즘을 짜는 입장에서는 고민을 하게됩니다.  
//! 각 문자를 음소로 분리해서 두음법칙에 해당하는 지를 판단하게 할지, 아니면 두음법칙에 해당하는 모든 문자를 미리 매핑해 놓고 변환하게 할지 고민이 됩니다.
//! 
//! 필자는 두 번째 방법을 선택했습니다. 두음법칙에 해당하는 문자가 모두 해봐야 52개이기 때문입니다.
//! 이 52개 문자가 들어 있는 것이 dueum.csv 파일입니다.
//! 
//! 알고리즘에서는, 한자 단어를 한글로 변환하고 나서, 두음법칙에 해당하는 문자가 단어의 첫 글자에 있으면, 그 문자를 두음법칙에 맞게 변환합니다.
//! 
//! 3. 불규칙 변환
//! 
//! 두음법칙에 의한 불규칙 말고도, 기본 변환에 의해 해결이 안되는 불규칙 변환들이 있습니다. 
//! 
//! '동음이자'에 의한 불규칙 변환, 사이시옷 규칙에 의한 불규칙 변환 등이 있는데, 이러한 불규칙 변환들은 'hanja_word.csv' 파일에 매핑되어 있습니다.
//! 
//! 알고리즘에서는, 한자 단어가 있고, 이 한자 단어가 'hanja_word.csv' 파일에 있으면, 해당 단어를 매핑된 한글로 변환합니다.
//! 
//! 실제 알고리즘에서는 이 불규칙변환이 제일 먼저 일어나고, 그 다음에 '기본변환' 그리고 그 다음에 '두음법칙' 변환이 일어납니다.
//! 
//! 실제 코드는 'chi2kor_str' 함수에서 확인할 수 있습니다. 이 함수가 실제 변환을 수행하는 핵심 함수입니다.
//! 

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

const KO_START:u32 = 44032;
const KO_END:u32 = 55203;

const CHI_S1:u32 = 13312;
const CHI_E1:u32 = 19903;

const CHI_S2:u32 = 19968;
const CHI_E2:u32 = 40959;

const CHI_S3:u32 = 63744;
const CHI_E3:u32 = 64045;

const CHI_S4:u32 = 64048;
const CHI_E4:u32 = 64109;

/// Load dictionary files and return the dictionary component as a tuple containing three HashMaps.
/// The tuple contains the following HashMaps: (hanja_dic, dueum_dic, word_dic)
/// 
/// hanja_dic: HashMap<char, char> - A dictionary that maps Chinese characters to Korean characters.
/// dueum_dic: HashMap<char, char> - A dictionary that maps Korean characters to Korean characters according to the dueum law.  
/// word_dic: HashMap<String, String> - A dictionary that maps Chinese words to Korean words especially for words that beyond the regular conversion rules.
///
/// The HashMaps should be sent as arguments to the chi2kor function.
/// 
/// The dictionary files should be exist in the same directory as the executable file.
/// The dictionary files should be named as follows:
///   * hanja_char.csv - A CSV file that contains the mapping of Chinese characters to Korean characters.
///   * dueum.csv - A CSV file that contains the mapping of Korean characters to Korean characters according to the dueum law.
///   * hanja_word.csv - A CSV file that contains the mapping of Chinese words to Korean words.
///  
/// # Errors
/// If the function fails to open the dictionary files, it will return an error.
/// 
/// # Examples
/// ```
/// use hanja_hangul as h2h;
/// use std::collections::HashMap;
/// use std::error::Error;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> { 
///   let (hanja_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
///   h2h::chi2kor_file("123.txt", "", &hanja_dic, &dueum_dic, &dongum_dic).await?;
///   Ok(())
/// }
/// ```
/// 
pub fn load_dictionary() 
        -> Result<(HashMap<char, char>, HashMap<char, char>, HashMap<String, String>), Box<dyn Error>> {
    let char_dic = generate_dic_char("hanja_char.csv").map_err(|_| "open hanja_char.csv file was failed.")?;
    let dueum_dic = generate_dic_char("dueum.csv").map_err(|_| "open dueum.csv file was failed.")?;
    let word_dic = generate_dic_str("hanja_word.csv").map_err(|_| "open hanja_word.csv file was failed.")?;
    Ok((char_dic, dueum_dic, word_dic))
}


/// Convert Chinese characters in the input file to Korean characters and write the result to the output file.
/// If the output file is not supplied, the function will create a new file attached with "(한글화)" to the input file name.
/// 
/// # Arguments
/// * `input_file` - The path of the input file that contains Chinese characters.
/// * `output_file` - The path of the output file that will contain the converted Korean characters. 
///                   If the output file is not supplied, as the blank string, the function will create a new file attached with "(한글화)" to the input file name.
/// * `char_dic` - A dictionary that maps Chinese characters to Korean characters.
/// * `dueum_dic` - A dictionary that maps Korean characters to Korean characters according to the dueum law.
/// * `word_dic` - A dictionary that maps Chinese words to Korean words especially for words that beyond the regular conversion rules.
/// 
/// # Errors
/// If the function fails to open the input file or create the output file, it will return an error.
/// 
/// # Examples
/// ```
/// use hanja_hangul as h2h;
/// use std::collections::HashMap;
/// use std::error::Error;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
///     h2h::chi2kor_file("test.txt", "test_converted.txt", &char_dic, &dueum_dic, &word_dic).await?;
///     Ok(())
/// }
/// ```
/// 
/// if the output_file is not supplied, the function will create a new file attached with "(한글화)" to the input file name.
/// ```
/// h2h::chi2kor_file("test.txt", "test_converted.txt", &char_dic, &dueum_dic, &word_dic).await?;
/// ```
/// 
/// # Note
/// The function is an asynchronous function that uses the tokio library.
/// 
/// # Note
/// Prior to using this function, the load_dictionary function should be called to load the dictionary files.
/// 
pub async fn chi2kor_file(
        input_file:&str,
        output_file:&str, 
        char_dic:&HashMap<char,char>, 
        dueum_dic:&HashMap<char,char>,
        word_dic:&HashMap<String, String>) -> Result<(),Box<dyn Error>>{
    //1. check whether output_file is supplied or not
    let is_output_file = if output_file.len() == 0 {false} else {true};

    //2. read input_file and generate string with the contents   
    let f = tokio::fs::File::open(input_file).await?;  
    let mut rdr = tokio::io::BufReader::new(f);
    let mut buf = String::new();    
    rdr.read_to_string(&mut buf).await?;   

    //3.convert to hangul.
    //  if there is no chinese character in the string, chi2kor_str function will return None.
    //  Therefore, it is None, create new file with the same contents.
    //  if exist, create new file with the converted string.
    let new_buf = match chi2kor_str(&buf, &char_dic, &dueum_dic, &word_dic) {
        Some(val) => {val},
        None => {buf},
    };
    
    //4. write to file.
    //  if there is no output_file, create new file using get_new_file_path function.
    //  if there is output_file, write new_buf to the file.
    let tgt_file_name = if !is_output_file {get_new_file_path(input_file)} else {output_file.to_string()};
    let new_file = tokio::fs::File::create(&tgt_file_name).await?;
    let mut writer = tokio::io::BufWriter::new(new_file);
    let _ = writer.write_all(&new_buf.as_bytes()).await;
    writer.flush().await?;

    Ok(())
}

/// Convert Chinese characters in the input string to Korean characters.
/// If there are no Chinese characters in the input string, the function will return None.
/// 
/// # Arguments
/// * `input_str` - A string that contains Chinese characters.  
/// * `char_dic` - A dictionary that maps Chinese characters to Korean characters.
/// * `dueum_dic` - A dictionary that maps Korean characters to Korean characters according to the dueum law.
/// * `word_dic` - A dictionary that maps Chinese words to Korean words especially for words that beyond the regular conversion rules.
///
/// # Returns
/// If there are no Chinese characters in the input string, the function will return None.
/// If there are Chinese characters in the input string, the function will return the converted string.
/// 
/// # Examples
/// ```
/// use hanja_hangul as h2h;
/// use std::collections::HashMap;
/// 
/// let char_dic: HashMap<char, char> = HashMap::new();
/// let dueum_dic: HashMap<char, char> = HashMap::new();
/// let word_dic: HashMap<String, String> = HashMap::new();
/// let input_str = "你好，我是韩国人。";
/// 
/// let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
/// let result = h2h::chi2kor_str(input_str, &char_dic, &dueum_dic, &word_dic);
/// println!("{:?}", result);
/// ```
/// 
pub fn chi2kor_str(
        input_str:&str,         
        char_dic:&HashMap<char,char>, 
        dueum_dic:&HashMap<char,char>,
        word_dic:&HashMap<String, String>) -> Option<String>{
    
        //1. obtain char array from input_str
        let mut c_iter = input_str.chars().peekable();           

        // 2. convert to hangul 
        let mut w_buf:String = String::new(); 
        let mut is_exist_chi:bool = false;     
        loop {    
            //2.1 pick a word only contains chinese character
            let mut word_buf:String = String::new();   
            let mut temp_iter = c_iter.clone();
            while let Some(c) = temp_iter.peek() {
                if is_chi(&c) {word_buf.push(*c); temp_iter.next();}
                else {break;}
            }

            //2.2 if word_buf is not empty, check whether it is in the word_dic or not.
            //    if exist, append the value to w_buf and continue.
            //    if not, revert the c_iter and continue.
            if word_buf.len() > 0 {
                match word_dic.get(&word_buf) {
                    Some(val) => {
                        w_buf.push_str(val); 
                        is_exist_chi = true; 
                        c_iter = temp_iter; // Move the main iterator forward
                        continue;
                    },
                    None => {}
                }
            }
            
            //2.3 pick a char. if c is None, it's end of file
            let c = match c_iter.next() { 
                Some(ch) => {ch},  None => {break;} 
            };
            let mut new_c = c.clone();
    
            //2.4 if hanja then convert to hangul else not change       
            if is_chi(&c) {         
                match char_dic.get(&c) {
                    Some(val) => {new_c = *val; is_exist_chi = true; },  
                    None => {},
                };          
    
                //2.5. dueum law(두음법칙)
                if let Some(c_peek) = c_iter.peek(){                
                    if is_kor_or_chi(&c_peek) { // if next char is exist
                        match dueum_dic.get(&new_c) {
                            Some(ch) => {new_c = *ch;},
                            None => {},
                        }
                    }                     
                }          
            }
            w_buf.push(new_c);
        }
        
        //3. if there is no chinese character in the string, return None.
        //   if exist, return the converted string.
        if !is_exist_chi {return None;}
        Some(w_buf)        
}    


/// Convert the contents of the input file to a HashMap that contains "character, character" format data.
///
/// # Arguments
/// * `file_path` - The path of the input file that contains key-value data separated by a comma.
/// 
/// # Returns
/// A HashMap that contains the mapping of key-value character data read from the input file.
/// 
/// # Errors
/// If the function fails to open the input file, it will return an error.
/// 
fn generate_dic_char(file_path:&str) -> Result<HashMap<char, char>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut dic: HashMap<char, char> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let key_char = parts[0].trim().chars().next().unwrap();
            let val_char = parts[1].trim().chars().next().unwrap();
            dic.insert(key_char, val_char);
        }
    }
    Ok(dic)
}


/// Convert the contents of the input file to a HashMap that contains "string, string" format data.
/// 
/// # Arguments
/// * `file_path` - The path of the input file that contains key-value data separated by a comma.
/// 
/// # Returns
/// A HashMap that contains the mapping of key-value string data read from the input file.
/// 
/// # Errors
/// If the function fails to open the input file, it will return an error.
/// 
fn generate_dic_str(file_path:&str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut dic: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let key_str = parts[0].trim().to_string();
            let val_str = parts[1].trim().to_string();
            dic.insert(key_str, val_str);
        }
    }
    Ok(dic)
}


// whether c is (korean or chinese character) or not
fn is_kor_or_chi(c:&char) -> bool {
    let n = *c as u32;
    if (n >= KO_START && n <= KO_END) || 
       ( (n >= CHI_S1 && n <= CHI_E1) || (n >= CHI_S2 && n <= CHI_E2) || 
         (n >= CHI_S3 && n <= CHI_E3) || (n >= CHI_S4 && n <= CHI_E4)) {
        true
    }else {
        false
    }    
}


// whether c is chinese character or not
fn is_chi(c:&char) -> bool {
    let n = *c as u32;
    if  (n >= CHI_S1 && n <= CHI_E1) || (n >= CHI_S2 && n <= CHI_E2) || 
        (n >= CHI_S3 && n <= CHI_E3) || (n >= CHI_S4 && n <= CHI_E4) { 
        true 
    }else { false }  
}


// hello.txt -> hello(한글화).txt
fn get_new_file_path(file_path:&str) -> String{
    let pos_point = file_path.len() - 4; // ".txt" 형태라야함
    "".to_owned() + &file_path[0..pos_point] + "(한글화).txt"
}
