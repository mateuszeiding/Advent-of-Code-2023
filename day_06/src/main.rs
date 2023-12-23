use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    // first_part(line_string_vec);
    second_part(line_string_vec);
}

fn first_part(line_string_vec: Vec<String>) {
    let times = line_string_vec[0]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances = line_string_vec[1]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut result = 1;

    for n in 0..distances.len() {
        let mut chances_to_beat_record = 0;

        for t in 0..times[n] {
            let speed = t;

            let time_to_travel = times[n] - t;

            if time_to_travel * speed > distances[n] {
                chances_to_beat_record += 1;
            }
        }

        result *= chances_to_beat_record;
    }

    println!("Result: {}", result);
}

fn second_part(line_string_vec: Vec<String>) {
    let times = line_string_vec[0]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .collect::<Vec<&str>>();

    let distances = line_string_vec[1]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .collect::<Vec<&str>>();

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let final_time = times.join("").parse::<u64>().unwrap();

    let final_distance = distances.join("").parse::<u64>().unwrap();

    let mut chances_to_beat_record = 0;

    for t in 0..final_time {
        let speed = t;

        let time_to_travel = final_time - t;

        if time_to_travel * speed > final_distance {
            chances_to_beat_record += 1;
        }
    }

    println!("chances_to_beat_record: {}", chances_to_beat_record);
}
