use std::fs::File;
use std::io::prelude::*;

pub fn get_input_string(day: i32) -> String {
    let path = format!(r"..\data\input{}.txt", day);
    let mut file = File::open(&path).expect("Unable to open the file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read the file");
    return input;
}
