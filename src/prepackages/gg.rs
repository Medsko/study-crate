use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn play_the_guessing_game() {

    println!("Welcome to Guess...the...NUMBER!!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess a number: ");

    loop {

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number!", guess);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
