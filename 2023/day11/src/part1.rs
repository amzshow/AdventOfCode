use std::{collections::HashSet, fs, vec};

fn expand_universe(universe: &mut Vec<Vec<char>>) {
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    // count empty rows
    for i in 0..universe.len() {
        let mut is_empty: bool = true;
        for j in 0..universe[0].len() {
            if universe[i][j] != '.' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_rows.push(i);
        }
    }

    // count empty cols
    for j in 0..universe[0].len() {
        let mut is_empty: bool = true;
        for i in 0..universe.len() {
            if universe[i][j] != '.' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_cols.push(j);
        }
    }

    // expand cols
    for (offset, col) in empty_cols.iter().enumerate() {
        for i in 0..universe.len() {
            universe[i].insert(col + offset, '.');
        }
    }

    // expand rows
    for (offset, row) in empty_rows.iter().enumerate() {
        let mut emptyness: Vec<char> = Vec::new();
        emptyness.resize(universe[0].len(), '.');
        universe.insert(row + offset, emptyness);
    }
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = vec![];

    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    return galaxies;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let mut universe: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    expand_universe(&mut universe);

    let galaxies: Vec<(usize, usize)> = find_galaxies(&universe);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut distances: i32 = 0;

    for i in 0..galaxies.len() {
        for j in 0..galaxies.len() {
            if i == j {
                continue;
            };
            if visited.contains(&(j, i)) || visited.contains(&(i, j)) {
                continue;
            };
            visited.insert((j, i));
            visited.insert((i, j));
            distances += i32::abs(galaxies[i].0 as i32 - galaxies[j].0 as i32)
                + i32::abs(galaxies[i].1 as i32 - galaxies[j].1 as i32);
        }
    }

    println!("{}", distances);
}
