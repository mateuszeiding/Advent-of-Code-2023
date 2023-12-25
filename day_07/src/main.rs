use std::{
    cmp::Ordering,
    collections::HashMap,
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

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    bid: u16,
}

fn first_part(line_string_vec: Vec<String>) {
    let hands_vec = line_string_vec
        .iter()
        .map(|line| {
            let hand = line.split(" ").collect::<Vec<&str>>();
            return Hand {
                cards: hand[0].to_string(),
                bid: hand[1].parse::<u16>().unwrap(),
            };
        })
        .collect::<Vec<Hand>>();

    let mut hands_map = HashMap::<String, Vec<Hand>>::new();

    hands_vec.iter().for_each(|hand| {
        let key = get_key_part_one(hand);
        if hands_map.get(key).is_none() {
            hands_map.insert(key.to_string(), vec![hand.clone()]);
        } else {
            hands_map.get_mut(key).unwrap().push(hand.clone());
        }
    });

    hands_map.iter_mut().for_each(|(_, value)| {
        value.sort_by(|a, b| {
            let a_cards = a
                .cards
                .split("")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            let b_cards = b
                .cards
                .split("")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            let a_val = match_card_value_part_one(a_cards);
            let b_val = match_card_value_part_one(b_cards);

            for n in 0..5 {
                if a_val[n] != b_val[n] {
                    return a_val[n].cmp(&b_val[n]);
                }
            }

            return Ordering::Equal;
        });
    });

    let mut multiplier: u32 = 1;
    let mut sum: u32 = 0;
    let hand_types = vec!["high", "one", "two", "three", "full", "four", "five"];

    hand_types.iter().for_each(|hand_type| {
        if hands_map.get(*hand_type).is_some() {
            let hands = hands_map.get_mut(*hand_type).unwrap();
            hands.iter_mut().for_each(|hand| {
                sum += hand.bid as u32 * multiplier;
                multiplier += 1;
            });
        }
    });
    println!("{:?}", sum);
}

fn second_part(line_string_vec: Vec<String>) {
    let hands_vec = line_string_vec
        .iter()
        .map(|line| {
            let hand = line.split(" ").collect::<Vec<&str>>();
            return Hand {
                cards: hand[0].to_string(),
                bid: hand[1].parse::<u16>().unwrap(),
            };
        })
        .collect::<Vec<Hand>>();

    let mut hands_map = HashMap::<String, Vec<Hand>>::new();

    hands_vec.iter().for_each(|hand| {
        let key = get_key_part_two(hand);
        if hands_map.get(key).is_none() {
            hands_map.insert(key.to_string(), vec![hand.clone()]);
        } else {
            hands_map.get_mut(key).unwrap().push(hand.clone());
        }
    });

    hands_map.iter_mut().for_each(|(_, value)| {
        value.sort_by(|a, b| {
            let a_cards = a
                .cards
                .split("")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            let b_cards = b
                .cards
                .split("")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            let a_val = match_card_value_part_two(a_cards);
            let b_val = match_card_value_part_two(b_cards);

            for n in 0..5 {
                if a_val[n] != b_val[n] {
                    return a_val[n].cmp(&b_val[n]);
                }
            }

            return Ordering::Equal;
        });
    });

    let mut multiplier: u32 = 1;
    let mut sum: u32 = 0;
    let hand_types = vec!["high", "one", "two", "three", "full", "four", "five"];

    hand_types.iter().for_each(|hand_type| {
        if hands_map.get(*hand_type).is_some() {
            let hands = hands_map.get_mut(*hand_type).unwrap();
            hands.iter_mut().for_each(|hand| {
                sum += hand.bid as u32 * multiplier;
                multiplier += 1;
            });
        }
    });
    println!("{:?}", sum);
}

fn match_card_value_part_one(hand: Vec<&str>) -> Vec<u8> {
    let mut cards_in_hand = Vec::<u8>::new();
    hand.iter().for_each(|card| {
        let val = match card {
            &"A" => 14,
            &"K" => 13,
            &"Q" => 12,
            &"J" => 11,
            &"T" => 10,
            _ => card.parse::<u8>().unwrap(),
        };

        cards_in_hand.push(val);
    });
    return cards_in_hand;
}

