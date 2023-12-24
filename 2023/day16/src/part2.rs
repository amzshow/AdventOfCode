use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3,
}

fn shoot_beam(
    map: &Vec<Vec<char>>,
    trail_map: &mut Vec<Vec<[bool; 4]>>,
    start_pos: (usize, usize),
    shoot_towards: Direction,
) {
    let mut i: usize = start_pos.0;
    let mut j: usize = start_pos.1;
    let mut beam_direction: Direction = shoot_towards;
    let follow: bool = true;

    while follow {
        // check if direction needs changing
        if ['/', '\\'].contains(&map[i][j]) {
            // update direction

            if beam_direction == Direction::UP && map[i][j] == '/' {
                beam_direction = Direction::RIGHT;
            } else if beam_direction == Direction::DOWN && map[i][j] == '/' {
                beam_direction = Direction::LEFT;
            } else if beam_direction == Direction::LEFT && map[i][j] == '/' {
                beam_direction = Direction::DOWN;
            } else if beam_direction == Direction::RIGHT && map[i][j] == '/' {
                beam_direction = Direction::UP;
            } else if beam_direction == Direction::UP && map[i][j] == '\\' {
                beam_direction = Direction::LEFT;
            } else if beam_direction == Direction::DOWN && map[i][j] == '\\' {
                beam_direction = Direction::RIGHT;
            } else if beam_direction == Direction::LEFT && map[i][j] == '\\' {
                beam_direction = Direction::UP;
            } else if beam_direction == Direction::RIGHT && map[i][j] == '\\' {
                beam_direction = Direction::DOWN;
            };
        } else if ['-', '|'].contains(&map[i][j]) {
            // split or pass
            if (beam_direction == Direction::UP || beam_direction == Direction::DOWN)
                && map[i][j] == '-'
            {
                beam_direction = Direction::LEFT;
                shoot_beam(map, trail_map, (i, j), Direction::RIGHT);
            } else if (beam_direction == Direction::LEFT || beam_direction == Direction::RIGHT)
                && map[i][j] == '|'
            {
                beam_direction = Direction::UP;
                shoot_beam(map, trail_map, (i, j), Direction::DOWN);
            }
        }

        if trail_map[i][j][beam_direction as usize] {
            break;
        } else {
            trail_map[i][j][beam_direction as usize] = true;
        }

        if beam_direction == Direction::LEFT && j > 0 {
            j = j - 1;
        } else if beam_direction == Direction::RIGHT && j < map[i].len() - 1 {
            j = j + 1;
        } else if beam_direction == Direction::UP && i > 0 {
            i = i - 1;
        } else if beam_direction == Direction::DOWN && i < map.len() - 1 {
            i = i + 1;
        } else {
            break;
        }
    }
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: Vec<&str> = content.split("\n").collect();

    let map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    let mut possible_starts: Vec<(usize, usize, Direction)> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 {
                possible_starts.push((i, j, Direction::DOWN));
            }
            if j == 0 {
                possible_starts.push((i, j, Direction::RIGHT));
            }
            if i == map.len() - 1 {
                possible_starts.push((i, j, Direction::UP));
            }
            if j == map[i].len() - 1 {
                possible_starts.push((i, j, Direction::LEFT));
            }
        }
    }

    let mut best: usize = 0;

    for starts in possible_starts {
        let mut trail_map: Vec<Vec<[bool; 4]>> =
            vec![vec![[false, false, false, false]; map[0].len()]; map.len()];

        shoot_beam(&map, &mut trail_map, (starts.0, starts.1), starts.2);

        let s = trail_map
            .iter()
            .flatten()
            .filter(|x| x.iter().any(|y| *y))
            .count();

        if s > best {
            best = s;
        }
    }

    println!("{}", best);
}
