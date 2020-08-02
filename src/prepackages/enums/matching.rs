
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Oregon,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
}

pub fn match_stuff() {
    let mut arr_declaration: [Option<i32>; 3] = [None; 3];
    arr_declaration[2] = Some(1);

    find_someone(&arr_declaration);

}

fn find_someone(optionals: &[Option<i32>]) {
    for opt_int in optionals.iter() {
        if let Some(1) = opt_int {
            println!("You found someone!");
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num + 1),
    }
}
