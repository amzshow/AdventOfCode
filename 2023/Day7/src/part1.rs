use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
    count: HashMap<char, i32>,
    rank: i32,
}

fn parse_hand_bid(lines: std::str::Split<'_, &str>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        let splt: Vec<&str> = line.split(" ").collect();
        let cards: String = splt[0].to_string();
        let bid: i32 = splt[1].parse::<i32>().unwrap();
        hands.push(Hand {
            cards: cards,
            bid: bid,
            count: HashMap::new(),
            rank: 0,
        })
    }

    return hands;
}

fn count_chars(hand: &Hand) -> HashMap<char, i32> {
    let mut count: HashMap<char, i32> = HashMap::new();

    for (_i, c) in hand.cards.char_indices() {
        *count.entry(c).or_insert(0) += 1;
    }

    return count;
}

fn hand_value(hand: &Hand) -> i32 {
    let mut count: Vec<&i32> = hand.count.values().collect();
    count.sort_by(|a, b| b.cmp(a));
    return match count.as_slice() {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => 0,
    };
}

fn card_to_int(c: char) -> i32 {
    return match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    };
}

fn rank(hand: &Hand) -> i32 {
    let mut num: i32 = 0;
    let chs: Vec<char> = hand.cards.chars().collect();
    for c in chs {
        let m: i32 = card_to_int(c);
        num = num * 14 + m;
    }
    return num;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let mut hands: Vec<Hand> = parse_hand_bid(lines);

    for i in 0..hands.len() {
        hands[i].count = count_chars(&hands[i]);
    }

    hands.sort_by(|a, b| {
        hand_value(a)
            .cmp(&hand_value(b))
            .then_with(|| rank(a).cmp(&rank(b)))
    });

    for i in 0..hands.len() {
        hands[i].rank = (i + 1) as i32;
    }

    let s: i32 = hands.iter().map(|x| x.rank * x.bid).sum();

    println!("{:?}", s);
}
