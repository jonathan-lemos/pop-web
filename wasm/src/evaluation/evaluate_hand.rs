use crate::card::{Card, Rank};
use crate::evaluation::flush_evaluator::FlushEvaluator;
use crate::evaluation::hand_result::HandResult;
use crate::evaluation::rank_counter::RankCounter;
use crate::evaluation::straight_evaluator::StraightEvaluator;
use crate::evaluation::straight_flush_evaluator::StraightFlushEvaluator;
use crate::evaluation::util::{is_sorted_desc, is_strictly_sorted_desc};

const MIN_EVALUATION_SIZE: usize = 5;
const MAX_EVALUATION_SIZE: usize = 7;

// The cards must be given in descending order of rank.
pub fn evaluate_hand<const EVALUATION_SIZE: usize>(cards: &[Card; EVALUATION_SIZE]) -> HandResult {
    const {
        assert!(EVALUATION_SIZE >= MIN_EVALUATION_SIZE);
        assert!(EVALUATION_SIZE <= MAX_EVALUATION_SIZE);
    }

    debug_assert!(is_strictly_sorted_desc(cards.iter()));

    let straight_flush_evaluator: StraightFlushEvaluator<EVALUATION_SIZE> =
        cards.iter().copied().collect();
    if let Some(rank) = straight_flush_evaluator.best_straight_flush_rank() {
        return HandResult::StraightFlush { highest_rank: rank };
    }

    let rank_counter: RankCounter = cards.iter().map(|c| c.rank()).collect();
    let rank_eval = rank_counter.evaluate();
    if let HandResult::FourOfAKind { rank: _, kicker: _ } = rank_eval {
        return rank_eval;
    }

    if let HandResult::FullHouse {
        trips_rank: _,
        pair_rank: _,
    } = rank_eval
    {
        return rank_eval;
    }

    let flush_evaluator: FlushEvaluator = cards.iter().copied().collect();
    if let Some(ranks) = flush_evaluator.best_flush_ranks() {
        return HandResult::Flush {
            ranks_sorted_desc: ranks,
        };
    }

    let straight_evaluator: StraightEvaluator<EVALUATION_SIZE> =
        cards.iter().map(|c| c.rank()).collect();

    if let Some(rank) = straight_evaluator.best_straight_high_rank() {
        return HandResult::Straight { highest_rank: rank };
    }

    rank_eval
}

pub fn evaluate_hand_suitless<const EVALUATION_SIZE: usize>(
    ranks: &[Rank; EVALUATION_SIZE],
) -> HandResult {
    const {
        assert!(EVALUATION_SIZE >= MIN_EVALUATION_SIZE);
        assert!(EVALUATION_SIZE <= MAX_EVALUATION_SIZE);
    }

    debug_assert!(is_sorted_desc(ranks.iter()));

    let rank_counter: RankCounter = ranks.iter().copied().collect();
    let rank_eval = rank_counter.evaluate();
    if let HandResult::FourOfAKind { rank: _, kicker: _ } = rank_eval {
        return rank_eval;
    }

    if let HandResult::FullHouse {
        trips_rank: _,
        pair_rank: _,
    } = rank_eval
    {
        return rank_eval;
    }

    let straight_evaluator: StraightEvaluator<EVALUATION_SIZE> = ranks.iter().copied().collect();

    if let Some(rank) = straight_evaluator.best_straight_high_rank() {
        return HandResult::Straight { highest_rank: rank };
    }

    rank_eval
}

