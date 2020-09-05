use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100! Passed value: {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn play_the_guessing_game() {

    println!("Welcome to Guess...the...NUMBER!!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess a number: ");

    loop {

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line!");

        let guess: Guess = match guess_input.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("{} is not a number!", guess_input);
                continue;
            },
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
