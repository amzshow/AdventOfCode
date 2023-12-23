use std::{collections::HashMap, collections::HashSet, fs};

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

fn roll_south(
    round_rocks: &HashSet<(usize, usize)>,
    heavy_rocks: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let mut new_pos: HashSet<(usize, usize)> = HashSet::new();

    for c in 0..cols {
        let mut free_pos: usize = rows - 1;
        for r in (0..rows).rev() {
            let pos: (usize, usize) = (r, c);
            if round_rocks.contains(&pos) {
                new_pos.insert((free_pos, c));
                if r != 0 {
                    free_pos -= 1;
                }
            }
            if heavy_rocks.contains(&pos) && r != 0 {
                free_pos = r - 1;
            }
        }
    }

    return new_pos;
}

fn roll_west(
    round_rocks: &HashSet<(usize, usize)>,
    heavy_rocks: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let mut new_pos: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..rows {
        let mut free_pos: usize = 0;
        for c in 0..cols {
            let pos: (usize, usize) = (r, c);
            if round_rocks.contains(&pos) {
                new_pos.insert((r, free_pos));
                free_pos += 1;
            }
            if heavy_rocks.contains(&pos) {
                free_pos = c + 1;
            }
        }
    }

    return new_pos;
}

fn roll_east(
    round_rocks: &HashSet<(usize, usize)>,
    heavy_rocks: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let mut new_pos: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..rows {
        let mut free_pos: usize = cols - 1;
        for c in (0..cols).rev() {
            let pos: (usize, usize) = (r, c);
            if round_rocks.contains(&pos) {
                new_pos.insert((r, free_pos));
                if c != 0 {
                    free_pos -= 1;
                }
            }
            if heavy_rocks.contains(&pos) && c != 0 {
                free_pos = c - 1;
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

    let mut round_rocks: HashSet<(usize, usize)> = lines
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
    let cycles: usize = 1000000000;
    let mut loads: Vec<usize> = Vec::new();
    let mut history: HashMap<String, usize> = HashMap::new();
    let mut rep_cycle_start: usize = 0;
    let mut rep_cycle_length: usize = 0;

    for _i in 0..300 {
        round_rocks = roll_north(&round_rocks, &heavy_rocks, rows, cols);
        round_rocks = roll_west(&round_rocks, &heavy_rocks, rows, cols);
        round_rocks = roll_south(&round_rocks, &heavy_rocks, rows, cols);
        round_rocks = roll_east(&round_rocks, &heavy_rocks, rows, cols);

        let total_load: usize = score_pos(&round_rocks, rows).iter().sum();
        loads.push(total_load);

        if _i > 20 {
            let state_hash: String = loads
                .iter()
                .rev()
                .take(20)
                .map(|&x| x.to_string())
                .collect();
            if history.contains_key(&state_hash) {
                rep_cycle_start = history.get(&state_hash).expect("").clone();
                rep_cycle_length = _i - rep_cycle_start;
            }
            history.insert(state_hash, _i);
        }
    }

    let offset: usize = (cycles - rep_cycle_start) % rep_cycle_length - 1;

    let s: usize = loads[rep_cycle_start + offset];

    println!("{}", s);
}
