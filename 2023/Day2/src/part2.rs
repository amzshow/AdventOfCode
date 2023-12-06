use std::fs;

fn get_power_parts(content: String) -> (i32, i32, i32) {

    let rounds: Vec<&str> = content.split(";").collect();
    let mut big_r: i32 = 0;
    let mut big_g: i32 = 0;
    let mut big_b: i32 = 0;

    for round in rounds {

        let balls: Vec<&str> = round.split(",").collect();

        for ball in balls {
            if ball.contains("red"){
                let count: i32 = ball.replace("red", "").trim().parse::<i32>().unwrap();
                if count > big_r {
                    big_r = count;
                }
            } else if ball.contains("green"){
                let count: i32 = ball.replace("green", "").trim().parse::<i32>().unwrap();
                if count > big_g {
                    big_g = count;
                }
            } else if ball.contains("blue"){
                let count: i32 = ball.replace("blue", "").trim().parse::<i32>().unwrap();
                if count > big_b {
                    big_b = count;
                }
            }
        }

    }

    return (big_r, big_g, big_b);
}

fn game_power(content: String) -> i32 {
    let sections: Vec<&str> = content.split(":").collect();
    let game_data_str = sections.last().expect("");
    let parts: (i32, i32, i32) = get_power_parts(game_data_str.to_string());
    return parts.0 * parts.1 * parts.2;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
    .expect("Read file");

    let lines: std::str::Split<'_, &str> = content.split("\n");

    let mut sum: i32 = 0;

    for line in lines {
        sum = sum + game_power(line.to_string());
    }

    print!("{}", sum)
}
