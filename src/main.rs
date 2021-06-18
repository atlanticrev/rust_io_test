mod file_reader;
use file_reader::*;

fn main() {
    let mut string = String::new();
    let string = FileReader::read_to_string(&mut string, "./test.vert.txt");
    println!("[Output] Read to string:\n\n{}\n", string);

    let buf = FileReader::read_to_byte_array("./test.vert.txt");
    println!("[Output] Read to buffer:\n\n{:?}\n", buf);
}
