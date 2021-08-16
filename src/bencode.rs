use core::fmt;
use itertools::Itertools;
use std::collections::HashMap;

pub enum BDecodedChunk {
    Dictionary(HashMap<String, Box<BDecodedChunk>>),
    Int(i64),
    List(Vec<Box<BDecodedChunk>>),
    Str(String),
    Null,
}

pub enum BencodeError {
    UnmatchedType,
    InvalidBencode,
    UnexpectedError,
}

fn string_from_hashmap<T>(inp: &HashMap<String, T>) -> String
where
    T: fmt::Display,
{
    let mut result = "".to_string();
    for (key, val) in inp {
        result.push_str(key);
        result.push_str(" : ");
        let val_str = &format!("{}", val)[..];
        result.push_str(val_str);
        result.push_str("\n    ");
    }

    //Hacky solution to remove last unnecessary newline at the end TODO: cleaner solution
    for _i in 0..5 {
        result.pop();
    }
    result
}

impl fmt::Display for BDecodedChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BDecodedChunk::Int(i) => write!(f, "{}", i),
            BDecodedChunk::Str(s) => write!(f, "{}", *s),
            BDecodedChunk::List(foo) => {
                write!(f, "<LIST>:\n    {}", foo.iter().format("\n    "))
            }

            BDecodedChunk::Dictionary(foo) => {
                write!(f, "<DICT>:\n    {}", string_from_hashmap(foo))
            }
            BDecodedChunk::Null => write!(f, "#!NULL!#"),
        }
    }
}

//TODO
impl fmt::Display for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "yeet")
    }
}

//TODO
impl fmt::Debug for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yeet")
    }
}

pub trait Bencode {
    fn bencode(&self) -> String;
}

pub trait BencodeDecodable<T>
where
    T: Bencode,
{
    fn decode_bencode(&self) -> Result<T, BencodeError>;
}

impl Bencode for i64 {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}

impl Bencode for Vec<Box<BDecodedChunk>> {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}

impl Bencode for String {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}

impl Bencode for Box<dyn Bencode> {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}
impl Bencode for HashMap<String, Box<BDecodedChunk>> {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}

impl Bencode for BDecodedChunk {
    fn bencode(&self) -> String {
        "encoding not yet implemented".to_string()
    }
}

impl BencodeDecodable<i64> for String {
    fn decode_bencode(&self) -> Result<i64, BencodeError> {
        let mut char_iter = self.chars().peekable();
        let first_char = *char_iter
            .peek()
            .expect("Couldn't read first character of bencoded int");
        if first_char != 'i' {
            return Err(BencodeError::UnmatchedType);
        }

        decode_int(&mut char_iter)
    }
}

impl BencodeDecodable<String> for String {
    fn decode_bencode(&self) -> Result<String, BencodeError> {
        let mut char_iter = self.chars().peekable();
        let first_char = *char_iter
            .peek()
            .expect("Error trying to decode empty string");
        if !first_char.is_digit(10) {
            return Err(BencodeError::UnmatchedType);
        }

        decode_string(&mut char_iter)
    }
}
impl BencodeDecodable<Vec<Box<BDecodedChunk>>> for String {
    fn decode_bencode(&self) -> Result<Vec<Box<BDecodedChunk>>, BencodeError> {
        let mut char_iter = self.chars().peekable();
        let first_char = *char_iter
            .peek()
            .expect("Error trying to read first char of list");

        if first_char != 'l' {
            return Err(BencodeError::UnmatchedType);
        }
        decode_list(&mut char_iter)
    }
}

impl BencodeDecodable<HashMap<String, Box<BDecodedChunk>>> for String {
    fn decode_bencode(&self) -> Result<HashMap<String, Box<BDecodedChunk>>, BencodeError> {
        let _first_char = self.chars().next().unwrap();

        let mut i: HashMap<String, Box<BDecodedChunk>> = HashMap::new();
        let test_string = "yeet".to_string();
        let j: i64 = 5;
        let k: String = "yeeqwatqawtr".to_string();

        i.insert(test_string, Box::new(BDecodedChunk::Int(j)));
        i.insert("aqwfra".to_string(), Box::new(BDecodedChunk::Str(k)));

        Ok(i)
    }
}

