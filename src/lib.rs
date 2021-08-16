//! A simple user-friendly bencoding encoding/decoding library
//!
//! Supports following features:
//!     - Decoding a bencoded String with unknown type
//!     - Decoding a bencoded string with known type
//!     - Error-handling
//!     - Encoding supported types (Hashmap<String, Box<BDecodedChunk>>, String,
//!     Vector<Box<BDecodedChunk>>, i64) into bencoded strings
//!
//! Source code: <https://github.com/AntoonBeres/oxybencode>

pub mod bencode;
