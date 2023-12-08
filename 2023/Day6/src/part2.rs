use std::fs;

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect::<Vec<&str>>();

    let mut possibility_mult: i64 = 1;

    let time: i64 = lines[0]
        .replace("Time:", "")
        .to_owned()
        .trim()
        .to_owned()
        .replace(" ", "")
        .to_owned()
        .parse::<i64>()
        .unwrap();

    let distance: i64 = lines[1]
        .replace("Distance:", "")
        .to_owned()
        .trim()
        .to_owned()
        .replace(" ", "")
        .to_owned()
        .parse::<i64>()
        .unwrap();

    let mut win_hold_ms: Vec<i64> = vec![];

    for hold_time in 1..=time {
        let remaining_time = time - hold_time;
        let distance_travelled = remaining_time * hold_time;
        if distance_travelled > distance {
            win_hold_ms.push(hold_time);
        }
    }

    possibility_mult = possibility_mult * win_hold_ms.len() as i64;

    println!("{:?}", possibility_mult);
}
