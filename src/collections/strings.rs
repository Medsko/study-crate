
pub fn string_stuff() {

    let mut s = String::from("This is the start");
    s.push_str(" of something");
    println!("{}", s);

    let s1 = "Hello ".to_string();
    let s2 = "World!".to_string();
    let s3 = s1 + &s2;

    println!("{}", s3);
    // println!("{}", s1);  // Will not compile: s1 was moved when creating s3 (could (NOT) have been
    // prevented by using reference (NOT, because + operator is overloaded to implement String.add()
    // - but only for String type, not for str slice)).
    // ...so maybe let s1 = s1 + &s2 would make more sense...yes it would. It also runs as expected.

    let s = got_game();
    println!("I feel I already formatted this, but anyways: {}", s);
}

fn got_game() -> String {
    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";
    format!("{}-{}-{}", s1, s2, s3)
}

//  Convert strings to pig latin. The first consonant of each word is moved to the end
//  of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a
//  vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind
//  the details about UTF-8 encoding!
pub fn to_pig_latin(kosher: &String) -> String {
    let trimmed = kosher.trim();
    let mut chars = trimmed.chars();
    let suffix: String;
    let mut result = String::new();

    let first_char = chars.next().unwrap();

    if is_vowel(&first_char) {
        suffix = "hay".to_string();
        result.push(first_char);
    } else {
        suffix = first_char.to_string() + "ay";
    }
    chars.for_each(|ch| result.push(ch));
    result.push_str(&suffix);

    result
}

fn is_vowel(maybe_vowel: &char) -> bool {
    vec!['a', 'e', 'i', 'o', 'u'].contains(maybe_vowel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pig_latin_identifies_pseudo_vowel() {
        let input = "äsdfasoesadf".to_string();

        let result = to_pig_latin(&input);

        assert_eq!("sdfasoesadfäay", result);
    }

    #[test]
    fn to_pig_latin_identifies_vowel() {
        let input = "asdfasoesadf".to_string();

        let result = to_pig_latin(&input);

        assert_eq!("asdfasoesadfhay", result);
    }

    #[test]
    fn to_pig_latin_identifies_consonant() {
        let input = "nx".to_string();

        let result = to_pig_latin(&input);

        assert_eq!("xnay", result);
    }

}