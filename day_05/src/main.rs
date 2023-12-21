use std::{
    fs::File,
    io::{prelude::*, BufReader},
    ops::Range,
};

fn main() {
    let file = File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    first_part(line_string_vec);
}

#[derive(Debug)]
struct MapRange {
    source_start: u64,
    source_range: Range<u64>,
    destination_start: u64,
}

fn first_part(line_string_vec: Vec<String>) {
    let mut all_seeds = line_string_vec[0].split_whitespace().collect::<Vec<&str>>();
    all_seeds.remove(0);

    let all_seeds_numbers = all_seeds
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let map_names = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let mut all_sources = all_seeds_numbers.clone();

    for n in 0..map_names.len() {
        let to_ranges = get_ranges(&line_string_vec, map_names[n]);

        let mut new_source = Vec::<u64>::new();
        all_sources.iter().for_each(|seed| {
            if to_ranges.iter().any(|x| x.source_range.contains(seed)) == false {
                new_source.push(*seed);
            } else {
                let source_map = to_ranges
                    .iter()
                    .find(|x| x.source_range.contains(seed))
                    .unwrap();

                let destination = source_map.destination_start + (seed - source_map.source_start);

                new_source.push(destination);
            }
        });

        all_sources = new_source.clone();
    }

    println!("min location: {:?}", all_sources.iter().min().unwrap());
}

fn get_ranges(line_string_vec: &Vec<String>, map_name: &str) -> Vec<MapRange> {
    let map_start_index = line_string_vec.iter().position(|x| x == map_name).unwrap();

    let mut ranges = Vec::new();

    for i in map_start_index + 1..line_string_vec.len() {
        if line_string_vec[i].is_empty() {
            break;
        }
        let line = line_string_vec[i].split_whitespace();
        let line_numbers = line
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        ranges.push(MapRange {
            source_start: line_numbers[1],
            source_range: line_numbers[1]..line_numbers[1] + line_numbers[2],
            destination_start: line_numbers[0],
        });
    }

    return ranges;
}
