extern crate brotli;

use std::io::Read;
use brotli::Decompressor;

fn main() {
    let mut input = vec![];
    let _ = Decompressor::new(&b"\x11\x3f\x00\x00\x24\xb0\xe2\x99\x80\x12".to_vec() as &[u8]).read_to_end(&mut input);

    println!("{:?}", input);
}