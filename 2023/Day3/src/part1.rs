use std::fs;

#[derive(Clone, Debug)]
struct NumPos {
    row: usize,
    col_i: usize,
    col_j: usize
}

fn find_num_pos(text: &Vec<Vec<char>>) -> Vec<NumPos> {
    let mut num_pos: Vec<NumPos> = vec![];

    let pos_default: NumPos = NumPos {
        row: usize::MAX,
        col_i: usize::MAX,
        col_j: usize::MAX
    };

    for (i, row) in text.iter().enumerate() {
        let mut pos: NumPos = pos_default.clone();
        for (j, char) in row.iter().enumerate() {
            if char.is_numeric() {
                if pos.row == usize::MAX {
                    pos.row = i;
                    pos.col_i = j;
                }
                pos.col_j = j;
            } else {
                if pos.row != usize::MAX {
                    num_pos.push(pos);
                    pos = pos_default.clone();
                }
            }

            if j == row.len() - 1 && pos.row != usize::MAX {
                num_pos.push(pos);
                pos = pos_default.clone();
            }
        }
    }

    return num_pos;
}

fn is_symbol(chr: char) -> bool {
    return chr != '.' && !chr.is_alphanumeric();
}

fn is_num_adjoining(text: &Vec<Vec<char>> , pos: &NumPos) -> i32 {
    // let num = text[pos.row as usize]
    // .iter()
    // .cloned()
    // .skip(pos.col_i as usize)
    // .take(pos.col_j  as usize - pos.col_i  as usize + 1)
    // .collect();

    let n: String = text[pos.row as usize][(pos.col_i as usize)..=(pos.col_j  as usize)].iter().collect();
    let num: i32 = n.parse::<i32>().unwrap();

    let rows: usize = text.len();

    if rows != 0 {

        let cols: usize = text[0].len();

        let move_left: bool = pos.col_i > 0;
        let move_right: bool = pos.col_j < cols - 1;
        let move_up: bool = pos.row > 0;
        let move_down: bool = pos.row < rows - 1;

        if move_left {

            if is_symbol(text[pos.row][pos.col_i - 1]) {
                return num;
            }

            if move_up {
                if is_symbol(text[pos.row - 1][pos.col_i - 1]) {
                    return num;
                }
            }
            if move_down {
                if is_symbol(text[pos.row + 1][pos.col_i - 1]) {
                    return num;
                }
            }
        }

        if move_right {
            if is_symbol(text[pos.row][pos.col_j + 1]) {
                return num;
            }

            if move_up {
                if is_symbol(text[pos.row - 1][pos.col_j + 1]) {
                    return num;
                }
            }
            if move_down {
                if is_symbol(text[pos.row + 1][pos.col_j + 1]) {
                    return num;
                }
            }
        }

        for i in pos.col_i..=pos.col_j {
            if move_up {
                if is_symbol(text[pos.row - 1][i]) {
                    return num;
                }
            }
            if move_down {
                if is_symbol(text[pos.row + 1][i]) {
                    return num;
                }
            }
        }

    }

    return 0;
}

fn find_nums_with_adjoining_symbols(text: &Vec<Vec<char>> , num_pos: &Vec<NumPos>) -> Vec<i32> {
    let mut nums: Vec<i32> = vec![];

    for pos in num_pos {
        nums.push(is_num_adjoining(text, pos));
    }

    return nums;
}

pub fn call() {

    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Lines<'_> = content.lines();
    let mut text: Vec<Vec<char>> = vec![];
    
    for line in lines {
        let mut subtext: Vec<char> = vec![];
        for char in line.chars() {
            subtext.push(char);
        }
        text.push(subtext);
    }

    let num_pos: Vec<NumPos> = find_num_pos(&text);
    
    let nums_with_adjoining_symbols: Vec<i32> = find_nums_with_adjoining_symbols(&text, &num_pos);
    let sum: i32 = nums_with_adjoining_symbols.iter().sum();

    println!("{:?}", nums_with_adjoining_symbols);

    println!("{:?}", sum);
    
}