use std::{collections::HashSet, fs};

fn roll_north(
    round_rocks: &HashSet<(usize, usize)>,
    heavy_rocks: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let mut new_pos: HashSet<(usize, usize)> = HashSet::new();

    for c in 0..cols {
        let mut free_pos: usize = 0;
        for r in 0..rows {
            let pos: (usize, usize) = (r, c);
            if round_rocks.contains(&pos) {
                new_pos.insert((free_pos, c));
                free_pos += 1;
            }
            if heavy_rocks.contains(&pos) {
                free_pos = r + 1;
            }
        }
    }

    return new_pos;
}

fn score_pos(round_rocks: &HashSet<(usize, usize)>, rows: usize) -> Vec<usize> {
    let mut scores: Vec<usize> = Vec::new();

    for pos in round_rocks {
        scores.push(rows - pos.0);
    }

    return scores;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let round_rocks: HashSet<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(row, &s)| {
            s.chars()
                .enumerate()
                .filter_map(move |(col, c)| if c == 'O' { Some((row, col)) } else { None })
        })
        .collect();

    let heavy_rocks: HashSet<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(row, &s)| {
            s.chars()
                .enumerate()
                .filter_map(move |(col, c)| if c == '#' { Some((row, col)) } else { None })
        })
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let new_pos: HashSet<(usize, usize)> = roll_north(&round_rocks, &heavy_rocks, rows, cols);

    let s: usize = score_pos(&new_pos, rows).iter().sum();

    println!("{}", s);
}
