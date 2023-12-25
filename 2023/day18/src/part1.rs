use std::{collections::HashSet, fs};

struct Instruction {
    direction: String,
    amount: i32,
}

fn get_instructions(list: &Vec<&str>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for i in list {
        let parts: Vec<_> = i.split(" ").collect();
        instructions.push(Instruction {
            direction: parts[0].to_owned(),
            amount: parts[1].parse::<i32>().unwrap(),
        });
    }

    return instructions;
}

fn dig(instructions: &Vec<Instruction>) -> HashSet<(usize, usize)> {
    // virtual borders may contain negative index => possible error
    let mut virtual_borders: Vec<(i32, i32)> = Vec::new();

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    for instruction in instructions {
        for _i in 0..instruction.amount {
            virtual_borders.push((i, j));

            if instruction.direction == "R" {
                j = j + 1;
            } else if instruction.direction == "L" {
                j = j - 1;
            } else if instruction.direction == "U" {
                i = i - 1;
            } else if instruction.direction == "D" {
                i = i + 1;
            }
        }
    }

    let mn_i = virtual_borders.iter().map(|x| x.0).min();
    let mn_j = virtual_borders.iter().map(|x| x.1).min();

    // borders with non-negative index
    let borders: HashSet<(usize, usize)> = virtual_borders
        .iter()
        .map(|x| {
            (
                (x.0 - mn_i.unwrap()) as usize,
                (x.1 - mn_j.unwrap()) as usize,
            )
        })
        .collect();

    return borders;
}

fn fill(
    outside_area: &mut HashSet<(usize, usize)>,
    borders: &HashSet<(usize, usize)>,
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
) {
    let mut nodes: Vec<(usize, usize)> = Vec::new();
    nodes.push((i, j));
    while nodes.len() != 0 {
        let node: (usize, usize) = nodes.remove(0);
        if outside_area.contains(&node) {
            continue;
        }
        if borders.contains(&node) {
            continue;
        }
        outside_area.insert(node);
        if node.0 > 0
            && !borders.contains(&(node.0 - 1, node.1))
            && !outside_area.contains(&(node.0 - 1, node.1))
        {
            nodes.push((node.0 - 1, node.1));
        }
        if node.0 < rows - 1
            && !borders.contains(&(node.0 + 1, node.1))
            && !outside_area.contains(&(node.0 + 1, node.1))
        {
            nodes.push((node.0 + 1, node.1));
        }
        if node.1 > 0
            && !borders.contains(&(node.0, node.1 - 1))
            && !outside_area.contains(&(node.0, node.1 - 1))
        {
            nodes.push((node.0, node.1 - 1));
        }
        if node.1 < cols - 1
            && !borders.contains(&(node.0, node.1 + 1))
            && !outside_area.contains(&(node.0, node.1 + 1))
        {
            nodes.push((node.0, node.1 + 1));
        }
    }
}

fn get_dimensions(borders: &HashSet<(usize, usize)>) -> (usize, usize) {
    if borders.len() == 0 {
        return (0, 0);
    }
    let rows: usize = borders.iter().map(|x| x.0 + 1).max().expect("");
    let cols: usize = borders.iter().map(|x| x.1 + 1).max().expect("");
    return (rows, cols);
}

fn find_outside_area(borders: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut outside_area: HashSet<(usize, usize)> = HashSet::new();

    if borders.len() == 0 {
        return outside_area;
    }
    let dims: (usize, usize) = get_dimensions(borders);
    let rows: usize = dims.0;
    let cols: usize = dims.1;

    // skip 0,0 since it would be a corner
    for i in 0..rows {
        fill(&mut outside_area, &borders, i, 0, rows, cols);
        fill(&mut outside_area, &borders, i, cols - 1, rows, cols);
        if i == 0 || i == rows - 1 {
            for j in 1..cols - 1 {
                fill(&mut outside_area, &borders, i, j, rows, cols);
            }
        }
    }

    return outside_area;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let instructions: Vec<_> = get_instructions(&lines);

    let borders: HashSet<_> = dig(&instructions);

    let outide_area: HashSet<(usize, usize)> = find_outside_area(&borders);

    let dims: (usize, usize) = get_dimensions(&borders);
    let rows: usize = dims.0;
    let cols: usize = dims.1;

    println!("{:?}", rows * cols - outide_area.len());
}
