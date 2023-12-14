use std::fs;

fn parse_numbers(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let numbers: Vec<Vec<i32>> = lines
        .iter()
        .map(|x| x.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    return numbers;
}

fn calculate_next_number(mut nums: Vec<i32>) -> i32 {
    let mut num: Vec<i32> = vec![];

    while nums.iter().filter(|x| **x != 0).count() != 0 {
        num.push(*nums.first().expect(""));
        nums = nums.windows(2).into_iter().map(|w| w[1] - w[0]).collect();
    }

    let mut s = 0;

    for (i, &n) in num.iter().enumerate() {
        s += if i % 2 == 0 { n } else { -n };
    }
    return s;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let numbers: Vec<Vec<i32>> = parse_numbers(&lines);

    let s: i32 = numbers
        .iter()
        .map(|x| calculate_next_number(x.clone()))
        .sum();

    println!("{:?}", s);
}
