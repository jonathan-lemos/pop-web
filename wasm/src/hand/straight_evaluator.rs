use crate::card::Rank;

#[derive(Clone, Copy, Debug)]
pub struct StraightEvaluator<const MAX_CAPACITY: usize> {
    ranks: [Rank; MAX_CAPACITY],
    length: usize,
}

impl<const MAX_CAPACITY: usize> StraightEvaluator<MAX_CAPACITY> {
    pub fn new() -> Self {
        const {
            assert!(
                MAX_CAPACITY >= 5,
                "MAX_CAPACITY of StraightEvaluator must be at least 5"
            );
        }
        StraightEvaluator {
            ranks: [Rank::Two; MAX_CAPACITY],
            length: 0,
        }
    }

    // Ranks must be pushed in descending order
    // No-op for duplicate ranks
    // Drops cards beyond MAX_CAPACITY
    pub fn push(&mut self, rank: Rank) {
        if self.length == MAX_CAPACITY {
            return;
        }
        if self.length > 0 && rank == self.ranks[self.length - 1] {
            return;
        }
        self.ranks[self.length] = rank;
        self.length += 1;
    }

    pub fn best_straight_high_rank(&self) -> Option<Rank> {
        if self.length < 5 {
            return None;
        }

        let mut top_rank = self.ranks[0];
        let mut last_rank = self.ranks[0];
        let mut current_len = 1;
        let has_ace = self.ranks[0] == Rank::Ace;

        for rank in &self.ranks[1..self.length] {
            if last_rank as usize == (*rank as usize + 1) {
                current_len += 1;
                last_rank = *rank;
            } else {
                current_len = 1;
                top_rank = *rank;
                last_rank = *rank;
            }

            if current_len == 5 {
                return Some(top_rank);
            }
        }

        println!(
            "{:?} {:?} {:?} {:?}",
            top_rank, last_rank, current_len, has_ace
        );

        if top_rank == Rank::Five && current_len == 4 && has_ace {
            Some(top_rank)
        } else {
            None
        }
    }
}

impl<const MAX_CAPACITY: usize> FromIterator<Rank> for StraightEvaluator<MAX_CAPACITY> {
    fn from_iter<T: IntoIterator<Item = Rank>>(iter: T) -> Self {
        let mut evaluator = StraightEvaluator::<MAX_CAPACITY>::new();
        for rank in iter {
            evaluator.push(rank);
        }
        evaluator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drops_duplicate_ranks() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Queen);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_drops_excess_ranks() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::Queen);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        evaluator.push(Rank::Eight);
        assert_eq!(evaluator.best_straight_high_rank(), None);
    }

    #[test]
    fn test_evaluates_ace_high_straight() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Queen);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_evaluates_ten_high_straight() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        evaluator.push(Rank::Eight);
        evaluator.push(Rank::Seven);
        evaluator.push(Rank::Six);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Ten));
    }

    #[test]
    fn test_evaluates_a2345_straight() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::Five);
        evaluator.push(Rank::Four);
        evaluator.push(Rank::Three);
        evaluator.push(Rank::Two);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Five));
    }

    #[test]
    fn test_evaluates_a2345_straight_with_extra_length() {
        let mut evaluator = StraightEvaluator::<7>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::Five);
        evaluator.push(Rank::Four);
        evaluator.push(Rank::Three);
        evaluator.push(Rank::Two);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Five));
    }

    #[test]
    fn test_evaluates_straight_in_middle_of_seven_ranks() {
        let mut evaluator = StraightEvaluator::<7>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        evaluator.push(Rank::Eight);
        evaluator.push(Rank::Seven);
        evaluator.push(Rank::Two);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Jack));
    }

    #[test]
    fn test_evaluates_straight_at_start_of_seven_ranks() {
        let mut evaluator = StraightEvaluator::<7>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Queen);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        evaluator.push(Rank::Eight);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Ace));
    }

    #[test]
    fn test_evaluates_straight_at_end_of_seven_ranks() {
        let mut evaluator = StraightEvaluator::<7>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        evaluator.push(Rank::Eight);
        evaluator.push(Rank::Seven);
        evaluator.push(Rank::Six);
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Ten));
    }

    #[test]
    fn test_returns_none_for_not_enough_ranks() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Queen);
        evaluator.push(Rank::Jack);
        assert_eq!(evaluator.best_straight_high_rank(), None);
    }

    #[test]
    fn test_returns_none_for_not_straight() {
        let mut evaluator = StraightEvaluator::<5>::new();
        evaluator.push(Rank::Ace);
        evaluator.push(Rank::King);
        evaluator.push(Rank::Jack);
        evaluator.push(Rank::Ten);
        evaluator.push(Rank::Nine);
        assert_eq!(evaluator.best_straight_high_rank(), None);
    }

    #[test]
    fn test_from_iterator() {
        let ranks = &[
            Rank::Ace,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Two,
        ];
        let evaluator: StraightEvaluator<7> = ranks.into_iter().copied().collect();
        assert_eq!(evaluator.best_straight_high_rank(), Some(Rank::Jack));
    }
}
