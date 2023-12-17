use std::{
    fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = fs::File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let text_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let collection = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut digits_in_line: Vec<(u8, usize)> = Vec::new();

            text_digits.iter().enumerate().for_each(|(index, text)| {
                let line_length = line.len();
                let mut line_substring = line.as_str();
                while line_substring.find(text).is_some() {
                    let found_digit_index = line_substring.find(text);
                    let line_substring_length = line_substring.len();
                    line_substring =
                        &line_substring[found_digit_index.unwrap() + 1..line_substring.len()];

                    digits_in_line.push((
                        (index + 1) as u8,
                        found_digit_index.unwrap() + line_length - line_substring_length,
                    ));
                }
            });

            let line_chars = line.chars();
            line_chars.enumerate().for_each(|(index, char)| {
                if char.is_digit(10) {
                    digits_in_line.push((char.to_digit(10).unwrap() as u8, index));
                }
            });

            digits_in_line.sort_by(|a, b| a.1.cmp(&b.1));

            let calibration_value = format!(
                "{}{}",
                digits_in_line[0].0,
                digits_in_line[digits_in_line.len() - 1].0
            );

            return calibration_value;
        })
        .collect::<Vec<String>>();

    let result: i32 = collection.iter().map(|x| x.parse::<i32>().unwrap()).sum();

    println!("Result: {}", result);
}
