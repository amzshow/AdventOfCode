use std::fs;

struct Instruction {
    direction: String,
    amount: i32,
}

fn to_direction(s: String) -> String {
    if s == "0" {
        "R".to_owned()
    } else if s == "1" {
        "D".to_owned()
    } else if s == "2" {
        "L".to_owned()
    } else {
        "U".to_owned()
    }
}

fn direction_to_dx(direction: String) -> (i32, i32) {
    if direction == "U" {
        (-1, 0)
    } else if direction == "D" {
        (1, 0)
    } else if direction == "L" {
        (0, -1)
    } else if direction == "R" {
        (0, 1)
    } else {
        (0, 0)
    }
}

fn get_instructions(list: &Vec<&str>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for i in list {
        let parts: Vec<_> = i.split(" ").collect();
        let hex: String = parts[2]
            .replace("#", "")
            .replace("(", "")
            .replace(")", "")
            .to_owned();

        instructions.push(Instruction {
            direction: to_direction(hex[5..6].to_string()),
            amount: i32::from_str_radix(&hex[0..5], 16).unwrap(),
        });
    }

    return instructions;
}

fn dig(instructions: &Vec<Instruction>) -> (Vec<(i32, i32)>, usize) {
    // virtual borders may contain negative index => possible error
    let mut corners: Vec<(i32, i32)> = Vec::new();

    corners.push((0, 0));

    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut boundary_area: usize = 0;

    for instruction in instructions {
        let d = direction_to_dx(instruction.direction.clone());
        i += instruction.amount * d.0;
        j += instruction.amount * d.1;
        boundary_area += instruction.amount as usize;
        corners.push((i, j));
    }

    return (corners, boundary_area);
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let instructions: Vec<_> = get_instructions(&lines);

    let r = dig(&instructions);

    let corners: Vec<_> = r.0;
    let boundary_area: usize = r.1;

    let mut interior_area: i64 = 0;

    // Shoelace method
    for i in 0..corners.len() - 1 {
        let y1: i32 = corners[i].0;
        let x1: i32 = corners[i].1;
        let y2: i32 = corners[i + 1].0;
        let x2: i32 = corners[i + 1].1;
        let l: i64 = x1 as i64 * y2 as i64;
        let r: i64 = x2 as i64 * y1 as i64;
        interior_area += l - r;
    }

    interior_area = i64::abs(interior_area) / 2;

    // Pick's theorem
    let area: i64 = interior_area + (boundary_area / 2) as i64 + 1;

    println!("{:?}", area);
}
