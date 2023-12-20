use std::{
    fs,
    io::{prelude::*, BufReader},
    iter,
};

fn main() {
    let file = fs::File::open("./src/input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    //   let map_width: usize = line_string_vec[0].len();
    // first_part(line_string_vec);
    second_part(line_string_vec);
}

// fn first_part(line_string_vec: Vec<String>) {
//     let chunked_cards = chunk_cards(line_string_vec);

//     let mut pile_worth = 0;
//     for n in 0..chunked_cards.len() {
//         let card_worth = get_card_worth(chunked_cards[n].1.to_vec(), chunked_cards[n].2.to_vec());
//         pile_worth += card_worth;
//     }
// }

fn second_part(line_string_vec: Vec<String>) {
    let chunked_cards = chunk_cards(line_string_vec);
    let mut card_quantity_vec = vec![1; chunked_cards.len()];

    for n in 0..chunked_cards.len() {
        let matching_numbers_count =
            get_matching_numbers_count(chunked_cards[n].1.to_vec(), chunked_cards[n].2.to_vec());

        println!("chunked_cards: {:?}", chunked_cards[n].0);
        for m in (chunked_cards[n].0)..(matching_numbers_count + chunked_cards[n].0) {
            println!("m: {:?}", m);
            if card_quantity_vec.get(m).is_some() {
                card_quantity_vec[m] += card_quantity_vec[n];
            }
            println!("card_quantity_vec: {:?}", card_quantity_vec);
        }
    }

    let mut cards_quantity_sum = 0;
    card_quantity_vec
        .iter()
        .for_each(|x| cards_quantity_sum += x);

    println!("card_quantity_vec: {:?}", card_quantity_vec);
    println!("cards_quantity_sum: {:?}", cards_quantity_sum);
}

fn chunk_cards(line_string_vec: Vec<String>) -> Vec<(usize, Vec<String>, Vec<String>)> {
    let mut line_vec_chunked = Vec::new();

    for n in 0..line_string_vec.len() {
        let line_split = line_string_vec[n]
            .split([':', '|'])
            .into_iter()
            .collect::<Vec<&str>>();

        let card_id = line_split[0]
            .split(" ")
            .filter(|x| x != &"")
            .collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let card_numbers = line_split[1]
            .split(' ')
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();

        let winning_numbers = line_split[2]
            .split(' ')
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();

        line_vec_chunked.push((
            card_id,
            card_numbers
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            winning_numbers
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ));
    }

    return line_vec_chunked;
}

// fn get_card_worth(card_numbers: Vec<String>, winning_numbers: Vec<String>) -> usize {
//     let mut card_worth = 0;
//     for n in 0..card_numbers.len() {
//         if winning_numbers.contains(&card_numbers[n]) {
//             if card_worth == 0 {
//                 card_worth = 1;
//             } else {
//                 card_worth *= 2;
//             }
//         }
//     }

//     return card_worth;
// }

fn get_matching_numbers_count(card_numbers: Vec<String>, winning_numbers: Vec<String>) -> usize {
    let mut matching_numbers_count = 0;
    for n in 0..card_numbers.len() {
        if winning_numbers.contains(&card_numbers[n]) {
            matching_numbers_count += 1;
        }
    }

    return matching_numbers_count;
}
