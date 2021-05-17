
pub fn own() {

    let s = "hello";

    println!("{} world!", s);

    let s: String = s.into();

    println!("{} world!", s);
    println!("That didn't look any different, did it? But now the initial string literal has been \
        converted into an instance of type String! Also, apparently this is how a multi-line \
        string is defined in Rust...");

    let s = String::from("Another");

    println!("{} way of instantiating a new String", s);

    // This invalidates the 's' variable, as only one variable can *own* any value at any time.
    // This operation is called a 'move' (value of variable 's' has been 'moved' to variable 's2').
    let s2 = s;

    // println!("{} variable has now taken over ownership over value 'Another', so this line won't work...", s);

    println!("{} variable is now pointing to the value 'Another'", s2);

    let s3 = s2.clone();
    println!("And {} one, and {} one!", s2, s3);

    takes_ownership(s2);

    // println!("{} value has been moved into the function takes_ownership(), so the variable is now invalid.", s2);
    // ...so a non-primitive value can be passed to a function only once? That will get annoying really quickly -
    // maybe pass it a reference instead

    let primitive_value = 3;
    makes_copy(primitive_value);

    // let s = s2;  // Forbidden, for some reason...only one move per value allowed?

    // See this function body for some more stuff about ownership
    references_and_borrowing();

    slice_stuff();
}

fn takes_ownership(val: String) {
    println!("{} will now go out of scope, which means its value will be dropped from memory.", val);
}

fn makes_copy(number: i32) {
    println!("Value {} will now leave scope, but it was a copy from the value passed to this function to begin with.", number);
}

/// Sure, this should be in another file...but defining and importing your own namespaces has not been mentioned yet in the book.
fn references_and_borrowing() {

    let stringy = "Some string".to_string();
    println!("Size of string '{}' in memory is: {}", stringy, calculate_length(&stringy));

    let complicated_string = "Söme string".to_string();
    print_length(&complicated_string);

    // Might as well shadow the variable, as the previous complicated_string has become invalid by
    // moving ownership of the value to the function.
    let complicated_string = string_to_length(complicated_string);
    println!("Length {} is now all that is left of that complicated string variable...", complicated_string);

    let yet_another_string = "Hi".to_string();
    // legalized_modification(&mut yet_another_string); // Can't borrow an immutable variable as mutable
    let mut yet_another_string = "Hi".to_string();
    legalized_modification(&mut yet_another_string);    // Now we're cooking

    // These two can co-exist without issue
    let immutable_pointer = &yet_another_string;
    let another_immutable_pointer = &yet_another_string;
    // However, this one will fuck shit up:
    let pointer = &mut yet_another_string;
    // let another_pointer = &mut yet_another_string;   // Only one mutable reference to value allowed at a time

    // Still, all above stuff will compile as long as the naughty variables aren't actually used/referenced
    // println!("First pointer: {}, second pointer: {}", pointer, immutable_pointer);

}

/// Returns the size of the given string in bytes of memory.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn string_to_length(s: String) -> usize {
    s.len()
}

fn print_length(s: &String) {
    println!("Size of string '{}' in memory is: {}", s, calculate_length(s));
}

// fn illegal_modification(s: &String) {
//     s.push_str("This borrowed string isn't yours to modify!");
// }

fn legalized_modification(s: &mut String) {
    s.push_str(", but this borrowed string CAN be modified (cuz it's marked mutable)!");
}


fn slice_stuff() {

    let s = String::from("Hello World!");
    let splice_first_four_chars = &s[0..4];
    let splice_first_four_chars_also = &s[..4];

    let splice_chars_from_fourth = &s[4..];


}

/// Returns a slice of the given string up to the first whitespace, or the entire string, if it
/// contains no whitespace.
fn slice_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // First whitespace! Return a slice of everything up to this index.
            return &s[..i];
        }
    }
    // No whitespaces, so return a slice that encompasses the entire given string.
    &s[..]
}


/// Returns the index of the first whitespace in the given string, or the entire string, if it
/// contains no whitespace.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    s.len()
}

#[cfg(test)]
mod tests {
    use super::slice_first_word;

    #[test]
    fn slice_first_word_returns_first() {
        let normal_sentence = "I'd love to jiggle your toes".to_string();
        let sliced_first_word = slice_first_word(&normal_sentence);

        assert_eq!(sliced_first_word, "I'd")
    }

    #[test]
    fn slice_first_word_returns_entire_word() {
        let normal_sentence = "Supercalifraetc".to_string();
        let sliced_first_word = slice_first_word(&normal_sentence);

        assert_eq!(sliced_first_word, "Supercalifraetc")
    }

}