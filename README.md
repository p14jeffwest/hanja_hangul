# Hanja to Hangul

`hanja_hangul` is a library that converts Chinese characters to Korean characters.

## Features

The library provides the following functions:

- `load_dictionary` - Load dictionary from the hard-coded data and return the dictionary component as a tuple containing three HashMaps.
- `load_dictionary_from_file` - Load dictionary from the files and return the dictionary component as a tuple containing three HashMaps.
- `chi2kor_file` - Convert Chinese characters in the input file to Korean characters and write the result to the output file.
- `chi2kor_str` - Convert Chinese characters in the input string to Korean characters.

## Dictionary Files

The dictionary files are required for the `load_dictionary_from_file` function and should be placed in the same directory as the executable file. These files can be downloaded from the following URLs:

- `hanja_char.csv` - Contains the mapping of Chinese characters to Korean characters. [Download](https://github.com/p14jeffwest/hanja_hangul/blob/main/hanja_char.csv)
- `dueum.csv` - Contains the mapping of Korean characters to Korean characters according to the dueum law. [Download](https://github.com/p14jeffwest/hanja_hangul/blob/main/dueum.csv)
- `hanja_word.csv` - Contains the mapping of Chinese words to Korean words, especially for words that require irregular conversion. [Download](https://github.com/p14jeffwest/hanja_hangul/blob/main/hanja_word.csv)

If you want to use your custom dictionary files, you can modify these CSV files by adding new mappings. For example:

- Add new Chinese-to-Korean character mappings in `hanja_char.csv`.
- Add new dueum law mappings in `dueum.csv`.
- Add new irregular word mappings in `hanja_word.csv`.

These files are loaded using the `load_dictionary_from_file` function, which expects the paths to these files as arguments.

## Examples

### Example 1: Using the Hardcoded Dictionary

```rust
use hanja_hangul as h2h;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the hardcoded dictionary
    let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary()?;
    
    // Convert Chinese characters in "sample.txt" to Korean and save to "sample_hangul1.txt"
    h2h::chi2kor_file("sample.txt", "sample_hangul1.txt", &char_dic, &dueum_dic, &word_dic).await?;
    Ok(())
}
```

### Example 2: Using Custom Dictionary Files

```rust
use hanja_hangul as h2h;
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the dictionary from custom CSV files
    let (char_dic, dueum_dic, word_dic) = h2h::load_dictionary_from_file(
        Path::new("hanja_char.csv"),
        Path::new("dueum.csv"),
        Path::new("hanja_word.csv")
    )?;
    
    // Convert Chinese characters in "sample.txt" to Korean and save to "sample_hangul2.txt"
    h2h::chi2kor_file("sample.txt", "sample_hangul2.txt", &char_dic, &dueum_dic, &word_dic).await?;
    Ok(())
}
```

### Example 3: Handling Missing Output File

If the `output_file` is not supplied, the function will create a new file with "(한글화)" appended to the input file name:

```rust
h2h::chi2kor_file("test.txt", "", &char_dic, &dueum_dic, &word_dic).await?;
```

## Note

The function is an asynchronous function that uses the `tokio` library.

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
hanja_hangul = "0.1.3"
tokio = { version = "1", features = ["full"] }
```

Or, you can use the following command to add `tokio` to your project:

```sh
cargo add tokio --features full
```

## Korean Explanation

`hanja_hangul`는 한글로 변환하는 라이브러리입니다.

한자로된 문자는 한글로 변환할 수 있습니다. 따라서, 각 한자 별로 한글로 어떻게 읽으면 될지를 매핑하는 사전을 만들 수 있습니다. 

그런데, 이와 같이 각각의 한자를 한글로 변환했을 때, 맞지 않는 경우가 있습니다. 예를 들어 '女子'라는 한자는 '녀자'로 변환되겠지만 실제로는 '여자'로 읽어야 합니다. 이러한 것을 '두음법칙'이라고 하는데, '녀, 뇨, 뉴, 니'가 단어 첫머리에 올 때, '여, 요, 유, 이'로 발음을 바꾸는 것을 말합니다. 

두음법칙만으로 해결되지 않는 경우도 있습니다. 대표적인 예가 '동음이자'입니다. '동음이자'란 한자가 여러 개의 음을 가질 때, 그 중에서 어떤 음으로 읽어야 할지를 말합니다. 예를 들어 '車'라는 한자는 '차'로 읽을 수도 있고 '거'로 읽을 수도 있습니다. 기본 변환 사전에는 '거'로 변환되게 되어 있습니다. 따라서, '客車'라는 한자 단어에 대해서 '객거'로 변환되지만, 실제로는 '객차'로 읽어야 합니다. 이러한 경우를 해결하기 위해 불규칙 변환 파일에, '동음이자'의 경우 원래의 규칙과 다르게 변환되어야 하는 단어를 매핑해 놓았습니다.

또한, 다른 불규칙한 변환이 필요한 단어들도 매핑되어 있습니다. 예를 들어, '庫間'이라는 단어는 '고간'으로 변환되지만, 실제로는 '곳간'이라고 읽어야 합니다. 이러한 불규칙한 변환이 일어나는 단어들을 모두 매핑해 놓았습니다.

이렇게 미리 만들어 놓은 사전은 load_dictionary() 함수를 호출하면 로드됩니다.

그런데 만약, 이 라이브러리에 내장된 사전을 이용하지 않고, 자신이 만든 사전을 이용하고 싶다면,
load_dictionary_from_file() 함수를 사용하면 됩니다.

이 함수는 'hanja_char.csv', 'dueum.csv', 'hanja_word.csv' 파일을 읽어서 사전을 생성합니다.
이 3개의 파일에 대해서 아래의 github 주소에서 다운로드 받을 수 있습니다.
'hanja_word.csv' 파일에다 자신이 원하는 한자와 한글 변환을 추가하면 됩니다.

https://github.com/p14jeffwest/hanja_hangul


## Example Code

아래 코드는 `hanja_hangul` 라이브러리를 사용하여 `sample.txt` 파일에 있는 한자를 한글로 변환한 후, `sample_hangul.txt` 파일에 저장하는 코드입니다.

```rust
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
```

원래 `sample.txt`에는 아래와 같이 한자가 포함되어 있습니다.

```
「宋寅壽기자」14일 농림해양수산위의 농협 국감에선 올해 추곡수매와 농산물시장 
개방에 따른 대책 등이 주요 쟁점으로 대두됐다. 

客車, 六月, 庫間, 女子

金泳鎭(국민회의) 韓灝鮮(자민련) 權五乙의원(민주당)은 ......
```

이것을 `chi2kor_file` 함수를 사용하여 한글로 변환하면 아래와 같이 변환됩니다.

```
「송인수기자」14일 농림해양수산위의 농협 국감에선 올해 추곡수매와 농산물시장
개방에 따른 대책 등이 주요 쟁점으로 대두됐다. 

객차, 유월, 곳간, 여자

김영진(국민회의) 한호선(자민련) 권오을의원(민주당)은 ......
```

일반 한자에 대한 한글 변환 및 두음법칙, 동음이자, 사이시옷 규칙에 의한 불규칙한 한글변환도 잘 되고 있음을 알 수 있습니다.

## Conversion Method

한자를 한글로 변환할 때 사용된 방법을 소개하겠습니다.

각 문자에는 유니코드가 할당되어 있습니다. 한자에도 코드가 부여되어 있고, 한국, 중국, 일본에서 한자를 사용하다 보니, 굉장히 많은 코드가 한자에 부여되어 있습니다. 이 라이브러리에서 사용하는 코드는 27,848개의 한자코드입니다. 모두 엑셀에서 한글로 변환 가능한 코드입니다.

- 한중일 통합 한자: 19968 ~ 40869 (20902개)
- 한중일 통합 한자 확장 A: 13312 ~ 19893 (6582개)
- 한중일 호환 한자1: 63744 ~ 64045 (302개)
- 한중일 호환 한자2: 64048 ~ 64109 (62개)

이 코드를 이용하여 한자를 한글로 변환하는 것이 이 라이브러리의 핵심입니다. 이 코드는 한자에 대한 유니코드에 대한 설명이 나와 있는 다른 자료와 코드 범위와 개수가 좀 다를 수 있습니다. 이 범위는 실제 엑셀에서 변환 가능한 범위를 기준으로 하였습니다. 기초 변환 자료를 엑셀로 만들어서 변환하다 보니, 이렇게 된 것입니다.

### Basic Conversion

기본 변환은, 위에서 설명한 27,848개의 한자코드에 대해 한글로 변환하는 것입니다.

변환 하려는 문장에서 문자 하나하나를 읽어서, 해당 문자가 한자이면, 한글로 변환합니다.

이때 이용되는 것이 `hanja_char.csv` 파일입니다. 이 파일에는 한자와 한글로 변환된 문자가 콤마로 구분되어 저장되어 있습니다.

### Dueum Law

두음법칙은 '녀, 뇨, 뉴, 니'가 단어 첫머리에 올 때, '여, 요, 유, 이'로 발음을 바꾸는 것을 말합니다.

예를 들어, '녀자'는 '여자'로 읽어야 합니다.

한자 단어를 한글로 변환 했을 때, 위 두음법칙에 해당하는 경우는 단어의 첫 글자를 두음법칙에 맞게 변환해야합니다.

여기서, 알고리즘을 짜는 입장에서는 고민을 하게됩니다. 각 문자를 음소로 분리해서 두음법칙에 해당하는 지를 판단하게 할지, 아니면 두음법칙에 해당하는 모든 문자를 미리 매핑해 놓고 변환하게 할지 고민이 됩니다.

필자는 두 번째 방법을 선택했습니다. 두음법칙에 해당하는 문자가 모두 해봐야 52개이기 때문입니다. 이 52개 문자가 들어 있는 것이 `dueum.csv` 파일입니다.

알고리즘에서는, 한자 단어를 한글로 변환하고 나서, 두음법칙에 해당하는 문자가 단어의 첫 글자에 있으면, 그 문자를 두음법칙에 맞게 변환합니다.

### Irregular Conversion

두음법칙에 의한 불규칙 말고도, 기본 변환에 의해 해결이 안되는 불규칙 변환들이 있습니다.

'동음이자'에 의한 불규칙 변환, 사이시옷 규칙에 의한 불규칙 변환 등이 있는데, 이러한 불규칙 변환들은 `hanja_word.csv` 파일에 매핑되어 있습니다.

알고리즘에서는, 한자 단어가 있고, 이 한자 단어가 `hanja_word.csv` 파일에 있으면, 해당 단어를 매핑된 한글로 변환합니다.

실제 알고리즘에서는 이 불규칙변환이 제일 먼저 일어나고, 그 다음에 '기본변환' 그리고 그 다음에 '두음법칙' 변환이 일어납니다.

실제 코드는 `chi2kor_str` 함수에서 확인할 수 있습니다. 이 함수가 실제 변환을 수행하는 핵심 함수입니다.
