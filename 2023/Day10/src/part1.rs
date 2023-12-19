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
    let right: bool = i < cols - 1;

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
        if directions.iter().eq(get_connection_directions(pipe).iter()) {
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

fn bfs(map: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut s: i32 = 0;

    if map.len() == 0 {
        return s;
    }

    let mut queue: Vec<(usize, usize, i32)> = vec![(i, j, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() != 0 {
        let node: (usize, usize, i32) = queue.remove(0);
        let pos: (usize, usize) = (node.0, node.1);
        let _s: i32 = node.2;

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        s = if _s > s { _s } else { s };

        let pipe: char = map[pos.0][pos.1];
        let _directions: [bool; 4] = get_connection_directions(pipe);

        if _directions[Direction::Up as usize] && pos.0 > 0 {
            queue.push((node.0 - 1, node.1, _s + 1));
        }
        if _directions[Direction::Down as usize] && pos.0 < map.len() - 1 {
            queue.push((node.0 + 1, node.1, _s + 1));
        }
        if _directions[Direction::Left as usize] && pos.1 > 0 {
            queue.push((node.0, node.1 - 1, _s + 1));
        }
        if _directions[Direction::Right as usize] && pos.1 < map[0].len() - 1 {
            queue.push((node.0, node.1 + 1, _s + 1));
        }
    }

    return s;
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

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let mut map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    let start_pos: [usize; 2] = find_and_determine_s(&mut map);

    println!("{}", bfs(&map, start_pos[0], start_pos[1]));
}
