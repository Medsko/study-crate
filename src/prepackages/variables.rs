
pub fn variate() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let character: char = 'x';

    println!("The value of {} is: {}", character, x);

    let typical_tuple = (500, 6.4, 'x');

    let (a, b, c) = typical_tuple;

    println!("Destructured tuple: {}, {}, {}", a, b, c);

    let typical_array = [1,2,3,4];
    println!("Length of array: {}", typical_array.len());

    let easy_initializing_array = [1; 15];
    println!("Array initialized with {} items of value {}", easy_initializing_array.len(), easy_initializing_array[0]);

    println!("Result of function: {}", function_stuff());

    fizz_buzz(15);
    let je_moeder: bool = true; // Nee, JOUW moeder heeft een typo

    let sexy_inline_conditional_assignment = if je_moeder { 3 } else { 4 };
    fizz_buzz(sexy_inline_conditional_assignment);

    println!("Result of loop function is: {}", loop_return(sexy_inline_conditional_assignment));

    countdown(10);

    for_loop(1);

    range_for_loop(4);

    reverse_range_for_loop(8);

}

fn function_stuff() -> i32 {

    let x = 5;  // semi-colon, therefore, a statement - meaning it does not return a value

    let y = {
        let x: i32 = 3;
        x + returns_one()   // No semi-colon, therefore, an expression - meaning it returns a value
    };  // semi-colon, therefore, statement
    // Implicit return
    y + x
}

fn returns_one() -> i32 {
    1   // No semi-colon (following last expression) means implicit return value
}

fn fizz_buzz(x: i32) {
    let mut to_print: String = "".to_string();
    if x % 3 == 0 {
        to_print.push_str("fizz");
    }
    if x % 5 == 0 {
        to_print.push_str("buzz");
    }
    println!("{}: {}", x, to_print);
}

fn loop_return(x: i32) -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break counter * x;
        }
    }
}

fn countdown(loops: i32) {
    let mut from = loops;
    while from != 0 {
        println!("{}!", from);
        from -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop(placement: i32) {
    let array = [placement; 3];

    for element in array.iter() {
        println!("Nederland op nummer {}!", element);
    }
}

fn range_for_loop(until: i32) {
    for element in 0..until {
        println!("Range includes: {}", element);
    }
}

fn reverse_range_for_loop(from: i32) {
    for element in (0..from).rev() {
        println!("Reverse range includes: {}", element);
    }
}