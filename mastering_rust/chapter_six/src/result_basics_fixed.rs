use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn print_result() {
    let path = Path::new("hello.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };

    let mut s = String::new();
    file.read_to_string(&mut s);
    println!("Message: {}", s);
}
