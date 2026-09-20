use crate::card::Card;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CardSet {
    bitset: u64,
}

impl CardSet {
    pub fn all() -> Self {
        Self {
            bitset: 0xFFFFFFFFFFFFF,
        }
    }

    pub fn disjoint_with(&self, other: CardSet) -> bool {
        *self & other == CardSet::empty()
    }

    pub fn empty() -> Self {
        Self { bitset: 0 }
    }

    pub fn contains(&self, card: Card) -> bool {
        self.bitset & (1 << card.to_index() as u64) > 0
    }

    pub fn iter(&self) -> CardSetIterator {
        CardSetIterator {
            bitset: self.bitset,
            shifted: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.bitset.count_ones() as usize
    }
}

impl Add<Card> for CardSet {
    type Output = Self;

    fn add(self, other: Card) -> Self {
        Self {
            bitset: self.bitset | (1 << other.to_index() as u64),
        }
    }
}

impl AddAssign<Card> for CardSet {
    fn add_assign(&mut self, other: Card) {
        *self = *self + other;
    }
}

impl BitAnd for CardSet {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self {
            bitset: self.bitset & other.bitset,
        }
    }
}

impl BitAndAssign for CardSet {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl BitOr for CardSet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            bitset: self.bitset | other.bitset,
        }
    }
}

impl BitOrAssign for CardSet {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl Sub for CardSet {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            bitset: self.bitset & !other.bitset,
        }
    }
}

impl Sub<Card> for CardSet {
    type Output = Self;

    fn sub(self, other: Card) -> Self {
        Self {
            bitset: self.bitset & !(1 << other.to_index() as u64),
        }
    }
}

impl SubAssign for CardSet {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<Card> for CardSet {
    fn sub_assign(&mut self, other: Card) {
        *self = *self - other;
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut set = Self::empty();
        for card in iter {
            set += card;
        }
        set
    }
}

#[derive(Debug)]
pub struct CardSetIterator {
    bitset: u64,
    shifted: usize,
}

impl Iterator for CardSetIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bitset & 1 == 0 {
            if self.bitset == 0 {
                return None;
            }
            self.bitset >>= 1;
            self.shifted += 1;
        }

        let card = Card::from_index(self.shifted);
        self.bitset >>= 1;
        self.shifted += 1;
        Some(card)
    }
}

#[cfg(test)]
struct CardSetShrinkIterator {
    cards: Vec<Card>,
    set: CardSet,
}

#[cfg(test)]
impl Iterator for CardSetShrinkIterator {
    type Item = CardSet;

    fn next(&mut self) -> Option<Self::Item> {
        self.set -= self.cards.pop()?;
        Some(self.set)
    }
}

