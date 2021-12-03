extern crate num;

use num::FromPrimitive;

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    pub fn from_string(s: &String) -> Suit {
        match s.as_str() {
            "CLUBS" => Suit::Clubs,
            "DIAMONDS" => Suit::Diamonds,
            "HEARTS" => Suit::Hearts,
            "SPADES" => Suit::Spades,
            _ => panic!("Wrong suit specified!"),
        }
    }
}

impl Rank {
    pub fn from_string(s: &String) -> Rank {
        let num: u8 = match s.as_str() {
            "ACE" => 1,
            "JACK" => 11,
            "QUEEN" => 12,
            "KING" => 13,
            "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => s.parse().unwrap(),
            _ => panic!("Wrong rank specified!"),
        };

        Rank::from_u8(num).unwrap()
    }
}

impl Card {
    pub fn from_string(s: String) -> Card {
        let tokens: Vec<String> = s.split('#').map(|s| s.to_string()).collect();
        if tokens.len() != 2 {
            panic!("Wrong card format provided. Correct: `SUIT#RANK`");
        }
        let suit = &tokens[0];
        let rank = &tokens[1];
        Card {
            suit: Suit::from_string(suit),
            rank: Rank::from_string(rank),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn correct_parsing() {
        assert_eq!(
            Card::from_string("CLUBS#ACE".to_string()),
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
        );
        assert_eq!(
            Card::from_string("HEARTS#JACK".to_string()),
            Card {
                suit: Suit::Hearts,
                rank: Rank::Jack
            }
        );
        assert_eq!(
            Card::from_string("SPADES#9".to_string()),
            Card {
                suit: Suit::Spades,
                rank: Rank::Nine
            }
        );
        assert_eq!(
            Card::from_string("DIAMONDS#QUEEN".to_string()),
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen
            }
        );
    }

    #[test]
    #[should_panic]
    fn not_enough_tokens() {
        Card::from_string("CLUBS".to_string());
    }

    #[test]
    #[should_panic]
    fn no_delimiter() {
        Card::from_string("CLUBS11".to_string());
    }

    #[test]
    #[should_panic]
    fn wrong_suit() {
        Card::from_string("CLUUBS#11".to_string());
    }

    #[test]
    #[should_panic]
    fn wrong_rank() {
        Card::from_string("CLUBS#QUUEEN".to_string());
    }
}
