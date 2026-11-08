use crate::card::{ALL_RANKS, Rank};
use crate::hand::hand_result::HandResult;
use std::cmp::max;
use std::ops::{Index, IndexMut};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RankCounter {
    counts: [u8; ALL_RANKS.len()],
}

impl RankCounter {
    pub fn new() -> Self {
        Self {
            counts: [0; ALL_RANKS.len()],
        }
    }

    pub fn full() -> Self {
        Self {
            counts: [4; ALL_RANKS.len()],
        }
    }

    // Evaluates the best count-based hand based on the given ranks.
    //
    // Will not output Straight, Flush, or StraightFlush.
    // You must have at least 5 ranks in the RankCounter before calling this function.
    pub(super) fn evaluate(&self) -> HandResult {
        let mut high_count_rank = (self[Rank::Ace], Rank::Ace);
        for rank in ALL_RANKS[0..ALL_RANKS.len() - 1].iter().rev() {
            high_count_rank = max(high_count_rank, (self[*rank], *rank));
        }
        debug_assert!(
            high_count_rank.0 <= 4,
            "Cannot have more than 4 of a single rank"
        );

        if high_count_rank.0 == 4 {
            for rank in ALL_RANKS.iter().rev() {
                if *rank == high_count_rank.1 {
                    continue;
                }
                if self[*rank] > 0 {
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
                if self[*rank] >= 2 {
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
                if self[*rank] > 0 {
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
                if self[*rank] >= 2 {
                    kicker_pair = Some(*rank);
                    break;
                }
            }

            if let Some(low_rank) = kicker_pair {
                for rank in ALL_RANKS.iter().rev() {
                    if *rank == high_count_rank.1 || *rank == low_rank {
                        continue;
                    }
                    if self[*rank] > 0 {
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
                    if self[*rank] > 0 {
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
            if self[*rank] > 0 {
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
            ret[rank] += 1;
        }
        ret
    }
}

impl Index<Rank> for RankCounter {
    type Output = u8;

    fn index(&self, rank: Rank) -> &Self::Output {
        debug_assert!(
            self.counts[rank as usize] <= 4,
            "Cannot have more than 4 of a single rank"
        );
        &self.counts[rank as usize]
    }
}

impl IndexMut<Rank> for RankCounter {
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self.counts[rank as usize]
    }
}

#[cfg(test)]
impl Arbitrary for RankCounter {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut ret = Self::new();
        for rank in ALL_RANKS {
            ret[rank] = u8::arbitrary(g) % 4;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::util::frequency;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_high_card() {
        let mut counter = RankCounter::new();
        counter[Rank::Seven] = 1;
        counter[Rank::Five] = 1;
        counter[Rank::Four] = 1;
        counter[Rank::Three] = 1;
        counter[Rank::Two] = 1;
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
        counter[Rank::Four] = 1;
        counter[Rank::Jack] = 1;
        counter[Rank::Six] = 1;
        counter[Rank::Ace] = 1;
        counter[Rank::King] = 1;
        counter[Rank::Eight] = 1;
        counter[Rank::Nine] = 1;
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
        counter[Rank::King] = 2;
        counter[Rank::Ace] = 1;
        counter[Rank::Queen] = 1;
        counter[Rank::Ten] = 1;
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
        counter[Rank::Ace] = 1;
        counter[Rank::Two] = 2;
        counter[Rank::Seven] = 1;
        counter[Rank::Nine] = 1;
        counter[Rank::Six] = 1;
        counter[Rank::Jack] = 1;
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
        counter[Rank::King] = 2;
        counter[Rank::Queen] = 2;
        counter[Rank::Ace] = 1;
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
        counter[Rank::Queen] = 2;
        counter[Rank::King] = 2;
        counter[Rank::Two] = 1;
        counter[Rank::Jack] = 1;
        counter[Rank::Five] = 1;
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
        counter[Rank::Jack] = 2;
        counter[Rank::King] = 2;
        counter[Rank::Queen] = 2;
        counter[Rank::Nine] = 1;
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
        counter[Rank::Jack] = 2;
        counter[Rank::King] = 2;
        counter[Rank::Queen] = 2;
        counter[Rank::Ace] = 1;
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
        counter[Rank::Ace] = 3;
        counter[Rank::King] = 1;
        counter[Rank::Queen] = 1;
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
        counter[Rank::Seven] = 3;
        counter[Rank::Jack] = 1;
        counter[Rank::Two] = 1;
        counter[Rank::Eight] = 1;
        counter[Rank::Six] = 1;
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
        counter[Rank::Nine] = 3;
        counter[Rank::King] = 2;
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
        counter[Rank::Nine] = 3;
        counter[Rank::Ten] = 2;
        counter[Rank::Jack] = 1;
        counter[Rank::King] = 1;
        counter[Rank::Six] = 1;
        counter[Rank::Eight] = 1;
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
        counter[Rank::Ace] = 3;
        counter[Rank::King] = 2;
        counter[Rank::Queen] = 2;
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
        counter[Rank::Ace] = 3;
        counter[Rank::King] = 3;
        counter[Rank::Queen] = 2;
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
        counter[Rank::Ace] = 3;
        counter[Rank::King] = 2;
        counter[Rank::Queen] = 3;
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
        counter[Rank::Ace] = 4;
        counter[Rank::King] = 1;
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
        counter[Rank::King] = 4;
        counter[Rank::Queen] = 1;
        counter[Rank::Ten] = 1;
        counter[Rank::Jack] = 1;
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

    #[test]
    fn test_add_subtract() {
        let ranks = &[
            Rank::Queen,
            Rank::Queen,
            Rank::King,
            Rank::King,
            Rank::Two,
            Rank::Jack,
            Rank::Five,
        ];
        let mut counter = ranks.iter().cloned().collect::<RankCounter>();
        counter[Rank::Queen] += 1;
        assert_eq!(counter[Rank::Queen], 3);
        counter[Rank::King] -= 2;
        assert_eq!(counter[Rank::King], 0);
    }

    #[test]
    fn test_get() {
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
        assert_eq!(counter[Rank::Two], 1);
        assert_eq!(counter[Rank::Three], 0);
        assert_eq!(counter[Rank::Queen], 2);
        assert_eq!(counter[Rank::King], 2);
    }

    #[test]
    fn test_remove() {
        let ranks = &[
            Rank::Queen,
            Rank::Queen,
            Rank::King,
            Rank::King,
            Rank::Two,
            Rank::Jack,
            Rank::Five,
        ];
        let mut counter = ranks.iter().cloned().collect::<RankCounter>();

        counter[Rank::Queen] -= 1;
        assert_eq!(counter[Rank::Queen], 1);
        counter[Rank::Two] -= 1;
        assert_eq!(counter[Rank::Two], 0);
    }

    #[test]
    fn test_full() {
        let counter = RankCounter::full();
        for rank in ALL_RANKS {
            assert_eq!(counter[rank], 4);
        }
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
