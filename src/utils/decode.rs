use std::io::prelude::*;
use std::io;
use flate2::read::{GzDecoder, DeflateDecoder};

pub fn gzdecode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}

pub fn dedecode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut deflater = DeflateDecoder::new(&bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s)?;
    Ok(s)
}