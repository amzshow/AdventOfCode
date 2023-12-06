use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_cards_points(lines: std::str::Split<'_, &str>) -> HashMap<String, i32> {
    let mut card_points: HashMap<String, i32> = HashMap::new();
    let mut card_copies: HashMap<String, i32> = HashMap::new();

    for line in lines {

        let new_line: String = line.replace("\r", "").to_owned();
        let data: Vec<&str> = new_line.split(&[':', '|'][..]).collect();

        let card_id: String = data[0].trim().to_owned();
        let id: i32 = card_id.replace("Card ", "").trim().parse::<i32>().expect("");
        let id_str: String = format!("{}", id);
        let winning_numbers: HashSet<&str> = data[1].trim().split(" ").filter(|x| !x.is_empty()).collect();
        let numbers: HashSet<&str> = data[2].trim().split(" ").filter(|x| !x.is_empty()).collect();

        let match_count: i32 = winning_numbers.intersection(&numbers).count() as i32;
        let copy_count: i32 = if card_copies.contains_key(&id_str) { card_copies.get(&id_str).expect("").to_owned() } else { 0 };

        *card_points.entry(card_id.clone()).or_insert(0) += 1 + copy_count;

        println!("{card_id} {:?}", card_points.get(&card_id));
        
        for i in 1..=match_count {
            *card_copies.entry(format!("{}", id + i)).or_insert(0) += copy_count + 1;
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