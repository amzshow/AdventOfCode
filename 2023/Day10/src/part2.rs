use core::panic;
use std::{collections::HashSet, fs, vec};

enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

fn get_valid_pipes() -> HashSet<char> {
    return ['|', '-', 'F', '7', 'L', 'J'].iter().cloned().collect();
}

fn determine_pipe(map: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    let rows: usize = map.len();
    let cols: usize = if rows > 0 {
        map.first().expect("").len()
    } else {
        0
    };
    // args check
    if i >= rows || j >= cols {
        return '.';
    }

    let up: bool = i > 0;
    let down: bool = i < rows - 1;
    let left: bool = j > 0;
    let right: bool = j < cols - 1;

    let mut directions: [bool; 4] = [false, false, false, false];

    if up {
        directions[Direction::Up as usize] =
            get_connection_directions(map[i - 1][j])[Direction::Down as usize];
    }
    if down {
        directions[Direction::Down as usize] =
            get_connection_directions(map[i + 1][j])[Direction::Up as usize];
    }
    if left {
        directions[Direction::Left as usize] =
            get_connection_directions(map[i][j - 1])[Direction::Right as usize];
    }
    if right {
        directions[Direction::Right as usize] =
            get_connection_directions(map[i][j + 1])[Direction::Left as usize];
    }

    for pipe in get_valid_pipes() {
        if directions
            .clone()
            .iter()
            .eq(get_connection_directions(pipe).iter())
        {
            return pipe;
        }
    }

    return '.';
}

fn get_connection_directions(c: char) -> [bool; 4] {
    // up, down, left, right
    let mut neighbor_directions: [bool; 4] = [false, false, false, false];
    if c == '|' {
        neighbor_directions[Direction::Up as usize] = true;
        neighbor_directions[Direction::Down as usize] = true;
    } else if c == '-' {
        neighbor_directions[Direction::Left as usize] = true;
        neighbor_directions[Direction::Right as usize] = true;
    } else if c == 'L' {
        neighbor_directions[Direction::Up as usize] = true;
        neighbor_directions[Direction::Right as usize] = true;
    } else if c == 'F' {
        neighbor_directions[Direction::Right as usize] = true;
        neighbor_directions[Direction::Down as usize] = true;
    } else if c == '7' {
        neighbor_directions[Direction::Left as usize] = true;
        neighbor_directions[Direction::Down as usize] = true;
    } else if c == 'J' {
        neighbor_directions[Direction::Up as usize] = true;
        neighbor_directions[Direction::Left as usize] = true;
    } else if c == 'S' {
        neighbor_directions[Direction::Up as usize] = true;
        neighbor_directions[Direction::Down as usize] = true;
        neighbor_directions[Direction::Left as usize] = true;
        neighbor_directions[Direction::Right as usize] = true;
    }
    return neighbor_directions;
}

fn bfs(map: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<(usize, usize)> {
    if map.len() == 0 {
        return HashSet::new();
    }

    let mut queue: Vec<(usize, usize)> = vec![(i, j)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() != 0 {
        let node: (usize, usize) = queue.remove(0);

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        let pipe: char = map[node.0][node.1];
        let _directions: [bool; 4] = get_connection_directions(pipe);

        if _directions[Direction::Up as usize] && node.0 > 0 {
            queue.push((node.0 - 1, node.1));
        }
        if _directions[Direction::Down as usize] && node.0 < map.len() - 1 {
            queue.push((node.0 + 1, node.1));
        }
        if _directions[Direction::Left as usize] && node.1 > 0 {
            queue.push((node.0, node.1 - 1));
        }
        if _directions[Direction::Right as usize] && node.1 < map[0].len() - 1 {
            queue.push((node.0, node.1 + 1));
        }
    }

    return visited;
}

fn find_and_determine_s(map: &mut Vec<Vec<char>>) -> [usize; 2] {
    let mut start_pos: [usize; 2] = [0, 0];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start_pos = [i, j];
                map[i][j] = determine_pipe(&map, i, j);
                break;
            }
        }
    }
    return start_pos;
}

fn clean_up_map(map: &Vec<Vec<char>>, boundary: &HashSet<(usize, usize)>) -> Vec<Vec<char>> {
    let mut clean_map: Vec<Vec<char>> = map.clone();
    // Leave only boundaries
    for (i, x) in map.iter().enumerate() {
        for (j, _y) in x.iter().enumerate() {
            clean_map[i][j] = if !boundary.contains(&(i, j)) {
                '.'
            } else {
                map[i][j]
            };
        }
    }
    return clean_map;
}

fn count_inversions(map: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count: i32 = 0;
    let mut up: char = ' ';
    for _j in 0..j {
        let c = map[i][_j];
        if c == '|' {
            count += 1;
            up = ' ';
        } else if ['L', 'F'].contains(&c) {
            up = c;
        } else if ['7', 'J'].contains(&c) {
            if (up == 'L' && c == '7') || (up == 'F' && c == 'J') {
                count += 1;
            } else {
                count += 2;
            }
            up = ' ';
        } else if c == '.' {
            up = ' ';
        } else if c == '-' {
            continue;
        } else {
            panic!("WTF")
        }
    }
    return count;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let mut map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    let start_pos: [usize; 2] = find_and_determine_s(&mut map);

    let boundary: HashSet<(usize, usize)> = bfs(&map, start_pos[0], start_pos[1]);

    let clean_map: Vec<Vec<char>> = clean_up_map(&map, &boundary);

    let mut insides: i32 = 0;

    for i in 0..clean_map.len() {
        for j in 0..clean_map[i].len() {
            if !boundary.contains(&(i, j)) {
                let s: i32 = count_inversions(&clean_map, i, j);
                if s % 2 == 1 {
                    insides += 1;
                }
            }
        }
    }

    println!("{}", insides)
}
