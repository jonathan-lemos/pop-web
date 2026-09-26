use crate::card::{ALL_SUITS, Card, Rank, Suit};
use crate::evaluation::straight_evaluator::StraightEvaluator;

#[derive(Debug)]
pub struct StraightFlushEvaluator<const MAX_CAPACITY: usize> {
    suit_straight_evaluators: [StraightEvaluator<MAX_CAPACITY>; ALL_SUITS.len()],
}

impl<const MAX_CAPACITY: usize> StraightFlushEvaluator<MAX_CAPACITY> {
    pub fn new() -> Self {
        Self {
            suit_straight_evaluators: [StraightEvaluator::new(); ALL_SUITS.len()],
        }
    }

    // Ranks must be pushed in descending order
    pub fn push(&mut self, card: Card) {
        self.suit_straight_evaluators[card.suit() as usize].push(card.rank());
    }

    // Ranks are sorted in descending order
    pub fn best_straight_flush_rank(&self) -> Option<Rank> {
        for suit_straight_evaluator in &self.suit_straight_evaluators {
            if let Some(high_rank) = suit_straight_evaluator.best_straight_high_rank() {
                return Some(high_rank);
            }
        }
        None
    }
}

impl<const MAX_CAPACITY: usize> FromIterator<Card> for StraightFlushEvaluator<MAX_CAPACITY> {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut evaluator = Self::new();
        for card in iter {
            evaluator.push(card);
        }
        evaluator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::card_array;

    #[test]
    fn test_returns_none_when_empty() {
        let evaluator = StraightFlushEvaluator::<7>::new();
        assert_eq!(evaluator.best_straight_flush_rank(), None);
    }

    #[test]
    fn test_returns_none_for_fewer_than_five_ranks_total() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Spade));
        evaluator.push(Card::new(Rank::King, Suit::Spade));
        evaluator.push(Card::new(Rank::Queen, Suit::Spade));
        evaluator.push(Card::new(Rank::Jack, Suit::Spade));
        assert_eq!(evaluator.best_straight_flush_rank(), None);
    }

    #[test]
    fn test_returns_none_for_fewer_than_five_ranks_in_any_suit() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Spade));
        evaluator.push(Card::new(Rank::King, Suit::Spade));
        evaluator.push(Card::new(Rank::Queen, Suit::Spade));
        evaluator.push(Card::new(Rank::Jack, Suit::Spade));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Club));
        evaluator.push(Card::new(Rank::Eight, Suit::Diamond));
        assert_eq!(evaluator.best_straight_flush_rank(), None);
    }

    #[test]
    fn test_returns_none_for_not_straight() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::King, Suit::Heart));
        evaluator.push(Card::new(Rank::Queen, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Heart));
        assert_eq!(evaluator.best_straight_flush_rank(), None);
    }

    #[test]
    fn test_evaluates_five_card_straight_flush() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::King, Suit::Heart));
        evaluator.push(Card::new(Rank::Queen, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_evaluates_straight_flush_at_beginning() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::King, Suit::Heart));
        evaluator.push(Card::new(Rank::Queen, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Heart));
        evaluator.push(Card::new(Rank::Eight, Suit::Heart));
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_evaluates_straight_flush_in_middle() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::Queen, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Heart));
        evaluator.push(Card::new(Rank::Eight, Suit::Heart));
        evaluator.push(Card::new(Rank::Seven, Suit::Heart));
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Queen));
    }

    #[test]
    fn test_evaluates_straight_flush_at_end() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::King, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Heart));
        evaluator.push(Card::new(Rank::Eight, Suit::Heart));
        evaluator.push(Card::new(Rank::Seven, Suit::Heart));
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Jack));
    }

    #[test]
    fn test_a23435_straight_flush() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::Five, Suit::Heart));
        evaluator.push(Card::new(Rank::Four, Suit::Heart));
        evaluator.push(Card::new(Rank::Three, Suit::Heart));
        evaluator.push(Card::new(Rank::Two, Suit::Heart));
        println!("{:?}", evaluator);
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Five));
    }

    #[test]
    fn test_ignores_excess_ranks() {
        let mut evaluator = StraightFlushEvaluator::<7>::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Club));
        evaluator.push(Card::new(Rank::King, Suit::Club));
        evaluator.push(Card::new(Rank::Queen, Suit::Club));
        evaluator.push(Card::new(Rank::Jack, Suit::Club));
        evaluator.push(Card::new(Rank::Ten, Suit::Club));
        evaluator.push(Card::new(Rank::Nine, Suit::Club));
        evaluator.push(Card::new(Rank::Eight, Suit::Club));
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_from_iterator() {
        let cards = card_array(["Ah", "Qh", "Jh", "Th", "9h", "8h", "7h"]);
        let evaluator: StraightFlushEvaluator<7> = cards.into_iter().collect();
        assert_eq!(evaluator.best_straight_flush_rank(), Some(Rank::Queen));
    }
}
