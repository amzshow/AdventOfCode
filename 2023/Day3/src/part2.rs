use std::fs;

#[derive(Clone, Debug)]
struct NumPos {
    row: usize,
    col_i: usize,
    col_j: usize,
    value: i32
}

#[derive(Clone, Debug)]
struct GearPos {
    row: usize,
    col: usize
}

fn find_gear_pos(text: &Vec<Vec<char>>) -> Vec<GearPos> {
    let mut gear_pos: Vec<GearPos> = vec![];

    for (i, row) in text.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == '*' {
                let mut pos: GearPos = GearPos {
                    row: i,
                    col: j
                };
                gear_pos.push(pos);
            }

        }
    }

    return gear_pos;
}

fn find_num_pos(text: &Vec<Vec<char>>) -> Vec<NumPos> {
    let mut num_pos: Vec<NumPos> = vec![];

    let pos_default: NumPos = NumPos {
        row: usize::MAX,
        col_i: usize::MAX,
        col_j: usize::MAX,
        value: -1
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
                    let n: String = text[pos.row as usize][(pos.col_i as usize)..=(pos.col_j  as usize)].iter().collect();
                    let num: i32 = n.parse::<i32>().unwrap();
                    pos.value = num;
                    num_pos.push(pos);
                    pos = pos_default.clone();
                }
            }

            if j == row.len() - 1 && pos.row != usize::MAX {
                let n: String = text[pos.row as usize][(pos.col_i as usize)..=(pos.col_j  as usize)].iter().collect();
                let num: i32 = n.parse::<i32>().unwrap();
                pos.value = num;
                num_pos.push(pos);
                pos = pos_default.clone();
            }
        }
    }

    return num_pos;
}

fn is_num_adjoining(num: &NumPos, gear: &GearPos) -> bool {

    let row_diff: i32 = (gear.row as i32 - num.row as i32).abs();
    let col_diff_start: i32 = (gear.col as i32 - num.col_i as i32).abs();
    let col_diff_end: i32 = (gear.col as i32 - num.col_j as i32).abs();

    return row_diff <= 1 && (col_diff_start <= 1 || col_diff_end <= 1)

}

fn find_nums_with_adjoining_gear(num_pos: &Vec<NumPos>, gear_pos: &Vec<GearPos>) -> Vec<i32> {
    let mut nums: Vec<i32> = vec![];

    for gear in gear_pos {
        let mut mul: Vec<i32> = vec![];
        for num in num_pos {
            if is_num_adjoining(num, gear) {
                mul.push(num.value);
            }
        }
        if mul.len() == 2 {
            let mut sum = 1;
            for m in mul {
                sum = sum * m;
            }
            nums.push(sum);
        }
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
    let gear_pos: Vec<GearPos> = find_gear_pos(&text);
    
    let nums_with_adjoining_symbols: Vec<i32> = find_nums_with_adjoining_gear(&num_pos, &gear_pos);
    let sum: i32 = nums_with_adjoining_symbols.iter().sum();

    println!("{:?}", nums_with_adjoining_symbols);

    println!("{:?}", sum);
    
}