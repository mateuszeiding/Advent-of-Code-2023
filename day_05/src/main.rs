use std::{
    fs::File,
    io::{prelude::*, BufReader},
    ops::Range,
};

fn main() {
    let file = File::open("./src/test_input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    // first_part(line_string_vec);
    second_part(line_string_vec);
}

#[derive(Debug, Clone)]
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
        println!("get_ranges: {:?}", to_ranges);
        let mut new_source = Vec::<u64>::new();
        all_sources.iter().for_each(|seed| {
            println!("Seed: {:?}", seed);
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

fn second_part(line_string_vec: Vec<String>) {
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

    let mut seeds_ranges = Vec::<MapRange>::new();
    for n in (0..all_seeds_numbers.len()).step_by(2) {
        seeds_ranges.push(MapRange {
            source_start: all_seeds_numbers[n],
            source_range: all_seeds_numbers[n]..all_seeds_numbers[n] + all_seeds_numbers[n + 1],
            destination_start: 0,
        });
    }
    seeds_ranges.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));
    println!("seed ranges: {:?}", seeds_ranges);

    let mut to_ranges = get_ranges(&line_string_vec, map_names[0]);
    to_ranges.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));

    let mut all_sources = seeds_ranges.clone();

    for n in 0..map_names.len() {
        let to_ranges = get_ranges(&line_string_vec, map_names[n]);
        println!("--------------------------------------");
        println!(
            "map: {}, all sources: {:?}",
            map_names[n],
            all_sources
                .iter()
                .map(|x| x.source_range.clone())
                .collect::<Vec<Range<u64>>>()
        );
        println!(
            "to ranges: {:?}",
            to_ranges
                .iter()
                .map(|x| x.source_range.clone())
                .collect::<Vec<Range<u64>>>()
        );
        let mut new_source = Vec::<MapRange>::new();
        all_sources.iter().for_each(|seed_range| {
            if to_ranges.iter().any(|x| {
                x.source_range.end > seed_range.source_range.start
                    || x.source_range.start < seed_range.source_range.end
            }) == false
            {
                new_source.push(seed_range.clone());
            } else if to_ranges.iter().any(|x| {
                x.source_range.contains(&seed_range.source_range.start)
                    && x.source_range.contains(&seed_range.source_range.end)
            }) {
                let source_map = to_ranges
                    .iter()
                    .find(|x| x.source_range.contains(&seed_range.source_range.start))
                    .unwrap();
                println!("source map: {:?}", source_map);
                let start_difference =
                    seed_range.source_range.start - source_map.source_range.start;
                let destination_range = source_map.destination_start + start_difference
                    ..source_map.destination_start + start_difference + seed_range.source_range.end
                        - seed_range.source_range.start
                        + 1;

                new_source.push(MapRange {
                    source_start: destination_range.start,
                    source_range: destination_range,
                    destination_start: 0,
                });
            } else {
                println!("seed range: {:?}", seed_range);

                let source_with_end = to_ranges
                    .iter()
                    .find(|x| x.source_range.contains(&seed_range.source_range.end));

                let source_with_start = to_ranges
                    .iter()
                    .find(|x| x.source_range.contains(&seed_range.source_range.start));
                println!("source with start: {:?}", source_with_start);
                println!("source with end: {:?}", source_with_end);
                if source_with_start.is_some() {
                    let source_map = source_with_start.unwrap();
                    let start_difference =
                        seed_range.source_range.start - source_map.source_range.start;
                    let destination_range = source_map.destination_start + start_difference
                        ..source_map.destination_start
                            + start_difference
                            + source_map.source_range.end
                            - seed_range.source_range.start;

                    println!("start destination range: {:?}", destination_range);
                    new_source.push(MapRange {
                        source_start: destination_range.start,
                        source_range: destination_range,
                        destination_start: 0,
                    });
                }

                if source_with_end.is_some() {
                    let source_map = source_with_end.unwrap();
                    let start_difference =
                        seed_range.source_range.end - source_map.source_range.start;
                    let destination_range = source_map.destination_start + start_difference
                        ..source_map.destination_start
                            + start_difference
                            + seed_range.source_range.end
                            - source_map.source_range.start;

                    println!("end destination range: {:?}", destination_range);
                    new_source.push(MapRange {
                        source_start: destination_range.start,
                        source_range: destination_range,
                        destination_start: 0,
                    });
                }

                if source_with_start.is_some()
                    && source_with_end.is_some()
                    && source_with_start.unwrap().source_range.end
                        == source_with_end.unwrap().source_range.start
                {
                    return;
                }

                let new_range_start = if source_with_start.is_some() {
                    source_with_start.unwrap().source_range.end
                } else {
                    seed_range.source_range.start
                };

                let new_range_end = if source_with_end.is_some() {
                    source_with_end.unwrap().source_range.start
                } else {
                    seed_range.source_range.end
                };
                let new_range = new_range_start..new_range_end;
                println!("new range: {:?}", new_range);
                new_source.push(MapRange {
                    source_start: new_range_start,
                    source_range: new_range,
                    destination_start: 0,
                });
            }
        });
        all_sources = new_source.clone();
    }

    all_sources.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));
    println!("min location: {:?}", all_sources);
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
