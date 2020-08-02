
pub fn options() {
    let some_number = Some(3);
    let some_other_number = Some(42);
    let no_number: Option<i32> = None;

    if no_number.is_some() {
        println!("{} is a number!", no_number.unwrap());
    }

    if some_number.is_some() && some_other_number.is_some() {
        let total = some_number.unwrap() + some_other_number.unwrap();
        println!("Sum bitches: {}", total);
    }

}