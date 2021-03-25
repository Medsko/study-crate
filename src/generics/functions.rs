use std::cmp::PartialOrd;

pub fn largest_i32(list: &[i32]) -> &i32 {
    if list.is_empty() {
        return &0;
    }
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_char(list: &[char]) -> &char {
    if list.is_empty() {
        return &' ';
    }
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Returns a copy of the largest value in given list.
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Returns a pointer to the largest value in given list.
pub fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn find_largests() {
    let number_list = vec![48, 234, 39,5];

    let result = largest_i32(&number_list);
    println!("Largest number: {}", result);

    let char_list = vec!['a', 'x', 'm'];

    let result = largest_char(&char_list);
    println!("Largest char: {}", result);

    println!("Largest number using generics: {}", largest(&number_list));
    println!("Largest char using generics: {}", largest(&char_list));
}