extern crate num;

use crate::zfinder::JOKER;
use num::FromPrimitive;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CasualCard {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    Casual(CasualCard),
    Joker,
}

impl Card {
    pub fn from_string(s: &str) -> Card {
        match s {
            "JOKER" => Card::Joker,
            _ => Card::Casual(CasualCard::from_string(s)),
        }
    }

    pub fn as_byte(self) -> u8 {
        match self {
            Card::Joker => JOKER,
            Card::Casual(card) => card.as_byte(),
        }
    }
}

impl Suit {
    pub fn from_string(s: &str) -> Suit {
        match s {
            "CLUBS" => Suit::Clubs,
            "DIAMONDS" => Suit::Diamonds,
            "HEARTS" => Suit::Hearts,
            "SPADES" => Suit::Spades,
            _ => panic!("Wrong suit specified!"),
        }
    }
}

impl Rank {
    pub fn from_string(s: &str) -> Rank {
        let num: u8 = match s {
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

impl CasualCard {
    pub fn from_string(s: &str) -> CasualCard {
        let tokens: Vec<String> = s.split('#').map(|s| s.to_string()).collect();
        if tokens.len() != 2 {
            panic!("Wrong CasualCard format provided. Correct: `SUIT#RANK`");
        }
        let suit = &tokens[0];
        let rank = &tokens[1];
        CasualCard {
            suit: Suit::from_string(suit),
            rank: Rank::from_string(rank),
        }
    }

    /// represent CasualCard as a byte
    pub fn as_byte(self) -> u8 {
        // formatted as follows: `0b00SSRRRR` where SS is a suit (bin 00-11) and RRRR is a rank (bin 0001-1101);
        let rank = self.rank as u8;
        let suit = self.suit as u8;

        (suit << 4) + rank
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, CasualCard, Rank, Suit};
    use crate::zfinder::JOKER;

    pub fn generate_all_cards() -> Vec<String> {
        let mut result = vec![];
        for suit in ["CLUBS", "DIAMONDS", "HEARTS", "SPADES"] {
            for rank in [
                "ACE", "2", "3", "4", "5", "6", "7", "8", "9", "10", "JACK", "QUEEN", "KING",
            ] {
                result.push(format!("{}#{}", suit, rank));
            }
        }

        result.push("JOKER".to_string());

        result
    }

    #[test]
    fn correct_parsing() {
        assert_eq!(
            CasualCard::from_string("CLUBS#ACE"),
            CasualCard {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            }
        );
        assert_eq!(
            CasualCard::from_string("HEARTS#JACK"),
            CasualCard {
                suit: Suit::Hearts,
                rank: Rank::Jack,
            }
        );
        assert_eq!(
            CasualCard::from_string("SPADES#9"),
            CasualCard {
                suit: Suit::Spades,
                rank: Rank::Nine,
            }
        );
        assert_eq!(
            CasualCard::from_string("DIAMONDS#QUEEN"),
            CasualCard {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            }
        );
    }

    #[test]
    fn all_possible_cards_are_parsed() {
        for card in generate_all_cards() {
            Card::from_string(&card);
        }
    }

    #[test]
    #[should_panic]
    fn not_enough_tokens() {
        CasualCard::from_string("CLUBS");
    }

    #[test]
    #[should_panic]
    fn no_delimiter() {
        CasualCard::from_string("CLUBS11");
    }

    #[test]
    #[should_panic]
    fn wrong_suit() {
        CasualCard::from_string("CLUUBS#11");
    }

    #[test]
    #[should_panic]
    fn wrong_rank() {
        CasualCard::from_string("CLUBS#QUUEEN");
    }

    #[test]
    fn correct_to_byte() {
        let card = CasualCard {
            suit: Suit::Spades,
            rank: Rank::Five,
        };
        assert_eq!(card.as_byte(), 0b00_11_0101);

        let card = CasualCard {
            suit: Suit::Hearts,
            rank: Rank::King,
        };
        assert_eq!(card.as_byte(), 0b00_10_1101);

        let card = Card::Joker;
        assert_eq!(card.as_byte(), JOKER);
    }
}
