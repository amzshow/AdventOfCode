use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_cards_points(lines: std::str::Split<'_, &str>) -> HashMap<String, i32> {
    let mut card_points: HashMap<String, i32> = HashMap::new();

    for line in lines {

        let new_line: String = line.replace("\r", "").to_owned();
        let data: Vec<&str> = new_line.split(&[':', '|'][..]).collect();

        let card_id: &str = data[0].trim();
        let winning_numbers: HashSet<&str> = data[1].trim().split(" ").filter(|x| !x.is_empty()).collect();
        let numbers: HashSet<&str> = data[2].trim().split(" ").filter(|x| !x.is_empty()).collect();

        let mut match_count: i32 = winning_numbers.intersection(&numbers).count() as i32;
        let base: i32 = 2;

        if match_count > 0 {
            match_count = match_count - 1;
            card_points.insert(card_id.to_owned(),  base.pow(match_count as u32));
        }

    }

    return card_points
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let card_points: HashMap<String, i32> = get_cards_points(lines);

    let sum: i32 = card_points.values().copied().sum();

    println!("{:?}", sum);

}