#[cfg(test)]
mod evaluate_hand_tests {
    use super::*;
    use crate::card::{Rank, card_array};
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    #[test]
    fn test_straight_flush() {
        let hand = card_array(["As", "Ks", "Qs", "Js", "Ts"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::StraightFlush {
                highest_rank: Rank::Ace
            }
        );
    }

    #[test]
    fn test_straight_flush_a2345() {
        let hand = card_array(["As", "5s", "4s", "3s", "2s"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::StraightFlush {
                highest_rank: Rank::Five
            }
        );
    }

    #[test]
    fn test_straight_flush_seven_cards() {
        let hand = card_array(["Kd", "Qd", "Jd", "Td", "9d", "8d", "7d"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::StraightFlush {
                highest_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_straight_flush_ignores_offsuit_cards() {
        let hand = card_array(["As", "Kd", "Qd", "Jd", "Td", "9d", "8h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::StraightFlush {
                highest_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = card_array(["As", "Ah", "Ad", "Ac", "Kd"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FourOfAKind {
                rank: Rank::Ace,
                kicker: Rank::King
            }
        );
    }

    #[test]
    fn test_four_of_a_kind_chooses_best_kicker() {
        let hand = card_array(["Ks", "Kh", "Kd", "Kc", "Qh", "Jd"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FourOfAKind {
                rank: Rank::King,
                kicker: Rank::Queen
            }
        );
    }

    #[test]
    fn test_full_house() {
        let hand = card_array(["As", "Ah", "Ad", "Ks", "Kh"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_full_house_ignores_irrelevant_card() {
        let hand = card_array(["Ac", "Ks", "Kh", "Kd", "Qs", "Qh"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::King,
                pair_rank: Rank::Queen
            }
        );
    }

    #[test]
    fn test_full_house_two_trips() {
        let hand = card_array(["As", "Ah", "Ad", "Ks", "Kh", "Kd", "Qc"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_full_house_trips_and_two_pairs() {
        let hand = card_array(["Js", "Jh", "Jd", "Ts", "Th", "9s", "9h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Jack,
                pair_rank: Rank::Ten
            }
        );
    }

    #[test]
    fn test_flush() {
        let hand = card_array(["Ah", "Jh", "8h", "6h", "3h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Flush {
                ranks_sorted_desc: [Rank::Ace, Rank::Jack, Rank::Eight, Rank::Six, Rank::Three],
            }
        );
    }

    #[test]
    fn test_flush_chooses_best_five() {
        let hand = card_array(["As", "Ks", "Js", "8s", "7s", "2s"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Flush {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Jack, Rank::Eight, Rank::Seven],
            }
        );
    }

    #[test]
    fn test_flush_ignores_offsuit_cards() {
        let hand = card_array(["Ah", "Kd", "Qd", "9d", "6d", "4d", "3c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Flush {
                ranks_sorted_desc: [Rank::King, Rank::Queen, Rank::Nine, Rank::Six, Rank::Four],
            }
        );
    }

    #[test]
    fn test_straight() {
        let hand = card_array(["As", "Kd", "Qh", "Jc", "Ts"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Straight {
                highest_rank: Rank::Ace
            }
        );
    }

    #[test]
    fn test_straight_a2345() {
        let hand = card_array(["As", "5d", "4h", "3s", "2c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Straight {
                highest_rank: Rank::Five
            }
        );
    }

    #[test]
    fn test_straight_chooses_highest() {
        let hand = card_array(["8s", "7d", "6h", "5c", "4s", "3d"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Straight {
                highest_rank: Rank::Eight
            }
        );
    }

    #[test]
    fn test_straight_ignores_duplicates() {
        let hand = card_array(["9s", "8c", "8d", "7h", "6s", "5c", "2d"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Straight {
                highest_rank: Rank::Nine
            }
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = card_array(["As", "Ah", "Ad", "Kc", "Qd"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ace,
                kickers: [Rank::King, Rank::Queen]
            }
        );
    }

    #[test]
    fn test_three_of_a_kind_chooses_best_kickers() {
        let hand = card_array(["Ac", "Ts", "Th", "Td", "8s", "4d"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ten,
                kickers: [Rank::Ace, Rank::Eight]
            }
        );
    }

    #[test]
    fn test_three_of_a_kind_seven_cards_chooses_best_kickers() {
        let hand = card_array(["Ac", "Ts", "Th", "Td", "8s", "5h", "4d"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ten,
                kickers: [Rank::Ace, Rank::Eight]
            }
        );
    }

    #[test]
    fn test_two_pair() {
        let hand = card_array(["As", "Ah", "Ks", "Kh", "Qd"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::TwoPair {
                high_rank: Rank::Ace,
                low_rank: Rank::King,
                kicker: Rank::Queen,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_best_kicker() {
        let hand = card_array(["Ks", "Kh", "Qs", "Qh", "Jd", "4c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_seven_cards_chooses_best_kicker() {
        let hand = card_array(["Ks", "Kh", "Qs", "Qh", "Jd", "Ts", "4c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_takes_kicker_from_third_pair() {
        let hand = card_array(["As", "Ah", "Ts", "Th", "5s", "5h", "2c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::TwoPair {
                high_rank: Rank::Ace,
                low_rank: Rank::Ten,
                kicker: Rank::Five,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_highest_kicker_when_three_pairs_are_present() {
        let hand = card_array(["Ac", "Ks", "Kh", "Qs", "Qh", "2s", "2h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_pair() {
        let hand = card_array(["As", "Ah", "Kd", "Qc", "Js"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Pair {
                rank: Rank::Ace,
                kickers_sorted_desc: [Rank::King, Rank::Queen, Rank::Jack],
            }
        );
    }

    #[test]
    fn test_pair_chooses_top_three_kickers() {
        let hand = card_array(["Ac", "Kd", "Qh", "9c", "4s", "2s", "2h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Pair {
                rank: Rank::Two,
                kickers_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen],
            }
        );
    }

    #[test]
    fn test_high_card() {
        let hand = card_array(["As", "Kd", "Qh", "Jc", "9s"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_high_card_chooses_top_five() {
        let hand = card_array(["As", "Kd", "Qh", "Jc", "9s", "2c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_high_card_seven_cards_chooses_top_five() {
        let hand = card_array(["As", "Kd", "Qh", "Jc", "9s", "6d", "2c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_straight_flush_precedence_over_straight_and_flush() {
        let sorted_hand = card_array(["Js", "Th", "9s", "8s", "7s", "6s", "5s"]);
        assert_eq!(
            evaluate_hand(&sorted_hand),
            HandResult::StraightFlush {
                highest_rank: Rank::Nine
            }
        );
    }

    #[test]
    fn test_four_of_a_kind_precedence_over_full_house() {
        let hand = card_array(["Ks", "Kh", "Kd", "9s", "9h", "9d", "9c"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::FourOfAKind {
                rank: Rank::Nine,
                kicker: Rank::King
            }
        );
    }

    #[test]
    fn test_flush_precedence_over_straight() {
        let hand = card_array(["As", "Kh", "Qh", "Jh", "Th", "9s", "8h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Flush {
                ranks_sorted_desc: [Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Eight],
            }
        );
    }

    #[test]
    fn test_straight_precedence_over_three_of_a_kind() {
        let hand = card_array(["9s", "9h", "9d", "8c", "7s", "6d", "5h"]);
        assert_eq!(
            evaluate_hand(&hand),
            HandResult::Straight {
                highest_rank: Rank::Nine
            }
        );
    }

    #[quickcheck]
    fn test_evaluate_hand_does_not_panic_on_valid_five_cards(cards: Vec<Card>) -> TestResult {
        let mut unique_cards: Vec<Card> = cards
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if unique_cards.len() < 5 {
            return TestResult::discard();
        }
        unique_cards.truncate(5);
        unique_cards.sort_by(|a, b| b.cmp(a));
        let hand: [Card; 5] = unique_cards.try_into().unwrap();
        let _ = evaluate_hand(&hand);
        TestResult::passed()
    }

    #[quickcheck]
    fn test_evaluate_hand_does_not_panic_on_valid_six_cards(cards: Vec<Card>) -> TestResult {
        let mut unique_cards: Vec<Card> = cards
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if unique_cards.len() < 6 {
            return TestResult::discard();
        }
        unique_cards.truncate(6);
        unique_cards.sort_by(|a, b| b.cmp(a));
        let hand: [Card; 6] = unique_cards.try_into().unwrap();
        let _ = evaluate_hand(&hand);
        TestResult::passed()
    }

    #[quickcheck]
    fn test_evaluate_hand_does_not_panic_on_valid_seven_cards(cards: Vec<Card>) -> TestResult {
        let mut unique_cards: Vec<Card> = cards
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if unique_cards.len() < 7 {
            return TestResult::discard();
        }
        unique_cards.truncate(7);
        unique_cards.sort_by(|a, b| b.cmp(a));
        let hand: [Card; 7] = unique_cards.try_into().unwrap();
        let _ = evaluate_hand(&hand);
        TestResult::passed()
    }
}

#[cfg(test)]
mod evaluate_hand_suitless_tests {
    use super::*;
    use crate::card::Rank;
    use crate::evaluation::util::frequency;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_four_of_a_kind() {
        let hand = [Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::King];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FourOfAKind {
                rank: Rank::Ace,
                kicker: Rank::King
            }
        );
    }

    #[test]
    fn test_four_of_a_kind_chooses_best_kicker() {
        let hand = [
            Rank::King,
            Rank::King,
            Rank::King,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FourOfAKind {
                rank: Rank::King,
                kicker: Rank::Queen
            }
        );
    }

    #[test]
    fn test_full_house() {
        let hand = [Rank::Ace, Rank::Ace, Rank::Ace, Rank::King, Rank::King];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_full_house_ignores_irrelevant_card() {
        let hand = [
            Rank::Ace,
            Rank::King,
            Rank::King,
            Rank::King,
            Rank::Queen,
            Rank::Queen,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::King,
                pair_rank: Rank::Queen
            }
        );
    }

    #[test]
    fn test_full_house_two_trips() {
        let hand = [
            Rank::Ace,
            Rank::Ace,
            Rank::Ace,
            Rank::King,
            Rank::King,
            Rank::King,
            Rank::Queen,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Ace,
                pair_rank: Rank::King
            }
        );
    }

    #[test]
    fn test_full_house_trips_and_two_pairs() {
        let hand = [
            Rank::Jack,
            Rank::Jack,
            Rank::Jack,
            Rank::Ten,
            Rank::Ten,
            Rank::Nine,
            Rank::Nine,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FullHouse {
                trips_rank: Rank::Jack,
                pair_rank: Rank::Ten
            }
        );
    }

    #[test]
    fn test_straight() {
        let hand = [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Straight {
                highest_rank: Rank::Ace
            }
        );
    }

    #[test]
    fn test_straight_a2345() {
        let hand = [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Straight {
                highest_rank: Rank::Five
            }
        );
    }

    #[test]
    fn test_straight_chooses_highest() {
        let hand = [
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Straight {
                highest_rank: Rank::Eight
            }
        );
    }

    #[test]
    fn test_straight_ignores_duplicates() {
        let hand = [
            Rank::Nine,
            Rank::Eight,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Straight {
                highest_rank: Rank::Nine
            }
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = [Rank::Ace, Rank::Ace, Rank::Ace, Rank::King, Rank::Queen];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ace,
                kickers: [Rank::King, Rank::Queen]
            }
        );
    }

    #[test]
    fn test_three_of_a_kind_chooses_best_kickers() {
        let hand = [
            Rank::Ace,
            Rank::Ten,
            Rank::Ten,
            Rank::Ten,
            Rank::Eight,
            Rank::Four,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ten,
                kickers: [Rank::Ace, Rank::Eight]
            }
        );
    }

    #[test]
    fn test_three_of_a_kind_seven_cards_chooses_best_kickers() {
        let hand = [
            Rank::Ace,
            Rank::Ten,
            Rank::Ten,
            Rank::Ten,
            Rank::Eight,
            Rank::Five,
            Rank::Four,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::ThreeOfAKind {
                rank: Rank::Ten,
                kickers: [Rank::Ace, Rank::Eight]
            }
        );
    }

    #[test]
    fn test_two_pair() {
        let hand = [Rank::Ace, Rank::Ace, Rank::King, Rank::King, Rank::Queen];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::TwoPair {
                high_rank: Rank::Ace,
                low_rank: Rank::King,
                kicker: Rank::Queen,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_best_kicker() {
        let hand = [
            Rank::King,
            Rank::King,
            Rank::Queen,
            Rank::Queen,
            Rank::Jack,
            Rank::Four,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_seven_cards_chooses_best_kicker() {
        let hand = [
            Rank::King,
            Rank::King,
            Rank::Queen,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Four,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_two_pair_takes_kicker_from_third_pair() {
        let hand = [
            Rank::Ace,
            Rank::Ace,
            Rank::Ten,
            Rank::Ten,
            Rank::Five,
            Rank::Five,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::TwoPair {
                high_rank: Rank::Ace,
                low_rank: Rank::Ten,
                kicker: Rank::Five,
            }
        );
    }

    #[test]
    fn test_two_pair_chooses_highest_kicker_when_three_pairs_are_present() {
        let hand = [
            Rank::Ace,
            Rank::King,
            Rank::King,
            Rank::Queen,
            Rank::Queen,
            Rank::Two,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::TwoPair {
                high_rank: Rank::King,
                low_rank: Rank::Queen,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_pair() {
        let hand = [Rank::Ace, Rank::Ace, Rank::King, Rank::Queen, Rank::Jack];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Pair {
                rank: Rank::Ace,
                kickers_sorted_desc: [Rank::King, Rank::Queen, Rank::Jack],
            }
        );
    }

    #[test]
    fn test_pair_chooses_top_three_kickers() {
        let hand = [
            Rank::Ace,
            Rank::King,
            Rank::Queen,
            Rank::Nine,
            Rank::Four,
            Rank::Two,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Pair {
                rank: Rank::Two,
                kickers_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen],
            }
        );
    }

    #[test]
    fn test_high_card() {
        let hand = [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_high_card_chooses_top_five() {
        let hand = [
            Rank::Ace,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Nine,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_high_card_seven_cards_chooses_top_five() {
        let hand = [
            Rank::Ace,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Nine,
            Rank::Six,
            Rank::Two,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::HighCard {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine],
            }
        );
    }

    #[test]
    fn test_four_of_a_kind_precedence_over_full_house() {
        let hand = [
            Rank::King,
            Rank::King,
            Rank::King,
            Rank::Nine,
            Rank::Nine,
            Rank::Nine,
            Rank::Nine,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::FourOfAKind {
                rank: Rank::Nine,
                kicker: Rank::King
            }
        );
    }

    #[test]
    fn test_straight_precedence_over_three_of_a_kind() {
        let hand = [
            Rank::Nine,
            Rank::Nine,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
        ];
        assert_eq!(
            evaluate_hand_suitless(&hand),
            HandResult::Straight {
                highest_rank: Rank::Nine
            }
        );
    }

    #[quickcheck]
    fn test_evaluate_hand_suitless_does_not_panic_on_valid_five_ranks(
        mut ranks: Vec<Rank>,
    ) -> TestResult {
        if ranks.len() < 5 {
            return TestResult::discard();
        }
        ranks.truncate(5);
        let rank_counts = frequency(ranks.iter());
        if rank_counts.values().any(|&count| count > 4) {
            return TestResult::discard();
        }
        ranks.sort_by(|a, b| b.cmp(a));
        let hand: [Rank; 5] = ranks.try_into().unwrap();
        let _ = evaluate_hand_suitless(&hand);
        TestResult::passed()
    }

    #[quickcheck]
    fn test_evaluate_hand_suitless_does_not_panic_on_valid_six_ranks(
        mut ranks: Vec<Rank>,
    ) -> TestResult {
        if ranks.len() < 6 {
            return TestResult::discard();
        }
        ranks.truncate(6);
        let rank_counts = frequency(ranks.iter());
        if rank_counts.values().any(|&count| count > 4) {
            return TestResult::discard();
        }
        ranks.sort_by(|a, b| b.cmp(a));
        let hand: [Rank; 6] = ranks.try_into().unwrap();
        let _ = evaluate_hand_suitless(&hand);
        TestResult::passed()
    }

    #[quickcheck]
    fn test_evaluate_hand_suitless_does_not_panic_on_valid_seven_ranks(
        mut ranks: Vec<Rank>,
    ) -> TestResult {
        if ranks.len() < 7 {
            return TestResult::discard();
        }
        ranks.truncate(7);
        let rank_counts = frequency(ranks.iter());
        if rank_counts.values().any(|&count| count > 4) {
            return TestResult::discard();
        }
        ranks.sort_by(|a, b| b.cmp(a));
        let hand: [Rank; 7] = ranks.try_into().unwrap();
        let _ = evaluate_hand_suitless(&hand);
        TestResult::passed()
    }
}
