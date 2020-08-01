use rand::Rng;

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

pub fn find_mean(vec: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in vec.iter() {
        total += i;
    }
    total / vec.len() as i32
}

pub fn find_floating_mean(vec: &Vec<i32>) -> f64 {
    let total = vec.iter().fold(0, |acc, x| acc + x);
    total as f64 / vec.len() as f64
}

pub fn generate_random_vec(lower_bound: i32, upper_bound: i32, size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut random_vec = Vec::with_capacity(size);
    for _ in 0..size {
        random_vec.push(rng.gen_range(lower_bound, upper_bound));
    }
    random_vec
}