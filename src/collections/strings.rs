
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