use std::fs;
use std::collections::HashMap;

fn remove_substring_in_place(original: &mut String, substring: &str) {
    if let Some(index) = original.find(substring) {
        let end_index = index + substring.len();
        original.drain(index..end_index);
    }
}

fn find_digits(content: String) -> Vec<i32> {
    let mut digits: Vec<i32>= Vec::new();
    for c in content.chars() {
        if c.is_numeric() {
            digits.push(c.to_string().parse::<i32>().unwrap());
        }
    }
    return digits;
}

fn find_spelled_digits(content: String) -> Vec<i32> {
    let mut digits: Vec<i32>= Vec::new();
    let mut scores: HashMap<&str, i32> = HashMap::new();
    let nums: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let _nums: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut digits_unsorted: Vec<(usize, &str)> = Vec::new();

    for (i, num) in nums.iter().enumerate() {
        scores.insert(num, (i + 1) as i32);
    }

    for (i, num) in _nums.iter().enumerate() {
        scores.insert(num, (i + 1) as i32);
    }

    for num in nums {
        let idx: Vec<_> = content.match_indices(num).collect();
        if idx.len() > 0{
            digits_unsorted.extend(idx);
        }
    }

    for num in _nums {
        let idx: Vec<_> = content.match_indices(num).collect();
        if idx.len() > 0{
            digits_unsorted.extend(idx);
        }
    }

    digits_unsorted.sort_by(|a: &(usize, &str), b: &(usize, &str)| a.cmp(&b));

    for (index, num) in digits_unsorted {
        if scores.contains_key(num){
            digits.push(*scores.get(num).expect(""));
        }
    }
    return digits;
}

fn part1() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let mut sum: i32 = 0;

    for line in lines {
        let digits: Vec<i32> = find_digits(line.to_string());
        let first: &i32 =  digits.first().expect("");
        let last: &i32 =  digits.last().expect("");
        let num: i32 = (first * 10) + last;
        sum = sum + num;
        println!("{:?}, {:?}", digits, num);
    }

    print!("{sum}");
}

fn part2() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let mut sum: i32 = 0;

    for line in lines {
        let digits: Vec<i32> = find_spelled_digits(line.to_string());
        if digits.len() > 0 {
            let first: &i32 =  digits.first().expect("");
            let last: &i32 =  digits.last().expect("");
            let num: i32 = (first * 10) + last;
            sum = sum + num;
            println!("{:?}, {:?}", digits, num);
        }
    }

    print!("{sum}")
}

fn main() {
    // part1();

    part2();
}