fn get_key_part_one(hand: &Hand) -> &str {
    let cards = hand
        .cards
        .split("")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut unique = HashMap::<&str, u8>::new();

    cards.iter().for_each(|card| {
        if unique.get(card) == None {
            unique.insert(card, 1);
        } else {
            unique.insert(card, unique.get(card).unwrap() + 1);
        }
    });

    let key = match unique.keys().len() {
        1 => "five",
        2 => {
            let mut values = unique.values().collect::<Vec<&u8>>();
            values.sort();
            if values[1] == &4 {
                "four"
            } else {
                "full"
            }
        }
        3 => {
            let mut values = unique.values().collect::<Vec<&u8>>();
            values.sort();
            if values[2] == &3 {
                "three"
            } else {
                "two"
            }
        }
        4 => "one",
        5 => "high",
        _ => "",
    };

    return key;
}

fn match_card_value_part_two(hand: Vec<&str>) -> Vec<u8> {
    let mut cards_in_hand = Vec::<u8>::new();
    hand.iter().for_each(|card| {
        let val = match card {
            &"A" => 14,
            &"K" => 13,
            &"Q" => 12,
            &"J" => 1,
            &"T" => 10,
            _ => card.parse::<u8>().unwrap(),
        };

        cards_in_hand.push(val);
    });
    return cards_in_hand;
}

fn get_key_part_two(hand: &Hand) -> &str {
    let cards = hand
        .cards
        .split("")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut unique = HashMap::<&str, u8>::new();

    cards.iter().for_each(|card| {
        if unique.get(card) == None {
            unique.insert(card, 1);
        } else {
            unique.insert(card, unique.get(card).unwrap() + 1);
        }
    });

    if unique.keys().any(|key| key == &"J") {
        let keys = unique.keys();

        match keys.len() {
            2 => {
                let key_to_add = keys.filter(|key| key != &&"J").collect::<Vec<&&str>>()[0];
                unique.insert(
                    key_to_add,
                    unique.get(key_to_add).unwrap() + unique.get("J").unwrap(),
                );
                unique.remove("J");
            }
            3 => {
                let rest_of_keys = keys.filter(|key| key != &&"J").collect::<Vec<&&str>>();
                let key_to_add = if unique.get(rest_of_keys[0]) > unique.get(rest_of_keys[1]) {
                    rest_of_keys[0]
                } else {
                    rest_of_keys[1]
                };
                unique.insert(
                    key_to_add,
                    unique.get(key_to_add).unwrap() + unique.get("J").unwrap(),
                );
                unique.remove("J");
            }
            4 => {
                if unique.get("J") == Some(&2) {
                    let rest_of_keys = keys.filter(|key| key != &&"J").collect::<Vec<&&str>>();
                    let key_to_add = rest_of_keys[0];
                    unique.insert(
                        key_to_add,
                        unique.get(key_to_add).unwrap() + unique.get("J").unwrap(),
                    );
                    unique.remove("J");
                } else {
                    let mut rest_of_keys = keys.filter(|key| key != &&"J").collect::<Vec<&&str>>();
                    rest_of_keys.sort_by(|a, b| {
                        unique
                            .get(*a)
                            .unwrap()
                            .cmp(&unique.get(*b).unwrap())
                            .reverse()
                    });
                    let key_to_add = rest_of_keys[0];
                    unique.insert(
                        key_to_add,
                        unique.get(key_to_add).unwrap() + unique.get("J").unwrap(),
                    );
                    unique.remove("J");
                }
            }
            5 => {
                let rest_of_keys = keys.filter(|key| key != &&"J").collect::<Vec<&&str>>();
                let key_to_add = rest_of_keys[0];
                unique.insert(
                    key_to_add,
                    unique.get(key_to_add).unwrap() + unique.get("J").unwrap(),
                );
                unique.remove("J");
            }
            _ => {}
        }
    }

    let key = match unique.keys().len() {
        1 => "five",
        2 => {
            let mut values = unique.values().collect::<Vec<&u8>>();
            values.sort();
            if values[1] == &4 {
                "four"
            } else {
                "full"
            }
        }
        3 => {
            let mut values = unique.values().collect::<Vec<&u8>>();
            values.sort();
            if values[2] == &3 {
                "three"
            } else {
                "two"
            }
        }
        4 => "one",
        5 => "high",
        _ => "",
    };

    return key;
}
