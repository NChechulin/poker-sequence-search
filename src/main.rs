#[macro_use]
extern crate enum_primitive;

use std::io;
use std::io::{BufRead, Write};
use std::process::exit;

use zfinder::ZFinder;

use crate::card::Card;

mod card;
mod zfinder;

fn parse_line(line: String) -> Vec<Card> {
    let mut result = vec![];

    for card in line.trim().split(' ') {
        let new_card = Card::from_string(&card.to_string());
        result.push(new_card);
    }

    result
}

fn read_cards(whose_cards: &str) -> Vec<Card> {
    print!("Input {} cards: ", whose_cards);
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");

    parse_line(line)
}

fn solve(computer_cards: Vec<Card>, player_cards: Vec<Card>) -> Result<Vec<usize>, ()> {
    if computer_cards.len() <= player_cards.len() {
        eprintln!("Computer should have more cards than player!");
        exit(1);
    }

    let data: Vec<u8> = computer_cards.iter().map(|card| card.as_byte()).collect();
    let pattern: Vec<u8> = player_cards.iter().map(|card| card.as_byte()).collect();

    let mut zfinder = ZFinder::new(&data, &pattern);

    match zfinder.find_all() {
        Ok(val) => Ok(val),
        Err(_) => Err(()),
    }
}

fn main() {
    let computer_cards = read_cards("computer's");
    let player_cards = read_cards("player's");

    match solve(computer_cards, player_cards) {
        Err(_) => println!("Loss"),
        Ok(occurrences) => {
            print!("Win: ");
            for occurrence in occurrences {
                print!("{} ", occurrence);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, solve};

    #[test]
    fn test_1() {
        let computer =
            "CLUBS#ACE CLUBS#ACE CLUBS#ACE HEARTS#JACK SPADES#9 SPADES#9 HEARTS#JACK SPADES#9";
        let user = "HEARTS#JACK SPADES#9";

        let computer = parse_line(computer.to_string());
        let user = parse_line(user.to_string());

        let result = solve(computer, user);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![3, 6]);
    }

    #[test]
    fn test_2() {
        let computer =
            "CLUBS#ACE CLUBS#ACE CLUBS#ACE HEARTS#JACK SPADES#9 SPADES#9 HEARTS#JACK SPADES#9";
        let user = "HEARTS#ACE SPADES#ACE";

        let computer = parse_line(computer.to_string());
        let user = parse_line(user.to_string());

        let result = solve(computer, user);
        assert!(result.is_err());
    }

    #[test]
    fn test_joker() {
        let computer =
            "CLUBS#ACE CLUBS#ACE CLUBS#ACE HEARTS#JACK SPADES#9 SPADES#9 HEARTS#JACK SPADES#9";
        let user = "JOKER HEARTS#JACK SPADES#9";

        let computer = parse_line(computer.to_string());
        let user = parse_line(user.to_string());

        let result = solve(computer, user);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![2, 5]);
    }

    // TODO: more tests
}
