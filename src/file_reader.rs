use std::io::prelude::*;
use std::fs::{self, File};

// @todo Drop for FileReader

pub struct FileReader {
    fd: Option<File>,
    pub byte_array: Option<Vec<u8>>,
}

impl FileReader {
    pub fn new() -> Self {
        FileReader {
            fd: None,
            byte_array: None,
        }
    }

    pub fn open(&mut self, path: &str) -> &mut Self {
        match File::open(path) {
            Ok(file) => self.fd = Some(file),
            Err(err) => println!("Error open file: {:?}", err),
        }
        self
    }

    pub fn read_to_byte_array(&mut self, path: &str) {
        match fs::read(path) {
            Ok(byte_array) => self.byte_array = Some(byte_array),
            Err(err) => println!("Error read to byte array: {:?}", err),
        }
    }

    pub fn read_to_string(&mut self) -> String {
        let mut string = String::new();
        match &mut self.fd {
            Some(file) => {
                match file.read_to_string(&mut string) {
                    Ok(num_bytes) => println!("Bytes was read: {:?}", num_bytes),
                    Err(err) => println!("{:?}", err),
                }
            },
            None => println!("Need to create a FileReader instance first"),
        }
        string
    }
}