#[cfg(test)]
impl Arbitrary for CardSet {
    fn arbitrary(g: &mut Gen) -> Self {
        let bitset: u64 = Arbitrary::arbitrary(g);
        Self {
            bitset: bitset & 0xFFFFFFFFFFFFF,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(CardSetShrinkIterator {
            cards: self.iter().collect(),
            set: *self,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{NUM_CARDS, card_vec};
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    #[test]
    fn test_all_cards_have_52() {
        let set = CardSet::all();
        assert_eq!(set.len(), NUM_CARDS);
    }

    #[test]
    fn test_empty_set_has_zero_len() {
        let set = CardSet::empty();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_add_to_empty_set() {
        let card = Card::parse("As").unwrap();
        let set = CardSet::empty() + card;

        assert_eq!(set.len(), 1);
        assert_eq!(set.contains(card), true);
    }

    #[test]
    fn test_add_is_idempotent() {
        let card = Card::parse("As").unwrap();
        let set = CardSet::empty() + card + card;

        assert_eq!(set.len(), 1);
        assert_eq!(set.contains(card), true);
    }

    #[test]
    fn test_addassign_to_empty_set() {
        let mut set = CardSet::empty();
        let card = Card::parse("As").unwrap();

        set += card;

        assert_eq!(set.len(), 1);
        assert_eq!(set.contains(card), true);
    }

    #[test]
    fn test_addassign_is_idempotent() {
        let mut set = CardSet::empty();
        let card = Card::parse("As").unwrap();

        set += card;
        set += card;

        assert_eq!(set.len(), 1);
        assert_eq!(set.contains(card), true);
    }

    #[test]
    fn test_addassign_all_52_cards() {
        let mut set = CardSet::empty();
        for i in 0..NUM_CARDS {
            set += Card::from_index(i);
        }
        assert_eq!(set, CardSet::all());
    }

    #[quickcheck]
    fn test_addassign_adds_card_quickcheck(mut cardset: CardSet, card: Card) -> TestResult {
        if cardset.contains(card) {
            return TestResult::discard();
        }

        let initial_len = cardset.len();

        cardset += card;
        assert!(cardset.contains(card));
        assert_eq!(cardset.len(), initial_len + 1);

        TestResult::passed()
    }

    #[test]
    fn test_bitand_returns_cardset_with_only_shared_cards() {
        let set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "Kh", "8s"])
            .into_iter()
            .collect::<CardSet>();
        let expected = card_vec(["Kh"]).into_iter().collect::<CardSet>();
        assert_eq!(set1 & set2, expected);
    }

    #[quickcheck]
    fn test_bitand_intersects_cards_quickcheck(set1: HashSet<Card>, set2: HashSet<Card>) {
        let cardset1 = set1.iter().cloned().collect::<CardSet>();
        let cardset2 = set2.iter().cloned().collect::<CardSet>();

        let actual_intersection = (cardset1 & cardset2).iter().collect::<HashSet<_>>();
        let expected_intersection = set1.intersection(&set2).cloned().collect::<HashSet<_>>();

        assert_eq!(actual_intersection, expected_intersection);
    }

    #[test]
    fn test_bitandassign_removes_all_but_shared_cards() {
        let mut set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "Kh", "8s"]).into_iter().collect::<CardSet>();
        set1 &= set2;
        let expected = card_vec(["Kh"]).into_iter().collect::<CardSet>();
        assert_eq!(set1, expected);
    }

    #[test]
    fn test_bitor_returns_cards_from_both() {
        let set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "Kh", "8s"])
            .into_iter()
            .collect::<CardSet>();
        let expected = card_vec(["As", "Kh", "Td", "8s"]).into_iter().collect::<CardSet>();
        assert_eq!(set1 | set2, expected);
    }

    #[quickcheck]
    fn test_bitor_unions_cards_quickcheck(set1: HashSet<Card>, set2: HashSet<Card>) {
        let cardset1 = set1.iter().cloned().collect::<CardSet>();
        let cardset2 = set2.iter().cloned().collect::<CardSet>();

        let actual_intersection = (cardset1 | cardset2).iter().collect::<HashSet<_>>();
        let expected_intersection = set1.union(&set2).cloned().collect::<HashSet<_>>();

        assert_eq!(actual_intersection, expected_intersection);
    }

    #[test]
    fn test_bitor_unions_cards() {
        let mut set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "Kh", "8s"]).into_iter().collect::<CardSet>();
        set1 |= set2;
        let expected = card_vec(["As", "Kh", "Td", "8s"]).into_iter().collect::<CardSet>();
        assert_eq!(set1, expected);
    }

    #[test]
    fn test_disjoint_with_returns_true_for_no_shared_cards() {
        let set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "9d", "8s"])
            .into_iter()
            .collect::<CardSet>();
        assert!(set1.disjoint_with(set2));
    }

    #[test]
    fn test_disjoint_with_returns_false_for_any_shared_card() {
        let set1 = card_vec(["As", "Kh"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["Td", "Kh", "8s"])
            .into_iter()
            .collect::<CardSet>();
        assert!(!set1.disjoint_with(set2));
    }

    #[quickcheck]
    fn test_disjoint_with_quickcheck(set1: HashSet<Card>, set2: HashSet<Card>) {
        let cardset1 = set1.iter().cloned().collect::<CardSet>();
        let cardset2 = set2.iter().cloned().collect::<CardSet>();

        assert_eq!(
            cardset1.disjoint_with(cardset2),
            set1.intersection(&set2).count() == 0
        );
    }

    #[test]
    fn test_from_iterator_adds_all_cards() {
        let cards = card_vec(["As", "Kh", "Tc", "9c", "8d"]);
        let cardset = cards.iter().cloned().collect::<CardSet>();

        for card in cards {
            assert!(cardset.contains(card));
        }
        assert_eq!(cardset.len(), 5);
    }

    #[quickcheck]
    fn test_from_to_iterator_is_bijective(cards: HashSet<Card>) {
        let cardset = cards.iter().cloned().collect::<CardSet>();
        let card_hashset: HashSet<_> = cardset.iter().collect();
        assert_eq!(card_hashset, cards);
    }

    #[test]
    fn test_iter_all_cards_have_52() {
        let set = CardSet::all();
        assert_eq!(set.iter().count(), NUM_CARDS);
    }

    #[test]
    fn test_iter_all_cards_has_no_duplicates() {
        let set = CardSet::all();
        let cards: HashSet<_> = set.iter().collect();
        assert_eq!(cards.len(), set.len());
    }

    #[test]
    fn test_iter_all_cards_sorted_ascending() {
        let set = CardSet::all();
        let cards: Vec<_> = set.iter().collect();
        let mut cards_sorted = cards.clone();
        cards_sorted.sort();

        assert_eq!(cards, cards_sorted);
    }

    #[test]
    fn test_sub_cardset_removes_cards_in_other_set() {
        let mut set1 = card_vec(["As", "Kh", "Qd"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["As", "Qd", "Th"]).into_iter().collect::<CardSet>();
        let expected = card_vec(["Kh"]).into_iter().collect::<CardSet>();
        assert_eq!(set1 - set2, expected);
    }

    #[test]
    fn test_subassign_cardset_removes_cards_in_other_set() {
        let mut set1 = card_vec(["As", "Kh", "Qd"]).into_iter().collect::<CardSet>();
        let set2 = card_vec(["As", "Qd", "Th"]).into_iter().collect::<CardSet>();
        set1 -= set2;
        let expected = card_vec(["Kh"]).into_iter().collect::<CardSet>();
        assert_eq!(set1, expected);
    }

    #[quickcheck]
    fn test_subassign_cardset_removes_all_from_other_set_quickcheck(set1: HashSet<Card>, set2: HashSet<Card>) {
        let mut cardset1 = set1.iter().cloned().collect::<CardSet>();
        let cardset2 = set2.iter().cloned().collect::<CardSet>();

        cardset1 -= cardset2;
        assert_eq!(cardset1, set1.difference(&set2).cloned().collect::<CardSet>());
    }

    #[test]
    fn test_sub_is_idempotent() {
        let card = Card::parse("As").unwrap();
        let set = CardSet::all() - card - card;
        assert!(!set.contains(card));
        assert_eq!(set.len(), NUM_CARDS - 1);
    }

    #[test]
    fn test_sub_removes_card() {
        let card = Card::parse("As").unwrap();
        let set = CardSet::all() - card;
        assert!(!set.contains(card));
        assert_eq!(set.len(), NUM_CARDS - 1);
    }

    #[test]
    fn test_subassign_removes_card() {
        let card = Card::parse("As").unwrap();
        let mut set = CardSet::all();
        set -= card;
        assert!(!set.contains(card));
        assert_eq!(set.len(), NUM_CARDS - 1);
    }

    #[quickcheck]
    fn test_subassign_removes_card_quickcheck(mut set: CardSet, card: Card) -> TestResult {
        if !set.contains(card) {
            return TestResult::discard();
        }

        set -= card;
        assert!(!set.contains(card));
        TestResult::passed()
    }
}
