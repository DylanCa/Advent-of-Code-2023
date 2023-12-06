use std::fs::File;
use std::io::BufReader;

pub fn get_file_reader(day: &str) -> BufReader<File> {
    let file = match File::open(format!("./src/{}/input.txt", day)) {
        Ok(file) => file,
        Err(err) => panic!("Failed to open file: {}", err),
    };
    return BufReader::new(file);
}