mod file_reader;
use file_reader::*;

fn main() {
    let mut file_reader = FileReader::new();
    let string = file_reader
        .open("./src/test2.txt")
        .read_to_string();
    println!("{}", string);
    
    file_reader.read_to_byte_array("./test.txt");
    println!("{:?}", file_reader.byte_array);
}
