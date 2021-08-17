//! # Study Crate
//! `study_crate` is just me working through the Rust book.

use std::collections::HashMap;
use std::hash::Hash;

use collections::vectors;

pub use self::prepackages::enums::enums::Message;

mod collections;
mod prepackages;

// This file is only here to make some typical integration tests

pub fn find_median(vec: &Vec<i32>) -> i32 {
    return vectors::find_median(vec);
}

pub fn find_mode(vec: &Vec<i32>) -> Option<i32> {
    return vectors::find_mode(vec);
}

/// Finds the mode (item that occurs the most times) in the given vector, in a generic way.
///
/// # Examples
/// ```
///  let chars = vec!['a', 'b', 'c', 'b'];
///  let result = study_crate::find_mode_generic(&chars);
///  assert!(result.is_some());
///  assert_eq!('b', *result.unwrap());
/// ```
/// ```
///  let ints = vec![5, 8, 9, 5];
///  let result = study_crate::find_mode_generic(&ints);
///  assert!(result.is_some());
///  assert_eq!(5, *result.unwrap());
/// ```
pub fn find_mode_generic<T: Eq + Hash>(vec: &Vec<T>) -> Option<&T> {
    let mut counts: HashMap<&T, i32> = HashMap::new();
    for i in vec.iter() {
        let copy = i.clone();
        *counts.entry(copy).or_insert(0) += 1;
    }
    match counts.iter().max_by_key(|&(_key, value)| value) {
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