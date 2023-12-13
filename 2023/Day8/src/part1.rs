use std::{collections::HashMap, fs};

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

    let mut current: &str = "AAA";
    let mut i: usize = 0;
    let mut steps: i32 = 0;

    while current != "ZZZ" {
        current = if ops[i] == 'L' {
            &map.get(current).expect("").0
        } else {
            &map.get(current).expect("").1
        };

        steps += 1;

        i = (i + 1) % ops.len();
    }

    println!("{:?}", steps)
}
