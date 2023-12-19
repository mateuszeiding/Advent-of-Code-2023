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

    // first_part(line_string_vec.clone(), map_width);
    second_part(line_string_vec.clone(), map_width);
}

fn second_part(line_string_vec: Vec<String>, map_width: usize) {
    let mut all_gears = Vec::new();
    for i in 0..line_string_vec.len() {
        let gears_from_line = get_gears_from_line(
            line_string_vec[i].clone(),
            i,
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
        all_gears.extend(gears_from_line);
    }

    let mut all_numbers = Vec::new();
    for n in 0..line_string_vec.len() {
        let mut current_index: usize = 0;
        while current_index < map_width {
            let number_from_line =
                get_number_from_line(line_string_vec[n].clone(), current_index, map_width);

            let start_index = number_from_line.1 + 1 - number_from_line.0.len();
            if !number_from_line.0.is_empty() {
                all_numbers.push((n, number_from_line.0, start_index, number_from_line.1));
            }
            current_index = number_from_line.1 + 1;
        }
    }

    let mut result_value = 0;

    for n in 0..all_gears.len() {
        let all_numbers_for_gear = all_numbers
            .iter()
            .filter(|&(line, _, _, _)| {
                *line == all_gears[n].0
                    || *line == all_gears[n].0 + 1
                    || *line
                        == if all_gears[n].0 == 0 {
                            all_gears[n].0
                        } else {
                            all_gears[n].0 - 1
                        }
            })
            .filter(|&(_, _, start, end)| {
                let range = all_gears[n].1 - 1..all_gears[n].1 + 2;
                return range.contains(&start) || range.contains(&end);
            })
            .collect::<Vec<&(usize, String, usize, usize)>>();
        // print!("Gear line: {:?}\n", all_gears[n]);

        println!("Gear: {:?}", all_gears[n]);
        print!("All numbers for gear: {:?}\n", all_numbers_for_gear);
        print!("--------------------------\n");
        if all_numbers_for_gear.len() == 2 {
            let result = all_numbers_for_gear[0].1.parse::<i32>().unwrap()
                * all_numbers_for_gear[1].1.parse::<i32>().unwrap();

            result_value += result;
        }

        // if all_gears[n].0 > 0 {
        //     print!("Gear: {:?}\n", line_string_vec[all_gears[n].0 - 1]);
        // }
        // print!("Line: {:?}\n", line_string_vec[all_gears[n].0]);
        // if all_gears[n].0 < line_string_vec.len() {
        //     print!("Gear: {:?}\n", line_string_vec[all_gears[n].0 + 1]);
        // }
    }

    println!("All gears: {:?}", result_value);
}

fn get_gears_from_line(
    current_line: String,
    line_index: usize,
    line_above: Option<&str>,
    line_below: Option<&str>,
) -> Vec<(usize, usize)> {
    let mut all_gears_in_line = Vec::new();
    let line_chars = current_line.chars().collect::<Vec<char>>();

    for n in 0..line_chars.len() {
        if line_chars[n] == '*' {
            let mut neibours_sum = 0;
            if line_above.is_some() {
                neibours_sum += quantity_of_numbers(line_above.unwrap(), n, false);
            }
            neibours_sum += quantity_of_numbers(&current_line, n, true);
            if line_below.is_some() {
                neibours_sum += quantity_of_numbers(line_below.unwrap(), n, false);
            }

            if neibours_sum == 2 {
                all_gears_in_line.push((line_index, n));
            }
        } else {
            continue;
        }
    }
    return all_gears_in_line;
}

fn quantity_of_numbers(line: &str, gear_index: usize, same_line: bool) -> u8 {
    let first = line.chars().nth(gear_index - 1).unwrap().is_numeric();
    let second = !same_line && line.chars().nth(gear_index).unwrap().is_numeric();
    let third = line.chars().nth(gear_index + 1).unwrap().is_numeric();
    // print!("gear_index: {:?}\n", gear_index);
    // print!("First: {:?}\n", first);
    // print!("Second: {:?}\n", second);
    // print!("Third: {:?}\n", third);
    // print!("--------------------------\n");
    match (first, second, third) {
        (true, true, false) => return 1,
        (false, true, true) => return 1,
        (true, false, false) => return 1,
        (false, false, true) => return 1,
        (true, false, true) => return 2,
        (true, true, true) => return 1,
        _ => return 0,
    }
}

// fn first_part(line_string_vec: Vec<String>, map_width: usize) {
//     let mut engine_schematic_sum = 0;
//     for i in 0..line_string_vec.len() {
//         let mut current_index: usize = 0;
//         while current_index < map_width {
//             let number_from_line =
//                 get_number_from_line(line_string_vec[i].clone(), current_index, map_width);

//             current_index = number_from_line.1;

//             if number_from_line.0.is_empty() {
//                 continue;
//             }
//             let should_include = should_include_number(
//                 &number_from_line.0,
//                 current_index,
//                 map_width,
//                 line_string_vec[i].clone(),
//                 if i > 0 {
//                     Some(&line_string_vec[i - 1])
//                 } else {
//                     None
//                 },
//                 if i < line_string_vec.len() - 1 {
//                     Some(&line_string_vec[i + 1])
//                 } else {
//                     None
//                 },
//             );

//             if should_include {
//                 engine_schematic_sum += number_from_line.0.parse::<i32>().unwrap();
//             }

//             current_index += 1;
//         }
//     }
//     println!("Engine Schematic Sum: {}", engine_schematic_sum);
// }

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

// fn should_include_number(
//     number: &String,
//     end_index: usize,
//     max_index: usize,
//     current_line: String,
//     line_above: Option<&str>,
//     line_below: Option<&str>,
// ) -> bool {
//     let start_index = end_index + 1 - number.len();
//     let mut number_indexes = (start_index..start_index + number.len()).collect::<Vec<usize>>();
//     let current_line_chars = current_line.chars().collect::<Vec<char>>();

//     if start_index != 0 {
//         if current_line_chars[end_index - number.len()] != '.' {
//             return true;
//         }
//         number_indexes.push(end_index - number.len());
//     }

//     if start_index + number.len() != max_index {
//         if current_line_chars[end_index + 1] != '.' {
//             return true;
//         }
//         number_indexes.push(end_index + 1);
//     }

//     if line_above.is_some() {
//         let line_above_chars = line_above.unwrap().chars().collect::<Vec<char>>();
//         for n in number_indexes.clone() {
//             if !line_above_chars[n].is_numeric() && line_above_chars[n] != '.' {
//                 return true;
//             }
//         }
//     }

//     if line_below.is_some() {
//         let line_below_chars = line_below.unwrap().chars().collect::<Vec<char>>();
//         for n in number_indexes.clone() {
//             if !line_below_chars[n].is_numeric() && line_below_chars[n] != '.' {
//                 return true;
//             }
//         }
//     }

//     return false;
// }
