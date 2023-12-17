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

    let lines = reader.lines().filter(|line| line.is_ok());

    let line_string_vec = lines
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .into_iter();

    let games_result: Vec<(u32, bool, u32, u32, u32)> = line_string_vec
        .map(|line| {
            let game_id = line.split(":").collect::<Vec<&str>>()[0][5..]
                .parse::<u32>()
                .unwrap();

            let mut is_it_possible = true;

            let mut blue_value = 0;
            let mut red_value = 0;
            let mut green_value = 0;
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

                            if value > blue_value {
                                blue_value = value;
                            }
                        }
                        "green" => {
                            if value > max_green {
                                is_it_possible = false;
                            }
                            if value > green_value {
                                green_value = value;
                            }
                        }
                        "red" => {
                            if value > max_red {
                                is_it_possible = false;
                            }
                            if value > red_value {
                                red_value = value;
                            }
                        }
                        _ => {}
                    }
                }
            });

            println!(
                "Game: {}, is_it_possible: {}, blue: {}, green: {}, red: {}",
                game_id, is_it_possible, blue_value, green_value, red_value
            );

            return (
                game_id,
                is_it_possible,
                match blue_value {
                    0 => 1,
                    _ => blue_value,
                },
                match green_value {
                    0 => 1,
                    _ => green_value,
                },
                match red_value {
                    0 => 1,
                    _ => red_value,
                },
            );
        })
        .collect::<Vec<(u32, bool, u32, u32, u32)>>();

    let result_iter = games_result.into_iter();
    let part_1_result = result_iter
        .clone()
        .filter(|result| result.1)
        .map(|id| id.0)
        .sum::<u32>();

    let part_2_result = result_iter
        .map(|(_, _, blue, green, red)| blue * green * red)
        .sum::<u32>();

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}
