use std::fs;

fn transpose(matrix: Vec<&str>) -> Vec<String> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed: Vec<String> = Vec::with_capacity(num_cols);

    for col_index in 0..num_cols {
        let mut col_string: String = String::with_capacity(num_rows);

        for row_index in 0..num_rows {
            col_string.push(matrix[row_index].as_bytes()[col_index] as char);
        }

        transposed.push(col_string);
    }

    transposed
}

fn find_hortizontal_reflection(map: &Vec<&str>) -> usize {
    if map.len() == 1 {
        return 1;
    } else if map.len() > 1 {
        let size: usize = map.len();

        for i in 1..size {
            
            let span:usize = usize::min(i, size - i);
            let start: usize = usize::min(i - span, map.len() - 1);
            let end: usize = usize::min(i + span, map.len());
            
            let mut top_v: Vec<_> = map[start..i].iter().map(|x| x.to_owned()).collect();
            top_v.reverse();
            
            let top: String = top_v.join(" ");
            let bot: String = map[i..end].join(" ");
            if top.chars().zip(bot.chars()).filter(|(c1, c2)| c1 != c2).count() == 1 {
                return i;
            }

        }
    }
    return 0;
}

fn find_vertical_reflection(map: &Vec<&str>) -> usize {
    if map.len() == 1 {
        return 1;
    } else if map.len() > 1 {

        let map: Vec<String> = transpose(map.clone());

        let size: usize = map.len();

        for i in 1..size {

            let span:usize = usize::min(i, size - i);
            let start: usize = usize::min(i - span, map.len() - 1);
            let end: usize = usize::min(i + span, map.len());
            
            let mut left_v: Vec<_> = map[start..i].iter().map(|x| x.to_owned()).collect();
            let right_v: Vec<_> = map[i..end].iter().map(|x| x.to_owned()).collect();
            left_v.reverse();
            
            let left: String = left_v.join(" ");
            let right: String = right_v.join(" ");
            
            if left.chars().zip(right.chars()).filter(|(c1, c2)| c1 != c2).count() == 1 {
                return i;
            }

        }
    }
    return 0;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let mut s: usize = 0;

    for (i, grid) in content.split("\n\n").enumerate() {
        let lines: Vec<&str> = grid.split("\n").collect();

        if i == 2 {
            print!("")
        }

        let hortizontal_index: usize = find_hortizontal_reflection(&lines);
        let vertical_index: usize = find_vertical_reflection(&lines);

        let formula: usize = if vertical_index != 0 {
            vertical_index
        } else {
            hortizontal_index * 100
        };
        s += formula;
    }
    println!("{}", s);
}
