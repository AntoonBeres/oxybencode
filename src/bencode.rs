use core::fmt;
use itertools::Itertools;
use std::collections::HashMap;

pub enum BDecodedChunk {
    Dictionary(HashMap<Box<String>, Box<BDecodedChunk>>),
    Int(i64),
    List(Vec<Box<BDecodedChunk>>),
    Str(Box<String>),
    Null,
}

pub enum BencodeError {
    UnmatchedType,
    InvalidBencode,
    UnexpectedError,
}

impl fmt::Display for BDecodedChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BDecodedChunk::Int(i) => write!(f, "{}", i),
            BDecodedChunk::Str(s) => write!(f, "{}", *s),
            BDecodedChunk::List(foo) => {
                write!(f, "<LIST>:\n    {}", foo.iter().format("\n    "))
            }
            BDecodedChunk::Dictionary(_foo) => write!(f, "<DICT>"),
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

pub struct Stack<T> {
    stack: Vec<T>,
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
        /*
        let j: i64 = 64;
        i.push(Box::new(j));
        Ok(i)*/
    }
}

impl BencodeDecodable<HashMap<String, Box<BDecodedChunk>>> for String {
    fn decode_bencode(&self) -> Result<HashMap<String, Box<BDecodedChunk>>, BencodeError> {
        let _first_char = self.chars().next().unwrap();
        /*if first_char == 'f' {
        }*/

        let mut i: HashMap<String, Box<BDecodedChunk>> = HashMap::new();
        let test_string = "yeet".to_string();
        let j: i64 = 5;
        let k: String = "yeeqwatqawtr".to_string();

        i.insert(test_string, Box::new(BDecodedChunk::Int(j)));
        i.insert(
            "aqwfra".to_string(),
            Box::new(BDecodedChunk::Str(Box::new(k))),
        );

        Ok(i)
    }
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
                let chunk_item = BDecodedChunk::Str(Box::new(string_read));
                result_vec.push(Box::new(chunk_item));
            }
            'l' => {
                let vec_read: Vec<Box<BDecodedChunk>> = decode_list(source).unwrap();
                let chunk_item = BDecodedChunk::List(vec_read);
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
    //source.next(); // Drop the trailing 'e'

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

/*impl BencodeDecodable<BDecodedChunk> for String {
    fn decode_bencode(&self) -> Result<BDecodedChunk, BencodeError> {
        let result: BDecodedChunk;
        let first_char = self.chars().next().unwrap();
        match first_char {
            'i' => {
                let int_result: i64 = self.decode_bencode().unwrap();
                return Ok(BDecodedChunk::Int(int_result));
            }
            'l' => {
                let list_result: Vec<Box<dyn Bencode>> = self.decode_bencode().unwrap();
                return Ok(BDecodedChunk::List(list_result));
            }
            _ => {
                return Ok(BDecodedChunk::Null);
            }
        }
    }
}*/

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn size(&self) -> usize {
        self.stack.len()
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

pub fn bdecode(data: &String) -> BDecodedChunk {
    if data.is_empty() {
        return BDecodedChunk::Null;
    }
    let char_vec: Vec<char> = data.chars().collect();
    let mut char_iter = data.chars();

    match char_iter.next().unwrap() {
        'y' => println!("it is y"),
        _ => println!("yfagft"),
    }

    match char_vec[0] {
        _ => println!("yeet"),
    }

    BDecodedChunk::Null
}
