use std::{collections::HashSet, fs, vec};

fn apply_expansion_to_galaxies(
    universe: &Vec<Vec<char>>,
    galaxies: &mut Vec<(usize, usize)>,
    expand_amount: usize,
) {
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];
    let expand_amount: usize = if expand_amount == 0 { 1 } else { expand_amount };

    // find empty row index
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

    // find empty col index
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

    for i in 0..galaxies.len() {
        let mut found_middle: bool = false;
        for j in 0..empty_rows.len() - 1 {
            if empty_rows[j] < galaxies[i].0 && galaxies[i].0 < empty_rows[j + 1] {
                galaxies[i].0 = galaxies[i].0 + (expand_amount * (j + 1));
                found_middle = true;
            }
        }

        if !found_middle && !empty_rows.is_empty() && galaxies[i].0 > *empty_rows.last().expect("")
        {
            galaxies[i].0 = galaxies[i].0 + (expand_amount * empty_rows.len());
        }

        found_middle = false;
        for j in 0..empty_cols.len() - 1 {
            if empty_cols[j] < galaxies[i].1 && galaxies[i].1 < empty_cols[j + 1] {
                galaxies[i].1 = galaxies[i].1 + (expand_amount * (j + 1));
                found_middle = true;
            }
        }

        if !found_middle && !empty_cols.is_empty() && galaxies[i].1 > *empty_cols.last().expect("")
        {
            galaxies[i].1 = galaxies[i].1 + (expand_amount * empty_cols.len());
        }
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

    let universe: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    let mut galaxies: Vec<(usize, usize)> = find_galaxies(&universe);

    apply_expansion_to_galaxies(&universe, &mut galaxies, 1000000 - 1);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut distances: i64 = 0;

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
            distances += i64::abs(galaxies[i].0 as i64 - galaxies[j].0 as i64)
                + i64::abs(galaxies[i].1 as i64 - galaxies[j].1 as i64);
        }
    }

    println!("{}", distances);
}
