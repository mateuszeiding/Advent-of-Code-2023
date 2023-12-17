use std::{
    fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = fs::File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let max_blue = 14;
    let max_green = 13;
    let max_red = 12;

    let games_result: Vec<(u32, bool)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let game_id = line.split(":").collect::<Vec<&str>>()[0][5..]
                .parse::<u32>()
                .unwrap();

            let mut is_it_possible = true;
            let balls = line.split(":").collect::<Vec<&str>>()[1].split([';', ',']);
            balls.into_iter().for_each(|ball| {
                let color = ball.trim().split(" ").collect::<Vec<&str>>()[1];

                let trimmed: String = ball.chars().filter(|c| !c.is_whitespace()).collect();
                let find = trimmed.find(color);

                if find.is_some() {
                    let value = trimmed[0..trimmed.len() - color.len()]
                        .parse::<u32>()
                        .unwrap();
                    match color {
                        "blue" => {
                            if value > max_blue {
                                is_it_possible = false;
                            }
                        }
                        "green" => {
                            if value > max_green {
                                is_it_possible = false;
                            }
                        }
                        "red" => {
                            if value > max_red {
                                is_it_possible = false;
                            }
                        }
                        _ => {}
                    }
                }
            });
            return (game_id, is_it_possible);
        })
        .collect();

    let final_result = games_result
        .into_iter()
        .filter(|(_, result)| *result)
        .map(|(id, _)| id)
        .sum::<u32>();

    println!("Final result: {}", final_result);
}
