use std::{
    fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = fs::File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    let map_width: usize = line_string_vec[0].len();

    let mut engine_schematic_sum = 0;
    for i in 0..line_string_vec.len() {
        let mut current_index: usize = 0;
        while current_index < map_width {
            let number_from_line =
                get_number_from_line(line_string_vec[i].clone(), current_index, map_width);

            current_index = number_from_line.1;

            if number_from_line.0.is_empty() {
                continue;
            }
            let should_include = should_include_number(
                &number_from_line.0,
                current_index,
                map_width,
                line_string_vec[i].clone(),
                if i > 0 {
                    Some(&line_string_vec[i - 1])
                } else {
                    None
                },
                if i < line_string_vec.len() - 1 {
                    Some(&line_string_vec[i + 1])
                } else {
                    None
                },
            );

            if should_include {
                engine_schematic_sum += number_from_line.0.parse::<i32>().unwrap();
            }

            current_index += 1;
        }
    }

    println!("Engine Schematic Sum: {}", engine_schematic_sum);
}

fn get_number_from_line(line: String, start_index: usize, max_index: usize) -> (String, usize) {
    let mut current_number_value = String::new();
    let mut return_index = start_index;
    let line_chars = line[start_index..line.len()].chars().collect::<Vec<char>>();
    for n in 0..line_chars.len() {
        if line_chars[n].is_numeric() {
            current_number_value.push(line_chars[n]);
            if n + 1 <= line_chars.len() - 1 && line_chars[n + 1].is_numeric() {
                continue;
            } else {
                return_index += n;
                break;
            }
        } else {
            continue;
        }
    }
    if current_number_value.is_empty() {
        return (current_number_value, max_index);
    }
    return (current_number_value, return_index);
}

fn should_include_number(
    number: &String,
    end_index: usize,
    max_index: usize,
    current_line: String,
    line_above: Option<&str>,
    line_below: Option<&str>,
) -> bool {
    let start_index = end_index + 1 - number.len();
    let mut number_indexes = (start_index..start_index + number.len()).collect::<Vec<usize>>();
    let current_line_chars = current_line.chars().collect::<Vec<char>>();

    if start_index != 0 {
        if current_line_chars[end_index - number.len()] != '.' {
            return true;
        }
        number_indexes.push(end_index - number.len());
    }

    if start_index + number.len() != max_index {
        if current_line_chars[end_index + 1] != '.' {
            return true;
        }
        number_indexes.push(end_index + 1);
    }

    if line_above.is_some() {
        let line_above_chars = line_above.unwrap().chars().collect::<Vec<char>>();
        for n in number_indexes.clone() {
            if !line_above_chars[n].is_numeric() && line_above_chars[n] != '.' {
                return true;
            }
        }
    }

    if line_below.is_some() {
        let line_below_chars = line_below.unwrap().chars().collect::<Vec<char>>();
        for n in number_indexes.clone() {
            if !line_below_chars[n].is_numeric() && line_below_chars[n] != '.' {
                return true;
            }
        }
    }

    return false;
}
