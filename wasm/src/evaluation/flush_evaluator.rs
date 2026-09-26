use crate::card::{ALL_SUITS, Card, Rank, Suit};

const FLUSH_LEN: usize = 5;

#[derive(Clone, Copy, Debug)]
struct RankCollector {
    ranks: [Rank; FLUSH_LEN],
    length: usize,
}

impl RankCollector {
    pub fn new() -> Self {
        RankCollector {
            ranks: [Rank::Two; FLUSH_LEN],
            length: 0,
        }
    }

    pub fn push(&mut self, rank: Rank) {
        if self.length >= FLUSH_LEN {
            return;
        }
        self.ranks[self.length] = rank;
        self.length += 1;
    }
}

#[derive(Debug)]
pub struct FlushEvaluator {
    suit_rank_collectors: [RankCollector; ALL_SUITS.len()],
}

impl FlushEvaluator {
    pub fn new() -> Self {
        Self {
            suit_rank_collectors: [RankCollector::new(); ALL_SUITS.len()],
        }
    }

    // Ranks must be pushed in descending order
    pub fn push(&mut self, card: Card) {
        self.suit_rank_collectors[card.suit() as usize].push(card.rank());
    }

    // Ranks are sorted in descending order
    pub fn best_flush_ranks(&self) -> Option<[Rank; 5]> {
        let mut best: Option<&[Rank; 5]> = None;
        for suit_rank_collector in &self.suit_rank_collectors {
            if suit_rank_collector.length == FLUSH_LEN {
                match best {
                    Some(best_so_far) => {
                        if &suit_rank_collector.ranks > best_so_far {
                            best = Some(&suit_rank_collector.ranks);
                        }
                    }
                    None => {
                        best = Some(&suit_rank_collector.ranks);
                    }
                }
            }
        }
        best.copied()
    }
}

impl FromIterator<Card> for FlushEvaluator {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut evaluator = FlushEvaluator::new();
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
        let evaluator = FlushEvaluator::new();
        assert_eq!(evaluator.best_flush_ranks(), None);
    }

    #[test]
    fn test_returns_none_for_fewer_than_five_ranks_total() {
        let mut evaluator = FlushEvaluator::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Spade));
        evaluator.push(Card::new(Rank::King, Suit::Spade));
        evaluator.push(Card::new(Rank::Queen, Suit::Spade));
        evaluator.push(Card::new(Rank::Jack, Suit::Spade));
        assert_eq!(evaluator.best_flush_ranks(), None);
    }

    #[test]
    fn test_returns_none_for_fewer_than_five_ranks_in_any_suit() {
        let mut evaluator = FlushEvaluator::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Spade));
        evaluator.push(Card::new(Rank::King, Suit::Spade));
        evaluator.push(Card::new(Rank::Queen, Suit::Spade));
        evaluator.push(Card::new(Rank::Jack, Suit::Spade));
        evaluator.push(Card::new(Rank::Ten, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Club));
        evaluator.push(Card::new(Rank::Eight, Suit::Diamond));
        assert_eq!(evaluator.best_flush_ranks(), None);
    }

    #[test]
    fn test_evaluates_five_card_flush() {
        let mut evaluator = FlushEvaluator::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Heart));
        evaluator.push(Card::new(Rank::King, Suit::Heart));
        evaluator.push(Card::new(Rank::Queen, Suit::Heart));
        evaluator.push(Card::new(Rank::Jack, Suit::Heart));
        evaluator.push(Card::new(Rank::Nine, Suit::Heart));
        assert_eq!(
            evaluator.best_flush_ranks(),
            Some([Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine])
        );
    }

    #[test]
    fn test_ignores_excess_ranks() {
        let mut evaluator = FlushEvaluator::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Club));
        evaluator.push(Card::new(Rank::King, Suit::Club));
        evaluator.push(Card::new(Rank::Jack, Suit::Club));
        evaluator.push(Card::new(Rank::Eight, Suit::Club));
        evaluator.push(Card::new(Rank::Seven, Suit::Club));
        evaluator.push(Card::new(Rank::Five, Suit::Club));
        evaluator.push(Card::new(Rank::Two, Suit::Club));
        assert_eq!(
            evaluator.best_flush_ranks(),
            Some([Rank::Ace, Rank::King, Rank::Jack, Rank::Eight, Rank::Seven])
        );
    }

    #[test]
    fn test_chooses_highest_flush() {
        let mut evaluator = FlushEvaluator::new();
        evaluator.push(Card::new(Rank::Ace, Suit::Club));
        evaluator.push(Card::new(Rank::King, Suit::Diamond));
        evaluator.push(Card::new(Rank::Ten, Suit::Club));
        evaluator.push(Card::new(Rank::Queen, Suit::Diamond));
        evaluator.push(Card::new(Rank::Eight, Suit::Club));
        evaluator.push(Card::new(Rank::Jack, Suit::Diamond));
        evaluator.push(Card::new(Rank::Six, Suit::Club));
        evaluator.push(Card::new(Rank::Ten, Suit::Diamond));
        evaluator.push(Card::new(Rank::Three, Suit::Club));
        evaluator.push(Card::new(Rank::Eight, Suit::Diamond));
        assert_eq!(
            evaluator.best_flush_ranks(),
            Some([Rank::Ace, Rank::Ten, Rank::Eight, Rank::Six, Rank::Three])
        );
    }

    #[test]
    fn test_from_iterator() {
        let cards = card_array(["Ac", "Kd", "Tc", "Qd", "8c", "Jd", "6c", "Td", "3c", "8d"]);
        let evaluator: FlushEvaluator = cards.into_iter().collect();
        assert_eq!(
            evaluator.best_flush_ranks(),
            Some([Rank::Ace, Rank::Ten, Rank::Eight, Rank::Six, Rank::Three])
        );
    }
}
