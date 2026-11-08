use std::collections::HashMap;
use std::hash::Hash;

pub fn is_sorted_desc<'a, T: Hash + PartialOrd + 'static, I: IntoIterator<Item = &'a T>>(
    elements: I,
) -> bool {
    let mut iter = elements.into_iter();
    let mut previous = match iter.next() {
        Some(c) => c,
        None => return true,
    };
    for elem in iter {
        if previous < elem {
            return false;
        }
        previous = elem;
    }
    true
}

pub fn is_strictly_sorted_desc<
    'a,
    T: Hash + PartialOrd + 'static,
    I: IntoIterator<Item = &'a T>,
>(
    elements: I,
) -> bool {
    let mut iter = elements.into_iter();
    let mut previous = match iter.next() {
        Some(c) => c,
        None => return true,
    };
    for elem in iter {
        if previous <= elem {
            return false;
        }
        previous = elem;
    }
    true
}

pub fn frequency<'a, T: Eq + Hash + 'a, I: IntoIterator<Item = &'a T>>(
    elements: I,
) -> HashMap<&'a T, usize> {
    let mut ret = HashMap::new();

    for elem in elements {
        *ret.entry(elem).or_insert(0) += 1;
    }

    ret
}

pub fn combinations(total: usize, picked: usize) -> usize {
    if picked > total / 2 {
        return combinations(total, total - picked);
    }

    let numerator: usize = (total - picked + 1..=total).product();
    let denominator: usize = (2..=picked).product();
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted_desc() {
        assert!(is_sorted_desc([] as [&i32; 0]));
        assert!(is_sorted_desc([&7]));
        assert!(is_sorted_desc([&9, &7, &7, &3]));
        assert!(is_sorted_desc([&9, &7, &6, &3]));

        assert!(!is_sorted_desc([&1, &2]));
        assert!(!is_sorted_desc([&9, &7, &8]));
    }

    #[test]
    fn test_is_strictly_sorted_desc() {
        assert!(is_sorted_desc([] as [&i32; 0]));
        assert!(is_sorted_desc([&7]));
        assert!(is_sorted_desc([&9, &7, &6, &3]));

        assert!(is_sorted_desc([&9, &7, &7, &3]));
        assert!(!is_sorted_desc([&1, &2]));
        assert!(!is_sorted_desc([&9, &7, &8]));
    }

    #[test]
    fn test_frequency() {
        let values = [1, 2, 1, 3, 2, 1];
        let counts = frequency(values.iter());

        assert_eq!(counts, [(&1, 3), (&2, 2), (&3, 1)].into_iter().collect());
    }

    #[test]
    fn test_combinations() {
        assert_eq!(combinations(0, 0), 1);
        assert_eq!(combinations(8, 0), 1);
        assert_eq!(combinations(8, 8), 1);
        assert_eq!(combinations(8, 1), 8);
        assert_eq!(combinations(8, 7), 8);
        assert_eq!(combinations(8, 5), 56);
        assert_eq!(combinations(8, 3), 56);
        assert_eq!(combinations(52, 7), 133784560);
    }
}
