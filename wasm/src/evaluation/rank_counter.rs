use crate::card::{ALL_RANKS, Rank};
use crate::evaluation::hand_result::HandResult;
use std::cmp::max;

#[derive(Debug)]
pub struct RankCounter {
    counts: [u8; ALL_RANKS.len()],
}

impl RankCounter {
    pub fn new() -> Self {
        Self {
            counts: [0; ALL_RANKS.len()],
        }
    }

    // Do not add more than 4 of the same rank.
    pub fn add(&mut self, rank: Rank) {
        debug_assert!(
            self.counts[rank as usize] < 4,
            "Cannot add more than 4 cards of the same rank"
        );
        self.counts[rank as usize] += 1;
    }

    // Evaluates the best count-based hand based on the given ranks.
    //
    // Will not output Straight, Flush, or StraightFlush.
    // You must put at least 5 cards into the RankCounter before calling this function.
    pub fn evaluate(&self) -> HandResult {
        let mut high_count_rank = (self.counts[Rank::Ace as usize], Rank::Ace);
        for rank in ALL_RANKS[0..ALL_RANKS.len() - 1].iter().rev() {
            high_count_rank = max(high_count_rank, (self.counts[*rank as usize], *rank));
        }

        if high_count_rank.0 == 4 {
            for rank in ALL_RANKS.iter().rev() {
                if *rank == high_count_rank.1 {
                    continue;
                }
                if self.counts[*rank as usize] > 0 {
                    return HandResult::FourOfAKind {
                        rank: high_count_rank.1,
                        kicker: *rank,
                    };
                }
            }
            panic!("Less than 5 cards were added to RankCounter");
        }

        if high_count_rank.0 == 3 {
            for rank in ALL_RANKS.iter().rev() {
                if *rank == high_count_rank.1 {
                    continue;
                }
                if self.counts[*rank as usize] >= 2 {
                    return HandResult::FullHouse {
                        trips_rank: high_count_rank.1,
                        pair_rank: *rank,
                    };
                }
            }

            let mut high_kicker: Option<Rank> = None;
            for rank in ALL_RANKS.iter().rev() {
                if *rank == high_count_rank.1 {
                    continue;
                }
                if self.counts[*rank as usize] > 0 {
                    match high_kicker {
                        Some(kicker) => {
                            return HandResult::ThreeOfAKind {
                                rank: high_count_rank.1,
                                kickers: [kicker, *rank],
                            };
                        }
                        None => high_kicker = Some(*rank),
                    }
                }
            }
            panic!("Less than 5 cards were added to RankCounter");
        }

        if high_count_rank.0 == 2 {
            let mut kicker_pair = None;
            for rank in ALL_RANKS.iter().rev() {
                if *rank == high_count_rank.1 {
                    continue;
                }
                if self.counts[*rank as usize] >= 2 {
                    kicker_pair = Some(*rank);
                    break;
                }
            }

            if let Some(low_rank) = kicker_pair {
                for rank in ALL_RANKS.iter().rev() {
                    if *rank == high_count_rank.1 || *rank == low_rank {
                        continue;
                    }
                    if self.counts[*rank as usize] > 0 {
                        return HandResult::TwoPair {
                            high_rank: high_count_rank.1,
                            low_rank,
                            kicker: *rank,
                        };
                    }
                }
            } else {
                let mut high_kicker = None;
                let mut mid_kicker = None;
                for rank in ALL_RANKS.iter().rev() {
                    if *rank == high_count_rank.1 {
                        continue;
                    }
                    if self.counts[*rank as usize] > 0 {
                        match (high_kicker, mid_kicker) {
                            (Some(high), Some(mid)) => {
                                return HandResult::Pair {
                                    rank: high_count_rank.1,
                                    kickers_sorted_desc: [high, mid, *rank],
                                };
                            }
                            (Some(_), None) => mid_kicker = Some(*rank),
                            _ => high_kicker = Some(*rank),
                        }
                    }
                }
            }
            panic!("Less than 5 cards were added to RankCounter");
        }

        let mut kickers: [Rank; 5] = [high_count_rank.1; 5];
        let mut kicker_count = 0;
        for rank in ALL_RANKS[0..high_count_rank.1 as usize].iter().rev() {
            if self.counts[*rank as usize] > 0 {
                kickers[kicker_count + 1] = *rank;
                kicker_count += 1;
            }
            if kicker_count == 4 {
                return HandResult::HighCard {
                    ranks_sorted_desc: kickers,
                };
            }
        }

        panic!("Less than 5 cards were added to RankCounter");
    }
}

