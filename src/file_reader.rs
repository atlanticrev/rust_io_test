use std::fs::{self, File};

// @todo Drop for FileReader

pub struct FileReader {
    fd: Option<File>,
    byte_array: Option<Vec<u8>>,
}

impl FileReader {
    fn new() -> Self {
        FileReader {
            fd: None,
            byte_array: None,
        }
    }

    fn open(&mut self, path: String) {
        match File::open(path) {
            Ok(file) => self.fd = Some(file),
            Err(err) => println!("{:?}", err)
        }
    }

    fn read_to_byte_array(&mut self, path: String) {
        match fs::read(path) {
            Ok(byte_array) => self.byte_array = Some(byte_array),
            Err(err) => println!("{:?}", err)
        }
    }
}
