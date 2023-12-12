use std::{collections::HashMap, fs};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(xs: Vec<i32>) -> i32 {
    let mut ans: i32 = 1;
    for x in xs {
        ans = (x * ans) / gcd(x, ans)
    }
    return ans;
}

fn parse_nodes(lines: Vec<&str>) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let op: Vec<&str> = line.split("=").into_iter().map(|x| x.trim()).collect();
        let lro = op[1].replace("(", "").replace(")", "");
        let lr: Vec<&str> = lro.split(',').map(|x| x.trim()).collect();

        map.insert(
            (*op[0]).to_string(),
            ((*lr[0]).to_string(), (*lr[1]).to_string()),
        );
    }

    return map;
}

fn is_power_of_10(num: u64) -> bool {
    (num > 0) && ((num as f64).log10() % 1.0 == 0.0)
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let ops: Vec<char> = lines[0].chars().collect();

    let map: HashMap<String, (String, String)> = parse_nodes((&lines[2..]).to_vec());
    let step_limit: i64 = 10000000000000000;

    let mut current: Vec<&String> = map.keys().filter(|x| x.ends_with('A')).collect();

    let mut i: usize = 0;
    let mut steps: i64 = 0;

    while !current.iter().all(|x| x.ends_with('Z')) {
        for j in 0..current.len() {
            current[j] = if ops[i] == 'L' {
                &map.get(current[j]).expect("").0
            } else {
                &map.get(current[j]).expect("").1
            };
        }

        steps += 1;

        i = (i + 1) % ops.len();

        if is_power_of_10(steps.try_into().unwrap()) {
            println!("{:?}", steps);
        }

        if steps > step_limit.try_into().unwrap() {
            println!("Too many steps");
            break;
        }
    }

    println!("{:?}", steps);
}
