use std::fs;

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();
}
