use wasm_bindgen::prelude::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

pub const ALL_RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    Diamond = 0,
    Club = 1,
    Heart = 2,
    Spade = 3,
}

pub const ALL_SUITS: [Suit; 4] = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];

pub const NUM_CARDS: usize = ALL_SUITS.len() * ALL_RANKS.len();
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[wasm_bindgen]
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self::new_const(rank, suit)
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut it = s.chars();

        let rank = match it.next()?.to_ascii_uppercase() {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => return None
        };

        let suit = match it.next()?.to_ascii_lowercase() {
            'c' => Suit::Club,
            'd' => Suit::Diamond,
            'h' => Suit::Heart,
            's' => Suit::Spade,
            _ => return None
        };

        if it.next().is_some() {
            return None;
        }

        Some(Self::new(rank, suit))
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl Card {
    pub const fn new_const(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub const fn to_index(&self) -> usize {
        ((self.rank as u8) * 4 + self.suit as u8) as usize
    }

    pub const fn from_index(index: usize) -> Self {
        if index >= NUM_CARDS {
            panic!("Card index must be 0-51");
        }
        Self {
            rank: ALL_RANKS[index / 4],
            suit: ALL_SUITS[index % 4],
        }
    }
}

#[cfg(test)]
pub fn card_vec<I: IntoIterator<Item=&'static str>>(cs: I) -> Vec<Card> {
    cs.into_iter().map(|c| Card::parse(c).unwrap()).collect()
}

#[cfg(test)]
impl Arbitrary for Card {
    fn arbitrary(g: &mut Gen) -> Self {
        let index: usize = Arbitrary::arbitrary(g);
        Self::from_index(index % NUM_CARDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn from_index_produces_sorted_values() {
        let cards_in_order: Vec<_> = (0..NUM_CARDS).map(Card::from_index).collect();
        let mut cards_sorted = cards_in_order.clone();
        cards_sorted.sort();
        assert_eq!(cards_in_order, cards_sorted);
    }

    #[test]
    fn from_index_has_no_duplicates() {
        let card_set: HashSet<_> = (0..NUM_CARDS).map(Card::from_index).collect();
        assert_eq!(card_set.len(), NUM_CARDS);
    }

    #[test]
    fn from_index_is_bijective_with_to_index() {
        for i in 0..NUM_CARDS {
            assert_eq!(Card::from_index(i).to_index(), i)
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(Card::parse("As"), Some(Card::new(Rank::Ace, Suit::Spade)));
        assert_eq!(Card::parse("aS"), Some(Card::new(Rank::Ace, Suit::Spade)));
        assert_eq!(Card::parse("Kh"), Some(Card::new(Rank::King, Suit::Heart)));
        assert_eq!(Card::parse("Tc"), Some(Card::new(Rank::Ten, Suit::Club)));
        assert_eq!(Card::parse("6d"), Some(Card::new(Rank::Six, Suit::Diamond)));
        assert_eq!(Card::parse("2f"), None);
        assert_eq!(Card::parse("Fh"), None);
        assert_eq!(Card::parse("Ash"), None);
        assert_eq!(Card::parse("A"), None);
        assert_eq!(Card::parse(""), None);
    }
}
