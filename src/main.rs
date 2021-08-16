use std::collections::HashMap;

use bencode::{BDecodedChunk, BencodeDecodable};

pub mod bencode;

fn main() {
    /*let contents: String = fs::read_to_string("/home/antoon/test.py").expect("error reading file");
    //println!("{}", contents);
    Stack::bdecode(&contents);*/
    let test_string: String = "li3e4:spami5eli66eei8ee".to_string();
    let test_result: Vec<Box<BDecodedChunk>> = test_string.decode_bencode().unwrap();

    for i in test_result {
        println!("{}", i);
    }
    //println!("Hello, world! --{}--", test_result);
}
