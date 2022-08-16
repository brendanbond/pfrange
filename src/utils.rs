use std::fs::OpenOptions;
use std::io::prelude::*;

const CARDS: [&str; 13] = [
    "Ace", "King", "Queen", "Jack", "Ten", "Nine", "Eight", "Seven", "Six", "Five", "Four",
    "Three", "Two",
];

pub fn generate_hands_list() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("basic.range")
        .unwrap();

    for i in 0..CARDS.len() {
        for j in 0..CARDS.len() {
            if i == j {
                if let Err(e) = writeln!(
                    file,
                    "Hand(CardType::{}, CardType::{}, SuitedType::None),",
                    CARDS[i], CARDS[j]
                ) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                if j < i {
                    continue;
                } else {
                    for k in 0..2 {
                        let suited = if k == 0 {
                            "SuitedType::Suited"
                        } else {
                            "SuitedType::Offsuit"
                        };
                        if let Err(e) = writeln!(
                            file,
                            "Hand(CardType::{}, CardType::{}, {}),",
                            CARDS[i], CARDS[j], suited
                        ) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }
                }
            }
        }
    }
}
