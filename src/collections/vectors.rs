
pub fn do_vector_stuff() {

    let _first_vector: Vec<i32> = Vec::new();

    let macro_initialized_vector = vec![1,2,3];

    let third: &i32 = &macro_initialized_vector[2];
    println!("Third element is {}", third);

    match macro_initialized_vector.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element!"),
    }

    // let not_there = &macro_initialized_vector[100];  // PANIC!!!

    match macro_initialized_vector.get(100) {   // No panic, just a None (get it? Just a - None?).
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element!"),
    }

    let mut mutable_vector = Vec::new();
    mutable_vector.push(100);
    mutable_vector.push(32);
    mutable_vector.push(57);

    print_vector(&macro_initialized_vector);

    for i in &mut mutable_vector {
        // Dereference each item to modify it in place.
        *i += 50;
    }

    print_vector(&mutable_vector);
    println!("{:?}", &mutable_vector);

}

fn print_vector(vec: &Vec<i32>) {
    for i in vec {
        print!("{}, ", i);
    }
    println!();
}