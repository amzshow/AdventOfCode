use std::fs;

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect::<Vec<&str>>();

    let mut possibility_mult: i32 = 1;

    let time: Vec<i32> = lines[0]
        .replace("Time:", "")
        .to_owned()
        .trim()
        .to_owned()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let distance: Vec<i32> = lines[1]
        .replace("Distance:", "")
        .to_owned()
        .trim()
        .to_owned()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for (i, t) in time.iter().enumerate() {
        let d: &i32 = distance.get(i).expect("");

        let mut win_hold_ms: Vec<i32> = vec![];

        for hold_time in 1..=*t {
            let remaining_time = t - hold_time;
            let distance_travelled = remaining_time * hold_time;
            if distance_travelled > *d {
                win_hold_ms.push(hold_time);
            }
        }

        possibility_mult = possibility_mult * win_hold_ms.len() as i32;
    }

    println!("{:?}", possibility_mult);
}
