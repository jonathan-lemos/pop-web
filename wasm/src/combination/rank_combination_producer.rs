use std::cmp::min;
use crate::card::{Rank, ALL_RANKS};
use crate::combination::combination_producer::CombinationProducer;
use crate::hand::rank_counter::RankCounter;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use crate::hand::util::combinations;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RankCounterCount {
    rank_counter: RankCounter,
    occurrences: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RankCombinationProducer<const COMBINATION_LEN: usize> {
    available_ranks: RankCounter,
    bias: RankCounter,
}

impl<const COMBINATION_LEN: usize> RankCombinationProducer<COMBINATION_LEN> {
    pub fn new(available_ranks: RankCounter) -> Self {
        Self {
            available_ranks,
            bias: RankCounter::new(),
        }
    }
}

impl<const COMBINATION_LEN: usize> RankCombinationProducer<COMBINATION_LEN> {
    fn forall_recursive<F: FnMut(&RankCounterCount)>(f: &mut F, needed: u8, current_rank: Rank, available_count: &RankCounter, current_count: &mut RankCounterCount) {
        if needed == 0 {
            f(current_count);
            return;
        }

        for i in 0..=min(available_count[current_rank], needed) {
            let combs = combinations(available_count[current_rank] as usize, i as usize);
            current_count.occurrences *= combs;
            current_count.rank_counter[current_rank] += i;

            if i == needed {
                f(current_count);
            } else {
                if current_rank != Rank::Ace {
                    let next_rank = ALL_RANKS[(current_rank as u8 + 1) as usize];
                    Self::forall_recursive(f, needed - i, next_rank, available_count, current_count);
                }
            }

            current_count.occurrences /= combs;
            current_count.rank_counter[current_rank] -= i;
        }
    }
}

impl<const COMBINATION_LEN: usize> CombinationProducer<RankCounterCount> for RankCombinationProducer<COMBINATION_LEN> {
    fn bisect(self) -> Result<(Self, Self), Self>
    where
        Self: Sized
    {
        // An odd rank can be split evenly
        // 1 -> {0}, {1}
        // 3 -> {0, 1}, {2, 3}
        for rank in ALL_RANKS {
            let available = self.available_ranks[rank];
            if available % 2 == 1 {
                let new_available = available / 2;
                let mut high_bias = self.bias;
                high_bias[rank] += (available / 2) + 1;

                let mut new_ranks = self.available_ranks;
                new_ranks[rank] = new_available;

                let low_combination_producer = Self {
                    available_ranks: new_ranks,
                    bias: self.bias
                };
                let high_combination_producer = Self {
                    available_ranks: new_ranks,
                    bias: high_bias
                };

                return Ok((low_combination_producer, high_combination_producer));
            }
        }

        // 4 delivers the next best split
        // 4 -> {0, 1, 2}, {3, 4}
        for rank in ALL_RANKS {
            if self.available_ranks[rank] == 4 {
                let mut low_ranks = self.available_ranks;
                low_ranks[rank] = 2;

                let mut high_ranks = self.available_ranks;
                high_ranks[rank] = 1;

                let mut high_bias = self.bias;
                high_bias[rank] += 3;

                let low_combination_producer = Self {
                    available_ranks: low_ranks,
                    bias: self.bias
                };
                let high_combination_producer = Self {
                    available_ranks: high_ranks,
                    bias: high_bias
                };

                return Ok((low_combination_producer, high_combination_producer));
            }
        }

        // 2 delivers a poor split, but is the only remaining option
        // 2 -> {0, 1}, {2}
        for rank in ALL_RANKS {
            if self.available_ranks[rank] == 2 {
                let mut low_ranks = self.available_ranks;
                low_ranks[rank] = 1;

                let mut high_ranks = self.available_ranks;
                high_ranks[rank] = 0;

                let mut high_bias = self.bias;
                high_bias[rank] += 2;

                let low_combination_producer = Self {
                    available_ranks: low_ranks,
                    bias: self.bias
                };
                let high_combination_producer = Self {
                    available_ranks: high_ranks,
                    bias: high_bias
                };

                return Ok((low_combination_producer, high_combination_producer));
            }
        }

        // All the available ranks are 0, so we cannot split
        Err(self)
    }

    fn forall<F: FnMut(&RankCounterCount)>(&self, mut f: F) {
        let mut rank_counter_count = RankCounterCount {
            rank_counter: self.bias,
            occurrences: 1,
        };

        Self::forall_recursive(&mut f, COMBINATION_LEN as u8, Rank::Two, &self.available_ranks, &mut rank_counter_count);
    }
}

#[cfg(test)]
impl<const COMBINATION_LEN: usize> Arbitrary for RankCombinationProducer<COMBINATION_LEN> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(RankCounter::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;

    fn rank_counter<I: IntoIterator<Item = (Rank, u8)>>(iterator: I) -> RankCounter {
        let mut counter = RankCounter::new();
        for (rank, count) in iterator.into_iter() {
            counter[rank] = count;
        }
        counter
    }

    fn rank_combination_producer<const COMBINATION_LEN: usize, I: IntoIterator<Item = (Rank, u8)>>(iterator: I) -> RankCombinationProducer<COMBINATION_LEN> {
        RankCombinationProducer::new(rank_counter(iterator))
    }

    fn forall_to_vec<const COMBINATION_LEN: usize>(producer: &RankCombinationProducer<COMBINATION_LEN>) -> Vec<RankCounterCount> {
        let mut vec = Vec::new();
        producer.forall(|rank_counter_count| {
            vec.push(*rank_counter_count);
        });
        vec
    }

    #[test]
    fn test_empty_producer_produces_no_combinations() {
        let producer: RankCombinationProducer<5> = rank_combination_producer([]);
        let vec = forall_to_vec(&producer);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_producer_of_5_combs_with_5_ranks_produces_single_combination() {
        let producer: RankCombinationProducer<5> = rank_combination_producer([(Rank::Two, 1), (Rank::Three, 1), (Rank::Four, 1), (Rank::Five, 1), (Rank::Six, 1)]);
        let actual = forall_to_vec(&producer);

        let expected = vec![
            RankCounterCount {
                rank_counter: rank_counter([(Rank::Two, 1), (Rank::Three, 1), (Rank::Four, 1), (Rank::Five, 1), (Rank::Six, 1)]),
                occurrences: 1,
            }
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_producer_produces_each_possibility_with_multiple_of_a_single_rank() {
        let producer: RankCombinationProducer<4> = rank_combination_producer([(Rank::Two, 4), (Rank::Three, 1), (Rank::Four, 1)]);
        let actual = forall_to_vec(&producer);

        let expected = vec![
            RankCounterCount {
                rank_counter: rank_counter([(Rank::Two, 2), (Rank::Three, 1), (Rank::Four, 1)]),
                occurrences: 6,
            },
            RankCounterCount {
                rank_counter: rank_counter([(Rank::Two, 3), (Rank::Three, 1)]),
                occurrences: 4,
            },
            RankCounterCount {
                rank_counter: rank_counter([(Rank::Two, 3), (Rank::Four, 1)]),
                occurrences: 4,
            },
            RankCounterCount {
                rank_counter: rank_counter([(Rank::Two, 4)]),
                occurrences: 1,
            },
        ];

        assert_eq_unordered!(actual, expected);
    }
}
