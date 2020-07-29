use std::collections::HashMap;

pub fn hash_it_up() {

    let mut money_in_the_bank = HashMap::new();
    money_in_the_bank.insert(String::from("ING"), 11000.00);
    money_in_the_bank.insert(String::from("ASN"), 1500.00);

    match money_in_the_bank.get("ING") {
        Some(saldo) => println!("You got {} moneys in this bank!", saldo),
        None => println!("You ain't got doodley-squat in this bank! Hurrah for Kurt Vonnegut.")
    }

    let legs = vec![4, 3, 177];
    let animals = vec![String::from("Table"), String::from("Senior human"), String::from("Centipede (non-human)")];
    let mut legs_per_animal: HashMap<_,_> = animals.into_iter().zip(legs.into_iter()).collect();

    for (key, value) in &legs_per_animal {   // Taking ownership? Better use a reference, or match block below won't compile!
        println!("A {} has {} legs!", key, value);
    }

    match legs_per_animal.get("Table") {
        Some(nr_of_legs) => println!("A table has {} legs!", nr_of_legs),
        None => println!("I don't know that animal.")
    }

    legs_per_animal.entry(String::from("Melle")).or_insert(2);  // This one will insert a new key/value pair...
    legs_per_animal.entry(String::from("Table")).or_insert(15); // ...but this one will not (as key is already present)
    println!("Of course, HashMap derives fmt::Debug: {:?}", legs_per_animal);

    let text = "hello world wonderful world";
    let mut count_per_word = HashMap::new();

    let text = text.replace(",", "");

    for word in text.split_whitespace() {
        let count = count_per_word.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", count_per_word);
}