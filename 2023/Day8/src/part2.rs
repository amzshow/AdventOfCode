use std::{collections::HashMap, fs};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(xs: Vec<i64>) -> i64 {
    let mut ans: i64 = 1;
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

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let ops: Vec<char> = lines[0].chars().collect();

    let map: HashMap<String, (String, String)> = parse_nodes((&lines[2..]).to_vec());

    let mut current: Vec<&String> = map.keys().filter(|x| x.ends_with('A')).collect();

    let mut i: usize = 0;
    let mut steps: i64 = 0;

    let mut checkpoint: Vec<i64> = vec![];

    for mut c in current {
        i = 0;
        steps = 0;
        while !(*c).ends_with('Z') {
            c = if ops[i] == 'L' {
                &map.get(c).expect("").0
            } else {
                &map.get(c).expect("").1
            };

            steps += 1;
            i = (i + 1) % ops.len();
        }
        checkpoint.push(steps)
    }

    println!("{:?}", lcm(checkpoint));
}