fn decode_dictionary(
    source: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<HashMap<String, Box<BDecodedChunk>>, BencodeError> {
    let mut result_dict: HashMap<String, Box<BDecodedChunk>> = HashMap::new();

    assert!(
        source.next().unwrap() == 'd',
        "attempt to decode dictionary, that doesn't start with \"d\""
    );

    let mut current_char = *source
        .peek()
        .expect("error reading next char, while parsing dictionary");

    while current_char != 'e' {
        assert!(
            current_char.is_digit(10),
            "found key in dictionary that isn't a string"
        );
        let key_read: String = decode_string(source).unwrap();
        current_char = *source.peek().expect("error reading dictionary value");

        let chunk_item = match current_char {
            'i' => {
                let int_read: i64 = decode_int(source).unwrap();
                BDecodedChunk::Int(int_read)
            }
            token if token.is_digit(10) => {
                let string_read: String = decode_string(source).unwrap();
                BDecodedChunk::Str(string_read)
            }
            'l' => {
                let vec_read: Vec<Box<BDecodedChunk>> = decode_list(source).unwrap();
                BDecodedChunk::List(vec_read)
            }
            'd' => {
                let dict_read: HashMap<String, Box<BDecodedChunk>> =
                    decode_dictionary(source).unwrap();
                BDecodedChunk::Dictionary(dict_read)
            }
            'e' => BDecodedChunk::Null,
            _ => BDecodedChunk::Null,
        };
        match chunk_item {
            BDecodedChunk::Null => return Err(BencodeError::UnexpectedError),
            _ => {
                result_dict.insert(key_read, Box::new(chunk_item));
            }
        }
        current_char = *source
            .peek()
            .expect("error reading next key-value pair in dict");
    }
    source.next();
    Ok(result_dict)
}

fn decode_list(
    source: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<Vec<Box<BDecodedChunk>>, BencodeError> {
    let mut result_vec: Vec<Box<BDecodedChunk>> = Vec::new();
    if source.next().unwrap() != 'l' {
        return Err(BencodeError::UnmatchedType);
    }
    let mut current_char = *source.peek().expect("error reading next char");

    while current_char != 'e' {
        match current_char {
            'i' => {
                let int_read: i64 = decode_int(source).unwrap();
                let chunk_item = BDecodedChunk::Int(int_read);
                result_vec.push(Box::new(chunk_item));
            }
            token if token.is_digit(10) => {
                let string_read: String = decode_string(source).unwrap();
                let chunk_item = BDecodedChunk::Str(string_read);
                result_vec.push(Box::new(chunk_item));
            }
            'l' => {
                let vec_read: Vec<Box<BDecodedChunk>> = decode_list(source).unwrap();
                let chunk_item = BDecodedChunk::List(vec_read);
                result_vec.push(Box::new(chunk_item));
            }
            'd' => {
                let dict_read: HashMap<String, Box<BDecodedChunk>> =
                    decode_dictionary(source).unwrap();
                let chunk_item = BDecodedChunk::Dictionary(dict_read);
                result_vec.push(Box::new(chunk_item));
            }
            'e' => {}
            _ => {
                return Err(BencodeError::UnexpectedError);
            }
        }
        current_char = *source.peek().expect("error reading next item in list");
    }
    Ok(result_vec)
}

fn decode_int(source: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<i64, BencodeError> {
    if source.next().unwrap() != 'i' {
        return Err(BencodeError::UnmatchedType);
    }

    let mut result_string: String = "".to_string();
    let mut current_char = source.next().expect("");
    while current_char != 'e' {
        result_string.push(current_char);
        current_char = source.next().expect("failed to read next character");
    }

    let result: i64 = result_string.parse().expect("error parsing");
    Ok(result)
}

fn decode_string(
    source: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, BencodeError> {
    let mut char_count_string: String = "".to_string();
    let mut current_char = source.next().expect("failed to read next character");
    while current_char != ':' {
        assert!(
            current_char.is_digit(10),
            "Found a non-digit character while reading string-length"
        );

        char_count_string.push(current_char);
        current_char = source.next().expect("failed to read next character");
    }

    let char_count: i64 = char_count_string
        .parse()
        .expect("error reading amount of characters in string");

    let mut result: String = "".to_string();
    for _i in 0..char_count {
        result.push(
            source
                .next()
                .expect("failed to read next character in string"),
        );
    }
    Ok(result)
}

impl BencodeDecodable<BDecodedChunk> for String {
    fn decode_bencode(&self) -> Result<BDecodedChunk, BencodeError> {
        let mut char_iter = self.chars().peekable();
        let first_char = *char_iter
            .peek()
            .expect("Couldn't read first character of bencoded int");

        match first_char {
            'i' => {
                let int_result: i64 = decode_int(&mut char_iter).unwrap();
                return Ok(BDecodedChunk::Int(int_result));
            }
            'l' => {
                let list_result: Vec<Box<BDecodedChunk>> = decode_list(&mut char_iter).unwrap();
                return Ok(BDecodedChunk::List(list_result));
            }
            'd' => {
                let dict_result: HashMap<String, Box<BDecodedChunk>> =
                    decode_dictionary(&mut &mut char_iter).unwrap();
                return Ok(BDecodedChunk::Dictionary(dict_result));
            }
            token if token.is_digit(10) => {
                let string_read: String = decode_string(&mut char_iter).unwrap();
                return Ok(BDecodedChunk::Str(string_read));
            }
            _ => {
                return Ok(BDecodedChunk::Null);
            }
        }
    }
}

pub fn bdecode(data: &String) -> Result<BDecodedChunk, BencodeError> {
    data.decode_bencode()
}