impl FromIterator<Rank> for RankCounter {
    fn from_iter<T: IntoIterator<Item = Rank>>(iter: T) -> Self {
        let mut ret = Self::new();
        for rank in iter {
            ret.add(rank);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluation::util::frequency;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::collections::HashMap;

    #[test]
    fn test_high_card() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Seven);
        counter.add(Rank::Five);
        counter.add(Rank::Four);
        counter.add(Rank::Three);
        counter.add(Rank::Two);
        assert_eq!(
            counter.evaluate(),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
            }
        );
    }

    #[test]
    fn test_high_card_chooses_highest_cards() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Four);
        counter.add(Rank::Jack);
        counter.add(Rank::Six);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        counter.add(Rank::Eight);
        counter.add(Rank::Nine);
        assert_eq!(
            counter.evaluate(),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Jack, Rank::Nine, Rank::Eight]
            }
        );
    }

    #[test]
    fn test_pair() {
        let mut counter = RankCounter::new();
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Ace);
        counter.add(Rank::Queen);
        counter.add(Rank::Ten);
        assert_eq!(
            counter.evaluate(),
            HandResult::Pair {
                rank: Rank::King,
                kickers_sorted_desc: [Rank::Ace, Rank::Queen, Rank::Ten],
            }
        );
    }

    #[test]
    fn test_pair_chooses_highest_kickers() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Two);
        counter.add(Rank::Seven);
        counter.add(Rank::Two);
        counter.add(Rank::Nine);
        counter.add(Rank::Six);
        counter.add(Rank::Jack);
        assert_eq!(
            counter.evaluate(),
            HandResult::Pair {
                rank: Rank::Two,
                kickers_sorted_desc: [Rank::Ace, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_two_pair() {
        let mut counter = RankCounter::new();
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        counter.add(Rank::Ace);
        assert_eq!(
            counter.evaluate(),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_highest_kicker() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Two);
        counter.add(Rank::Jack);
        counter.add(Rank::Five);
        assert_eq!(
            counter.evaluate(),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_kicker_from_third_pair_if_its_the_highest() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Jack);
        counter.add(Rank::Jack);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        counter.add(Rank::Nine);
        assert_eq!(
            counter.evaluate(),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_highest_kicker_if_not_in_third_pair() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Jack);
        counter.add(Rank::Jack);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        counter.add(Rank::Ace);
        assert_eq!(
            counter.evaluate(),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        assert_eq!(
            counter.evaluate(),
            HandResult::ThreeOfAKind {
                rank: Rank::Ace,
                kickers: [Rank::King, Rank::Queen],
            }
        );
    }

    #[test]
    fn test_three_of_a_kind_chooses_highest_kickers() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Seven);
        counter.add(Rank::Seven);
        counter.add(Rank::Seven);
        counter.add(Rank::Jack);
        counter.add(Rank::Two);
        counter.add(Rank::Eight);
        counter.add(Rank::Six);
        assert_eq!(
            counter.evaluate(),
            HandResult::ThreeOfAKind {
                rank: Rank::Seven,
                kickers: [Rank::Jack, Rank::Eight],
            }
        );
    }

    #[test]
    fn test_full_house() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Nine);
        counter.add(Rank::Nine);
        counter.add(Rank::Nine);
        counter.add(Rank::King);
        counter.add(Rank::King);
        assert_eq!(
            counter.evaluate(),
            HandResult::FullHouse {
                trips_rank: Rank::Nine,
                pair_rank: Rank::King,
            }
        );
    }

    #[test]
    fn test_full_house_ignores_irrelevant_kickers() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Nine);
        counter.add(Rank::Nine);
        counter.add(Rank::Nine);
        counter.add(Rank::Ten);
        counter.add(Rank::Ten);
        counter.add(Rank::Jack);
        counter.add(Rank::King);
        counter.add(Rank::Six);
        counter.add(Rank::Eight);
        assert_eq!(
            counter.evaluate(),
            HandResult::FullHouse {
                trips_rank: Rank::Nine,
                pair_rank: Rank::Ten,
            }
        );
    }

    #[test]
    fn test_full_house_chooses_highest_pair() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        assert_eq!(
            counter.evaluate(),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King,
            }
        );
    }

    #[test]
    fn test_full_house_takes_from_other_trips_if_necessary() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        assert_eq!(
            counter.evaluate(),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King,
            }
        );
    }

    #[test]
    fn test_full_house_ignores_other_trips_when_lower_than_best_pair() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        counter.add(Rank::Queen);
        assert_eq!(
            counter.evaluate(),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King,
            }
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let mut counter = RankCounter::new();
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::Ace);
        counter.add(Rank::King);
        assert_eq!(
            counter.evaluate(),
            HandResult::FourOfAKind {
                rank: Rank::Ace,
                kicker: Rank::King,
            }
        );
    }

    #[test]
    fn test_four_of_a_kind_chooses_best_kicker() {
        let mut counter = RankCounter::new();
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::King);
        counter.add(Rank::Queen);
        counter.add(Rank::Ten);
        counter.add(Rank::Jack);
        assert_eq!(
            counter.evaluate(),
            HandResult::FourOfAKind {
                rank: Rank::King,
                kicker: Rank::Queen,
            }
        );
    }

    #[test]
    fn test_from_iterator() {
        let ranks = &[
            Rank::Queen,
            Rank::Queen,
            Rank::King,
            Rank::King,
            Rank::Two,
            Rank::Jack,
            Rank::Five,
        ];
        let counter = ranks.iter().cloned().collect::<RankCounter>();
        assert_eq!(
            counter.evaluate(),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[quickcheck]
    fn test_does_not_panic_on_valid_rank_sequence(ranks: Vec<Rank>) -> TestResult {
        if ranks.len() < 5 {
            return TestResult::discard();
        }
        let rank_counts = frequency(ranks.iter());
        if rank_counts.values().any(|&count| count > 4) {
            return TestResult::discard();
        }

        let counter = ranks.into_iter().collect::<RankCounter>();
        counter.evaluate();
        TestResult::passed()
    }
}
