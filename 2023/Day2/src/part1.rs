use std::fs;

fn validate_prob(content: String) -> bool {

    let rounds: Vec<&str> = content.split(";").collect();

    for round in rounds {

        let balls: Vec<&str> = round.split(",").collect();

        for ball in balls {
            if ball.contains("red"){
                let count: i32 = ball.replace("red", "").trim().parse::<i32>().unwrap();
                if count > 12 {
                    return false;
                }
            } else if ball.contains("green"){
                let count: i32 = ball.replace("green", "").trim().parse::<i32>().unwrap();
                if count > 13 {
                    return false;
                }
            } else if ball.contains("blue"){
                let count: i32 = ball.replace("blue", "").trim().parse::<i32>().unwrap();
                if count > 14 {
                    return false;
                }
            }
        }

    }

    return true;
}

fn game_valid(content: String) -> i32 {
    let sections: Vec<&str> = content.split(":").collect();
    let game_id_str = sections.first().expect("");
    let game_data_str = sections.last().expect("");

    if validate_prob(game_data_str.to_string()) {
        return game_id_str.replace("Game ", "").parse::<i32>().unwrap();
    }
    return 0;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let mut sum: i32 = 0;

    for line in lines {
        sum = sum + game_valid(line.to_string());
    }

    print!("{}", sum)
}
