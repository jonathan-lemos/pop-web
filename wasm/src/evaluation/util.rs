use std::collections::HashMap;
use std::hash::Hash;

pub fn is_sorted_desc<'a, T: Hash + PartialOrd + 'static, I: IntoIterator<Item = &'a T>>(
    elements: I,
) -> bool {
    let mut iter = elements.into_iter();
    let previous = match iter.next() {
        Some(c) => c,
        None => return true,
    };
    for elem in iter {
        if previous < elem {
            return false;
        }
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
    let previous = match iter.next() {
        Some(c) => c,
        None => return true,
    };
    for elem in iter {
        if previous <= elem {
            return false;
        }
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
