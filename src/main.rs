use std::collections::HashMap;

use bencode::BDecodedChunk;

use crate::bencode::BencodeDecodable;

pub mod bencode;

fn main() {
    /*let contents: String = fs::read_to_string("/home/antoon/test.py").expect("error reading file");
    //println!("{}", contents);
    Stack::bdecode(&contents);*/
    let test_string: String = "d4:spami3ee".to_string();
    let test_result: HashMap<String, Box<BDecodedChunk>> = test_string.decode_bencode().unwrap();

    /*for i in test_result {
        println!("{}", i);
    }*/

    //println!("Hello, world! --{}--", test_result);
}
