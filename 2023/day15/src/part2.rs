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
    let mut boxes: [Vec<(String, i32)>; 256] = vec![Vec::new(); 256].try_into().expect("static");

    for line in lines {
        for input in line.split(",") {
            let op: char = if input.contains('-') { '-' } else { '=' };
            let parts: Vec<_> = input.split(&['-', '=']).collect();
            let hash: &str = parts[0];
            let box_index: usize = hash_string(hash, 17, 256) as usize;

            if op == '=' {
                let value: i32 = parts[1].parse::<i32>().unwrap();
                let mut found_index: i32 = -1;
                for i in 0..boxes[box_index].len() {
                    if boxes[box_index][i].0 == hash {
                        found_index = i as i32;
                        break;
                    }
                }
                if found_index >= 0 {
                    boxes[box_index][found_index as usize].1 = value;
                } else {
                    boxes[box_index].push((hash.to_string(), value));
                }
            } else if op == '-' {
                let mut found_index: i32 = -1;
                for i in 0..boxes[box_index].len() {
                    if boxes[box_index][i].0 == hash {
                        found_index = i as i32;
                        break;
                    }
                }
                if found_index >= 0 {
                    boxes[box_index].remove(found_index as usize);
                }
            }
        }
    }

    let mut s: i32 = 0;

    for i in 0..256 {
        for j in 0..boxes[i].len() {
            s += (i as i32 + 1) * (j as i32 + 1) * boxes[i][j].1 as i32;
        }
    }

    println!("{}", s);
}
