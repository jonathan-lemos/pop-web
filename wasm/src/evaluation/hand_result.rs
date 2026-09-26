use crate::card::Rank;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandResult {
    HighCard {
        ranks_sorted_desc: [Rank; 5],
    },
    Pair {
        rank: Rank,
        kickers_sorted_desc: [Rank; 3],
    },
    TwoPair {
        high_rank: Rank,
        low_rank: Rank,
        kicker: Rank,
    },
    ThreeOfAKind {
        rank: Rank,
        kickers: [Rank; 2],
    },
    Straight {
        highest_rank: Rank,
    },
    Flush {
        ranks_sorted_desc: [Rank; 5],
    },
    FullHouse {
        trips_rank: Rank,
        pair_rank: Rank,
    },
    FourOfAKind {
        rank: Rank,
        kicker: Rank,
    },
    StraightFlush {
        highest_rank: Rank,
    },
}
