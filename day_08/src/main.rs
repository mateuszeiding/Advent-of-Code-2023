use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
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

#[derive(Debug)]
struct Directions {
    l: String,
    r: String,
}

fn first_part(line_string_vec: Vec<String>) {
    let navigation = line_string_vec[0]
        .split("")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut directions: HashMap<String, Directions> = HashMap::new();

    let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for n in 2..line_string_vec.len() {
        let capture = pattern.captures(&line_string_vec[n]).unwrap();
        let key = capture.get(1).map_or("", |m| m.as_str());
        let left = capture.get(2).map_or("", |m| m.as_str());
        let right = capture.get(3).map_or("", |m| m.as_str());
        directions.insert(
            key.to_string(),
            Directions {
                l: left.to_string(),
                r: right.to_string(),
            },
        );
    }

    let mut curr_direction = "AAA";
    let mut nav_index = 0;
    let mut steps = 0;

    while curr_direction != "ZZZ" {
        let curr_nav = navigation[nav_index];
        let curr_directions = directions.get(curr_direction).unwrap();
        if curr_nav == "R" {
            curr_direction = curr_directions.r.as_str();
        } else {
            curr_direction = curr_directions.l.as_str();
        }
        steps += 1;
        if nav_index < navigation.len() - 1 {
            nav_index += 1;
        } else {
            nav_index = 0;
        }
    }
    println!("{:?}", steps);
}

fn second_part(line_string_vec: Vec<String>) {
    let navigation = line_string_vec[0]
        .split("")
        .filter(|x| x != &"")
        .map(|x| x)
        .collect::<Vec<&str>>();

    let mut directions: BTreeMap<&str, Directions> = BTreeMap::new();

    let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for n in 2..line_string_vec.len() {
        let capture = pattern.captures(&line_string_vec[n]).unwrap();
        let key = capture.get(1).map_or("", |m| m.as_str());
        let left = capture.get(2).map_or("", |m| m.as_str());
        let right = capture.get(3).map_or("", |m| m.as_str());
        directions.insert(
            key,
            Directions {
                l: left.to_string(),
                r: right.to_string(),
            },
        );
    }

    let start_regex = Regex::new(r"^.*[A]$").unwrap();
    let end_regex = Regex::new(r"^.*[Z]$").unwrap();
    let mut curr_directions = directions
        .iter()
        .filter(|(key, _)| start_regex.captures(key).is_some())
        .map(|(key, _)| *key)
        .collect::<Vec<&str>>();

    let mut nav_index = 0;
    let mut status = false;
    while !status {
        let mut curr_status = true;
        curr_directions = curr_directions
            .iter()
            .map(|curr| {
                let dir = directions.get(*curr).unwrap();
                if navigation[nav_index % navigation.len()] == "R" {
                    let val = dir.r.as_str();
                    curr_status = curr_status && end_regex.captures(val).is_some();
                    return val;
                } else {
                    let val = dir.l.as_str();
                    curr_status = curr_status && end_regex.captures(val).is_some();
                    return val;
                }
            })
            .collect::<Vec<&str>>();
        nav_index += 1;
        status = curr_status;
    }
    println!("{:?}", nav_index);
}
