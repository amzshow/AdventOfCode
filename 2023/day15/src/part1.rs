use std::fs;

fn hash_string(input: &str, mul: i32, rem: i32) -> i32 {
    let mut s: i32 = 0;
    for c in input.chars() {
        s += c as i32;
        s *= mul;
        s %= rem;
    }
    return s;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();
    let mut s: i32 = 0;

    for line in lines {
        for input in line.split(",") {
            s += hash_string(input, 17, 256)
        }
    }

    println!("{}", s);
}
