use std::fs;

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

}