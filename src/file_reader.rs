use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;

pub struct FileReader {}

impl FileReader {
    pub fn read_to_byte_array(path: &str) -> Vec<u8> {
        let byte_array = fs::read(path).expect("Error read to byte array");
        byte_array
    }

    pub fn read_to_string<'a>(string: &'a mut String, path: &'a str) -> &'a mut String {
        let file = File::open(path).expect("Error open file");
        let mut buf_reader = BufReader::new(file);
        buf_reader
            .read_to_string(string)
            .expect("Error read to string");
        string
    }
}
