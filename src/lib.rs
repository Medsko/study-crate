mod collections;

use collections::vectors;
use std::collections::HashMap;
use std::hash::Hash;

// This file is only here to make some typical integration tests

pub fn find_median(vec: &Vec<i32>) -> i32 {
    return vectors::find_median(vec);
}

pub fn find_mode(vec: &Vec<i32>) -> Option<i32> {
    return vectors::find_mode(vec);
}

pub fn find_mode_generic<T: Eq + Hash>(vec: &Vec<T>) -> Option<&T> {
    let mut counts: HashMap<&T, i32> = HashMap::new();
    for i in vec.iter() {
        let copy = i.clone();
        *counts.entry(copy).or_insert(0) += 1;
    }
    match counts.iter().max_by_key(|&(key, value)| value) {
        Some(entry) => Some(entry.0),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use crate::find_mode_generic;

    #[test]
    fn find_mode_chars() {
        let chars = vec!['a', 'b', 'c', 'b'];
        let result = find_mode_generic(&chars);

        assert!(result.is_some());
        assert_eq!('b', *result.unwrap());
    }

}